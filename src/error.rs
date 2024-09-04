use thiserror::Error;

use crate::entity::api::MusicbrainzError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Musicbrainz returned an unknown error")]
    MusicbrainzError(MusicbrainzError),

    #[error("Musicbrainz returned \"Not found\" for querry \"{0}\"")]
    NotFound(String),
}
