use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

use super::{Include, Relationship, Subquery};
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::discid::Disc;
use crate::entity::genre::Genre;
use crate::entity::label::LabelInfo;
use crate::entity::recording::Recording;
use crate::entity::relations::Relation;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

/// A MusicBrainz release represents the unique release (i.e. issuing) of a product on a specific
/// date with specific release information such as the country, label, barcode and packaging.
/// If you walk into a store and purchase an album or single, they are each represented in
/// MusicBrainz as one release.
///
/// Each release belongs to a release group and contains at least one medium (commonly referred to
/// as a disc when talking about a CD release). Each medium has a tracklist.
/// A medium is the actual physical medium that stores the audio content. This means that each CD
/// in a multi-disc release will be entered as separate mediums within the release, and that both
/// sides of a vinyl record or cassette will exist on one medium. Mediums have a format (e.g. CD,
/// DVD, vinyl, and cassette) and can optionally also have a title. Sometimes a medium can be a
/// side of a disc. For example, the two sides of a hybrid SACD (the CD side and the SACD side)
/// should be entered as two mediums.
/// Tracklists represent the set and ordering of tracks as listed on a liner, and the same tracklist
/// can appear on more than one release. For example, a boxset compilation that contains previously
/// released CDs would share the same tracklists as the separate releases.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Release {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The title of the release.
    pub title: String,

    #[serde(rename = "status-id")]
    pub status_id: Option<String>,

    /// The status describes how "official" a release is.
    pub status: Option<ReleaseStatus>,

    /// The date the release was issued.
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    pub date: Option<NaiveDate>,

    /// The country the release was issued in.
    pub country: Option<String>,

    ///  Data quality indicates how good the data for a release is. It is not a mark of how good or
    /// bad the music itself is - for that, use ratings.
    pub quality: Option<ReleaseQuality>,

    /// The barcode, if the release has one. The most common types found on releases are 12-digit
    /// UPCs and 13-digit EANs.
    pub barcode: Option<String>,

    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,

    #[serde(rename = "packaging-id")]
    pub packaging_id: Option<String>,

    /// The physical packaging that accompanies the release. See the
    /// [list of packaging](https://musicbrainz.org/doc/Release/Packaging) for more information.
    pub packaging: Option<ReleasePackaging>,

    pub relations: Option<Vec<Relation>>,
    /// The release group associated with this release.
    pub release_group: Option<ReleaseGroup>,
    /// Artist credits indicate who is the main credited artist (or artists) for releases, release
    /// groups, tracks and recordings, and how they are credited.
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub media: Option<Vec<Media>>,
    /// The label which issued the release. There may be more than one.
    pub label_info: Option<Vec<LabelInfo>>,
    pub tags: Option<Vec<Tag>>,
    /// Aliases are alternate names for a release.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,

    /// The [Amazon Standard Identification Number (ASIN)](https://musicbrainz.org/doc/ASIN) of the
    /// release.
    pub asin: Option<String>,

    /// The text representation on the release.
    pub text_representation: Option<ReleaseTextRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ReleaseTextRepresentation {
    /// The language a release's track list is written in. The possible values are taken from the ISO
    /// 639-3 standard.
    pub language: Option<Language>,
    /// The script used to write the release's track list. The possible values are taken from the
    /// ISO 15924 standard.
    pub script: Option<ReleaseScript>,
}

