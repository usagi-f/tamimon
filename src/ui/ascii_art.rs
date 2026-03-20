//! ASCII art routing module.
//!
//! Delegates to stage-specific modules for hand-crafted art:
//! - `ascii_art_s1`: Stage 1 (egg + 10 baby species)
//! - `ascii_art_s2`: Stage 2 (30 species, 5 evo types × 6)
//! - `ascii_art_s3`: Stage 3 (80 species)
//! - `ascii_art_s4`: Stage 4 (8 mutation species)

use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

type ArtLookup = fn(&str, MoodLevel, usize) -> Option<Vec<String>>;
type ActionArtLookup = fn(&str, Action, usize) -> Option<Vec<String>>;

// ===== Public API =====

/// Egg art (re-exported from s1 module for external use).
pub fn egg_art() -> &'static [&'static str] {
    super::ascii_art_s1::egg_art()
}

/// Fallback art for unknown species.
fn fallback_art(species: &str) -> Vec<String> {
    vec![
        String::new(),
        String::new(),
        format!("    (？_？)  [{}]", species),
        String::new(),
        String::new(),
    ]
}

pub fn get_art(species: &str, mood: MoodLevel, frame: usize) -> Vec<String> {
    let lookups: [ArtLookup; 4] = [
        super::ascii_art_s1::get_s1_art,
        super::ascii_art_s2::get_s2_art,
        super::ascii_art_s3::get_s3_art,
        super::ascii_art_s4::get_s4_art,
    ];
    for lookup in &lookups {
        if let Some(art) = lookup(species, mood, frame) {
            return art;
        }
    }
    fallback_art(species)
}

/// Action-specific effect text that cycles with animation frames.
pub fn get_action_effect(action: Action, frame: usize) -> &'static str {
    match action {
        Action::Talk => match frame % 4 {
            0 => "「 ・・・ 」",
            1 => "「 ・・・・・・ 」",
            2 => "「 ・・・・・・・・・ 」",
            _ => "「 ！ 」",
        },
        Action::Play => match frame % 4 {
            0 => "♪",
            1 => "♪ ♪",
            2 => "♪ ♪ ♪",
            _ => "♪ ♪ ♪ ♪",
        },
        Action::Train => match frame % 4 {
            0 => "...!",
            1 => "ﾌﾝｯ !!",
            2 => "ﾊｧ ﾊｧ ...!",
            _ => "ﾌﾝﾌﾝｯ !!!",
        },
        Action::Relax => match frame % 4 {
            0 => "～",
            1 => "～ ～",
            2 => "～ ～ ～",
            _ => "～ ～ ～ ～",
        },
    }
}

/// Species-specific action animation art.
pub fn get_action_art(species: &str, action: Action, frame: usize) -> Vec<String> {
    let lookups: [ActionArtLookup; 4] = [
        super::ascii_art_s1::get_s1_action_art,
        super::ascii_art_s2::get_s2_action_art,
        super::ascii_art_s3::get_s3_action_art,
        super::ascii_art_s4::get_s4_action_art,
    ];
    for lookup in &lookups {
        if let Some(art) = lookup(species, action, frame) {
            return art;
        }
    }
    fallback_art(species)
}

pub fn get_idle_speech(species: &str, mood: MoodLevel) -> &'static [&'static str] {
    match (species, mood) {
        ("たまご", _) => &["（ぴくっ…ぴくっ…）", "（もぞもぞ…）", "（かたかた…）"],
        (_, MoodLevel::High) => &[
            "今日もげんきだよ！",
            "うれしい！",
            "なんかいい日！",
            "るんるん♪",
        ],
        (_, MoodLevel::Normal) => &["…ぼーっとしてた", "なんかいい日", "ふぁ〜", "…ん？"],
        (_, MoodLevel::Low) => &["…", "ねむい", "…べつに", "（ぼんやり）"],
    }
}
