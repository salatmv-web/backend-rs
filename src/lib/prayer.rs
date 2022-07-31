use chrono::Local;

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

impl Prayer {
    pub fn get_atoll(&self, query: i8) -> Option<Atoll> {
        self.atolls.iter().find(|e| e.category_id == query).cloned()
    }

    pub fn get_island(&self, query: i16) -> Option<Island> {
        self.islands.iter().find(|s| s.island_id == query).cloned()
    }

    pub fn get_entry_from_day(&self, day: i64, island: Island) -> Option<PrayerTimes> {
        self.prayers
            .iter()
            .find(|p| p.date == day && p.category_id == island.category_id)
            .cloned()
    }

    pub fn get_today(&self, island: Island) -> Option<PrayerTimes> {
        self.get_entry_from_day(days_into_year(Local::now().date()), island)
    }
}
