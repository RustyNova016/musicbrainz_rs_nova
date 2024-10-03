use glob::glob;
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! write_test {
    ($output_filepath:expr, $glob_pattern:literal, $template_path:literal) => {
        let mut output_file = File::create($output_filepath).expect("failed to write test file");

        glob($glob_pattern)
            .expect("failed to read glob pattern")
            .map(|entry| entry.unwrap())
            .for_each(|path| {
                let components = path
                    .iter()
                    .skip(3)
                    .map(|component| component.to_str().unwrap())
                    .collect::<Vec<&str>>();
                let name = components.join("_").replace(".", "_").replace("-", "_");
                eprintln!("Writing setting test: {:?}", name);

                let type_name = match components[1] {
                    "annotation" => "musicbrainz_rs_nova::entity::annotation::Annotation",
                    "area" => "musicbrainz_rs_nova::entity::area::Area",
                    "artist" => "musicbrainz_rs_nova::entity::artist::Artist",
                    "cdstub" => "musicbrainz_rs_nova::entity::cdstub::CDStub",
                    "event" => "musicbrainz_rs_nova::entity::event::Event",
                    "genre" => "musicbrainz_rs_nova::entity::genre::Genre",
                    "instrument" => "musicbrainz_rs_nova::entity::instrument::Instrument",
                    "label" => "musicbrainz_rs_nova::entity::label::Label",
                    "place" => "musicbrainz_rs_nova::entity::place::Place",
                    "recording" => "musicbrainz_rs_nova::entity::recording::Recording",
                    "release" => "musicbrainz_rs_nova::entity::release::Release",
                    "release-group" => "musicbrainz_rs_nova::entity::release_group::ReleaseGroup",
                    "series" => "musicbrainz_rs_nova::entity::series::Series",
                    "tag" => "musicbrainz_rs_nova::entity::tag::Tag",
                    "url" => "musicbrainz_rs_nova::entity::url::Url",
                    "work" => "musicbrainz_rs_nova::entity::work::Work",
                    _ => unreachable!(),
                };

                let type_name = match components[0] {
                    "lookup" => Cow::from(type_name),
                    "browse" => Cow::from(format!(
                        "musicbrainz_rs_nova::entity::BrowseResult<{type_name}>"
                    )),
                    _ => unreachable!(),
                };

                writeln!(
                    output_file,
                    include_str!($template_path),
                    type_name = type_name,
                    name = name,
                    filepath = path.canonicalize().unwrap().to_str().unwrap(),
                )
                .expect("failed to write test file");
            });
    };
}

fn main() {
    // Make cargo rerun the build script if the data directory changes.
    println!("cargo:rerun-if-changed=tests/serde/data");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    eprintln!("Writing tests to: {:?}", out_dir);

    write_test!(
        out_dir.join("lookup.rs"),
        "tests/serde/data/lookup/*/*.json",
        "./tests/serde/roundtrip.rs.in"
    );

    write_test!(
        out_dir.join("browse.rs"),
        "tests/serde/data/browse/*/*.json",
        "./tests/serde/roundtrip.rs.in"
    );
}
