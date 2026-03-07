use crate::game::actions::Action;
use crate::game::evolution::{self, EvoType, STAGE2_SPECIES, STAGE3_SPECIES, STAGE4_SPECIES};
use crate::game::pet::MoodLevel;

// ===== Public API =====

pub fn egg_art() -> &'static [&'static str] {
    &[
        "",
        "      ＿＿",
        "    （　　　）",
        "     ￣￣￣",
        "",
    ]
}

pub fn get_art(species: &str, mood: MoodLevel, frame: usize) -> Vec<String> {
    match species {
        "たまご" => to_vec(egg_art()),
        "コロコロ" => to_vec(korokoro_art(mood, frame)),
        "ニョロ" => to_vec(nyoro_art(mood, frame)),
        "フワ" => to_vec(fuwa_art(mood, frame)),
        "ツブ" => to_vec(tsubu_art(mood, frame)),
        "プク" => to_vec(puku_art(mood, frame)),
        "ミジン" => to_vec(mijin_art(mood, frame)),
        "ネロ" => to_vec(nero_art(mood, frame)),
        "ボテ" => to_vec(bote_art(mood, frame)),
        "ピリリ" => to_vec(piriri_art(mood, frame)),
        "モグモ" => to_vec(mogumo_art(mood, frame)),
        _ => compose_idle(species, mood, frame),
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

/// Species-specific action animation art.
pub fn get_action_art(species: &str, action: Action, frame: usize) -> Vec<String> {
    match species {
        "たまご" => to_vec(egg_art()),
        "コロコロ" => to_vec(korokoro_action(action, frame)),
        "ニョロ" => to_vec(nyoro_action(action, frame)),
        "フワ" => to_vec(fuwa_action(action, frame)),
        "ツブ" => to_vec(tsubu_action(action, frame)),
        "プク" => to_vec(puku_action(action, frame)),
        "ミジン" => to_vec(mijin_action(action, frame)),
        "ネロ" => to_vec(nero_action(action, frame)),
        "ボテ" => to_vec(bote_action(action, frame)),
        "ピリリ" => to_vec(piriri_action(action, frame)),
        "モグモ" => to_vec(mogumo_action(action, frame)),
        _ => compose_action(species, action, frame),
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

fn to_vec(art: &[&str]) -> Vec<String> {
    art.iter().map(|s| s.to_string()).collect()
}

// ===== Stage 1 Species Art (hand-crafted, 1-line each) =====

// 1. コロコロ（口: ω）- 丸いスライム
fn korokoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "    (≧▽≦)ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "   ﾉ(≧▽≦)", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "    (˘ω˘)", "", ""],
        (MoodLevel::Normal, _) => &["", "", "     (˘ω˘)", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    (￣_￣)", "", ""],
        (MoodLevel::Low, _) => &["", "", "    (￣ ￣)", "", ""],
    }
}

// 2. ニョロ（口: へ）- ヘビ/ミミズ
fn nyoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ～～(＞へ＜)！", "", ""],
        (MoodLevel::High, _) => &["", "", "  ！(＞へ＜)～～", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ～～(・へ・)", "", ""],
        (MoodLevel::Normal, _) => &["", "", "  (・へ・)～～", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  ～(￣へ￣)", "", ""],
        (MoodLevel::Low, _) => &["", "", "  (￣ ￣)～", "", ""],
    }
}

// 3. フワ（口: ᵕ）- ふわふわ雲
fn fuwa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ☁(≧▽≦)☁♪", "", ""],
        (MoodLevel::High, _) => &["", "", "  ♪☁(≧▽≦)☁", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ☁(˘ᵕ˘)☁", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   ☁(˘ᵕ˘)☁", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  ☁(￣_￣)☁", "", ""],
        (MoodLevel::Low, _) => &["", "", "   ☁(￣ ￣)☁", "", ""],
    }
}

// 4. ツブ（口: _）- 微小粒
fn tsubu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "    ⊙▽⊙ !", "", ""],
        (MoodLevel::High, _) => &["", "", "   ! ⊙▽⊙", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "    ⊙_⊙", "", ""],
        (MoodLevel::Normal, _) => &["", "", "    ⊙ ⊙", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    -_-", "", ""],
        (MoodLevel::Low, _) => &["", "", "    - -", "", ""],
    }
}

// 5. プク（口: ◡）- ぷくぷく
fn puku_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  (｡＞◡＜｡)ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ(｡＞◡＜｡)", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  (｡・◡・｡)", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   (｡・◡・｡)", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  (｡-◡-｡)", "", ""],
        (MoodLevel::Low, _) => &["", "", "  (｡- -｡)", "", ""],
    }
}

