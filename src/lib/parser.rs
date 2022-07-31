use eyre::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Atoll {
    pub category_id: i8,
    pub name: String,
    pub arabic_name: String,
    pub dhivehi_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn get_value(&self, name: &str) -> i16 {
        match name {
            "fajr" => self.fajr,
            "sunrise" => self.sunrise,
            "duhr" => self.duhr,
            "asr" => self.asr,
            "maghrib" => self.maghrib,
            "isha" => self.isha,
            _ => 0,
        }
    }
}

pub fn convert_csv<D>(name: String) -> Result<Vec<D>>
where
    D: DeserializeOwned,
{
    let file = File::open(format!("assets/{}.csv", &name))
        .unwrap_or_else(|_| panic!("Failed to open {}.csv", name));
    let mut reader = csv::ReaderBuilder::new().from_reader(file);
    let mut contents = vec![];

    for content in reader.deserialize::<D>() {
        contents.push(content?);
    }

    Ok(contents)
}
