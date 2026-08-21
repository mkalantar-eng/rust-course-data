//! Top-level error types

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Time tracker: something went wrong")]
pub struct TrackError;

pub struct Suggestion(pub &'static str);