// 6. ミジン（口: ▿）- 極小微生物
fn mijin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "    ＞▿＜ !", "", ""],
        (MoodLevel::High, _) => &["", "", "   ! ＞▿＜", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "    ･▿･", "", ""],
        (MoodLevel::Normal, _) => &["", "", "     ･▿･", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    -▿-", "", ""],
        (MoodLevel::Low, _) => &["", "", "    - -", "", ""],
    }
}

// 7. ネロ（口: ー）- 子猫
fn nero_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  (=^▽^=)ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ(=^▽^=)", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  (=ー=)～", "", ""],
        (MoodLevel::Normal, _) => &["", "", "  (=ー=)～～", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  (=- -)zzZ", "", ""],
        (MoodLevel::Low, _) => &["", "", "  (= =)zzZ", "", ""],
    }
}

// 8. ボテ（口: □）- でっぷり
fn bote_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  《・▽・》ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ《・▽・》", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  《・□・》", "", ""],
        (MoodLevel::Normal, _) => &["", "", "  《・ ・》", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  《・_・》", "", ""],
        (MoodLevel::Low, _) => &["", "", "  《- -》", "", ""],
    }
}

// 9. ピリリ（口: ∀）- 電気
fn piriri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ⚡°▽°⚡!", "", ""],
        (MoodLevel::High, _) => &["", "", "  !⚡°▽°⚡", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ⚡°∀°⚡", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   ⚡°∀°⚡", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    °_°", "", ""],
        (MoodLevel::Low, _) => &["", "", "    ° °", "", ""],
    }
}

// 10. モグモ（口: 〇）- もぐもぐ耳つき
fn mogumo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ∩(°▽°)∩ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ∩(°▽°)∩", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ∩(°〇°)∩", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   ∩(°〇°)∩", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  ∩(°_°)∩", "", ""],
        (MoodLevel::Low, _) => &["", "", "  ∩(° °)∩", "", ""],
    }
}

// ===== Stage 1 Action Art (hand-crafted, 1-line each) =====

// 1. コロコロ（口: ω）
fn korokoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "    (˘ω˘)ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "   ﾉ(˘ω˘)", "", ""],
        (Action::Play, 0) => &["", "", "   (≧▽≦)ﾉ♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪ﾉ(≧▽≦)", "", ""],
        (Action::Train, 0) => &["", "", "    (≧ω≦)9", "", ""],
        (Action::Train, _) => &["", "", "   9(≧ω≦)", "", ""],
        (Action::Relax, 0) => &["", "", "    (˘ω˘)～", "", ""],
        (Action::Relax, _) => &["", "", "    (˘ω˘)～z", "", ""],
    }
}

// 2. ニョロ（口: へ）
fn nyoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ～～(・へ・)ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ(・へ・)～～", "", ""],
        (Action::Play, 0) => &["", "", "  ～～(＞へ＜)！♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪！(＞へ＜)～～", "", ""],
        (Action::Train, 0) => &["", "", "  ～～(｀へ´)9", "", ""],
        (Action::Train, _) => &["", "", "  9(｀へ´)～～", "", ""],
        (Action::Relax, 0) => &["", "", "  ～～(˘へ˘)～", "", ""],
        (Action::Relax, _) => &["", "", "  ～～(˘へ˘)～z", "", ""],
    }
}

// 3. フワ（口: ᵕ）
fn fuwa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ☁(˘ᵕ˘)ﾉ☁", "", ""],
        (Action::Talk, _) => &["", "", "  ☁ﾉ(˘ᵕ˘)☁", "", ""],
        (Action::Play, 0) => &["", "", "  ☁(≧▽≦)☁♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪☁(≧▽≦)☁", "", ""],
        (Action::Train, 0) => &["", "", "  ☁(≧ᵕ≦)9☁", "", ""],
        (Action::Train, _) => &["", "", "  ☁9(≧ᵕ≦)☁", "", ""],
        (Action::Relax, 0) => &["", "", "  ☁(˘ᵕ˘)☁～", "", ""],
        (Action::Relax, _) => &["", "", "  ☁(˘ᵕ˘)☁～z", "", ""],
    }
}

// 4. ツブ（口: _）
fn tsubu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "    ⊙_⊙ ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "   ﾉ ⊙_⊙", "", ""],
        (Action::Play, 0) => &["", "", "    ⊙▽⊙ !", "", ""],
        (Action::Play, _) => &["", "", "   ! ⊙▽⊙", "", ""],
        (Action::Train, 0) => &["", "", "    ⊙益⊙ !!", "", ""],
        (Action::Train, _) => &["", "", "   !! ⊙益⊙", "", ""],
        (Action::Relax, 0) => &["", "", "    ⊙_⊙ ～", "", ""],
        (Action::Relax, _) => &["", "", "    ⊙_⊙ ～z", "", ""],
    }
}

