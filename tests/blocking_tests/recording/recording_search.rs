use musicbrainz_rs_nova::entity::recording::*;
use musicbrainz_rs_nova::Search;

#[test]
fn should_search_recording() {
    let query = RecordingSearchQuery::query_builder()
        .recording("basket case")
        .and()
        .artist_name("green day")
        .and()
        .comment("leeds festival")
        .build();

    let result = Recording::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|recording| recording.length.unwrap() == 182000));
}
