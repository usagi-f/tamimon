use crate::game::actions::Action;
use crate::game::evolution::{self, EvoType};
use crate::game::pet::MoodLevel;

pub fn egg_art() -> &'static [&'static str] {
    &[
        "",
        "      ＿＿",
        "    （　　　）",
        "     ￣￣￣",
        "",
    ]
}

pub fn get_art(species: &str, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match species {
        "たまご" => egg_art(),
        "コロコロ" => korokoro_art(mood, frame),
        "ニョロ" => nyoro_art(mood, frame),
        "フワ" => fuwa_art(mood, frame),
        "ツブ" => tsubu_art(mood, frame),
        "プク" => puku_art(mood, frame),
        "ミジン" => mijin_art(mood, frame),
        "ネロ" => nero_art(mood, frame),
        "ボテ" => bote_art(mood, frame),
        _ => get_template_art(species, mood, frame),
    }
}

fn name_hash(name: &str) -> usize {
    let mut hash: usize = 5381;
    for byte in name.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as usize);
    }
    hash
}

fn get_template_art(species: &str, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    let evo_type = evolution::get_evo_type(species).unwrap_or(EvoType::Normal);
    let stage = evolution::get_stage(species).unwrap_or(2);
    let variant = name_hash(species);

    match (stage, evo_type) {
        (2, EvoType::Chikara) => stage2_chikara_art(variant, mood, frame),
        (2, EvoType::Odayaka) => stage2_odayaka_art(variant, mood, frame),
        (2, EvoType::Bouken)  => stage2_bouken_art(variant, mood, frame),
        (2, EvoType::Normal)  => stage2_normal_art(variant, mood, frame),
        (2, EvoType::Wild)    => stage2_wild_art(variant, mood, frame),
        (3, EvoType::Chikara) => stage3_chikara_art(variant, mood, frame),
        (3, EvoType::Odayaka) => stage3_odayaka_art(variant, mood, frame),
        (3, EvoType::Bouken)  => stage3_bouken_art(variant, mood, frame),
        (3, EvoType::Normal)  => stage3_normal_art(variant, mood, frame),
        (3, EvoType::Wild)    => stage3_wild_art(variant, mood, frame),
        (4, _)                => stage4_art(variant, mood, frame),
        _                     => egg_art(),
    }
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

pub fn get_idle_speech(species: &str, mood: MoodLevel) -> &'static [&'static str] {
    match (species, mood) {
        ("たまご", _) => &[
            "（ぴくっ…ぴくっ…）",
            "（もぞもぞ…）",
            "（かたかた…）",
        ],
        (_, MoodLevel::High) => &[
            "今日もげんきだよ！",
            "うれしい！",
            "なんかいい日！",
            "るんるん♪",
        ],
        (_, MoodLevel::Normal) => &[
            "…ぼーっとしてた",
            "なんかいい日",
            "ふぁ〜",
            "…ん？",
        ],
        (_, MoodLevel::Low) => &[
            "…",
            "ねむい",
            "…べつに",
            "（ぼんやり）",
        ],
    }
}

// --- Stage 1 Species Art ---

fn korokoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "    (≧▽≦)ノ", "     ヾ|", "     /|", ""],
        (MoodLevel::High, _) => &["", "     (≧▽≦)", "      |ノ", "      |\\", ""],
        (MoodLevel::Normal, 0) => &["", "    (˘ω˘)", "      |", "     / \\", ""],
        (MoodLevel::Normal, _) => &["", "    (˘─˘)", "      |", "     / \\", ""],
        (MoodLevel::Low, 0) => &["", "    (￣_￣)", "      |", "     / \\", ""],
        (MoodLevel::Low, _) => &["", "    (￣ ￣)", "      |", "     / \\", ""],
    }
}

fn nyoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ≋(＞ω＜)≋", "", ""],
        (MoodLevel::High, _) => &["", "≋(＞ω＜)≋ ", "", ""],
        (MoodLevel::Normal, 0) => &["", " ≋(・ω・)", "", ""],
        (MoodLevel::Normal, _) => &["", " ≋(・─・)", "", ""],
        (MoodLevel::Low, 0) => &["", " ≋(￣_￣)", "", ""],
        (MoodLevel::Low, _) => &["", "≋(￣ ￣) ", "", ""],
    }
}