/// The script used to write the release's track list. The possible values are taken from the
/// [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) standard.
///
/// The values for this enum have been generated with the following command:
///
/// ```bash
/// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
///     sort | \
///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,\/\/\/ \1\n\2\,,'
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleaseScript {
    /// Arabic
    Arab,
    /// Armenian
    Armn,
    /// Bengali
    Beng,
    /// Braille
    Brai,
    /// Buginese
    Bugi,
    /// Canadian Syllabics
    Cans,
    /// Cherokee
    Cher,
    /// Coptic
    Copt,
    /// Cuneiform, Sumero-Akkadian
    Xsux,
    /// Cyrillic
    Cyrl,
    /// Devanagari
    Deva,
    /// Egyptian hieroglyphs
    Egyp,
    /// Ethiopic
    Ethi,
    /// Georgian
    Geor,
    /// Gothic
    Goth,
    /// Greek
    Grek,
    /// Gujarati
    Gujr,
    /// Gurmukhi
    Guru,
    /// Hangul
    Hang,
    /// Han (Hanzi, Kanji, Hanja)
    Hani,
    /// Han (Simplified variant)
    Hans,
    /// Han (Traditional variant)
    Hant,
    /// Hebrew
    Hebr,
    /// Hiragana
    Hira,
    /// Japanese syllabaries
    Hrkt,
    /// Japanese
    Jpan,
    /// Kannada
    Knda,
    /// Katakana
    Kana,
    /// Khmer
    Khmr,
    /// Korean
    Kore,
    /// Lao
    Laoo,
    /// Latin (also known as Roman or, incorrectly, "English")
    ///
    /// Latin is the most common script, and usually the correct choice. It is used
    /// for all Western European languages, and many others. It is also the most common script used for transliterations.
    Latn,
    /// Malayalam
    Mlym,
    /// Mathematical notation
    Zmth,
    /// [Multiple scripts]
    Qaaa,
    /// Myanmar
    Mymr,
    /// Old Turkic
    Orkh,
    /// Oriya
    Orya,
    /// Phags-pa
    Phag,
    /// Runic
    Runr,
    /// Sinhala
    Sinh,
    /// Symbols
    Zsym,
    /// Syriac
    Syrc,
    /// Tamil
    Taml,
    /// Telugu
    Telu,
    /// Thai
    Thai,
    /// Tibetan
    Tibt,
    /// Vai
    Vaii,
}

