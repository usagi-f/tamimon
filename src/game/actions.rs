use rand::seq::SliceRandom;
use rand::Rng;

use crate::game::pet::MoodLevel;
use crate::save::schema::PetData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Talk,
    Play,
    Train,
    Relax,
}

impl Action {
    pub fn label(&self) -> &'static str {
        match self {
            Action::Talk => "話しかける",
            Action::Play => "あそぶ",
            Action::Train => "特訓",
            Action::Relax => "まったり",
        }
    }
}

pub struct ActionResult {
    pub action: Action,
    pub reaction_text: String,
}

/// Apply only action stat effects (reaction text handled separately)
pub fn apply_action_effects(action: Action, pet: &mut PetData, rng: &mut impl Rng) {
    match action {
        Action::Talk => {
            pet.nakayoshi = (pet.nakayoshi + 5.0 + rng.gen_range(-3.0..8.0)).clamp(0.0, 100.0);
            pet.kimochi = (pet.kimochi + 3.0 + rng.gen_range(-5.0..5.0)).clamp(0.0, 100.0);
        }
        Action::Play => {
            pet.kimochi = (pet.kimochi + 8.0 + rng.gen_range(-4.0..10.0)).clamp(0.0, 100.0);
            pet.genki = (pet.genki + 5.0 + rng.gen_range(-8.0..5.0)).clamp(0.0, 100.0);
            pet.type_scores.bouken += 1;
            pet.weight += rng.gen_range(-0.3..0.3);
        }
        Action::Train => {
            pet.genki = (pet.genki + 8.0 + rng.gen_range(-10.0..8.0)).clamp(0.0, 100.0);
            pet.type_scores.chikara += 1;
            pet.weight += rng.gen_range(-1.5..-0.5);
        }
        Action::Relax => {
            pet.kimochi = (pet.kimochi + 8.0 + rng.gen_range(-2.0..12.0)).clamp(0.0, 100.0);
            pet.type_scores.odayaka += 1;
            pet.weight += rng.gen_range(0.5..1.5);
        }
    }

    pet.weight = pet.weight.max(0.1);
}

/// Generic reactions for Stage1 (Phase1 compatible)
pub fn select_generic_reaction(action: Action, mood: MoodLevel, rng: &mut impl Rng) -> String {
    let pool = match (action, mood) {
        (Action::Talk, MoodLevel::High) => &[
            "「あ、きた！きた！」",
            "「うれしい！話そ話そ！」",
            "「まってたよ！」",
            "「なんか楽しいね！」",
        ][..],
        (Action::Talk, MoodLevel::Normal) => &[
            "「…ん？」",
            "「あ、いたの」",
            "「…ぼーっとしてた」",
            "「なに？」",
        ],
        (Action::Talk, MoodLevel::Low) => {
            &["「…」", "「……ねむい」", "「べつに」", "（こちらを見ない）"]
        }
        (Action::Play, MoodLevel::High) => &[
            "「やったー！あそぼ！」",
            "「たのしい！もっかい！」",
            "「えへへ！」",
            "「きゃー！」",
        ],
        (Action::Play, MoodLevel::Normal) => &[
            "「まあ、いっか」",
            "「ふーん、おもしろいかも」",
            "「…お？」",
            "「もうちょいやる？」",
        ],
        (Action::Play, MoodLevel::Low) => &[
            "「…つかれた」",
            "「いいけど…」",
            "「……」（少し動いた）",
            "「やる気でない…」",
        ],
        (Action::Train, MoodLevel::High) => &[
            "「よっしゃ！いくぞー！」",
            "「まだまだいける！」",
            "「つよくなった気がする！」",
            "「いい汗かいた！」",
        ],
        (Action::Train, MoodLevel::Normal) => &[
            "「ふー…」",
            "「まあ、やるか」",
            "「こんなもんかな」",
            "「…がんばった」",
        ],
        (Action::Train, MoodLevel::Low) => &[
            "「むり…」",
            "「もうだめ…」",
            "（へたりこんだ）",
            "「…ぜーはー」",
        ],
        (Action::Relax, MoodLevel::High) => &[
            "「はぁ〜きもちいい〜」",
            "「ぽかぽかだね！」",
            "「しあわせ…」",
            "「zzz...えへへ」",
        ],
        (Action::Relax, MoodLevel::Normal) => {
            &["「…zzz」", "「のんびり〜」", "「ふぁ〜」", "「…いい天気」"]
        }
        (Action::Relax, MoodLevel::Low) => &[
            "「…」（丸くなった）",
            "「…ねる」",
            "（動かない）",
            "「…zzz」",
        ],
    };

    pool.choose(rng).unwrap().to_string()
}