fn fuwa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  ୧(˶≧▽≦)୨", "", ""],
        (MoodLevel::High, _) => &["", " ୧(˶≧▽≦)୨ ", "", ""],
        (MoodLevel::Normal, 0) => &["", "  ୧(˶˘ᵕ˘)", "", ""],
        (MoodLevel::Normal, _) => &["", "  ୧(˶˘─˘)", "", ""],
        (MoodLevel::Low, 0) => &["", "  ୧(˶￣_￣)", "", ""],
        (MoodLevel::Low, _) => &["", "  ୧(˶￣ ￣)", "", ""],
    }
}

fn tsubu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "    ⊙▽⊙", "", ""],
        (MoodLevel::High, _) => &["", "    ⊙▽⊙ !", "", ""],
        (MoodLevel::Normal, 0) => &["", "    ⊙_⊙", "", ""],
        (MoodLevel::Normal, _) => &["", "    ⊙─⊙", "", ""],
        (MoodLevel::Low, 0) => &["", "    ⊙_⊙", "", ""],
        (MoodLevel::Low, _) => &["", "    -_-", "", ""],
    }
}

fn puku_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "   (｡>ω<｡)", "", ""],
        (MoodLevel::High, _) => &["", "   (｡>ω<｡)ノ", "", ""],
        (MoodLevel::Normal, 0) => &["", "   (｡･ω･｡)", "", ""],
        (MoodLevel::Normal, _) => &["", "   (｡･─･｡)", "", ""],
        (MoodLevel::Low, 0) => &["", "   (｡-ω-｡)", "", ""],
        (MoodLevel::Low, _) => &["", "   (｡- -｡)", "", ""],
    }
}

fn mijin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "    ＞ω＜", "", ""],
        (MoodLevel::High, _) => &["", "   ＞ω＜ !", "", ""],
        (MoodLevel::Normal, 0) => &["", "    ･ω･", "", ""],
        (MoodLevel::Normal, _) => &["", "    ･─･", "", ""],
        (MoodLevel::Low, 0) => &["", "    -ω-", "", ""],
        (MoodLevel::Low, _) => &["", "    - -", "", ""],
    }
}

fn nero_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "   (^ω^)", "", ""],
        (MoodLevel::High, _) => &["", "   (^ω^)ノ", "", ""],
        (MoodLevel::Normal, 0) => &["", "   (=ω=)", "", ""],
        (MoodLevel::Normal, _) => &["", "   (=─=)", "", ""],
        (MoodLevel::Low, 0) => &["", "   (-ω-) zzZ", "", ""],
        (MoodLevel::Low, _) => &["", "   (- -) zzZ", "", ""],
    }
}

fn bote_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "   (・▽・)", "", ""],
        (MoodLevel::High, _) => &["", "   (・▽・)ノ", "", ""],
        (MoodLevel::Normal, 0) => &["", "   (・●・)", "", ""],
        (MoodLevel::Normal, _) => &["", "   (・─・)", "", ""],
        (MoodLevel::Low, 0) => &["", "   (・_・)", "", ""],
        (MoodLevel::Low, _) => &["", "   (・ ・)", "", ""],
    }
}

// ===== Stage2+ Template Art System =====
// 5 evo types x 3 stages = 15 template groups + 1 unified Stage4
// Each has 3 variants x 3 moods x 2 frames

// --- Stage2 Chikara (power) type ---
fn stage2_chikara_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["", "   ᕙ(≧▽≧)ᕗ", "     ┃┃", "    ╚╝╚╝", ""],
        (0, MoodLevel::High, _) => &["", "  ᕙ(≧▽≧)ᕗ !", "     ┃┃", "    ╚╝╚╝", ""],
        (0, MoodLevel::Normal, 0) => &["", "   ᕙ(・益・)ᕗ", "     ┃┃", "    ╚╝╚╝", ""],
        (0, MoodLevel::Normal, _) => &["", "   ᕙ(・─・)ᕗ", "     ┃┃", "    ╚╝╚╝", ""],
        (0, MoodLevel::Low, 0) => &["", "   ᕙ(￣_￣)ᕗ", "     ┃┃", "    ╚╝╚╝", ""],
        (0, MoodLevel::Low, _) => &["", "   ᕙ(￣ ￣)ᕗ", "     ┃┃", "    ╚╝╚╝", ""],
        (1, MoodLevel::High, 0) => &["", "    (ˊ益ˋ)ᕤ", "      |", "     / \\", ""],
        (1, MoodLevel::High, _) => &["", "   (ˊ益ˋ)ᕤ !", "      |", "     / \\", ""],
        (1, MoodLevel::Normal, 0) => &["", "    (ˊ_ˋ)ᕤ", "      |", "     / \\", ""],
        (1, MoodLevel::Normal, _) => &["", "    (ˊ─ˋ)ᕤ", "      |", "     / \\", ""],
        (1, MoodLevel::Low, 0) => &["", "    (￣_￣)ᕤ", "      |", "     / \\", ""],
        (1, MoodLevel::Low, _) => &["", "    (￣ ￣)", "      |", "     / \\", ""],
        (_, MoodLevel::High, 0) => &["", "   ┏(≧▽≧)┓", "     ┃┃", "    ╚╝╚╝", ""],
        (_, MoodLevel::High, _) => &["", "  ┏(≧▽≧)┓ !", "     ┃┃", "    ╚╝╚╝", ""],
        (_, MoodLevel::Normal, 0) => &["", "   ┏(・ω・)┓", "     ┃┃", "    ╚╝╚╝", ""],
        (_, MoodLevel::Normal, _) => &["", "   ┏(・─・)┓", "     ┃┃", "    ╚╝╚╝", ""],
        (_, MoodLevel::Low, 0) => &["", "   ┏(￣_￣)┓", "     ┃┃", "    ╚╝╚╝", ""],
        (_, MoodLevel::Low, _) => &["", "   ┏(￣ ￣)┓", "     ┃┃", "    ╚╝╚╝", ""],
    }
}