/// The language the release title and track titles are written in. The possible values are taken
/// from the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
///
/// The values for this enum have been generated with the following command:
///
/// ```bash
/// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
///     sort | \
///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,\/\/\/ \1\n\u\2\,,'
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Abkhazian
    Abk,
    /// Achinese
    Ace,
    /// Acoli
    Ach,
    /// Adangme
    Ada,
    /// Adyghe
    Ady,
    /// Afar
    Aar,
    /// Afrikaans
    Afr,
    /// Ainu
    Ain,
    /// Akan
    Aka,
    /// Akkadian
    Akk,
    /// Albanian
    Sqi,
    /// Algonquin
    Alq,
    /// Amharic
    Amh,
    /// Angika
    Anp,
    /// Arabic
    Ara,
    /// Aragonese
    Arg,
    /// Arapaho
    Arp,
    /// Ardhamāgadhī Prākrit
    Pka,
    /// Armenian
    Hye,
    /// Aromanian
    Rup,
    /// [Artificial (Other)]
    Qaa,
    /// Assamese
    Asm,
    /// Asturian
    Ast,
    /// Atikamekw
    Atj,
    /// Avaric
    Ava,
    /// Awadhi
    Awa,
    /// Aymara
    Aym,
    /// Azerbaijani
    Aze,
    /// Baeggu
    Bvd,
    /// Balinese
    Ban,
    /// Baluchi
    Bal,
    /// Bambara
    Bam,
    /// Basa
    Bas,
    /// Basque
    Eus,
    /// Bavarian
    Bar,
    /// Beja
    Bej,
    /// Belarusian
    Bel,
    /// Bemba
    Bem,
    /// Bengali
    Ben,
    /// Bhojpuri
    Bho,
    /// Bikol
    Bik,
    /// Bini
    Bin,
    /// Bislama
    Bis,
    /// Bosnian
    Bos,
    /// Braj
    Bra,
    /// Breton
    Bre,
    /// Buamu
    Box,
    /// Buginese
    Bug,
    /// Bulgarian
    Bul,
    /// Buriat
    Bua,
    /// Burmese
    Mya,
    /// Burushaski
    Bsk,
    /// Caddo
    Cad,
    /// Cajun French
    Frc,
    /// Catalan
    Cat,
    /// Cebuano
    Ceb,
    /// Celtiberian
    Xce,
    /// Central Okinawan
    Ryu,
    /// Central Yupik
    Esu,
    /// Chamorro
    Cha,
    /// Chechen
    Che,
    /// Cherokee
    Chr,
    /// Chichewa
    Nya,
    /// Chinese
    Zho,
    /// Church Slavic
    Chu,
    /// Chuvash
    Chv,
    /// Coptic
    Cop,
    /// Cornish
    Cor,
    /// Corsican
    Cos,
    /// Creek
    Mus,
    /// Cree
    Cre,
    /// Crimean Tatar
    Crh,
    /// Croatian
    Hrv,
    /// Czech
    Ces,
    /// Danish
    Dan,
    /// Delaware
    Del,
    /// Divehi
    Div,
    /// Duala
    Dua,
    /// Dutch, Middle (ca.1050-1350)
    Dum,
    /// Dutch
    Nld,
    /// Dzongkha
    Dzo,
    /// Eastern Arrernte
    Aer,
    /// Egyptian (Ancient)
    Egy,
    /// Elamite
    Elx,
    /// English, Middle (1100-1500)
    Enm,
    /// English, Old (ca.450-1100)
    Ang,
    /// English
    Eng,
    /// Erzya
    Myv,
    /// Esperanto
    Epo,
    /// Estonian
    Est,
    /// Ewe
    Ewe,
    /// Fang
    Fan,
    /// Fanti
    Fat,
    /// Faroese
    Fao,
    /// Fijian
    Fij,
    /// Filipino
    Fil,
    /// Finnish
    Fin,
    /// Fon
    Fon,
    /// French, Old (842-ca.1400)
    Fro,
    /// French
    Fra,
    /// Frisian, Eastern
    Frs,
    /// Frisian, Northern
    Frr,
    /// Frisian, Western
    Fry,
    /// Friulian
    Fur,
    /// Fulah
    Ful,
    /// Galician
    Glg,
    /// Ganda
    Lug,
    /// Garifuna
    Cab,
    /// Ga
    Gaa,
    /// Geez
    Gez,
    /// Georgian
    Kat,
    /// German, Low
    Nds,
    /// German, Middle High (ca.1050-1500)
    Gmh,
    /// German, Old High (ca.750-1050)
    Goh,
    /// German, Swiss
    Gsw,
    /// German
    Deu,
    /// Gondi
    Gon,
    /// Gothic
    Got,
    /// Greek, Ancient
    Grc,
    /// Greek
    Ell,
    /// Greenlandic
    Kal,
    /// Gronings
    Gos,
    /// Guadeloupean Creole French
    Gcf,
    /// Guarani
    Grn,
    /// Gujarati
    Guj,
    /// Gupapuyngu
    Guf,
    /// Guyanese Creole English
    Gyn,
    /// Haitian Creole
    Hat,
    /// Hausa
    Hau,
    /// Hawaiian
    Haw,
    /// Hebrew
    Heb,
    /// Herero
    Her,
    /// Hindi
    Hin,
    /// Hiri Motu
    Hmo,
    /// Hmong
    Hmn,
    /// Hungarian
    Hun,
    /// Icelandic
    Isl,
    /// Igbo
    Ibo,
    /// Iloko
    Ilo,
    /// Indonesian
    Ind,
    /// Ingrian
    Izh,
    /// Innu
    Moe,
    /// Inuktitut
    Iku,
    /// Irish
    Gle,
    /// Italian
    Ita,
    /// Jamaican Creole English
    Jam,
    /// Japanese
    Jpn,
    /// Javanese
    Jav,
    /// Jewish Babylonian Aramaic (ca. 200-1200 CE)
    Tmr,
    /// Kabardian
    Kbd,
    /// Kabuverdianu
    Kea,
    /// Kabyle
    Kab,
    /// Kalmyk
    Xal,
    /// Kannada
    Kan,
    /// Karachay-Balkar
    Krc,
    /// Karelian
    Krl,
    /// Kashmiri
    Kas,
    /// Kazakh
    Kaz,
    /// Khanty
    Kca,
    /// Khasi
    Kha,
    /// Khmer, Central
    Khm,
    /// Kikuyu
    Kik,
    /// Kimbundu
    Kmb,
    /// Kinyarwanda
    Kin,
    /// Kirghiz
    Kir,
    /// Klingon
    Tlh,
    /// Kölsch
    Ksh,
    /// Komi
    Kom,
    /// Kongo
    Kon,
    /// Konkani
    Kok,
    /// Korean
    Kor,
    /// Kunigami
    Xug,
    /// Kurdish
    Kur,
    /// Ladino
    Lad,
    /// Ladin
    Lld,
    /// Lakota
    Lkt,
    /// Lao
    Lao,
    /// Latin
    Lat,
    /// Latvian
    Lav,
    /// Laz
    Lzz,
    /// Limburgish
    Lim,
    /// Lingala
    Lin,
    /// Lithuanian
    Lit,
    /// Liv
    Liv,
    /// Lojban
    Jbo,
    /// Louisiana Creole French
    Lou,
    /// Luba-Katanga
    Lub,
    /// Luba-Lulua
    Lua,
    /// Luo
    Luo,
    /// Luxembourgish
    Ltz,
    /// Luyia
    Luy,
    /// Macedonian
    Mkd,
    /// Madurese
    Mad,
    /// Malagasy
    Mlg,
    /// Malayalam
    Mal,
    /// Malay
    Msa,
    /// Maltese
    Mlt,
    /// Manchu
    Mnc,
    /// Mandarin Chinese
    Cmn,
    /// Mandar
    Mdr,
    /// Mandingo
    Man,
    /// Mansi
    Mns,
    /// Manx
    Glv,
    /// Maori
    Mri,
    /// Mapudungun
    Arn,
    /// Marathi
    Mar,
    /// Mari
    Chm,
    /// Marwari
    Mwr,
    /// Mende
    Men,
    /// Mina (Cameroon)
    Hna,
    /// Min Nan Chinese
    Nan,
    /// Miyako
    Mvi,
    /// Mohawk
    Moh,
    /// Moksha
    Mdf,
    /// Mongolian
    Mon,
    /// Mongo
    Lol,
    /// Mossi
    Mos,
    /// [Multiple languages]
    Mul,
    /// Nauru
    Nau,
    /// Navajo
    Nav,
    /// Ndebele, North
    Nde,
    /// Ndebele, South
    Nbl,
    /// Ndonga
    Ndo,
    /// Neapolitan
    Nap,
    /// Nepal Bhasa
    New,
    /// Nepali
    Nep,
    /// Nhengatu
    Yrl,
    /// Nogai
    Nog,
    /// [No linguistic content]
    Zxx,
    /// Norn
    Nrn,
    /// Norse, Old
    Non,
    /// Norwegian Bokmål
    Nob,
    /// Norwegian Nynorsk
    Nno,
    /// Norwegian
    Nor,
    /// Nzima
    Nzi,
    /// Occitan
    Oci,
    /// Oriya
    Ori,
    /// Oromo
    Orm,
    /// Osage
    Osa,
    /// Pahlavi
    Pal,
    /// Papiamento
    Pap,
    /// Persian
    Fas,
    /// Pitjantjatjara
    Pjt,
    /// Pohnpeian
    Pon,
    /// Polish
    Pol,
    /// Portuguese
    Por,
    /// Provençal, Old (to 1500)
    Pro,
    /// Prussian
    Prg,
    /// Pulaar
    Fuc,
    /// Punjabi
    Pan,
    /// Pushto
    Pus,
    /// Puyuma
    Pyu,
    /// Quechua
    Que,
    /// Quenya
    Qya,
    /// Rajasthani
    Raj,
    /// Rapanui
    Rap,
    /// Rarotongan
    Rar,
    /// Réunion Creole French
    Rcf,
    /// Romanian
    Ron,
    /// Romansh
    Roh,
    /// Romany
    Rom,
    /// Rundi
    Run,
    /// Russian
    Rus,
    /// Rusyn
    Rue,
    /// Sami, Inari
    Smn,
    /// Sami, Lule
    Smj,
    /// Sami, Northern
    Sme,
    /// Sami, Skolt
    Sms,
    /// Sami, Southern
    Sma,
    /// Samoan
    Smo,
    /// Sango
    Sag,
    /// Sanskrit
    San,
    /// Santali
    Sat,
    /// Sardinian
    Srd,
    /// Scots
    Sco,
    /// Scottish Gaelic
    Gla,
    /// Sea Island Creole English
    Gul,
    /// Serbian
    Srp,
    /// Serer
    Srr,
    /// Shan
    Shn,
    /// Shona
    Sna,
    /// Sicilian
    Scn,
    /// Sindarin
    Sjn,
    /// Sindhi
    Snd,
    /// Sinhala
    Sin,
    /// Slovak
    Slk,
    /// Slovenian
    Slv,
    /// Somali
    Som,
    /// Soninke
    Snk,
    /// Sorbian, Upper
    Hsb,
    /// Sotho, Northern
    Nso,
    /// Sotho, Southern
    Sot,
    /// Southern Altai
    Alt,
    /// Spanish
    Spa,
    /// Sranan Tongo
    Srn,
    /// Sundanese
    Sun,
    /// Susu
    Sus,
    /// Svan
    Sva,
    /// Swahili
    Swa,
    /// Swati
    Ssw,
    /// Swedish
    Swe,
    /// Syriac
    Syr,
    /// Tagalog
    Tgl,
    /// Tahitian
    Tah,
    /// Tajik
    Tgk,
    /// Tamashek
    Tmh,
    /// Tamil
    Tam,
    /// Tatar
    Tat,
    /// Telugu
    Tel,
    /// Tetum
    Tet,
    /// Thai
    Tha,
    /// Tibetan
    Bod,
    /// Tigrinya
    Tir,
    /// Tokelau
    Tkl,
    /// Toki Pona
    Tok,
    /// Tok Pisin
    Tpi,
    /// Tonga (Tonga Islands)
    Ton,
    /// Tsonga
    Tso,
    /// Tswana
    Tsn,
    /// Turkish, Ottoman
    Ota,
    /// Turkish
    Tur,
    /// Turkmen
    Tuk,
    /// Tuvalu
    Tvl,
    /// Tuvinian
    Tyv,
    /// Twi
    Twi,
    /// Udmurt
    Udm,
    /// Uighur
    Uig,
    /// Ukrainian
    Ukr,
    /// Umbundu
    Umb,
    /// Ume Sami
    Sju,
    /// Urdu
    Urd,
    /// Uzbek
    Uzb,
    /// Vai
    Vai,
    /// Venda
    Ven,
    /// Veps
    Vep,
    /// Vietnamese
    Vie,
    /// Võro
    Vro,
    /// Votic
    Vot,
    /// Walloon
    Wln,
    /// Walser
    Wae,
    /// Warlpiri
    Wbp,
    /// Washo
    Was,
    /// Welsh
    Cym,
    /// Western Arrarnta
    Are,
    /// Wolaitta
    Wal,
    /// Wolof
    Wol,
    /// Wyandot
    Wya,
    /// Xhosa
    Xho,
    /// Yaeyama
    Rys,
    /// Yakut
    Sah,
    /// Yiddish
    Yid,
    /// Yoron
    Yox,
    /// Yoruba
    Yor,
    /// Yucateco
    Yua,
    /// Yue Chinese
    Yue,
    /// Zapotec
    Zap,
    /// Zarma
    Dje,
    /// Zaza
    Zza,
    /// Zulu
    Zul,
    /// Zuni
    Zun,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "lowercase"))]
