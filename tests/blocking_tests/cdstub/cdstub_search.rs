use musicbrainz_rs_nova::entity::cdstub::*;
use musicbrainz_rs_nova::Search;

#[test]
fn should_search_cdstub() {
    let query = CDStubSearchQuery::query_builder().title("Dookie").build();

    let result = CDStub::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|cdstub| cdstub.artist == "Cleatus and Jimmy"));
}
