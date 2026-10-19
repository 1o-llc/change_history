// Copyright © 2026 1o. All rights reserved.

//! Lines touched from a job's counts, summed across jobs; change records from two counts.

use change_history::{changes, job_lines, lines_touched_of, lines_within, render_jobs, slug, taken, JobLines, Taken};
use std::path::{Path, PathBuf};
use tree_history::count::{CountReport, FileCount, Lines};

fn dir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("change-history-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn count(files: &[(&str, &str, usize, &str)]) -> CountReport {
    CountReport::from_files(
        "*".into(),
        None,
        None,
        files.iter().map(|(k, p, n, d)| (k.to_string(), FileCount { path: p.to_string(), language: "Rust".into(), lines: Lines { lines: *n, code: *n, comments: 0, blanks: 0 }, tests: Lines::default(), digest: d.to_string() })).collect(),
    )
}

/// A count reading's record as the kit writes it: the reading's fields, its count among them.
fn write_reading(folder: &Path, rel: &str, c: &CountReport) {
    let p = folder.join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    let record = serde_json::json!({ "schema": "custodian.reading.v1", "name": "count", "stamp": "20261019T120000Z", "target": "*", "count": c });
    std::fs::write(p, serde_json::to_string_pretty(&record).unwrap()).unwrap();
}

#[test]
fn a_job_s_counts_give_lines_touched_per_step_and_for_the_job_and_sum_across_jobs() {
    let folder = dir("job");
    write_reading(&folder, "readings/open.json", &count(&[("a", "a/src/lib.rs", 10, "d1")]));
    write_reading(&folder, "readings/step.json", &count(&[("a", "a/src/lib.rs", 14, "d2")]));
    write_reading(&folder, "readings/close.json", &count(&[("a", "a/src/lib.rs", 14, "d2"), ("b", "b/src/lib.rs", 3, "d3")]));
    let counts = vec![
        Taken { label: "open".into(), route: "*".into(), reading: "readings/open.json".into() },
        Taken { label: "step 3 apply".into(), route: "*".into(), reading: "readings/step.json".into() },
        Taken { label: "close".into(), route: "*".into(), reading: "readings/close.json".into() },
    ];
    std::fs::create_dir_all(folder.join("data")).unwrap();
    std::fs::write(folder.join(change_history::COUNTS_FILE), serde_json::to_string(&counts).unwrap()).unwrap();
    assert_eq!(taken(&folder), counts);
    let touched = lines_touched_of(&folder, &counts).unwrap();
    assert_eq!(touched.iter().map(|t| t.line.as_str()).collect::<Vec<_>>(), ["step 3 apply: +4 \u{2212}0 across 1 files in 1 crates", "job: +7 \u{2212}0 across 2 files in 2 crates"]);
    assert_eq!(touched[1].diff, "data/counts/job-workspace.diff.json");
    assert!(folder.join(&touched[1].diff).is_file());
    // the closing receipt carries them; jobs are summed from it
    let closing = JobLines { id: "j-1".into(), name: "batch".into(), kind: "specs".into(), closed_utc: "2026-10-19T12:00:00Z".into(), result: "accepted".into(), lines_touched: touched };
    std::fs::write(folder.join("closing.json"), serde_json::to_string(&closing).unwrap()).unwrap();
    let w = lines_within(&[folder.as_path(), folder.as_path()]).unwrap();
    assert_eq!((w.diffs, w.moved.added.lines, w.files.len(), w.crates.len()), (2, 14, 2, 2));
    let back = job_lines(&folder).unwrap();
    assert_eq!(back, closing);
    let page = render_jobs(&[back.clone(), back]);
    assert!(page.contains("| lines + | 7 | 7 |"), "{page}");
    assert!(page.contains("- job: +7 \u{2212}0 across 2 files in 2 crates (*), diff `data/counts/job-workspace.diff.json`"), "{page}");
    assert_eq!(slug("Step 3: Apply!"), "step-3-apply");
}

#[test]
fn a_reading_that_is_not_a_count_is_refused() {
    let folder = dir("not-count");
    std::fs::write(folder.join("r.json"), r#"{"name":"env","stamp":"x","target":"y"}"#).unwrap();
    let counts = vec![Taken { label: "open".into(), route: "*".into(), reading: "r.json".into() }, Taken { label: "close".into(), route: "*".into(), reading: "r.json".into() }];
    assert_eq!(lines_touched_of(&folder, &counts).unwrap_err().refusal, change_history::Refusal::NotCounts);
    assert_eq!(lines_touched_of(&folder, &[Taken { label: "open".into(), route: "*".into(), reading: "gone.json".into() }]).unwrap_err().refusal, change_history::Refusal::Unreadable);
}

#[test]
fn two_counts_become_change_records_in_the_job_s_revision() {
    let tree = dir("tree");
    let history = dir("history");
    std::fs::create_dir_all(tree.join("a/src")).unwrap();
    std::fs::write(tree.join("a/src/lib.rs"), "fn now() {}\n").unwrap();
    std::fs::write(tree.join("a/src/new.rs"), "fn new() {}\n").unwrap();
    let old = receipt_history::digest::of("fn before() {}\n");
    let before = count(&[("a", "a/src/lib.rs", 1, &old), ("a", "a/src/gone.rs", 1, &old)]);
    let after = count(&[("a", "a/src/lib.rs", 1, &receipt_history::digest::of("fn now() {}\n")), ("a", "a/src/new.rs", 1, &receipt_history::digest::of("fn new() {}\n"))]);
    let t = revision_history::open(&history, "batch", "specs", "j-1", "change a").unwrap();
    let records = changes(&t, &tree, &before, &after).unwrap();
    let ops: Vec<String> = records.iter().map(|d| format!("{} {}", d.one("operation").unwrap(), d.one("path").unwrap())).collect();
    assert_eq!(ops, ["remove a/src/gone.rs", "modify a/src/lib.rs", "add a/src/new.rs"]);
}