pub enum ReleaseQuality {
    /// The release needs serious fixes, or its existence is hard to prove (but it's not clearly fake).
    Low,

    /// All available data has been added, if possible including cover art with liner info that
    /// proves it.
    High,

    /// This is the default setting - technically "unknown" if the quality has never been modified,
    /// "normal" if it has.
    Normal,

    Unknown,

    None,
}

/// The release status describes how "official" a release is.
/// Note that this enum is `non_exhaustive`; The list of release types is subject to change and
/// these changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `release_status` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleaseStatus {
    /// Any release officially sanctioned by the artist and/or their record company. Most releases
    /// will fit into this category.
    Official,
    /// A give-away release or a release intended to promote an upcoming official release (e.g.
    /// pre-release versions, releases included with a magazine, versions supplied to radio DJs
    /// for air-play).
    Promotion,
    /// An unofficial/underground release that was not sanctioned by the artist and/or the record
    /// company. This includes unofficial live recordings and pirated releases.
    Bootleg,
    /// An alternate version of a release where the titles have been changed. These don't correspond
    /// to any real release and should be linked to the original release using the transl(iter)ation
    /// [transl(iter)ation relationship](https://musicbrainz.org/relationship/fc399d47-23a7-4c28-bfcf-0607a562b644).
    #[serde(rename = "Pseudo-Release")]
    PseudoRelease,
    /// Any release_status that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ReleaseStatus::UnrecognizedReleaseStatus` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedReleaseStatus,
}

