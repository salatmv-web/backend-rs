use chrono::Utc;

use super::{
    parser::{Atoll, Island, PrayerTimes},
    utils::days_into_year,
};

pub struct Prayer {
    pub atolls: Vec<Atoll>,
    pub islands: Vec<Island>,
    pub prayers: Vec<PrayerTimes>,
    pub timings: Vec<String>,
}

impl Salat for Prayer {
    fn get_atoll(&self, query: i8) -> Option<Atoll> {
        self.atolls.iter().find(|e| e.category_id == query).cloned()
    }

    fn get_island(&self, query: i16) -> Option<Island> {
        self.islands.iter().find(|s| s.island_id == query).cloned()
    }

    fn get_entry_from_day(&self, day: i64, island: Island) -> Option<PrayerTimes> {
        self.prayers
            .iter()
            .find(|p| p.date == day && p.category_id == island.category_id)
            .cloned()
    }

    fn get_today(&self, island: Island) -> Option<PrayerTimes> {
        self.get_entry_from_day(days_into_year(Utc::now().date()), island)
    }
}

pub trait Salat {
    fn get_atoll(&self, query: i8) -> Option<Atoll>;
    fn get_island(&self, query: i16) -> Option<Island>;
    fn get_entry_from_day(&self, day: i64, island: Island) -> Option<PrayerTimes>;
    fn get_today(&self, island: Island) -> Option<PrayerTimes>;
}
