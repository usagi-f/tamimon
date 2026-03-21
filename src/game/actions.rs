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

    pub fn key(&self) -> &'static str {
        match self {
            Action::Talk => "T",
            Action::Play => "P",
            Action::Train => "R",
            Action::Relax => "E",
        }
    }
}

pub struct ActionResult {
    pub action: Action,
    pub reaction_lines: Vec<String>,
    /// For Talk: what the player "says" before each pet response.
    pub player_lines: Vec<String>,
    pub current_line: usize,
}

/// Player lines shown before the pet's Talk response.
pub fn select_talk_player_lines(count: usize, rng: &mut impl Rng) -> Vec<String> {
    let opening: &[&str] = &[
        "「ねえ！」",
        "「やほー！」",
        "「ちょっといい？」",
        "「げんきー？」",
        "「あのさ」",
        "「いたいた！」",
    ];
    (0..count)
        .map(|_| opening.choose(rng).unwrap().to_string())
        .collect()
}

fn pick_distinct(pool: &[&str], n: usize, rng: &mut impl Rng) -> Vec<String> {
    use rand::seq::SliceRandom;
    let n = n.min(pool.len());
    let mut indices: Vec<usize> = (0..pool.len()).collect();
    indices.shuffle(rng);
    indices[..n].iter().map(|&i| pool[i].to_string()).collect()
}

/// Short exclamations for Play コマ送り (Stage 1 generic).
pub fn select_play_exclamations(mood: MoodLevel, rng: &mut impl Rng) -> Vec<String> {
    let pool: &[&str] = match mood {
        MoodLevel::High => &[
            "「わーい！」",
            "「きゃー！」",
            "「もっかい！」",
            "「たのしい！」",
            "「えへへ！」",
            "「さいこう！」",
        ],
        MoodLevel::Normal => &[
            "「まあ、いっか」",
            "「ふーん…」",
            "「…お？」",
            "「おもしろいかも」",
            "「もうちょい」",
        ],
        MoodLevel::Low => &[
            "「…やるか」",
            "「だるい…」",
            "「ふぅ…」",
            "「つかれた」",
            "「…まあ」",
        ],
    };
    pick_distinct(pool, 3, rng)
}

/// Three-beat training arc: preparation → peak effort → aftermath.
pub fn select_train_beats(mood: MoodLevel, rng: &mut impl Rng) -> Vec<String> {
    let (prep, peak, after): (&[&str], &[&str], &[&str]) = match mood {
        MoodLevel::High => (
            &[
                "「いくぞ！」",
                "「さあ始める！」",
                "「気合い入れる！」",
                "「よし、やるか！」",
            ],
            &[
                "「はあっ！！！」",
                "「ふんぬー！！」",
                "「えいっ！！」",
                "「うりゃあ！！」",
            ],
            &[
                "「よっしゃ！」",
                "「やったぞ！」",
                "「いい汗だ！」",
                "「やりきった！」",
            ],
        ),
        MoodLevel::Normal => (
            &[
                "「んー、やるか」",
                "「まあ、いこう」",
                "「じゃあ、はじめる」",
                "「ふむ…」",
            ],
            &["「んっ！！」", "「はっ！」", "「ふんっ！」", "「んー！」"],
            &[
                "「ふぅ…」",
                "「まあまあかな」",
                "「こんなもんか」",
                "「…がんばった」",
            ],
        ),
        MoodLevel::Low => (
            &[
                "「…やるか」",
                "「はぁ…」",
                "「いやだけど…」",
                "「むり…でもやる」",
            ],
            &[
                "「うぅ…！」",
                "「つらい…！」",
                "「ぜーはー…」",
                "「むぐ…！」",
            ],
            &[
                "「…もうだめ」",
                "「ぜーはー…」",
                "「やっと終わり…」",
                "「つかれた…」",
            ],
        ),
    };
    vec![
        prep.choose(rng).unwrap().to_string(),
        peak.choose(rng).unwrap().to_string(),
        after.choose(rng).unwrap().to_string(),
    ]
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
