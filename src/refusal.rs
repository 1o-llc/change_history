// Copyright © 2026 1o. All rights reserved.

//! Every refusal `change_history` gives, declared once as liaise codes.

use liaise::{Liaise, LiaiseCodes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, LiaiseCodes)]
#[liaise(prefix = "CHNG", domain = "change_history")]
pub enum Refusal {
    #[liaise(code = 1, msg = "a count reading the job took cannot be read back")]
    Unreadable,
    #[liaise(code = 2, msg = "compare takes two count readings")]
    NotCounts,
    #[liaise(code = 3, msg = "a closing receipt or count diff this needs is missing")]
    MissingInput,
    #[liaise(code = 4, msg = "a file the job keeps cannot be written")]
    Write,
}

impl Refusal {
    /// The error this refusal raises, with what was refused.
    pub fn refuse(self, detail: impl std::fmt::Display) -> Error {
        Error { refusal: self, detail: detail.to_string() }
    }
}

/// A refusal and what was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub refusal: Refusal,
    pub detail: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.refusal.code(), self.refusal.message(), self.detail)
    }
}

impl std::error::Error for Error {}

impl From<receipt_history::Error> for Error {
    fn from(e: receipt_history::Error) -> Self {
        Refusal::Write.refuse(e)
    }
}

impl From<revision_history::Error> for Error {
    fn from(e: revision_history::Error) -> Self {
        Refusal::Write.refuse(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