// 5. プク（口: ◡）
fn puku_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  (｡・◡・｡)ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ(｡・◡・｡)", "", ""],
        (Action::Play, 0) => &["", "", "  (｡＞◡＜｡)♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪(｡＞◡＜｡)", "", ""],
        (Action::Train, 0) => &["", "", "  (｡＞◡＜｡)9", "", ""],
        (Action::Train, _) => &["", "", "  9(｡＞◡＜｡)", "", ""],
        (Action::Relax, 0) => &["", "", "  (｡-◡-｡)～", "", ""],
        (Action::Relax, _) => &["", "", "  (｡-◡-｡)～z", "", ""],
    }
}

// 6. ミジン（口: ▿）
fn mijin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "    ･▿･ ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "   ﾉ ･▿･", "", ""],
        (Action::Play, 0) => &["", "", "    ＞▿＜ !", "", ""],
        (Action::Play, _) => &["", "", "   ! ＞▿＜", "", ""],
        (Action::Train, 0) => &["", "", "    ＞益＜ !!", "", ""],
        (Action::Train, _) => &["", "", "   !! ＞益＜", "", ""],
        (Action::Relax, 0) => &["", "", "    -▿- ～", "", ""],
        (Action::Relax, _) => &["", "", "    -▿- ～z", "", ""],
    }
}

// 7. ネロ（口: ー）
fn nero_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  (=ー=)ﾉ～", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ(=ー=)～", "", ""],
        (Action::Play, 0) => &["", "", "  (=^▽^=)♪～", "", ""],
        (Action::Play, _) => &["", "", "  ～♪(=^▽^=)", "", ""],
        (Action::Train, 0) => &["", "", "  (=`益´=)9～", "", ""],
        (Action::Train, _) => &["", "", "  ～9(=`益´=)", "", ""],
        (Action::Relax, 0) => &["", "", "  (=ー=)～zzZ", "", ""],
        (Action::Relax, _) => &["", "", "  (= =)～zzZ", "", ""],
    }
}

// 8. ボテ（口: □）
fn bote_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  《・□・》ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ《・□・》", "", ""],
        (Action::Play, 0) => &["", "", "  《・▽・》♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪《・▽・》", "", ""],
        (Action::Train, 0) => &["", "", "  《・益・》9", "", ""],
        (Action::Train, _) => &["", "", "  9《・益・》", "", ""],
        (Action::Relax, 0) => &["", "", "  《・□・》～", "", ""],
        (Action::Relax, _) => &["", "", "  《・ ・》～z", "", ""],
    }
}

// 9. ピリリ（口: ∀）
fn piriri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ⚡°∀°⚡ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ⚡°∀°⚡", "", ""],
        (Action::Play, 0) => &["", "", "  ⚡°▽°⚡♪!", "", ""],
        (Action::Play, _) => &["", "", "  !♪⚡°▽°⚡", "", ""],
        (Action::Train, 0) => &["", "", "  ⚡°益°⚡9", "", ""],
        (Action::Train, _) => &["", "", "  9⚡°益°⚡", "", ""],
        (Action::Relax, 0) => &["", "", "    °∀°～", "", ""],
        (Action::Relax, _) => &["", "", "    °∀°～z", "", ""],
    }
}

// 10. モグモ（口: 〇）
fn mogumo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ∩(°〇°)∩ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ∩(°〇°)∩", "", ""],
        (Action::Play, 0) => &["", "", "  ∩(°▽°)∩♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪∩(°▽°)∩", "", ""],
        (Action::Train, 0) => &["", "", "  ∩(°益°)∩9", "", ""],
        (Action::Train, _) => &["", "", "  9∩(°益°)∩", "", ""],
        (Action::Relax, 0) => &["", "", "  ∩(°〇°)∩～", "", ""],
        (Action::Relax, _) => &["", "", "  ∩(° °)∩～z", "", ""],
    }
}

// =====================================================================
// Composable Art System for Stage 2+ (unique visuals per species)
// =====================================================================
//
// Each species gets a unique visual through component combination:
// - Face: unique eye characters per species (selected by index within group)
// - Body: arm decorations, head/body/legs template (varies by evo type + sub-variant)
//
// Index within group is deterministic: based on position in the species array.

// --- Face component ---
struct FC {
    fl: &'static str,  // face frame left, e.g. "("
    fr: &'static str,  // face frame right, e.g. ")"
    e: &'static str,   // normal mood eye (same both sides)
    eh: &'static str,  // high mood eye
    el: &'static str,  // low mood eye
    mh: &'static str,  // high mood mouth
    mn: &'static str,  // normal mood mouth
}

// --- Body template ---
struct BC {
    al: &'static str,  // arm left
    ar: &'static str,  // arm right
    hd: &'static str,  // head decoration (line 0) or ""
    bd: &'static str,  // body (line 2) or ""
    lg: &'static str,  // legs (line 3) or ""
    hs: &'static str,  // high mood suffix " !" or " ♪"
}

