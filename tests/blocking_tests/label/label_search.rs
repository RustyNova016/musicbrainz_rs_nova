use musicbrainz_rs_nova::entity::label::*;
use musicbrainz_rs_nova::Search;

#[test]
fn should_search_label() {
    let query = LabelSearchQuery::query_builder()
        .label("Abbey Road Studios")
        .build();

    let result = Label::search(query).execute().unwrap();

    assert!(result
        .entities
        .iter()
        .any(|label| label.label_type.as_ref().unwrap() == &LabelType::Production));
}
