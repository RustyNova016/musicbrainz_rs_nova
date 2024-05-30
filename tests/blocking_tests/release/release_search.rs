use musicbrainz_rs_nova::entity::release::*;
use musicbrainz_rs_nova::Search;

#[test]
fn should_search_artist() {
    let query = ReleaseSearchQuery::query_builder()
        .release("Drivers License")
        .build();

    let result = Release::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|release| release.title == "drivers license"));
}
