use musicbrainz_rs_nova::entity::artist::*;
use musicbrainz_rs_nova::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute()
        .await;

    assert_eq!(nirvana.unwrap().name, "Nirvana".to_string());
}