/// The type of packaging of a MusicBrainz release entity.
/// Note that this enum is `non_exhaustive`; The list of release types is subject to change and
/// these changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `release_packaging` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleasePackaging {
    Book,
    Box,
    #[serde(rename = "Cardboard/Paper Sleeve")]
    CardboardPaperSleeve,
    #[serde(rename = "Cassette Case")]
    CassetteCase,
    /// A perfect bound book with a sleeve at the end to hold a CD
    Digibook,
    Digipak,
    #[serde(rename = "Discbox Slider")]
    DiscboxSlider,
    Fatbox,
    #[serde(rename = "Gatefold Cover")]
    GatefoldCover,
    /// The traditional CD case, made of hard, brittle plastic.
    #[serde(rename = "Jewel Case")]
    JewelCase,
    #[serde(rename = "Keep Case")]
    KeepCase,
    #[serde(rename = "Plastic Sleeve")]
    PlasticSleeve,
    /// Plastic CD tray inside a cardboard slipcover
    Slidepack,
    /// A thinner jewel case, commonly used for CD singles.
    #[serde(rename = "Slim Jewel Case")]
    SlimJewelCase,
    #[serde(rename = "Snap Case")]
    SnapCase,
    /// Japanese case that holds an 8cm CD. It is rectangular but can be snapped to make it more
    /// compact (hence the name).
    #[serde(rename = "SnapPack")]
    Snappack,
    #[serde(rename = "Super Jewel Box")]
    SuperJewelBox,
    Other,
    None,
    /// Any release_packaging that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ReleasePackaging::UnrecognizedReleasePackaging` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedReleasePackaging,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Media {
    pub discs: Option<Vec<Disc>>,
    pub title: Option<String>,
    pub position: Option<u32>,
    pub track_count: u32,
    pub disc_count: Option<u32>,
    pub format_id: Option<String>,
    pub format: Option<String>,
    pub tracks: Option<Vec<Track>>,
    pub track_offset: Option<u32>,
}

