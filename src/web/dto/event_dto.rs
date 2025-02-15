use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventDto {
    pub id: String,
    pub name: String,
    pub version: u64,
}