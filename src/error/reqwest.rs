use crate::error::Retryable;
use thiserror::Error;

// reqwest::Error's fmt::Display appends the error descriptions of all sources.
// For this reason, it is not marked as #[source] here, so that we don't
// display the error descriptions of all sources repeatedly.
#[derive(Error, Debug)]
#[error("{}", .0)]
pub struct WrappedReqwestError(pub reqwest::Error);

impl Retryable for WrappedReqwestError {
    fn is_retryable(&self) -> bool {
        let err = &self.0;
        err.is_timeout()
    }
}
