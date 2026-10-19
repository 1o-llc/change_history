// Copyright © 2026 1o. All rights reserved.

//! Lines touched, from a job's counts; change records, from two counts.

use crate::refusal::{Refusal, Result};
use revision_history::TaskRef;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use task_history::DiffDocument;
use tree_history::count::{compare_counts, load_count, within, CountDiff, CountReport, Moved, Within};

/// One count a job took: when (`open`, `step 3 apply`, `close`), of which route, its reading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Taken {
    pub label: String,
    pub route: String,
    /// The count reading's JSON, relative to the job folder.
    pub reading: String,
}

/// Lines touched between two counts of a route, as the closing receipt lists them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Touched {
    /// `job` (open to close) or the step (`step 3 apply`).
    pub label: String,
    pub route: String,
    pub before_digest: String,
    pub after_digest: String,
    pub diff_digest: String,
    pub moved: Moved,
    pub files: usize,
    pub crates: usize,
    /// `step 3 apply: +412 −38 across 6 files in 2 crates`.
    pub line: String,
    /// The count diff's file, relative to the job folder.
    pub diff: String,
}

/// The counts file in a job folder: every count the job took, in order.
pub const COUNTS_FILE: &str = "data/counts.json";

/// A job folder's own data (counts, count diffs) sits under this sub-folder,
/// so it is never listed as a spec.
pub const JOB_DATA: &str = "data";

/// A name as a job's names take it: lowercase letters and digits, every other run of characters one `-`.
pub fn slug(s: &str) -> String {
    let t: String = s.chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' }).collect();
    t.split('-').filter(|p| !p.is_empty()).collect::<Vec<_>>().join("-")
}