// --- Stage2 Odayaka (gentle) type ---
fn stage2_odayaka_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["", "  ☁(˶≧▽≦)☁", "", "", ""],
        (0, MoodLevel::High, _) => &["", " ☁(˶≧▽≦)☁ ", "", "", ""],
        (0, MoodLevel::Normal, 0) => &["", "  ☁(˶˘ᵕ˘)☁", "", "", ""],
        (0, MoodLevel::Normal, _) => &["", "  ☁(˶˘─˘)☁", "", "", ""],
        (0, MoodLevel::Low, 0) => &["", "  ☁(˶￣_￣)☁", "", "", ""],
        (0, MoodLevel::Low, _) => &["", "  ☁(˶￣ ￣)☁", "", "", ""],
        (1, MoodLevel::High, 0) => &["", "   ∩(´▽`)∩", "", "", ""],
        (1, MoodLevel::High, _) => &["", "  ∩(´▽`)∩ ♪", "", "", ""],
        (1, MoodLevel::Normal, 0) => &["", "   ∩(´ω`)∩", "", "", ""],
        (1, MoodLevel::Normal, _) => &["", "   ∩(´─`)∩", "", "", ""],
        (1, MoodLevel::Low, 0) => &["", "   ∩(´_`)∩", "", "", ""],
        (1, MoodLevel::Low, _) => &["", "   ∩(´ `)∩", "", "", ""],
        (_, MoodLevel::High, 0) => &["", "  __(≧ω≦)__", "", "", ""],
        (_, MoodLevel::High, _) => &["", " __(≧ω≦)__ ♪", "", "", ""],
        (_, MoodLevel::Normal, 0) => &["", "  __(˘ω˘)__", "", "", ""],
        (_, MoodLevel::Normal, _) => &["", "  __(˘─˘)__", "", "", ""],
        (_, MoodLevel::Low, 0) => &["", "  __(￣_￣)__", "", "", ""],
        (_, MoodLevel::Low, _) => &["", "  __(￣ ￣)__", "", "", ""],
    }
}

// --- Stage2 Bouken (adventure) type ---
fn stage2_bouken_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["", "  ＜(＞▽＜)＞", "      |", "     / \\", ""],
        (0, MoodLevel::High, _) => &["", " ＜(＞▽＜)＞ !", "      |", "     / \\", ""],
        (0, MoodLevel::Normal, 0) => &["", "  ＜(・ω・)＞", "      |", "     / \\", ""],
        (0, MoodLevel::Normal, _) => &["", "  ＜(・─・)＞", "      |", "     / \\", ""],
        (0, MoodLevel::Low, 0) => &["", "  ＜(￣_￣)＞", "      |", "     / \\", ""],
        (0, MoodLevel::Low, _) => &["", "  ＜(￣ ￣)＞", "      |", "     / \\", ""],
        (1, MoodLevel::High, 0) => &["", "  ┗(＾▽＾)┛", "      |", "     / \\", ""],
        (1, MoodLevel::High, _) => &["", " ┗(＾▽＾)┛ !", "      |", "     / \\", ""],
        (1, MoodLevel::Normal, 0) => &["", "  ┗(＾ω＾)┛", "      |", "     / \\", ""],
        (1, MoodLevel::Normal, _) => &["", "  ┗(＾─＾)┛", "      |", "     / \\", ""],
        (1, MoodLevel::Low, 0) => &["", "  ┗(￣_￣)┛", "      |", "     / \\", ""],
        (1, MoodLevel::Low, _) => &["", "  ┗(￣ ￣)┛", "      |", "     / \\", ""],
        (_, MoodLevel::High, 0) => &["", "  ≫(´▽`)ノ", "      |", "     / \\", ""],
        (_, MoodLevel::High, _) => &["", " ≫(´▽`)ノ !", "      |", "     / \\", ""],
        (_, MoodLevel::Normal, 0) => &["", "  ≫(´ω`)ノ", "      |", "     / \\", ""],
        (_, MoodLevel::Normal, _) => &["", "  ≫(´─`)ノ", "      |", "     / \\", ""],
        (_, MoodLevel::Low, 0) => &["", "  ≫(´_`)ノ", "      |", "     / \\", ""],
        (_, MoodLevel::Low, _) => &["", "  ≫(´ `)ノ", "      |", "     / \\", ""],
    }
}

