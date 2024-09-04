use serde::Deserialize;
use serde::Serialize;

use crate::Error;

/// Represent a result coming directly from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MusicbrainzResult<T> {
    Ok(T),
    Err(MusicbrainzError),
}

impl<T> MusicbrainzResult<T> {
    pub fn into_result(self, querry: String) -> Result<T, Error> {
        match self {
            Self::Ok(val) => Ok(val),
            Self::Err(err) => Err(err.into_error(querry)),
        }
    }
}

/// An error given by musicbrainz's API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MusicbrainzError {
    error: String,
    help: String,
}

impl MusicbrainzError {
    pub fn into_error(self, querry: String) -> Error {
        if self.is_not_found() {
            return Error::NotFound(querry);
        }

        Error::MusicbrainzError(self)
    }

    pub fn is_not_found(&self) -> bool {
        self.error == "Not Found"
    }
}