/// The counts a job folder's counts file lists, in order (none when it has none).
pub fn taken(folder: &Path) -> Vec<Taken> {
    std::fs::read_to_string(folder.join(COUNTS_FILE)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn count_at(folder: &Path, t: &Taken) -> Result<CountReport> {
    let p = folder.join(&t.reading);
    if !p.is_file() {
        return Err(Refusal::Unreadable.refuse(p.display()));
    }
    load_count(&p).map(|(c, _, _)| c).ok_or_else(|| Refusal::NotCounts.refuse(format!("{} ({})", t.reading, t.label)))
}

/// Lines touched from a job's counts: per route, each count against the one
/// before it (a step's own delta), and `job` from the first to the last. The
/// count diffs are written beside them under `data/counts/`.
pub fn lines_touched_of(folder: &Path, counts: &[Taken]) -> Result<Vec<Touched>> {
    let mut out = Vec::new();
    let mut routes: Vec<&String> = counts.iter().map(|t| &t.route).collect();
    routes.dedup();
    let mut seen = BTreeSet::new();
    for route in routes.into_iter().filter(|r| seen.insert(r.to_string())) {
        let of: Vec<&Taken> = counts.iter().filter(|t| &t.route == route).collect();
        let mut reports = Vec::new();
        for t in &of {
            reports.push((t.label.clone(), count_at(folder, t)?));
        }
        let mut pairs: Vec<(String, usize, usize)> = (1..reports.len()).filter(|&i| reports[i].0 != "close").map(|i| (reports[i].0.clone(), i - 1, i)).collect();
        if reports.len() >= 2 {
            pairs.push(("job".to_string(), 0, reports.len() - 1));
        }
        for (label, a, b) in pairs {
            let d = compare_counts(&reports[a].1, &reports[b].1);
            let file = folder.join(JOB_DATA).join("counts").join(format!("{}-{}.diff.json", slug(&label), if route == "*" { "workspace".to_string() } else { slug(route) }));
            receipt_history::write_whole(&file, (serde_json::to_string_pretty(&d).map_err(|e| Refusal::Write.refuse(e))? + "\n").as_bytes())?;
            out.push(Touched {
                line: d.line(&label),
                label,
                route: route.clone(),
                before_digest: d.before.digest.clone(),
                after_digest: d.after.digest.clone(),
                diff_digest: d.digest.clone(),
                moved: d.moved,
                files: d.files().len(),
                crates: d.crates.len(),
                diff: file.strip_prefix(folder).unwrap_or(&file).to_string_lossy().into_owned(),
            });
        }
    }
    Ok(out)
}

/// What a closing receipt (`closing.json`) says about a job's lines; its other fields are the job's own.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobLines {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub closed_utc: String,
    #[serde(default)]
    pub result: String,
    #[serde(default)]
    pub lines_touched: Vec<Touched>,
}

/// A closed job's lines, from the closing receipt in its folder.
pub fn job_lines(folder: &Path) -> Result<JobLines> {
    let closing = folder.join("closing.json");
    let text = std::fs::read_to_string(&closing).map_err(|e| Refusal::MissingInput.refuse(format!("{}: {e}", closing.display())))?;
    serde_json::from_str(&text).map_err(|e| Refusal::MissingInput.refuse(format!("{}: {e}", closing.display())))
}

/// What a set of closed jobs (their folders) changed together, from their
/// closing receipts' `job` lines touched.
pub fn lines_within(folders: &[&Path]) -> Result<Within> {
    let mut diffs = Vec::new();
    for folder in folders {
        let c = job_lines(folder)?;
        for t in c.lines_touched.iter().filter(|t| t.label == "job") {
            let text = std::fs::read_to_string(folder.join(&t.diff)).map_err(|e| Refusal::MissingInput.refuse(format!("{}: {e}", t.diff)))?;
            let d: CountDiff = serde_json::from_str(&text).map_err(|e| Refusal::MissingInput.refuse(format!("{}: {e}", t.diff)))?;
            diffs.push(d);
        }
    }
    Ok(within(diffs.iter()))
}

/// The files that differ between two counts of the tree (by their digests),
/// as change records written into the job's revision: the digest before from
/// `before`, after from the file as it is under `tree_root`.
pub fn changes(t: &TaskRef, tree_root: &Path, before: &CountReport, after: &CountReport) -> Result<Vec<DiffDocument>> {
    let digests = |c: &CountReport| -> BTreeMap<String, String> { c.crates.iter().flat_map(|k| k.file_counts.iter()).filter(|f| !f.path.contains('#')).map(|f| (f.path.clone(), f.digest.clone())).collect() };
    let (b, a) = (digests(before), digests(after));
    let mut paths: Vec<&String> = b.keys().chain(a.keys()).collect();
    paths.sort();
    paths.dedup();
    let mut out = Vec::new();
    for p in paths {
        if b.get(p) == a.get(p) {
            continue;
        }
        let before_digest = b.get(p).cloned().unwrap_or_else(|| "none".to_string());
        out.push(t.write(task_history::change_record(tree_root, p, &before_digest))?);
    }
    Ok(out)
}

/// Jobs side by side: one column per job, its result and its lines touched
/// for the job; then each job's steps.
pub fn render_jobs(jobs: &[JobLines]) -> String {
    let mut s = String::from("# changes\n\n|  |");
    for j in jobs {
        s.push_str(&format!(" {} |", j.id));
    }
    s.push_str("\n|---|");
    s.push_str(&"---|".repeat(jobs.len()));
    s.push('\n');
    let job_of = |j: &JobLines| j.lines_touched.iter().filter(|t| t.label == "job").cloned().collect::<Vec<_>>();
    let row = |s: &mut String, what: &str, f: &dyn Fn(&JobLines) -> String| {
        s.push_str(&format!("| {what} |"));
        for j in jobs {
            s.push_str(&format!(" {} |", f(j)));
        }
        s.push('\n');
    };
    row(&mut s, "name", &|j| j.name.clone());
    row(&mut s, "closed", &|j| j.closed_utc.clone());
    row(&mut s, "result", &|j| if j.result.is_empty() { "-".into() } else { j.result.clone() });
    row(&mut s, "lines +", &|j| job_of(j).iter().map(|t| t.moved.added.lines).sum::<usize>().to_string());
    row(&mut s, "lines −", &|j| job_of(j).iter().map(|t| t.moved.removed.lines).sum::<usize>().to_string());
    row(&mut s, "files", &|j| job_of(j).iter().map(|t| t.files).sum::<usize>().to_string());
    row(&mut s, "crates", &|j| job_of(j).iter().map(|t| t.crates).sum::<usize>().to_string());
    row(&mut s, "steps counted", &|j| j.lines_touched.iter().filter(|t| t.label != "job").count().to_string());
    for j in jobs {
        s.push_str(&format!("\n## {} ({})\n\n", j.id, j.name));
        if j.lines_touched.is_empty() {
            s.push_str("- the job took no counts: no lines touched are known\n");
        }
        for t in &j.lines_touched {
            s.push_str(&format!("- {} ({}), diff `{}`\n", t.line, t.route, t.diff));
        }
    }
    s
}