// --- Stage2 Normal type ---
fn stage2_normal_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["", "   (´・▽・`)", "", "", ""],
        (0, MoodLevel::High, _) => &["", "   (´・▽・`)ノ", "", "", ""],
        (0, MoodLevel::Normal, 0) => &["", "   (´・ω・`)", "", "", ""],
        (0, MoodLevel::Normal, _) => &["", "   (´・─・`)", "", "", ""],
        (0, MoodLevel::Low, 0) => &["", "   (´・_・`)", "", "", ""],
        (0, MoodLevel::Low, _) => &["", "   (´・ ・`)", "", "", ""],
        (1, MoodLevel::High, 0) => &["", "   (°▽°)", "", "", ""],
        (1, MoodLevel::High, _) => &["", "   (°▽°)ノ", "", "", ""],
        (1, MoodLevel::Normal, 0) => &["", "   (°ω°)", "", "", ""],
        (1, MoodLevel::Normal, _) => &["", "   (°─°)", "", "", ""],
        (1, MoodLevel::Low, 0) => &["", "   (°_°)", "", "", ""],
        (1, MoodLevel::Low, _) => &["", "   (° °)", "", "", ""],
        (_, MoodLevel::High, 0) => &["", "   (˙▽˙)", "", "", ""],
        (_, MoodLevel::High, _) => &["", "   (˙▽˙)ノ", "", "", ""],
        (_, MoodLevel::Normal, 0) => &["", "   (˙ᵕ˙)", "", "", ""],
        (_, MoodLevel::Normal, _) => &["", "   (˙─˙)", "", "", ""],
        (_, MoodLevel::Low, 0) => &["", "   (˙_˙)", "", "", ""],
        (_, MoodLevel::Low, _) => &["", "   (˙ ˙)", "", "", ""],
    }
}

// --- Stage2 Wild type ---
fn stage2_wild_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["", "  ◉(⊙▽⊙)◉", "", "", ""],
        (0, MoodLevel::High, _) => &["", " ◉(⊙▽⊙)◉ !", "", "", ""],
        (0, MoodLevel::Normal, 0) => &["", "  ◉(⊙_⊙)◉", "", "", ""],
        (0, MoodLevel::Normal, _) => &["", "  ◉(⊙─⊙)◉", "", "", ""],
        (0, MoodLevel::Low, 0) => &["", "  ◉(⊙ ⊙)◉", "", "", ""],
        (0, MoodLevel::Low, _) => &["", "  ◉(- -)◉", "", "", ""],
        (1, MoodLevel::High, 0) => &["", "  ψ(⊙▽⊙)ψ", "", "", ""],
        (1, MoodLevel::High, _) => &["", " ψ(⊙▽⊙)ψ !", "", "", ""],
        (1, MoodLevel::Normal, 0) => &["", "  ψ(⊙ω⊙)ψ", "", "", ""],
        (1, MoodLevel::Normal, _) => &["", "  ψ(⊙─⊙)ψ", "", "", ""],
        (1, MoodLevel::Low, 0) => &["", "  ψ(⊙_⊙)ψ", "", "", ""],
        (1, MoodLevel::Low, _) => &["", "  ψ(- -)ψ", "", "", ""],
        (_, MoodLevel::High, 0) => &["", "  ‡(◎▽◎)‡", "", "", ""],
        (_, MoodLevel::High, _) => &["", " ‡(◎▽◎)‡ !", "", "", ""],
        (_, MoodLevel::Normal, 0) => &["", "  ‡(◎_◎)‡", "", "", ""],
        (_, MoodLevel::Normal, _) => &["", "  ‡(◎─◎)‡", "", "", ""],
        (_, MoodLevel::Low, 0) => &["", "  ‡(◎ ◎)‡", "", "", ""],
        (_, MoodLevel::Low, _) => &["", "  ‡(- -)‡", "", "", ""],
    }
}

