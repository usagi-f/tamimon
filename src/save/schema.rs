use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub pet: Option<PetData>,
    pub last_check_time: DateTime<Utc>,
    pub records: Records,
    pub album: Vec<AlbumEntry>,
}

impl SaveData {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            version: "0.1.0".to_string(),
            pet: None,
            last_check_time: now,
            records: Records {
                longest_survival_ticks: 0,
                total_monsters: 0,
            },
            album: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetData {
    pub nickname: String,
    pub species: String,
    pub stage: u8,
    pub evolution_line: Vec<String>,
    pub age_ticks: u64,
    pub kimochi: f64,
    pub genki: f64,
    pub nakayoshi: f64,
    pub weight: f64,
    pub type_scores: TypeScores,
    pub survived_accident: bool,
    pub birth_timestamp: DateTime<Utc>,
    pub cumulative_kimochi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScores {
    pub chikara: u32,
    pub odayaka: u32,
    pub bouken: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Records {
    pub longest_survival_ticks: u64,
    pub total_monsters: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumEntry {
    pub nickname: String,
    pub species: String,
    pub days_lived: u32,
    pub weight_kg: f64,
    pub weight_label: String,
    pub cause_of_death: String,
    pub evolution_line: Vec<String>,
    pub reached_stage4: bool,
    pub date: String,
}
