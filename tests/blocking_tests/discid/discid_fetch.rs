use musicbrainz_rs_nova::entity::discid::Discid;
use musicbrainz_rs_nova::Fetch;

#[test]
fn fetch_cd_id() {
    let discids = [
        // Clashing DiscId
        "lwHl8fGzJyLXQR33ug60E8jhf4k-",
        // Example disc id
        "XzPS7vW.HPHsYemQh0HBUGr8vuU-",
        // Part of multiple CD release
        "hUbE3HKkLSkkWDvf4WliXO9OLm4-",
    ];

    for discid in discids {
        assert!(
            Discid::fetch().id(discid).execute().is_ok(),
            "{} is err",
            discid
        );
    }

    assert!(
        Discid::fetch()
            .id("hUbE6HKkLSkkWDvf4WliXO9OLm4-")
            .execute()
            .is_err(),
        "The diskid does not exist"
    );
}
