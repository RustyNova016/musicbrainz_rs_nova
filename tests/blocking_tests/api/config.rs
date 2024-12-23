use musicbrainz_rs_nova::client::MusicBrainzClient;
use musicbrainz_rs_nova::entity::artist::*;
use musicbrainz_rs_nova::prelude::*;

#[test]
fn set_user_agent_should_work() {
    let mut client = MusicBrainzClient::default();
    client.set_user_agent("musicbrainz_rs_testing/0.9").unwrap();

    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute_with_client(&client);

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
