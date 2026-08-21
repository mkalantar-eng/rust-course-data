use error_stack::{Report, Result};

use track::errors::TrackError;
use track::init;

fn main() -> Result<(), TrackError> {
    init::error_reporting();

    return Err(Report::from(TrackError));
    Ok(())
}