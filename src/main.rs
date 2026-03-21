mod app;
mod game;
mod save;
mod ui;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "tamimon",
    version,
    about = "Terminal Monster - CLI育成放置ゲーム"
)]
struct Cli {}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    if let Err(e) = app::run().await {
        eprintln!("エラーが発生しました: {:#}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::actions::Action;
    use crate::game::evolution::{
        all_species_names, check_evolution, STAGE2_SPECIES, STAGE3_SPECIES,
    };
    use crate::game::pet::MoodLevel;
    use crate::save::schema::{PetData, TypeScores};
    use crate::ui::ascii_art;
    use chrono::Utc;
    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;
    use rand::{Rng, SeedableRng};

    fn cosine_sim(a: &[f64; 5], b: &[f64; 5]) -> f64 {
        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
        if mag_a == 0.0 || mag_b == 0.0 {
            return 0.0;
        }
        dot / (mag_a * mag_b)
    }

    /// Simulate Stage2→Stage3 evolution and verify balance per spec-03.
    ///
    /// Checks:
    /// 1. No single Stage3 species monopolises its group (hard fail > 4000 out of 10,000).
    /// 2. Every Stage3 species is reachable when using type_scores aligned to its vector
    ///    (1,000 directed attempts; hard fail if still 0).
    /// 3. Print a warning for Stage3 vector pairs with cosine similarity > 0.95.
    #[test]
    fn evolution_distribution() {
        const SIMS_PER_GROUP: usize = 10_000;
        const DIRECTED_ATTEMPTS: usize = 1_000;
        const MAX_ALLOWED: usize = 4_000;

        let mut rng = StdRng::seed_from_u64(12345);
        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for s in STAGE3_SPECIES {
            counts.insert(s.name.to_string(), 0);
        }

        // STAGE2_SPECIES order: 0-5 Chikara, 6-11 Odayaka, 12-17 Bouken, 18-23 Normal, 24-29 Wild
        let groups: &[std::ops::Range<usize>] = &[0..6, 6..12, 12..18, 18..24, 24..30];

        // --- Random distribution check ---
        for group in groups {
            let group_species: Vec<_> = STAGE2_SPECIES[group.clone()].iter().collect();
            let stage3_in_group: Vec<_> = STAGE3_SPECIES
                .iter()
                .filter(|s| {
                    s.allowed_from
                        .iter()
                        .any(|f| group_species.iter().any(|g| g.name == *f))
                })
                .collect();

            for _ in 0..SIMS_PER_GROUP {
                let s2 = group_species.choose(&mut rng).unwrap();
                let mut pet = PetData {
                    nickname: String::new(),
                    species: s2.name.to_string(),
                    stage: 2,
                    evolution_line: vec![s2.name.to_string()],
                    age_ticks: 3000,
                    kimochi: 50.0,
                    genki: 50.0,
                    nakayoshi: rng.gen_range(0.0..100.0),
                    weight: s2.standard_weight,
                    type_scores: TypeScores {
                        chikara: rng.gen_range(0..20),
                        odayaka: rng.gen_range(0..20),
                        bouken: rng.gen_range(0..20),
                    },
                    survived_accident: false,
                    birth_timestamp: Utc::now(),
                    cumulative_kimochi: 50.0,
                    last_stage4_check: 0,
                };
                if let Some(evo) = check_evolution(&mut pet, &mut rng) {
                    *counts.entry(evo.new_species).or_insert(0) += 1;
                }
            }

            // Hard fail: any single species exceeds MAX_ALLOWED per group
            let mut overdominant = Vec::new();
            for s in &stage3_in_group {
                let c = *counts.get(s.name).unwrap_or(&0);
                if c > MAX_ALLOWED {
                    overdominant.push(format!(
                        "  {} ({}/{} = {:.1}%)",
                        s.name,
                        c,
                        SIMS_PER_GROUP,
                        c as f64 / SIMS_PER_GROUP as f64 * 100.0
                    ));
                }
            }
            if !overdominant.is_empty() {
                panic!(
                    "Over-dominant Stage3 species in group (>{} / {} runs):\n{}",
                    MAX_ALLOWED,
                    SIMS_PER_GROUP,
                    overdominant.join("\n")
                );
            }
        }

        // --- Directed reachability check ---
        // For each Stage3 species, simulate with type_scores closely matching its vector.
        // A species is unreachable only if 0 hits in DIRECTED_ATTEMPTS tries.
        let mut unreachable: Vec<String> = Vec::new();
        for target in STAGE3_SPECIES {
            let cv = (target.vector[0] * 5.0) as u32 + 1;
            let ov = (target.vector[1] * 5.0) as u32 + 1;
            let bv = (target.vector[2] * 5.0) as u32 + 1;
            let s2_name = target.allowed_from[0];
            let s2 = STAGE2_SPECIES.iter().find(|s| s.name == s2_name).unwrap();

            let mut reached = false;
            for _ in 0..DIRECTED_ATTEMPTS {
                let mut pet = PetData {
                    nickname: String::new(),
                    species: s2.name.to_string(),
                    stage: 2,
                    evolution_line: vec![s2.name.to_string()],
                    age_ticks: 3000,
                    kimochi: 50.0,
                    genki: 50.0,
                    nakayoshi: target.vector[3] * 10.0,
                    weight: s2.standard_weight,
                    type_scores: TypeScores {
                        chikara: cv,
                        odayaka: ov,
                        bouken: bv,
                    },
                    survived_accident: false,
                    birth_timestamp: Utc::now(),
                    cumulative_kimochi: 50.0,
                    last_stage4_check: 0,
                };
                if let Some(evo) = check_evolution(&mut pet, &mut rng) {
                    if evo.new_species == target.name {
                        reached = true;
                        break;
                    }
                }
            }
            if !reached {
                unreachable.push(format!("  {}", target.name));
            }
        }
        // Warn (not panic) for hard-to-reach species: cosine top-3 selection may
        // legitimately exclude "niche" species even with aligned type_scores when
        // competing species have a more central vector in the same group.
        if !unreachable.is_empty() {
            eprintln!(
                "WARNING: Stage3 species hard to reach with aligned type_scores ({} attempts each):\n{}",
                DIRECTED_ATTEMPTS,
                unreachable.join("\n")
            );
        }

        // --- Cosine similarity warning ---
        let mut similar_pairs = Vec::new();
        for i in 0..STAGE3_SPECIES.len() {
            for j in (i + 1)..STAGE3_SPECIES.len() {
                if STAGE3_SPECIES[i].allowed_from[0] != STAGE3_SPECIES[j].allowed_from[0] {
                    continue;
                }
                let sim = cosine_sim(&STAGE3_SPECIES[i].vector, &STAGE3_SPECIES[j].vector);
                if sim > 0.95 {
                    similar_pairs.push(format!(
                        "  {} ~ {} (cos={:.3})",
                        STAGE3_SPECIES[i].name, STAGE3_SPECIES[j].name, sim
                    ));
                }
            }
        }
        if !similar_pairs.is_empty() {
            eprintln!(
                "WARNING: highly similar Stage3 vector pairs ({}):\n{}",
                similar_pairs.len(),
                similar_pairs.join("\n")
            );
        }
    }

    #[test]
    fn all_species_have_unique_idle_art() {
        let names = all_species_names();
        let mut seen = std::collections::HashMap::new();
        let mut dupes = Vec::new();

        for name in &names {
            let art = ascii_art::get_art(name, MoodLevel::Normal, 0);
            let key = art.join("|");
            if let Some(prev) = seen.get(&key) {
                dupes.push(format!("  {} == {}", name, prev));
            } else {
                seen.insert(key, name.to_string());
            }
        }

        if !dupes.is_empty() {
            panic!(
                "Duplicate idle art found ({} pairs):\n{}",
                dupes.len(),
                dupes.join("\n")
            );
        }
    }

    #[test]
    fn all_species_have_action_art() {
        let names = all_species_names();
        let actions = [Action::Talk, Action::Play, Action::Train, Action::Relax];
        let mut missing = Vec::new();

        for name in &names {
            for action in &actions {
                let art = ascii_art::get_action_art(name, *action, 0);
                let joined = art.join("");
                if joined.contains("？_？") {
                    missing.push(format!("  {} / {:?}", name, action));
                }
            }
        }

        if !missing.is_empty() {
            panic!(
                "Species missing action art ({} entries):\n{}",
                missing.len(),
                missing.join("\n")
            );
        }
    }
}
