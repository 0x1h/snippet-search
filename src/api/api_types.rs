use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Daum>,
    pub total: i64,
    pub next: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub id: i64,
    pub readable: bool,
    pub title: String,
    #[serde(rename = "title_short")]
    pub title_short: String,
    #[serde(rename = "title_version")]
    pub title_version: Option<String>,
    pub link: String,
    pub duration: i64,
    pub rank: i64,
    #[serde(rename = "explicit_lyrics")]
    pub explicit_lyrics: bool,
    #[serde(rename = "explicit_content_lyrics")]
    pub explicit_content_lyrics: i64,
    #[serde(rename = "explicit_content_cover")]
    pub explicit_content_cover: i64,
    pub preview: String,
    #[serde(rename = "md5_image")]
    pub md5_image: String,
    pub artist: Artist,
    pub album: Album,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub link: String,
    pub picture: String,
    #[serde(rename = "picture_small")]
    pub picture_small: String,
    #[serde(rename = "picture_medium")]
    pub picture_medium: String,
    #[serde(rename = "picture_big")]
    pub picture_big: String,
    #[serde(rename = "picture_xl")]
    pub picture_xl: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: String,
    #[serde(rename = "cover_small")]
    pub cover_small: String,
    #[serde(rename = "cover_medium")]
    pub cover_medium: String,
    #[serde(rename = "cover_big")]
    pub cover_big: String,
    #[serde(rename = "cover_xl")]
    pub cover_xl: String,
    #[serde(rename = "md5_image")]
    pub md5_image: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
