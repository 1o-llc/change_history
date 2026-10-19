# Changelog

## 0.1.0 (2026-10-19)

The lines each job touched, as a crate of their own. custodian_kit read a
job's lines touched and its change records inside its job code; both are
moved here, and custodian_kit re-exports them at the same paths. Every link
is pinned to 388254f.

- `Taken` ([src/lines.rs#L15](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L15)), `Touched` ([src/lines.rs#L24](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L24)),
  `COUNTS_FILE` and `JOB_DATA` ([src/lines.rs#L41](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L41)) are moved from
  [custodian_kit src/jobs.rs#L208](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L208), [custodian_kit src/jobs.rs#L217](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L217), [custodian_kit src/jobs.rs#L234](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L234) and
  [custodian_kit src/jobs.rs#L238](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L238). `COUNTS_FILE` is public now.
- `slug` ([src/lines.rs#L48](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L48)) and `taken` ([src/lines.rs#L54](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L54)) are moved
  from [custodian_kit src/jobs.rs#L252](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L252) and [custodian_kit src/jobs.rs#L257](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L257). custodian_kit's job code
  calls them.
- `lines_touched_of` ([src/lines.rs#L69](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L69)) is moved from
  [custodian_kit src/jobs.rs#L302](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L302). Per route, it compares each count with the one before
  it, and the first with the last for `job`. It writes each count diff
  under `data/counts/`. It reads a count's record through tree_history's
  `load_count`, not as custodian_kit's whole reading. A record that holds no
  count is refused as `CHNG0002`, which custodian_kit reads as `CUST0021`,
  as before the move.
- `lines_within` ([src/lines.rs#L131](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L131)) sums the `job` lines touched of a
  set of closed jobs. It takes their folders and reads their closing
  receipts. custodian_kit's `jobs::lines_within` resolves the jobs through
  the journal and calls it. Moved from [custodian_kit src/jobs.rs#L340](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/jobs.rs#L340).
- `changes` ([src/lines.rs#L147](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L147)) writes the files two counts differ in as
  change records into the job's revision. Moved from
  [custodian_kit src/task_records.rs#L88](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/task_records.rs#L88).
- New: `job_lines` ([src/lines.rs#L123](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L123)) reads a closed job's id, name,
  result and lines touched from its closing receipt. `render_jobs`
  ([src/lines.rs#L166](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/src/lines.rs#L166)) puts jobs side by side.
- Proven by
  `a_job_s_counts_give_lines_touched_per_step_and_for_the_job_and_sum_across_jobs`
  ([tests/lines.rs#L33](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/tests/lines.rs#L33)), `a_reading_that_is_not_a_count_is_refused`
  ([tests/lines.rs#L64](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/tests/lines.rs#L64)) and
  `two_counts_become_change_records_in_the_job_s_revision`
  ([tests/lines.rs#L73](https://github.com/1o-llc/change_history/blob/388254f1a5ca124395809ce05ecb6d10478ef235/tests/lines.rs#L73)). custodian_kit's
  `a_jobs_counts_give_lines_touched_per_step_and_for_the_job_and_sum_across_jobs`
  ([custodian_kit tests/count.rs#L195](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/tests/count.rs#L195))
  passes through the re-exports.

Refusals are liaise codes `CHNG0001` to `CHNG0004`.

Proof: `cargo test`, 3 tests in tests/lines.rs.
