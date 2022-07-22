use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Island {
    pub category_id: i8,
    pub island_id: i16,
    pub atoll: i16,
    pub english_name: String,
    pub dhivehi_name: String,
    pub arabic_name: String,
    pub offset: f32,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub status: i8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Atoll {
    pub category_id: i8,
    pub name: String,
    pub arabic_name: String,
    pub dhivehi_name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PrayerTimes {
    pub category_id: i8,
    pub date: i64,
    pub fajr: i16,
    pub sunrise: i16,
    pub duhr: i16,
    pub asr: i16,
    pub maghrib: i16,
    pub isha: i16,
}

impl PrayerTimes {
    fn to_value(&self) {}
}

pub fn convert_csv<D>(name: &str) -> Vec<D>
where
    D: DeserializeOwned,
{
    let file = File::open(format!("assets/{}.csv", name))
        .expect(format!("Failed to open {}.csv", name).as_str());
    let mut reader = csv::ReaderBuilder::new().from_reader(file);
    let mut contents = vec![];

    for content in reader.deserialize::<D>() {
        contents.push(content.expect("Failed to parse a line."));
    }

    contents
}
