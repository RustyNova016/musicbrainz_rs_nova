use musicbrainz_rs_nova::entity::release_group::*;
use musicbrainz_rs_nova::Search;

#[test]
fn should_search_artist() {
    let query = ReleaseGroupSearchQuery::query_builder()
        .release("Tonight")
        .build();

    let result = ReleaseGroup::search(query).execute().unwrap();

    assert!(!result.entities.is_empty());
}
