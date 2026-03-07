use chrono::{DateTime, Utc};
use rand::Rng;

use crate::save::schema::{PetData, TypeScores};

pub struct SpeciesInfo {
    pub name: &'static str,
    pub standard_weight: f64,
}

pub const STAGE1_SPECIES: [SpeciesInfo; 10] = [
    SpeciesInfo { name: "コロコロ", standard_weight: 10.0 },
    SpeciesInfo { name: "ニョロ", standard_weight: 8.0 },
    SpeciesInfo { name: "フワ", standard_weight: 5.0 },
    SpeciesInfo { name: "ツブ", standard_weight: 3.0 },
    SpeciesInfo { name: "プク", standard_weight: 12.0 },
    SpeciesInfo { name: "ミジン", standard_weight: 1.5 },
    SpeciesInfo { name: "ネロ", standard_weight: 9.0 },
    SpeciesInfo { name: "ボテ", standard_weight: 15.0 },
    SpeciesInfo { name: "ピリリ", standard_weight: 4.0 },
    SpeciesInfo { name: "モグモ", standard_weight: 7.0 },
];

const HATCHING_TICKS: u64 = 60; // 1 hour

pub struct HatchEvent {
    pub new_species: String,
}

pub fn new_egg(nickname: String, now: DateTime<Utc>) -> PetData {
    PetData {
        nickname,
        species: "たまご".to_string(),
        stage: 0,
        evolution_line: Vec::new(),
        age_ticks: 0,
        kimochi: 50.0,
        genki: 50.0,
        nakayoshi: 30.0,
        weight: 1.0,
        type_scores: TypeScores {
            chikara: 0,
            odayaka: 0,
            bouken: 0,
        },
        survived_accident: false,
        birth_timestamp: now,
        cumulative_kimochi: 50.0,
        last_stage4_check: 0,
    }
}

pub fn apply_decay(pet: &mut PetData, ticks: u64, rng: &mut impl Rng) {
    if ticks == 0 {
        return;
    }

    let ticks_f = ticks as f64;
    let sqrt_ticks = ticks_f.sqrt();

    // Deterministic decay + random component scaled by sqrt(ticks)
    let kimochi_change = -0.05 * ticks_f + rng.gen_range(-0.15..0.15) * sqrt_ticks;
    let genki_change = -0.03 * ticks_f + rng.gen_range(-0.10..0.10) * sqrt_ticks;
    let nakayoshi_change = -0.02 * ticks_f + rng.gen_range(-0.05..0.05) * sqrt_ticks;
    let weight_change = rng.gen_range(-0.1..0.1) * sqrt_ticks * 0.1;

    pet.kimochi = (pet.kimochi + kimochi_change).clamp(0.0, 100.0);
    pet.genki = (pet.genki + genki_change).clamp(0.0, 100.0);
    pet.nakayoshi = (pet.nakayoshi + nakayoshi_change).clamp(0.0, 100.0);
    pet.weight = (pet.weight + weight_change).max(0.1);

    let old_ticks = pet.age_ticks;
    pet.age_ticks += ticks;

    // Update cumulative kimochi (running average)
    if pet.age_ticks > 0 {
        pet.cumulative_kimochi = (pet.cumulative_kimochi * old_ticks as f64
            + pet.kimochi * ticks_f)
            / pet.age_ticks as f64;
    }
}

pub fn check_hatching(pet: &mut PetData, rng: &mut impl Rng) -> Option<HatchEvent> {
    if pet.stage != 0 || pet.age_ticks < HATCHING_TICKS {
        return None;
    }

    let idx = rng.gen_range(0..STAGE1_SPECIES.len());
    let species = &STAGE1_SPECIES[idx];

    pet.species = species.name.to_string();
    pet.stage = 1;
    pet.weight = species.standard_weight;
    pet.evolution_line.push(species.name.to_string());

    Some(HatchEvent {
        new_species: species.name.to_string(),
    })
}

pub fn weight_label(species: &str, current_weight: f64) -> &'static str {
    let standard = find_standard_weight(species);
    let ratio = (current_weight - standard) / standard;

    if ratio < -0.30 {
        "ほぼ空気"
    } else if ratio < -0.20 {
        "ふっとびそう"
    } else if ratio < -0.10 {
        "ガリ"
    } else if ratio <= 0.10 {
        "標準体重"
    } else if ratio <= 0.20 {
        "ぷにってる"
    } else if ratio <= 0.30 {
        "ややデブ"
    } else if ratio <= 0.50 {
        "デブ"
    } else {
        "限界突破"
    }
}

fn find_standard_weight(species: &str) -> f64 {
    if species == "たまご" {
        return 1.0;
    }
    for s in &STAGE1_SPECIES {
        if s.name == species {
            return s.standard_weight;
        }
    }
    // For Stage2+, get from evolution module
    if let Some(w) = crate::game::evolution::get_standard_weight(species) {
        return w;
    }
    10.0
}

pub fn mood_level(kimochi: f64) -> MoodLevel {
    if kimochi > 60.0 {
        MoodLevel::High
    } else if kimochi > 30.0 {
        MoodLevel::Normal
    } else {
        MoodLevel::Low
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoodLevel {
    High,
    Normal,
    Low,
}