// --- Stage3 Chikara (larger, more imposing) ---
fn stage3_chikara_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["    ╔══╗", "  ᕙ(≧▽≧)ᕗ", "   ┃████┃", "   ╚╝  ╚╝", ""],
        (0, MoodLevel::High, _) => &["    ╔══╗ !", "  ᕙ(≧▽≧)ᕗ", "   ┃████┃", "   ╚╝  ╚╝", ""],
        (0, MoodLevel::Normal, 0) => &["    ╔══╗", "  ᕙ(・益・)ᕗ", "   ┃████┃", "   ╚╝  ╚╝", ""],
        (0, MoodLevel::Normal, _) => &["    ╔══╗", "  ᕙ(・─・)ᕗ", "   ┃████┃", "   ╚╝  ╚╝", ""],
        (0, MoodLevel::Low, 0) => &["    ╔══╗", "  ᕙ(￣_￣)ᕗ", "   ┃████┃", "   ╚╝  ╚╝", ""],
        (0, MoodLevel::Low, _) => &["    ╔══╗", "  ᕙ(￣ ￣)ᕗ", "   ┃████┃", "   ╚╝  ╚╝", ""],
        (1, MoodLevel::High, 0) => &["   ／■＼", "  (≧益≧)9", "   |████|", "   ╚╝ ╚╝", ""],
        (1, MoodLevel::High, _) => &["   ／■＼ !", "  (≧益≧)9", "   |████|", "   ╚╝ ╚╝", ""],
        (1, MoodLevel::Normal, 0) => &["   ／■＼", "  (・益・)9", "   |████|", "   ╚╝ ╚╝", ""],
        (1, MoodLevel::Normal, _) => &["   ／■＼", "  (・─・)9", "   |████|", "   ╚╝ ╚╝", ""],
        (1, MoodLevel::Low, 0) => &["   ／■＼", "  (￣_￣)9", "   |████|", "   ╚╝ ╚╝", ""],
        (1, MoodLevel::Low, _) => &["   ／■＼", "  (￣ ￣)", "   |████|", "   ╚╝ ╚╝", ""],
        (_, MoodLevel::High, 0) => &["    ┏━┓", "  ᕙ(≧▽≧)ᕗ", "    ┃██┃", "   ╚╝╚╝", ""],
        (_, MoodLevel::High, _) => &["    ┏━┓ !", "  ᕙ(≧▽≧)ᕗ", "    ┃██┃", "   ╚╝╚╝", ""],
        (_, MoodLevel::Normal, 0) => &["    ┏━┓", "  ᕙ(・ω・)ᕗ", "    ┃██┃", "   ╚╝╚╝", ""],
        (_, MoodLevel::Normal, _) => &["    ┏━┓", "  ᕙ(・─・)ᕗ", "    ┃██┃", "   ╚╝╚╝", ""],
        (_, MoodLevel::Low, 0) => &["    ┏━┓", "  ᕙ(￣_￣)ᕗ", "    ┃██┃", "   ╚╝╚╝", ""],
        (_, MoodLevel::Low, _) => &["    ┏━┓", "  ᕙ(￣ ￣)ᕗ", "    ┃██┃", "   ╚╝╚╝", ""],
    }
}

