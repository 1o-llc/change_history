// Copyright © 2026 1o. All rights reserved.

//! `change_history`: what each job changed.
//!
//! A job that counts takes a count of its routes at open, after each step
//! that changes code, and at close ([`Taken`], kept in the job folder's
//! counts file). Its lines touched ([`Touched`]) are read from those counts:
//! per route, each count against the one before it (a step's own delta), and
//! `job` from the first to the last, each count diff written beside them.
//! [`lines_within`] sums what a set of jobs touched; [`changes`] writes the
//! files two counts differ in as change records into the job's revision;
//! [`job_lines`] reads a closed job's lines back and [`render_jobs`] puts
//! jobs side by side.

pub mod lines;
pub mod refusal;

pub use lines::{changes, job_lines, lines_touched_of, lines_within, render_jobs, slug, taken, JobLines, Taken, Touched, COUNTS_FILE, JOB_DATA};
pub use refusal::{Error, Refusal, Result};
