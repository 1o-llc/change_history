# change_history

What each job changed.

A job that counts takes a count of its routes at open, after each step that changes code, and at close (`Taken`,
kept in the job folder's `data/counts.json`). Its lines touched (`Touched`) are read from those counts: per route,
each count against the one before it (a step's own delta), and `job` from the first to the last, each count diff
written beside them under `data/counts/` (`lines_touched_of`). `lines_within` sums what a set of closed jobs
touched, from their closing receipts; `changes` writes the files two counts differ in as change records (path,
operation, the digests before and after) into the job's revision; `job_lines` reads a closed job's lines back and
`render_jobs` puts jobs side by side.

```rust,no_run
# fn demo(folder: &std::path::Path) -> change_history::Result<()> {
let touched = change_history::lines_touched_of(folder, &change_history::taken(folder))?;
for t in &touched {
    println!("{} ({})", t.line, t.route);     // step 3 apply: +412 −38 across 6 files in 2 crates
}
# Ok(()) }
```

The lines touched came out of custodian_kit's `jobs`, the change records out of its `task_records`; custodian_kit
re-exports them there.

## Refusals

`CHNG0001` a count reading the job took cannot be read back; `CHNG0002` compare takes two count readings;
`CHNG0003` a closing receipt or count diff this needs is missing; `CHNG0004` a file the job keeps cannot be written.

## Releasing

Bump `version` in Cargo.toml, add a dated `## <version> (YYYY-MM-DD)` entry to CHANGELOG.md, and push to
`main`. CI (.github/workflows/ci.yml) runs the tests on stable and Rust 1.89 and clippy, then publishes that
version to crates.io through trusted publishing; a version already on crates.io publishes nothing.

## Ownership

Developed and published by James Hunt. The repository is held by the 1o organization
(https://github.com/1o-llc). Assignment of the copyright to 1o is pending documentation. The license texts
(LICENSE-MIT, LICENSE-APACHE) are left as they were found. Every file new in 0.1.0 carries the notice
Copyright © 2026 1o. All rights reserved.

## License

MIT OR Apache-2.0