/// A track is the way a recording is represented on a particular release (or, more exactly, on a
/// particular medium).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Track {
    pub recording: Option<Recording>,
    pub title: String,
    pub number: String,
    pub length: Option<u32>,
    pub position: u32,
    pub id: String,
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct ReleaseSearchQuery {
    /// (part of) any alias attached to the release group (diacritics are ignored)
    alias: String,
    /// the MBID of any of the release group artists
    arid: String,
    /// (part of) the combined credited artist name for the release group, including join phrases (e.g. "Artist X feat.")
    artist: String,
    /// (part of) the name of any of the release group artists
    #[query_builder_field = "artistname"]
    artist_name: String,
    /// an Amazon ASIN for the release
    asin: String,
    /// the barcode for the release
    barcode: String,
    /// any catalog number for this release (insensitive to case, spaces and separators)
    #[query_builder_field = "catno"]
    catalog_number: String,
    /// (part of) the release group's disambiguation comment
    comment: String,
    /// the 2-letter code (ISO 3166-1 alpha-2) for any country the release was released in
    country: String,
    /// (part of) the credited name of any of the release group artists on this particular release group
    #[query_builder_field = "creditname"]
    credit_name: String,
    /// a release date for the release (e.g. "1980-01-22")
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    date: Option<NaiveDate>,
    /// the total number of disc IDs attached to all mediums on the release
    discids: u32,
    /// the number of disc IDs attached to any one medium on the release
    #[query_builder_field = "discidsmedium"]
    discids_medium: u32,
    /// the format of any medium in the release (insensitive to case, spaces, and separators)
    format: String,
    /// the MBID of any of the release labels
    laid: String,
    /// (part of) the name of any of the release labels
    label: String,
    /// the ISO 639-3 code for the release language
    lang: String,
    /// the number of mediums on the release
    mediums: u32,
    /// the format of the release (insensitive to case, spaces, and separators)
    packaging: String,
    /// the primary type of the release group
    #[query_builder_field = "primarytype"]
    primary_type: String,
    /// the listed quality of the data for the release (one of "low", "normal", "high")
    quality: String,
    /// the MBID of any of the releases in the release group
    reid: String,
    /// (part of) the title of any of the releases in the release group
    release: String,
    /// (part of) the release's title (with the specified diacritics)
    #[query_builder_field = "releaseaccent"]
    release_accent: String,
    /// the release group's MBID
    rgid: String,
    /// the ISO 15924 code for the release script
    script: String,
    /// any of the secondary types of the release group
    #[query_builder_field = "secondarytype"]
    secondary_type: String,
    /// the status of any of the releases in the release group
    status: String,
    /// the status of any of the releases in the release group
    tag: String,
    /// the total number of tracks on the release
    tracks: u32,
    /// the number of tracks on any one medium on the release
    #[query_builder_field = "tracksmedium"]
    tracks_medium: u32,
    /// legacy release group type field that predates the ability to set multiple types (see calculation code)
    #[query_builder_field = "type"]
    release_type: String,
}

impl_browse! {
Release,
   (by_area, BrowseBy::Area),
   (by_artist, BrowseBy::Artist),
   (by_label, BrowseBy::Label),
   (by_track, BrowseBy::Track),
   (by_track_artist, BrowseBy::TrackArtist),
   (by_recording, BrowseBy::Recording),
   (by_release_group, BrowseBy::ReleaseGroup),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Release,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_labels, Include::Subquery(Subquery::Labels)),
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_work_relations,
        Include::Relationship(Relationship::Work)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (
        with_work_level_relations,
        Include::Relationship(Relationship::WorkLevel)
    ),
    (
        with_recording_level_relations,
        Include::Relationship(Relationship::RecordingLevel)
    ),
    (with_recordings, Include::Subquery(Subquery::Recordings)),
    (
        with_release_groups,
        Include::Subquery(Subquery::ReleaseGroups)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations)),
    (
        with_artist_credits,
        Include::Subquery(Subquery::ArtistCredits)
    )
);
