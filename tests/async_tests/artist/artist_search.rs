use musicbrainz_rs_nova::entity::artist::*;
use musicbrainz_rs_nova::Search;

#[tokio::test]
async fn should_search_artist() {
    let query = ArtistSearchQuery::query_builder()
        .artist("Nirvana")
        .and()
        .artist_type("Group")
        .build();

    let result = Artist::search(query).execute().await.unwrap();

    assert!(!result.entities.is_empty());
}