// --- Stage 4 frame ---
struct S4F {
    top: &'static str,    // line 0: top frame
    fl: &'static str,     // face line frame left (replaces arm)
    fr: &'static str,     // face line frame right
    mid: &'static str,    // line 2: mid frame
    bd: &'static str,     // line 3: body
    lg: &'static str,     // line 4: legs
    hs: &'static str,     // high suffix
}

// ===== Face tables per evo type (20+ entries each for Stage 3 coverage) =====

const CHIKARA_FACES: &[FC] = &[
    FC { fl: "(", fr: ")", e: "・", eh: "≧", el: "￣", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "・", eh: "≧", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "°", eh: "°", el: "°", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "∀", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "◎", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "˙", eh: "˙", el: "˙", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◕", eh: "◕", el: "◕", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "★", eh: "★", el: "★", mh: "∀", mn: "益" },
    FC { fl: "(", fr: ")", e: "⊙", eh: "⊙", el: "⊙", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◆", eh: "◆", el: "◆", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "♦", eh: "♦", el: "♦", mh: "∀", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◇", eh: "◇", el: "◇", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "○", eh: "○", el: "○", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "□", eh: "□", el: "□", mh: "∀", mn: "益" },
    FC { fl: "(", fr: ")", e: "△", eh: "△", el: "△", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "▲", eh: "▲", el: "▲", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "■", eh: "■", el: "■", mh: "∀", mn: "ω" },
    FC { fl: "(", fr: ")", e: "⊕", eh: "⊕", el: "⊕", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "☆", eh: "☆", el: "☆", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "▪", eh: "▪", el: "▪", mh: "∀", mn: "益" },
];

const ODAYAKA_FACES: &[FC] = &[
    FC { fl: "(˶", fr: ")", e: "˘", eh: "≧", el: "￣", mh: "▽", mn: "ᵕ" },
    FC { fl: "(´", fr: "`)", e: "・", eh: "≧", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "˘", eh: "≧", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(˶", fr: ")", e: "°", eh: "°", el: "°", mh: "▽", mn: "ᵕ" },
    FC { fl: "(´", fr: "`)", e: "˘", eh: "≧", el: "￣", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "○", eh: "○", el: "○", mh: "▽", mn: "ᵕ" },
    FC { fl: "(˶", fr: ")", e: "◎", eh: "◎", el: "◎", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "°", eh: "°", el: "°", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "˙", eh: "˙", el: "˙", mh: "▽", mn: "ᵕ" },
    FC { fl: "(˶", fr: ")", e: "・", eh: "≧", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "˙", eh: "˙", el: "˙", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "◕", eh: "◕", el: "◕", mh: "▽", mn: "ω" },
    FC { fl: "(˶", fr: ")", e: "☆", eh: "☆", el: "☆", mh: "▽", mn: "ᵕ" },
    FC { fl: "(´", fr: "`)", e: "◇", eh: "◇", el: "◇", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "▽", mn: "ᵕ" },
    FC { fl: "(˶", fr: ")", e: "♥", eh: "♥", el: "♥", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "○", eh: "○", el: "○", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "◆", eh: "◆", el: "◆", mh: "▽", mn: "ω" },
    FC { fl: "(˶", fr: ")", e: "△", eh: "△", el: "△", mh: "▽", mn: "ᵕ" },
    FC { fl: "(´", fr: "`)", e: "★", eh: "★", el: "★", mh: "▽", mn: "ω" },
];

const BOUKEN_FACES: &[FC] = &[
    FC { fl: "(", fr: ")", e: "・", eh: "≧", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "＾", eh: "＾", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "・", eh: "≧", el: "￣", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "°", eh: "°", el: "°", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "▽", mn: "∀" },
    FC { fl: "(´", fr: "`)", e: "˙", eh: "˙", el: "˙", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "★", eh: "★", el: "★", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "◎", mh: "▽", mn: "∀" },
    FC { fl: "(´", fr: "`)", e: "°", eh: "°", el: "°", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◕", eh: "◕", el: "◕", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "⊙", eh: "⊙", el: "⊙", mh: "▽", mn: "∀" },
    FC { fl: "(´", fr: "`)", e: "☆", eh: "☆", el: "☆", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◆", eh: "◆", el: "◆", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "♦", eh: "♦", el: "♦", mh: "▽", mn: "∀" },
    FC { fl: "(´", fr: "`)", e: "◇", eh: "◇", el: "◇", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "△", eh: "△", el: "△", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "○", eh: "○", el: "○", mh: "▽", mn: "∀" },
    FC { fl: "(´", fr: "`)", e: "▲", eh: "▲", el: "▲", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "□", eh: "□", el: "□", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "⊕", eh: "⊕", el: "⊕", mh: "▽", mn: "∀" },
];

const NORMAL_FACES: &[FC] = &[
    FC { fl: "(´", fr: "`)", e: "・", eh: "・", el: "・", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "°", eh: "°", el: "°", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "˙", eh: "˙", el: "˙", mh: "▽", mn: "ᵕ" },
    FC { fl: "(´", fr: "`)", e: "˘", eh: "˘", el: "˘", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "・", eh: "・", el: "・", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "°", eh: "°", el: "°", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "˘", eh: "˘", el: "˘", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "◎", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "˙", eh: "˙", el: "˙", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "○", eh: "○", el: "○", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "◕", eh: "◕", el: "◕", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "☆", eh: "☆", el: "☆", mh: "▽", mn: "ᵕ" },
    FC { fl: "(´", fr: "`)", e: "◇", eh: "◇", el: "◇", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "△", eh: "△", el: "△", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "□", eh: "□", el: "□", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "◆", eh: "◆", el: "◆", mh: "▽", mn: "ω" },
    FC { fl: "(´", fr: "`)", e: "♦", eh: "♦", el: "♦", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "★", eh: "★", el: "★", mh: "▽", mn: "ᵕ" },
    FC { fl: "(", fr: ")", e: "⊙", eh: "⊙", el: "⊙", mh: "▽", mn: "ω" },
];

const WILD_FACES: &[FC] = &[
    FC { fl: "(", fr: ")", e: "⊙", eh: "⊙", el: "-", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "⊙", eh: "⊙", el: "-", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "-", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "-", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◕", eh: "◕", el: "◕", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "★", eh: "★", el: "★", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "☆", eh: "☆", el: "☆", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "◆", eh: "◆", el: "◆", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "♦", eh: "♦", el: "♦", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "◇", eh: "◇", el: "◇", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "○", eh: "○", el: "○", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "□", eh: "□", el: "□", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "△", eh: "△", el: "△", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "▲", eh: "▲", el: "▲", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "■", eh: "■", el: "■", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "⊕", eh: "⊕", el: "⊕", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "×", eh: "×", el: "×", mh: "▽", mn: "_" },
    FC { fl: "(", fr: ")", e: "▪", eh: "▪", el: "▪", mh: "▽", mn: "ω" },
];

// ===== Body templates per evo type (indexed by sub-variant) =====

const S2_CHIKARA_BODIES: &[BC] = &[
    BC { al: "ᕙ", ar: "ᕗ", hd: "", bd: "     ┃┃", lg: "    ╚╝╚╝", hs: " !" },
    BC { al: "",  ar: "ᕤ", hd: "", bd: "      |",  lg: "     / \\", hs: " !" },
    BC { al: "┏", ar: "┓", hd: "", bd: "     ┃┃", lg: "    ╚╝╚╝", hs: " !" },
    BC { al: "9", ar: "9", hd: "", bd: "      |",  lg: "     / \\", hs: " !" },
];

const S2_ODAYAKA_BODIES: &[BC] = &[
    BC { al: "☁", ar: "☁", hd: "", bd: "", lg: "", hs: " ♪" },
    BC { al: "∩", ar: "∩", hd: "", bd: "", lg: "", hs: " ♪" },
    BC { al: "__", ar: "__", hd: "", bd: "", lg: "", hs: " ♪" },
    BC { al: "～", ar: "～", hd: "", bd: "", lg: "", hs: " ♪" },
];

const S2_BOUKEN_BODIES: &[BC] = &[
    BC { al: "＜", ar: "＞", hd: "", bd: "      |", lg: "     / \\", hs: " !" },
    BC { al: "┗", ar: "┛", hd: "", bd: "      |", lg: "     / \\", hs: " !" },
    BC { al: "≫", ar: "ノ", hd: "", bd: "      |", lg: "     / \\", hs: " !" },
    BC { al: "⊂", ar: "⊃", hd: "", bd: "      |", lg: "     / \\", hs: " !" },
];

const S2_NORMAL_BODIES: &[BC] = &[
    BC { al: "", ar: "", hd: "", bd: "", lg: "", hs: " !" },
    BC { al: "", ar: "ノ", hd: "", bd: "", lg: "", hs: " !" },
    BC { al: "＼", ar: "／", hd: "", bd: "", lg: "", hs: " !" },
    BC { al: "ヽ", ar: "ノ", hd: "", bd: "", lg: "", hs: " ♪" },
];

const S2_WILD_BODIES: &[BC] = &[
    BC { al: "◉", ar: "◉", hd: "", bd: "", lg: "", hs: " !" },
    BC { al: "ψ", ar: "ψ", hd: "", bd: "", lg: "", hs: " !" },
    BC { al: "‡", ar: "‡", hd: "", bd: "", lg: "", hs: " !" },
    BC { al: "†", ar: "†", hd: "", bd: "", lg: "", hs: " !" },
];

// Stage 3 bodies (with head decorations and bigger bodies)
const S3_CHIKARA_BODIES: &[BC] = &[
    BC { al: "ᕙ", ar: "ᕗ", hd: "    ╔══╗", bd: "   ┃████┃", lg: "   ╚╝  ╚╝", hs: " !" },
    BC { al: "",  ar: "9",  hd: "   ／■＼", bd: "   |████|", lg: "   ╚╝ ╚╝",  hs: " !" },
    BC { al: "ᕙ", ar: "ᕗ", hd: "    ┏━┓",  bd: "    ┃██┃",  lg: "   ╚╝╚╝",   hs: " !" },
    BC { al: "┏", ar: "┓", hd: "    ╔═╗",  bd: "   ┃███┃",  lg: "   ╚╝ ╚╝",  hs: " !" },
    BC { al: "9", ar: "ᕤ", hd: "   ┌──┐",  bd: "   |████|", lg: "   ╚╝  ╚╝", hs: " !" },
];

const S3_ODAYAKA_BODIES: &[BC] = &[
    BC { al: "☁", ar: "☁", hd: "   ☁☁☁",  bd: "  ☁☁☁☁", lg: "", hs: " ♪" },
    BC { al: "",  ar: "",   hd: "   ～～～",  bd: "  ～～～～", lg: "", hs: " ♪" },
    BC { al: "∩", ar: "∩", hd: "   ＊＊＊", bd: "  ＊＊＊＊", lg: "", hs: " ♪" },
    BC { al: "~", ar: "~", hd: "   ○○○",   bd: "  ○○○○",  lg: "", hs: " ♪" },
    BC { al: "☁", ar: "☁", hd: "   ♪♪♪",  bd: "  ♪♪♪♪", lg: "", hs: " ♪" },
];

const S3_BOUKEN_BODIES: &[BC] = &[
    BC { al: "＜", ar: "＞", hd: "    ★",    bd: "    ┃┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "┗", ar: "┛", hd: "    ⚡",   bd: "    ┃┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "≫", ar: "ノ", hd: "    ☆★☆", bd: "    ┃┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "⊂", ar: "⊃", hd: "    ◆◇◆", bd: "    ┃┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "＜", ar: "＞", hd: "    ⊹⊹⊹", bd: "    ┃┃", lg: "   ╱  ╲", hs: " !" },
];

const S3_NORMAL_BODIES: &[BC] = &[
    BC { al: "",   ar: "",   hd: "", bd: "    |__|",  lg: "   / \\/ \\", hs: " !" },
    BC { al: "＼", ar: "／", hd: "", bd: "    |__|",  lg: "   / \\/ \\", hs: " !" },
    BC { al: "",   ar: "ノ", hd: "", bd: "    |__|",  lg: "   / \\/ \\", hs: " ♪" },
    BC { al: "ヽ", ar: "ノ", hd: "", bd: "    |__|",  lg: "   / \\/ \\", hs: " !" },
    BC { al: "",   ar: "",   hd: "", bd: "    |__|",  lg: "    /  \\",   hs: " ♪" },
];

const S3_WILD_BODIES: &[BC] = &[
    BC { al: "◉", ar: "◉", hd: "   ≪≫≪≫", bd: "   ┃▓▓┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "ψ", ar: "ψ", hd: "   ～⌇～",  bd: "   ┃▒▒┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "‡", ar: "‡", hd: "   ‡‡‡‡",  bd: "   ┃░░┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "†", ar: "†", hd: "   †††",    bd: "   ┃▓▓┃", lg: "   ╱  ╲", hs: " !" },
    BC { al: "◉", ar: "◉", hd: "   ◇◆◇◆",  bd: "   ┃▒▒┃", lg: "   ╱  ╲", hs: " !" },
];

// Stage 4 frames (8 variants for 8 species)
const S4_FRAMES: &[S4F] = &[
    S4F { top: "  ╔═══╗", fl: " ║", fr: "║", mid: " ╚═════╝", bd: "  ███████", lg: " ╚╝   ╚╝", hs: " !" },
    S4F { top: "  ☆═══☆", fl: " ║", fr: "║", mid: " ☆═══☆",   bd: "  ██▓██",   lg: " ╱╲   ╱╲", hs: " !" },
    S4F { top: "  ◆◇◆◇◆", fl: " ◇", fr: "◇", mid: " ◆◇◆◇◆", bd: "   ████",   lg: "  ╚╝╚╝",   hs: " !" },
    S4F { top: "  ┏━━━┓", fl: " ┃", fr: "┃", mid: " ┗━━━┛",   bd: "  ███████", lg: "  ╚╝ ╚╝",  hs: " !" },
    S4F { top: "  ╔╦═╦╗", fl: " ║", fr: "║", mid: " ╚╩═╩╝",   bd: "  ██▓██",   lg: " ╚╝   ╚╝", hs: " !" },
    S4F { top: "  ★═══★", fl: " ║", fr: "║", mid: " ★═══★",   bd: "  ██░██",   lg: " ╱╲   ╱╲", hs: " ♪" },
    S4F { top: "  ░▓█▓░", fl: " █", fr: "█", mid: " ░▓█▓░",   bd: "  ███████", lg: " ╚╝   ╚╝", hs: " !" },
    S4F { top: "  ○●○●○", fl: " ●", fr: "●", mid: " ●○●○●",   bd: "   ████",   lg: "  ╱╲╱╲",   hs: " !" },
];

// Stage 4 faces (8 entries)
const S4_FACES: &[FC] = &[
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "◎", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "★", eh: "★", el: "★", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◈", eh: "◈", el: "◈", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "◎", eh: "◎", el: "◎", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "●", eh: "●", el: "●", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "♥", eh: "♥", el: "♥", mh: "▽", mn: "ω" },
    FC { fl: "(", fr: ")", e: "⊙", eh: "⊙", el: "⊙", mh: "▽", mn: "益" },
    FC { fl: "(", fr: ")", e: "◆", eh: "◆", el: "◆", mh: "▽", mn: "ω" },
];

// ===== Species index lookup =====

/// Returns (stage, evo_type, index_within_group) for a species.
fn species_visual_info(species: &str) -> (u8, EvoType, usize) {
    for (i, s) in STAGE2_SPECIES.iter().enumerate() {
        if s.name == species {
            return (2, s.evo_type, i % 8);
        }
    }
    for (i, s) in STAGE3_SPECIES.iter().enumerate() {
        if s.name == species {
            let evo = evolution::get_evo_type(species).unwrap_or(EvoType::Normal);
            return (3, evo, i % 20);
        }
    }
    for (i, s) in STAGE4_SPECIES.iter().enumerate() {
        if s.name == species {
            return (4, EvoType::Normal, i);
        }
    }
    (2, EvoType::Normal, 0)
}

fn get_face(evo_type: EvoType, idx: usize) -> &'static FC {
    let table = match evo_type {
        EvoType::Chikara => CHIKARA_FACES,
        EvoType::Odayaka => ODAYAKA_FACES,
        EvoType::Bouken  => BOUKEN_FACES,
        EvoType::Normal  => NORMAL_FACES,
        EvoType::Wild    => WILD_FACES,
    };
    &table[idx % table.len()]
}

fn get_body(stage: u8, evo_type: EvoType, idx: usize) -> &'static BC {
    let table = match (stage, evo_type) {
        (2, EvoType::Chikara) => S2_CHIKARA_BODIES,
        (2, EvoType::Odayaka) => S2_ODAYAKA_BODIES,
        (2, EvoType::Bouken)  => S2_BOUKEN_BODIES,
        (2, EvoType::Normal)  => S2_NORMAL_BODIES,
        (2, EvoType::Wild)    => S2_WILD_BODIES,
        (3, EvoType::Chikara) => S3_CHIKARA_BODIES,
        (3, EvoType::Odayaka) => S3_ODAYAKA_BODIES,
        (3, EvoType::Bouken)  => S3_BOUKEN_BODIES,
        (3, EvoType::Normal)  => S3_NORMAL_BODIES,
        (3, EvoType::Wild)    => S3_WILD_BODIES,
        _ => S2_NORMAL_BODIES,
    };
    &table[idx % table.len()]
}

// ===== Face string composition =====

fn make_face(fc: &FC, mood: MoodLevel, frame: usize) -> String {
    let f = frame % 2;
    match (mood, f) {
        (MoodLevel::High, _) =>
            format!("{}{}{}{}{}", fc.fl, fc.eh, fc.mh, fc.eh, fc.fr),
        (MoodLevel::Normal, 0) =>
            format!("{}{}{}{}{}", fc.fl, fc.e, fc.mn, fc.e, fc.fr),
        (MoodLevel::Normal, _) =>
            format!("{}{}─{}{}", fc.fl, fc.e, fc.e, fc.fr),
        (MoodLevel::Low, 0) =>
            format!("{}{}_{}{}", fc.fl, fc.el, fc.el, fc.fr),
        (MoodLevel::Low, _) =>
            format!("{}{} {}{}", fc.fl, fc.el, fc.el, fc.fr),
    }
}

fn make_action_face(fc: &FC, action: Action) -> String {
    match action {
        Action::Talk =>
            format!("{}{}{}{}{}", fc.fl, fc.e, fc.mn, fc.e, fc.fr),
        Action::Play =>
            format!("{}{}{}{}{}", fc.fl, fc.eh, fc.mh, fc.eh, fc.fr),
        Action::Train =>
            format!("{}{}益{}{}", fc.fl, fc.eh, fc.eh, fc.fr),
        Action::Relax =>
            format!("{}{}_{}{}", fc.fl, fc.el, fc.el, fc.fr),
    }
}

// ===== Idle art composition =====

fn compose_idle(species: &str, mood: MoodLevel, frame: usize) -> Vec<String> {
    let (stage, evo_type, idx) = species_visual_info(species);

    if stage == 4 {
        return compose_s4_idle(idx, mood, frame);
    }

    let fc = get_face(evo_type, idx);
    let bc = get_body(stage, evo_type, idx);
    let face = make_face(fc, mood, frame);

    let f = frame % 2;
    let suffix = if mood == MoodLevel::High && f == 1 { bc.hs } else { "" };

    let face_line = format!("   {}{}{}{}", bc.al, face, bc.ar, suffix);

    vec![
        bc.hd.to_string(),
        face_line,
        bc.bd.to_string(),
        bc.lg.to_string(),
        String::new(),
    ]
}

fn compose_s4_idle(idx: usize, mood: MoodLevel, frame: usize) -> Vec<String> {
    let sf = &S4_FRAMES[idx % S4_FRAMES.len()];
    let fc = &S4_FACES[idx % S4_FACES.len()];
    let face = make_face(fc, mood, frame);

    let f = frame % 2;
    let top = if mood == MoodLevel::High && f == 1 {
        format!("{}{}", sf.top, sf.hs)
    } else {
        sf.top.to_string()
    };

    vec![
        top,
        format!(" {}{}{}", sf.fl, face, sf.fr),
        sf.mid.to_string(),
        sf.bd.to_string(),
        sf.lg.to_string(),
    ]
}

// ===== Action art composition =====

fn compose_action(species: &str, action: Action, frame: usize) -> Vec<String> {
    let (stage, evo_type, idx) = species_visual_info(species);

    if stage == 4 {
        return compose_s4_action(idx, action, frame);
    }

    let fc = get_face(evo_type, idx);
    let bc = get_body(stage, evo_type, idx);
    let face = make_action_face(fc, action);
    let f = frame % 2;

    let (arm_l, arm_r, suffix) = match (action, f) {
        (Action::Talk, 0) => (bc.al, "ﾉ", ""),
        (Action::Talk, _) => ("ﾉ", bc.ar, ""),
        (Action::Play, 0) => (bc.al, bc.ar, " ♪"),
        (Action::Play, _) => (bc.al, bc.ar, ""),
        (Action::Train, 0) => (bc.al, "9", " !!"),
        (Action::Train, _) => ("9", bc.ar, ""),
        (Action::Relax, 0) => ("_", "_", ""),
        (Action::Relax, _) => ("_", "_", " z"),
    };

    let face_line = format!("   {}{}{}{}", arm_l, face, arm_r, suffix);

    // For Play frame 1, shift the ♪ to the left (prefix)
    let face_line = if action == Action::Play && f == 1 {
        format!(" ♪ {}{}{}", arm_l, face, arm_r)
    } else {
        face_line
    };

    vec![
        bc.hd.to_string(),
        face_line,
        bc.bd.to_string(),
        bc.lg.to_string(),
        String::new(),
    ]
}

fn compose_s4_action(idx: usize, action: Action, frame: usize) -> Vec<String> {
    let sf = &S4_FRAMES[idx % S4_FRAMES.len()];
    let fc = &S4_FACES[idx % S4_FACES.len()];
    let face = make_action_face(fc, action);
    let f = frame % 2;

    let (face_line, top) = match (action, f) {
        (Action::Talk, 0) => (
            format!(" {}{}ﾉ", sf.fl, face),
            sf.top.to_string(),
        ),
        (Action::Talk, _) => (
            format!("ﾉ{}{}", face, sf.fr),
            sf.top.to_string(),
        ),
        (Action::Play, 0) => (
            format!(" {}{}{}", sf.fl, face, sf.fr),
            format!("{} ♪", sf.top),
        ),
        (Action::Play, _) => (
            format!(" {}{}{}", sf.fl, face, sf.fr),
            format!("♪ {}", sf.top),
        ),
        (Action::Train, 0) => (
            format!(" {}{}{}", sf.fl, face, sf.fr),
            format!("{} !!", sf.top),
        ),
        (Action::Train, _) => (
            format!(" {}{}{}", sf.fl, face, sf.fr),
            format!("!!{}", sf.top),
        ),
        (Action::Relax, 0) => (
            format!(" {}{}{}", sf.fl, face, sf.fr),
            sf.top.to_string(),
        ),
        (Action::Relax, _) => (
            format!(" {}{}{} z", sf.fl, face, sf.fr),
            sf.top.to_string(),
        ),
    };

    vec![
        top,
        face_line,
        sf.mid.to_string(),
        sf.bd.to_string(),
        sf.lg.to_string(),
    ]
}