// --- Stage3 Odayaka (larger, fluffier) ---
fn stage3_odayaka_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["   ☁☁☁", " ☁(˶≧▽≦)☁", "  ☁☁☁☁", "", ""],
        (0, MoodLevel::High, _) => &["   ☁☁☁ ♪", " ☁(˶≧▽≦)☁", "  ☁☁☁☁", "", ""],
        (0, MoodLevel::Normal, 0) => &["   ☁☁☁", " ☁(˶˘ᵕ˘)☁", "  ☁☁☁☁", "", ""],
        (0, MoodLevel::Normal, _) => &["   ☁☁☁", " ☁(˶˘─˘)☁", "  ☁☁☁☁", "", ""],
        (0, MoodLevel::Low, 0) => &["   ☁☁☁", " ☁(˶￣_￣)☁", "  ☁☁☁☁", "", ""],
        (0, MoodLevel::Low, _) => &["   ☁☁☁", " ☁(˶￣ ￣)☁", "  ☁☁☁☁", "", ""],
        (1, MoodLevel::High, 0) => &["   ～～～", "  (´▽`*)", "  ～～～～", "", ""],
        (1, MoodLevel::High, _) => &["   ～～～ ♪", "  (´▽`*)", "  ～～～～", "", ""],
        (1, MoodLevel::Normal, 0) => &["   ～～～", "  (´ω`*)", "  ～～～～", "", ""],
        (1, MoodLevel::Normal, _) => &["   ～～～", "  (´─`*)", "  ～～～～", "", ""],
        (1, MoodLevel::Low, 0) => &["   ～～～", "  (´_`*)", "  ～～～～", "", ""],
        (1, MoodLevel::Low, _) => &["   ～～～", "  (´ `*)", "  ～～～～", "", ""],
        (_, MoodLevel::High, 0) => &["   ＊＊＊", " ∩(≧ω≦)∩", "  ＊＊＊＊", "", ""],
        (_, MoodLevel::High, _) => &["   ＊＊＊ ♪", " ∩(≧ω≦)∩", "  ＊＊＊＊", "", ""],
        (_, MoodLevel::Normal, 0) => &["   ＊＊＊", " ∩(˘ω˘)∩", "  ＊＊＊＊", "", ""],
        (_, MoodLevel::Normal, _) => &["   ＊＊＊", " ∩(˘─˘)∩", "  ＊＊＊＊", "", ""],
        (_, MoodLevel::Low, 0) => &["   ＊＊＊", " ∩(￣_￣)∩", "  ＊＊＊＊", "", ""],
        (_, MoodLevel::Low, _) => &["   ＊＊＊", " ∩(￣ ￣)∩", "  ＊＊＊＊", "", ""],
    }
}

// --- Stage3 Bouken (larger, winged/star motifs) ---
fn stage3_bouken_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["    ★", " ＜(≧▽≦)＞", "    ┃┃", "   ╱  ╲", ""],
        (0, MoodLevel::High, _) => &["    ★ !", " ＜(≧▽≦)＞", "    ┃┃", "   ╱  ╲", ""],
        (0, MoodLevel::Normal, 0) => &["    ☆", " ＜(・ω・)＞", "    ┃┃", "   ╱  ╲", ""],
        (0, MoodLevel::Normal, _) => &["    ☆", " ＜(・─・)＞", "    ┃┃", "   ╱  ╲", ""],
        (0, MoodLevel::Low, 0) => &["", " ＜(￣_￣)＞", "    ┃┃", "   ╱  ╲", ""],
        (0, MoodLevel::Low, _) => &["", " ＜(￣ ￣)＞", "    ┃┃", "   ╱  ╲", ""],
        (1, MoodLevel::High, 0) => &["    ⚡", " ┗(≧▽≦)┛", "    ┃┃", "   ╱  ╲", ""],
        (1, MoodLevel::High, _) => &["    ⚡ !", " ┗(≧▽≦)┛", "    ┃┃", "   ╱  ╲", ""],
        (1, MoodLevel::Normal, 0) => &["", " ┗(・ω・)┛", "    ┃┃", "   ╱  ╲", ""],
        (1, MoodLevel::Normal, _) => &["", " ┗(・─・)┛", "    ┃┃", "   ╱  ╲", ""],
        (1, MoodLevel::Low, 0) => &["", " ┗(￣_￣)┛", "    ┃┃", "   ╱  ╲", ""],
        (1, MoodLevel::Low, _) => &["", " ┗(￣ ￣)┛", "    ┃┃", "   ╱  ╲", ""],
        (_, MoodLevel::High, 0) => &["    ☆★☆", " ≫(≧▽≦)ノ", "    ┃┃", "   ╱  ╲", ""],
        (_, MoodLevel::High, _) => &["    ★☆★", " ≫(≧▽≦)ノ", "    ┃┃", "   ╱  ╲", ""],
        (_, MoodLevel::Normal, 0) => &["    ☆", " ≫(´ω`)ノ", "    ┃┃", "   ╱  ╲", ""],
        (_, MoodLevel::Normal, _) => &["    ☆", " ≫(´─`)ノ", "    ┃┃", "   ╱  ╲", ""],
        (_, MoodLevel::Low, 0) => &["", " ≫(´_`)ノ", "    ┃┃", "   ╱  ╲", ""],
        (_, MoodLevel::Low, _) => &["", " ≫(´ `)ノ", "    ┃┃", "   ╱  ╲", ""],
    }
}

