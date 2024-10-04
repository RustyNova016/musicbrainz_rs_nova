use crate::entity::release::Release;
use crate::entity::{Include, Relationship, Subquery};
use serde::{Deserialize, Serialize};

/// Disc ID is the code number which MusicBrainz uses to link a physical CD to a release listing.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Discid {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub offset_count: u32,
    pub sectors: u32,
    pub offsets: Vec<u32>,
    pub releases: Option<Vec<Release>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Disc {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub offset_count: u32,
    pub sectors: u32,
    pub offsets: Vec<u32>,
}

impl_includes!(
    Discid,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_labels, Include::Subquery(Subquery::Labels)),
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_work_relations,
        Include::Relationship(Relationship::Work)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (
        with_work_level_relations,
        Include::Relationship(Relationship::WorkLevel)
    ),
    (
        with_recording_level_relations,
        Include::Relationship(Relationship::RecordingLevel)
    ),
    (with_recordings, Include::Subquery(Subquery::Recordings)),
    (
        with_release_groups,
        Include::Subquery(Subquery::ReleaseGroups)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations)),
    (
        with_artist_credits,
        Include::Subquery(Subquery::ArtistCredits)
    )
);