// --- Stage3 Normal type (slightly bigger, balanced) ---
fn stage3_normal_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["", "  (´・▽・`)ノ", "    |__|", "   / \\/ \\", ""],
        (0, MoodLevel::High, _) => &["", " (´・▽・`)ノ !", "    |__|", "   / \\/ \\", ""],
        (0, MoodLevel::Normal, 0) => &["", "  (´・ω・`)", "    |__|", "   / \\/ \\", ""],
        (0, MoodLevel::Normal, _) => &["", "  (´・─・`)", "    |__|", "   / \\/ \\", ""],
        (0, MoodLevel::Low, 0) => &["", "  (´・_・`)", "    |__|", "   / \\/ \\", ""],
        (0, MoodLevel::Low, _) => &["", "  (´・ ・`)", "    |__|", "   / \\/ \\", ""],
        (1, MoodLevel::High, 0) => &["", "  ＼(°▽°)／", "    |__|", "   / \\/ \\", ""],
        (1, MoodLevel::High, _) => &["", " ＼(°▽°)／ !", "    |__|", "   / \\/ \\", ""],
        (1, MoodLevel::Normal, 0) => &["", "   (°ω°)", "    |__|", "   / \\/ \\", ""],
        (1, MoodLevel::Normal, _) => &["", "   (°─°)", "    |__|", "   / \\/ \\", ""],
        (1, MoodLevel::Low, 0) => &["", "   (°_°)", "    |__|", "   / \\/ \\", ""],
        (1, MoodLevel::Low, _) => &["", "   (° °)", "    |__|", "   / \\/ \\", ""],
        (_, MoodLevel::High, 0) => &["", "  (˙▽˙)ノ", "    |__|", "   / \\/ \\", ""],
        (_, MoodLevel::High, _) => &["", " (˙▽˙)ノ ♪", "    |__|", "   / \\/ \\", ""],
        (_, MoodLevel::Normal, 0) => &["", "   (˙ᵕ˙)", "    |__|", "   / \\/ \\", ""],
        (_, MoodLevel::Normal, _) => &["", "   (˙─˙)", "    |__|", "   / \\/ \\", ""],
        (_, MoodLevel::Low, 0) => &["", "   (˙_˙)", "    |__|", "   / \\/ \\", ""],
        (_, MoodLevel::Low, _) => &["", "   (˙ ˙)", "    |__|", "   / \\/ \\", ""],
    }
}

// --- Stage3 Wild type (larger, eerie) ---
fn stage3_wild_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["   ≪≫≪≫", " ◉(⊙▽⊙)◉", "   ┃▓▓┃", "   ╱  ╲", ""],
        (0, MoodLevel::High, _) => &["   ≫≪≫≪", " ◉(⊙▽⊙)◉", "   ┃▓▓┃", "   ╱  ╲", ""],
        (0, MoodLevel::Normal, 0) => &["   ≪≫≪≫", " ◉(⊙_⊙)◉", "   ┃▓▓┃", "   ╱  ╲", ""],
        (0, MoodLevel::Normal, _) => &["   ≫≪≫≪", " ◉(⊙─⊙)◉", "   ┃▓▓┃", "   ╱  ╲", ""],
        (0, MoodLevel::Low, 0) => &["", " ◉(⊙ ⊙)◉", "   ┃▓▓┃", "   ╱  ╲", ""],
        (0, MoodLevel::Low, _) => &["", " ◉(- -)◉", "   ┃▓▓┃", "   ╱  ╲", ""],
        (1, MoodLevel::High, 0) => &["   ～⌇～", " ψ(⊙▽⊙)ψ", "   ┃▒▒┃", "   ╱  ╲", ""],
        (1, MoodLevel::High, _) => &["   ⌇～⌇", " ψ(⊙▽⊙)ψ", "   ┃▒▒┃", "   ╱  ╲", ""],
        (1, MoodLevel::Normal, 0) => &["", " ψ(⊙ω⊙)ψ", "   ┃▒▒┃", "   ╱  ╲", ""],
        (1, MoodLevel::Normal, _) => &["", " ψ(⊙─⊙)ψ", "   ┃▒▒┃", "   ╱  ╲", ""],
        (1, MoodLevel::Low, 0) => &["", " ψ(⊙_⊙)ψ", "   ┃▒▒┃", "   ╱  ╲", ""],
        (1, MoodLevel::Low, _) => &["", " ψ(- -)ψ", "   ┃▒▒┃", "   ╱  ╲", ""],
        (_, MoodLevel::High, 0) => &["   ‡‡‡‡", " ‡(◎▽◎)‡", "   ┃░░┃", "   ╱  ╲", ""],
        (_, MoodLevel::High, _) => &["   ‡‡‡‡ !", " ‡(◎▽◎)‡", "   ┃░░┃", "   ╱  ╲", ""],
        (_, MoodLevel::Normal, 0) => &["", " ‡(◎_◎)‡", "   ┃░░┃", "   ╱  ╲", ""],
        (_, MoodLevel::Normal, _) => &["", " ‡(◎─◎)‡", "   ┃░░┃", "   ╱  ╲", ""],
        (_, MoodLevel::Low, 0) => &["", " ‡(◎ ◎)‡", "   ┃░░┃", "   ╱  ╲", ""],
        (_, MoodLevel::Low, _) => &["", " ‡(- -)‡", "   ┃░░┃", "   ╱  ╲", ""],
    }
}

// --- Stage4 Mutation (special, unified across types) ---
fn stage4_art(variant: usize, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (variant % 3, mood, frame % 2) {
        (0, MoodLevel::High, 0) => &["  ╔═══╗", " ║(◎▽◎)║", " ╚═════╝", "  ███████", " ╚╝   ╚╝"],
        (0, MoodLevel::High, _) => &["  ╔═══╗ !", " ║(◎▽◎)║", " ╚═════╝", "  ███████", " ╚╝   ╚╝"],
        (0, MoodLevel::Normal, 0) => &["  ╔═══╗", " ║(◎ω◎)║", " ╚═════╝", "  ███████", " ╚╝   ╚╝"],
        (0, MoodLevel::Normal, _) => &["  ╔═══╗", " ║(◎─◎)║", " ╚═════╝", "  ███████", " ╚╝   ╚╝"],
        (0, MoodLevel::Low, 0) => &["  ╔═══╗", " ║(◎_◎)║", " ╚═════╝", "  ███████", " ╚╝   ╚╝"],
        (0, MoodLevel::Low, _) => &["  ╔═══╗", " ║(◎ ◎)║", " ╚═════╝", "  ███████", " ╚╝   ╚╝"],
        (1, MoodLevel::High, 0) => &["  ☆═══☆", " ║(★▽★)║", " ☆═══☆", "  ██▓██", " ╱╲   ╱╲"],
        (1, MoodLevel::High, _) => &["  ★═══★", " ║(☆▽☆)║", " ★═══★", "  ██▓██", " ╱╲   ╱╲"],
        (1, MoodLevel::Normal, 0) => &["  ☆═══☆", " ║(★ω★)║", " ☆═══☆", "  ██▓██", " ╱╲   ╱╲"],
        (1, MoodLevel::Normal, _) => &["  ☆═══☆", " ║(★─★)║", " ☆═══☆", "  ██▓██", " ╱╲   ╱╲"],
        (1, MoodLevel::Low, 0) => &["  ☆═══☆", " ║(★_★)║", " ☆═══☆", "  ██▓██", " ╱╲   ╱╲"],
        (1, MoodLevel::Low, _) => &["  ☆═══☆", " ║(★ ★)║", " ☆═══☆", "  ██▓██", " ╱╲   ╱╲"],
        (_, MoodLevel::High, 0) => &["  ◆◇◆◇◆", " ◇(◈▽◈)◇", " ◆◇◆◇◆", "   ████", "  ╚╝╚╝"],
        (_, MoodLevel::High, _) => &["  ◇◆◇◆◇", " ◆(◈▽◈)◆", " ◇◆◇◆◇", "   ████", "  ╚╝╚╝"],
        (_, MoodLevel::Normal, 0) => &["  ◆◇◆◇◆", " ◇(◈ω◈)◇", " ◆◇◆◇◆", "   ████", "  ╚╝╚╝"],
        (_, MoodLevel::Normal, _) => &["  ◆◇◆◇◆", " ◇(◈─◈)◇", " ◆◇◆◇◆", "   ████", "  ╚╝╚╝"],
        (_, MoodLevel::Low, 0) => &["  ◆◇◆◇◆", " ◇(◈_◈)◇", " ◆◇◆◇◆", "   ████", "  ╚╝╚╝"],
        (_, MoodLevel::Low, _) => &["  ◆◇◆◇◆", " ◇(◈ ◈)◇", " ◆◇◆◇◆", "   ████", "  ╚╝╚╝"],
    }
}
