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
        // Stage 2 - Chikara type
        "ドタン" => to_vec(dotan_art(mood, frame)),
        "ガシャ" => to_vec(gasha_art(mood, frame)),
        "ズンズン" => to_vec(zunzun_art(mood, frame)),
        "デカオ" => to_vec(dekao_art(mood, frame)),
        "ゴツモリ" => to_vec(gotsumori_art(mood, frame)),
        "ドンガメ" => to_vec(dongame_art(mood, frame)),
        // Stage 2 - Odayaka type
        "ヒョロン" => to_vec(hyoron_art(mood, frame)),
        "フワモン" => to_vec(fuwamon_art(mood, frame)),
        "ユラリ" => to_vec(yurari_art(mood, frame)),
        "ネムタ" => to_vec(nemuta_art(mood, frame)),
        "ポワン" => to_vec(powan_art(mood, frame)),
        "ホワモコ" => to_vec(howamoko_art(mood, frame)),
        // Stage 2 - Bouken type
        "クルル" => to_vec(kururu_art(mood, frame)),
        "トゲたろう" => to_vec(togetarou_art(mood, frame)),
        "ハネオ" => to_vec(haneo_art(mood, frame)),
        "ビョーン" => to_vec(byoon_art(mood, frame)),
        "ダッシュ" => to_vec(dashu_art(mood, frame)),
        "グルグル" => to_vec(guruguru_art(mood, frame)),
        // Stage 2 - Normal type
        "ペタ" => to_vec(peta_art(mood, frame)),
        "ノホホ" => to_vec(nohoho_art(mood, frame)),
        "マジメ" => to_vec(majime_art(mood, frame)),
        "フツウ" => to_vec(futsuu_art(mood, frame)),
        "ナミナミ" => to_vec(naminami_art(mood, frame)),
        "テキトー" => to_vec(tekitoo_art(mood, frame)),
        // Stage 2 - Wild type
        "メダマ" => to_vec(medama_art(mood, frame)),
        "ケモノ" => to_vec(kemono_art(mood, frame)),
        "ヌシ" => to_vec(nushi_art(mood, frame)),
        "カゲ" => to_vec(kage_art(mood, frame)),
        "ザワザワ" => to_vec(zawazawa_art(mood, frame)),
        "ヒトダマ" => to_vec(hitodama_art(mood, frame)),
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
        // Stage 2 - Chikara type
        "ドタン" => to_vec(dotan_action(action, frame)),
        "ガシャ" => to_vec(gasha_action(action, frame)),
        "ズンズン" => to_vec(zunzun_action(action, frame)),
        "デカオ" => to_vec(dekao_action(action, frame)),
        "ゴツモリ" => to_vec(gotsumori_action(action, frame)),
        "ドンガメ" => to_vec(dongame_action(action, frame)),
        // Stage 2 - Odayaka type
        "ヒョロン" => to_vec(hyoron_action(action, frame)),
        "フワモン" => to_vec(fuwamon_action(action, frame)),
        "ユラリ" => to_vec(yurari_action(action, frame)),
        "ネムタ" => to_vec(nemuta_action(action, frame)),
        "ポワン" => to_vec(powan_action(action, frame)),
        "ホワモコ" => to_vec(howamoko_action(action, frame)),
        // Stage 2 - Bouken type
        "クルル" => to_vec(kururu_action(action, frame)),
        "トゲたろう" => to_vec(togetarou_action(action, frame)),
        "ハネオ" => to_vec(haneo_action(action, frame)),
        "ビョーン" => to_vec(byoon_action(action, frame)),
        "ダッシュ" => to_vec(dashu_action(action, frame)),
        "グルグル" => to_vec(guruguru_action(action, frame)),
        // Stage 2 - Normal type
        "ペタ" => to_vec(peta_action(action, frame)),
        "ノホホ" => to_vec(nohoho_action(action, frame)),
        "マジメ" => to_vec(majime_action(action, frame)),
        "フツウ" => to_vec(futsuu_action(action, frame)),
        "ナミナミ" => to_vec(naminami_action(action, frame)),
        "テキトー" => to_vec(tekitoo_action(action, frame)),
        // Stage 2 - Wild type
        "メダマ" => to_vec(medama_action(action, frame)),
        "ケモノ" => to_vec(kemono_action(action, frame)),
        "ヌシ" => to_vec(nushi_action(action, frame)),
        "カゲ" => to_vec(kage_action(action, frame)),
        "ザワザワ" => to_vec(zawazawa_action(action, frame)),
        "ヒトダマ" => to_vec(hitodama_action(action, frame)),
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

// ===== Stage 2 Species Art (hand-crafted, 2-line each) =====

// --- Chikara type ---

// 1. ドタン - Muscular stomper with thick arms
fn dotan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  ᕙ(≧▽≧)ᕗ!", "    ╚╝╚╝", "", ""],
        (MoodLevel::High, _) => &["", " !ᕙ(≧▽≧)ᕗ", "    ╚╝╚╝", "", ""],
        (MoodLevel::Normal, 0) => &["", "  ᕙ(・益・)ᕗ", "    ╚╝╚╝", "", ""],
        (MoodLevel::Normal, _) => &["", "   ᕙ(・益・)ᕗ", "     ╚╝╚╝", "", ""],
        (MoodLevel::Low, 0) => &["", "  ᕙ(￣_￣)ᕗ", "    ╚╝╚╝", "", ""],
        (MoodLevel::Low, _) => &["", "  ᕙ(￣ ￣)ᕗ", "    ╚╝╚╝", "", ""],
    }
}

fn dotan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  ᕙ(・益・)ᕗﾉ", "    ╚╝╚╝", "", ""],
        (Action::Talk, _) => &["", " ﾉᕙ(・益・)ᕗ", "    ╚╝╚╝", "", ""],
        (Action::Play, 0) => &["", "  ᕙ(≧▽≧)ᕗ♪", "    ╚╝╚╝", "", ""],
        (Action::Play, _) => &["", " ♪ᕙ(≧▽≧)ᕗ", "    ╚╝╚╝", "", ""],
        (Action::Train, 0) => &["", "  ᕙ(≧益≧)ᕗ!!", "    ╚╝  ╚╝", "", ""],
        (Action::Train, _) => &["", "  ᕙ(≧益≧)ᕗ", "   ╚╝╚╝", "", ""],
        (Action::Relax, 0) => &["", "  ᕙ(˘_˘)ᕗ～", "    ╚╝╚╝", "", ""],
        (Action::Relax, _) => &["", "  ᕙ(˘ ˘)ᕗ～z", "    ╚╝╚╝", "", ""],
    }
}

// 2. ガシャ - Armored shell creature
fn gasha_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  ┏(≧▽≧)┓!", "   ┗━━━┛", "", ""],
        (MoodLevel::High, _) => &["", " !┏(≧▽≧)┓", "   ┗━━━┛", "", ""],
        (MoodLevel::Normal, 0) => &["", "  ┏(・ω・)┓", "   ┗━━━┛", "", ""],
        (MoodLevel::Normal, _) => &["", "   ┏(・ω・)┓", "    ┗━━━┛", "", ""],
        (MoodLevel::Low, 0) => &["", "  ┏(￣_￣)┓", "   ┗━━━┛", "", ""],
        (MoodLevel::Low, _) => &["", "  ┏(￣ ￣)┓", "   ┗━━━┛", "", ""],
    }
}

fn gasha_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  ┏(・ω・)┓ﾉ", "   ┗━━━┛", "", ""],
        (Action::Talk, _) => &["", " ﾉ┏(・ω・)┓", "   ┗━━━┛", "", ""],
        (Action::Play, 0) => &["", "  ┏(≧▽≧)┓♪", "   ┗━━━┛", "", ""],
        (Action::Play, _) => &["", " ♪┏(≧▽≧)┓", "   ┗━━━┛", "", ""],
        (Action::Train, 0) => &["", "  ┏(≧益≧)┓!!", "   ┗━━━━┛", "", ""],
        (Action::Train, _) => &["", "  ┏(≧益≧)┓", "  ┗━━━━┛", "", ""],
        (Action::Relax, 0) => &["", "  ┏(˘ω˘)┓～", "   ┗━━━┛", "", ""],
        (Action::Relax, _) => &["", "  ┏(˘ ˘)┓～z", "   ┗━━━┛", "", ""],
    }
}

// 3. ズンズン - Forward-charging marcher
fn zunzun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  >(°▽°)>!", "    /▓▓\\", "", ""],
        (MoodLevel::High, _) => &["", "   >(°▽°)>!", "     /▓▓\\", "", ""],
        (MoodLevel::Normal, 0) => &["", "  >(°益°)>", "    /▓▓\\", "", ""],
        (MoodLevel::Normal, _) => &["", "   >(°益°)>", "     /▓▓\\", "", ""],
        (MoodLevel::Low, 0) => &["", "  >(°_°)>", "    /▓▓\\", "", ""],
        (MoodLevel::Low, _) => &["", "  >(° °)>", "    /▓▓\\", "", ""],
    }
}

fn zunzun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  >(°益°)>ﾉ", "    /▓▓\\", "", ""],
        (Action::Talk, _) => &["", " ﾉ>(°益°)>", "    /▓▓\\", "", ""],
        (Action::Play, 0) => &["", "  >(°▽°)>♪", "    /▓▓\\", "", ""],
        (Action::Play, _) => &["", " ♪>(°▽°)>", "     /▓▓\\", "", ""],
        (Action::Train, 0) => &["", "  >>(°益°)>>!!", "     /▓▓\\", "", ""],
        (Action::Train, _) => &["", "  >(°益°)>", "   /▓▓\\", "", ""],
        (Action::Relax, 0) => &["", "  >(˘_˘)>～", "    /▓▓\\", "", ""],
        (Action::Relax, _) => &["", "  >(˘ ˘)>～z", "    /▓▓\\", "", ""],
    }
}

// 4. デカオ - Comically huge round face
fn dekao_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ((◎▽◎))!", "     ωω", "", ""],
        (MoodLevel::High, _) => &["", "!((◎▽◎))", "     ωω", "", ""],
        (MoodLevel::Normal, 0) => &["", " ((◎ω◎))", "     ωω", "", ""],
        (MoodLevel::Normal, _) => &["", "  ((◎ω◎))", "      ωω", "", ""],
        (MoodLevel::Low, 0) => &["", " ((◎_◎))", "     ωω", "", ""],
        (MoodLevel::Low, _) => &["", " ((◎ ◎))", "     ωω", "", ""],
    }
}

fn dekao_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ((◎ω◎))ﾉ", "     ωω", "", ""],
        (Action::Talk, _) => &["", "ﾉ((◎ω◎))", "     ωω", "", ""],
        (Action::Play, 0) => &["", " ((◎▽◎))♪", "     ωω", "", ""],
        (Action::Play, _) => &["", "♪((◎▽◎))", "     ωω", "", ""],
        (Action::Train, 0) => &["", " ((◎益◎))9", "     ωω", "", ""],
        (Action::Train, _) => &["", "9((◎益◎))", "     ωω", "", ""],
        (Action::Relax, 0) => &["", " ((◎_◎))～", "     ωω", "", ""],
        (Action::Relax, _) => &["", " ((◎ ◎))～z", "     ωω", "", ""],
    }
}

// 5. ゴツモリ - Rocky armored creature
fn gotsumori_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ◆(▲▽▲)◆!", "  ╚████╝", "", ""],
        (MoodLevel::High, _) => &["", "!◆(▲▽▲)◆", "  ╚████╝", "", ""],
        (MoodLevel::Normal, 0) => &["", " ◆(▲益▲)◆", "  ╚████╝", "", ""],
        (MoodLevel::Normal, _) => &["", "  ◆(▲益▲)◆", "   ╚████╝", "", ""],
        (MoodLevel::Low, 0) => &["", " ◆(▲_▲)◆", "  ╚████╝", "", ""],
        (MoodLevel::Low, _) => &["", " ◆(▲ ▲)◆", "  ╚████╝", "", ""],
    }
}

fn gotsumori_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ◆(▲益▲)◆ﾉ", "  ╚████╝", "", ""],
        (Action::Talk, _) => &["", "ﾉ◆(▲益▲)◆", "  ╚████╝", "", ""],
        (Action::Play, 0) => &["", " ◆(▲▽▲)◆♪", "  ╚████╝", "", ""],
        (Action::Play, _) => &["", "♪◆(▲▽▲)◆", "  ╚████╝", "", ""],
        (Action::Train, 0) => &["", " ◆(▲益▲)◆!!", "  ╚████╝", "", ""],
        (Action::Train, _) => &["", " ◆(▲益▲)◆", " ╚████╝", "", ""],
        (Action::Relax, 0) => &["", " ◆(▲_▲)◆～", "  ╚████╝", "", ""],
        (Action::Relax, _) => &["", " ◆(▲ ▲)◆～z", "  ╚████╝", "", ""],
    }
}

// 6. ドンガメ - Slow turtle with shell
fn dongame_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (◎▽◎)⊃!", "  甲甲甲甲", "", ""],
        (MoodLevel::High, _) => &["", " !(◎▽◎)⊃", "  甲甲甲甲", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (◎ω◎)⊃", "  甲甲甲甲", "", ""],
        (MoodLevel::Normal, _) => &["", "   (◎ω◎)⊃", "   甲甲甲甲", "", ""],
        (MoodLevel::Low, 0) => &["", "  (◎_◎)⊃", "  甲甲甲甲", "", ""],
        (MoodLevel::Low, _) => &["", "  (◎ ◎)⊃", "  甲甲甲甲", "", ""],
    }
}

fn dongame_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (◎ω◎)⊃ﾉ", "  甲甲甲甲", "", ""],
        (Action::Talk, _) => &["", " ﾉ(◎ω◎)⊃", "  甲甲甲甲", "", ""],
        (Action::Play, 0) => &["", "  (◎▽◎)⊃♪", "  甲甲甲甲", "", ""],
        (Action::Play, _) => &["", " ♪(◎▽◎)⊃", "   甲甲甲甲", "", ""],
        (Action::Train, 0) => &["", "  (◎益◎)⊃!!", "  甲甲甲甲", "", ""],
        (Action::Train, _) => &["", "  (◎益◎)⊃", "  甲甲甲甲", "", ""],
        (Action::Relax, 0) => &["", "  (◎_◎)⊃～", "  甲甲甲甲", "", ""],
        (Action::Relax, _) => &["", "  (◎ ◎)⊃～z", "  甲甲甲甲", "", ""],
    }
}

// --- Odayaka type ---

// 7. ヒョロン - Thin and lanky
fn hyoron_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)ﾉ♪", "   |  |", "", ""],
        (MoodLevel::High, _) => &["", " ♪ﾉ(˘▽˘)", "    |  |", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘ᵕ˘)ﾉ", "   |  |", "", ""],
        (MoodLevel::Normal, _) => &["", "   (˘ᵕ˘)ﾉ", "    |  |", "", ""],
        (MoodLevel::Low, 0) => &["", "  (￣_￣)", "   |  |", "", ""],
        (MoodLevel::Low, _) => &["", "  (￣ ￣)", "   |  |", "", ""],
    }
}

fn hyoron_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘ᵕ˘)ﾉ", "   |  |", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘ᵕ˘)", "   |  |", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)ﾉ♪", "   |  |", "", ""],
        (Action::Play, _) => &["", " ♪ﾉ(˘▽˘)", "    |  |", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9", "   |  |", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "   |  |", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)～", "   |  |", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)～z", "   |  |", "", ""],
    }
}

// 8. フワモン - Cloud-wrapped fluffy monster
fn fuwamon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ☁(≧▽≦)☁♪", "  ☁☁☁☁", "", ""],
        (MoodLevel::High, _) => &["", "♪☁(≧▽≦)☁", "  ☁☁☁☁", "", ""],
        (MoodLevel::Normal, 0) => &["", " ☁(˘ω˘)☁", "  ☁☁☁☁", "", ""],
        (MoodLevel::Normal, _) => &["", "  ☁(˘ω˘)☁", "   ☁☁☁☁", "", ""],
        (MoodLevel::Low, 0) => &["", " ☁(￣_￣)☁", "  ☁☁☁☁", "", ""],
        (MoodLevel::Low, _) => &["", " ☁(￣ ￣)☁", "  ☁☁☁☁", "", ""],
    }
}

fn fuwamon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ☁(˘ω˘)☁ﾉ", "  ☁☁☁☁", "", ""],
        (Action::Talk, _) => &["", "ﾉ☁(˘ω˘)☁", "  ☁☁☁☁", "", ""],
        (Action::Play, 0) => &["", " ☁(≧▽≦)☁♪", "  ☁☁☁☁", "", ""],
        (Action::Play, _) => &["", "♪☁(≧▽≦)☁", "   ☁☁☁☁", "", ""],
        (Action::Train, 0) => &["", " ☁(≧益≦)☁!", "  ☁☁☁☁", "", ""],
        (Action::Train, _) => &["", " ☁(≧益≦)☁", "  ☁☁☁☁", "", ""],
        (Action::Relax, 0) => &["", " ☁(˘_˘)☁～", "  ☁☁☁☁", "", ""],
        (Action::Relax, _) => &["", " ☁(˘ ˘)☁～z", "  ☁☁☁☁", "", ""],
    }
}

// 9. ユラリ - Jellyfish-like swayer
fn yurari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ～(・▽・)～♪", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::High, _) => &["", "♪～(・▽・)～", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, 0) => &["", " ～(・ᵕ・)～", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, _) => &["", "  ～(・ᵕ・)～", "    ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, 0) => &["", " ～(・_・)～", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, _) => &["", " ～(・ ・)～", "   ∫∫∫∫∫", "", ""],
    }
}

fn yurari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ～(・ᵕ・)～ﾉ", "   ∫∫∫∫∫", "", ""],
        (Action::Talk, _) => &["", "ﾉ～(・ᵕ・)～", "   ∫∫∫∫∫", "", ""],
        (Action::Play, 0) => &["", " ～(・▽・)～♪", "   ∫∫∫∫∫", "", ""],
        (Action::Play, _) => &["", "♪～(・▽・)～", "    ∫∫∫∫∫", "", ""],
        (Action::Train, 0) => &["", " ～(・益・)～!", "   ∫∫∫∫∫", "", ""],
        (Action::Train, _) => &["", " ～(・益・)～", "   ∫∫∫∫∫", "", ""],
        (Action::Relax, 0) => &["", " ～(・_・)～～", "   ∫∫∫∫∫", "", ""],
        (Action::Relax, _) => &["", " ～(・ ・)～～z", "   ∫∫∫∫∫", "", ""],
    }
}

// 10. ネムタ - Always sleepy, pudgy
fn nemuta_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)ﾉ!", "   ∪∪∪", "", ""],
        (MoodLevel::High, _) => &["", " !ﾉ(˘▽˘)", "   ∪∪∪", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘o˘)zzZ", "   ∪∪∪", "", ""],
        (MoodLevel::Normal, _) => &["", "  (˘o˘)zZ", "    ∪∪∪", "", ""],
        (MoodLevel::Low, 0) => &["", "  (￣o￣)zzZ", "   ∪∪∪", "", ""],
        (MoodLevel::Low, _) => &["", "  (￣ ￣)zZ", "   ∪∪∪", "", ""],
    }
}

fn nemuta_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘o˘)ﾉ", "   ∪∪∪", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘o˘)", "   ∪∪∪", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)ﾉ♪", "   ∪∪∪", "", ""],
        (Action::Play, _) => &["", " ♪ﾉ(˘▽˘)", "   ∪∪∪", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9!", "   ∪∪∪", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "   ∪∪∪", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)zzZ", "   ∪∪∪", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)zzZZ", "   ∪∪∪", "", ""],
    }
}

// 11. ポワン - Dreamy bubble creature
fn powan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " °(˘▽˘)°♪", "   °○°○", "", ""],
        (MoodLevel::High, _) => &["", "♪°(˘▽˘)°", "   °○°○", "", ""],
        (MoodLevel::Normal, 0) => &["", " °(˘ᵕ˘)°", "   °○°○", "", ""],
        (MoodLevel::Normal, _) => &["", "  °(˘ᵕ˘)°", "    °○°○", "", ""],
        (MoodLevel::Low, 0) => &["", " °(￣_￣)°", "   °○°○", "", ""],
        (MoodLevel::Low, _) => &["", " °(￣ ￣)°", "   °○°○", "", ""],
    }
}

fn powan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " °(˘ᵕ˘)°ﾉ", "   °○°○", "", ""],
        (Action::Talk, _) => &["", "ﾉ°(˘ᵕ˘)°", "   °○°○", "", ""],
        (Action::Play, 0) => &["", " °(˘▽˘)°♪", "   °○°○", "", ""],
        (Action::Play, _) => &["", "♪°(˘▽˘)°", "    °○°○", "", ""],
        (Action::Train, 0) => &["", " °(˘益˘)°!", "   °○°○", "", ""],
        (Action::Train, _) => &["", " °(˘益˘)°", "   °○°○", "", ""],
        (Action::Relax, 0) => &["", " °(˘_˘)°～", "   °○°○", "", ""],
        (Action::Relax, _) => &["", " °(˘ ˘)°～z", "   °○°○", "", ""],
    }
}

// 12. ホワモコ - Warm fluffy creature with ears
fn howamoko_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ∩(˘▽˘)∩♪", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::High, _) => &["", "♪∩(˘▽˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Normal, 0) => &["", " ∩(˘ω˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Normal, _) => &["", "  ∩(˘ω˘)∩", "   ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Low, 0) => &["", " ∩(￣_￣)∩", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Low, _) => &["", " ∩(￣ ￣)∩", "  ﾜﾜﾜﾜ", "", ""],
    }
}

fn howamoko_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ∩(˘ω˘)∩ﾉ", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Talk, _) => &["", "ﾉ∩(˘ω˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Play, 0) => &["", " ∩(˘▽˘)∩♪", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Play, _) => &["", "♪∩(˘▽˘)∩", "   ﾜﾜﾜﾜ", "", ""],
        (Action::Train, 0) => &["", " ∩(˘益˘)∩!", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Train, _) => &["", " ∩(˘益˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Relax, 0) => &["", " ∩(˘_˘)∩～", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Relax, _) => &["", " ∩(˘ ˘)∩～z", "  ﾜﾜﾜﾜ", "", ""],
    }
}

// --- Bouken type ---

// 13. クルル - Spinning wheel creature
fn kururu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ◎(・▽・)◎!", "   ○○○○", "", ""],
        (MoodLevel::High, _) => &["", "!◎(・▽・)◎", "   ○○○○", "", ""],
        (MoodLevel::Normal, 0) => &["", " ◎(・∀・)◎", "   ○○○○", "", ""],
        (MoodLevel::Normal, _) => &["", "  ◎(・∀・)◎", "    ○○○○", "", ""],
        (MoodLevel::Low, 0) => &["", " ◎(・_・)◎", "   ○○○○", "", ""],
        (MoodLevel::Low, _) => &["", " ◎(・ ・)◎", "   ○○○○", "", ""],
    }
}

fn kururu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ◎(・∀・)◎ﾉ", "   ○○○○", "", ""],
        (Action::Talk, _) => &["", "ﾉ◎(・∀・)◎", "   ○○○○", "", ""],
        (Action::Play, 0) => &["", " ◎(・▽・)◎♪", "   ○○○○", "", ""],
        (Action::Play, _) => &["", "♪◎(・▽・)◎", "    ○○○○", "", ""],
        (Action::Train, 0) => &["", " ◎(・益・)◎!!", "   ○○○○", "", ""],
        (Action::Train, _) => &["", " ◎(・益・)◎", "   ○○○○", "", ""],
        (Action::Relax, 0) => &["", " ◎(・_・)◎～", "   ○○○○", "", ""],
        (Action::Relax, _) => &["", " ◎(・ ・)◎～z", "   ○○○○", "", ""],
    }
}

// 14. トゲたろう - Spiky adventurer
fn togetarou_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " /{＞▽＜}\\!", "  ＞＜＞＜", "", ""],
        (MoodLevel::High, _) => &["", "!/{＞▽＜}\\", "  ＞＜＞＜", "", ""],
        (MoodLevel::Normal, 0) => &["", " /{＞ω＜}\\", "  ＞＜＞＜", "", ""],
        (MoodLevel::Normal, _) => &["", "  /{＞ω＜}\\", "   ＞＜＞＜", "", ""],
        (MoodLevel::Low, 0) => &["", " /{＞_＜}\\", "  ＞＜＞＜", "", ""],
        (MoodLevel::Low, _) => &["", " /{＞ ＜}\\", "  ＞＜＞＜", "", ""],
    }
}

fn togetarou_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " /{＞ω＜}\\ﾉ", "  ＞＜＞＜", "", ""],
        (Action::Talk, _) => &["", "ﾉ/{＞ω＜}\\", "  ＞＜＞＜", "", ""],
        (Action::Play, 0) => &["", " /{＞▽＜}\\♪", "  ＞＜＞＜", "", ""],
        (Action::Play, _) => &["", "♪/{＞▽＜}\\", "   ＞＜＞＜", "", ""],
        (Action::Train, 0) => &["", " /{＞益＜}\\!!", "  ＞＜＞＜", "", ""],
        (Action::Train, _) => &["", " /{＞益＜}\\", "  ＞＜＞＜", "", ""],
        (Action::Relax, 0) => &["", " /{＞_＜}\\～", "  ＞＜＞＜", "", ""],
        (Action::Relax, _) => &["", " /{＞ ＜}\\～z", "  ＞＜＞＜", "", ""],
    }
}

// 15. ハネオ - Winged creature
fn haneo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " 彡(・▽・)彡♪", "     ∧∧", "", ""],
        (MoodLevel::High, _) => &["", "♪彡(・▽・)彡", "     ∧∧", "", ""],
        (MoodLevel::Normal, 0) => &["", " 彡(・ω・)彡", "     ∧∧", "", ""],
        (MoodLevel::Normal, _) => &["", "  彡(・ω・)彡", "      ∧∧", "", ""],
        (MoodLevel::Low, 0) => &["", " 彡(・_・)彡", "     ∧∧", "", ""],
        (MoodLevel::Low, _) => &["", " 彡(・ ・)彡", "     ∧∧", "", ""],
    }
}

fn haneo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " 彡(・ω・)彡ﾉ", "     ∧∧", "", ""],
        (Action::Talk, _) => &["", "ﾉ彡(・ω・)彡", "     ∧∧", "", ""],
        (Action::Play, 0) => &["", " 彡(・▽・)彡♪", "     ∧∧", "", ""],
        (Action::Play, _) => &["", "♪彡(・▽・)彡", "      ∧∧", "", ""],
        (Action::Train, 0) => &["", " 彡(・益・)彡!!", "     ∧∧", "", ""],
        (Action::Train, _) => &["", " 彡(・益・)彡", "     ∧∧", "", ""],
        (Action::Relax, 0) => &["", " 彡(・_・)彡～", "     ∧∧", "", ""],
        (Action::Relax, _) => &["", " 彡(・ ・)彡～z", "     ∧∧", "", ""],
    }
}

// 16. ビョーン - Bouncy spring creature
fn byoon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (＞◡＜)!", "   ∞∞∞∞", "", ""],
        (MoodLevel::High, _) => &["", "  !(＞◡＜)", "    ∞∞∞∞", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (・◡・)", "   ∞∞∞∞", "", ""],
        (MoodLevel::Normal, _) => &["", "   (・◡・)", "    ∞∞∞∞", "", ""],
        (MoodLevel::Low, 0) => &["", "  (・_・)", "   ∞∞∞∞", "", ""],
        (MoodLevel::Low, _) => &["", "  (・ ・)", "   ∞∞∞∞", "", ""],
    }
}

fn byoon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (・◡・)ﾉ", "   ∞∞∞∞", "", ""],
        (Action::Talk, _) => &["", " ﾉ(・◡・)", "   ∞∞∞∞", "", ""],
        (Action::Play, 0) => &["", "  (＞◡＜)♪", "   ∞∞∞∞", "", ""],
        (Action::Play, _) => &["", " ♪(＞◡＜)", "    ∞∞∞∞", "", ""],
        (Action::Train, 0) => &["", "  (＞益＜)!!", "   ∞∞∞∞", "", ""],
        (Action::Train, _) => &["", "  (＞益＜)", "   ∞∞∞∞", "", ""],
        (Action::Relax, 0) => &["", "  (・_・)～", "   ∞∞∞∞", "", ""],
        (Action::Relax, _) => &["", "  (・ ・)～z", "   ∞∞∞∞", "", ""],
    }
}

// 17. ダッシュ - Speed runner with motion lines
fn dashu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ≫(°▽°)⊃!", "    ⊂⊃", "", ""],
        (MoodLevel::High, _) => &["", "  ≫(°▽°)⊃!", "     ⊂⊃", "", ""],
        (MoodLevel::Normal, 0) => &["", " ≫(°∀°)⊃", "    ⊂⊃", "", ""],
        (MoodLevel::Normal, _) => &["", "  ≫(°∀°)⊃", "     ⊂⊃", "", ""],
        (MoodLevel::Low, 0) => &["", " ≫(°_°)⊃", "    ⊂⊃", "", ""],
        (MoodLevel::Low, _) => &["", " ≫(° °)⊃", "    ⊂⊃", "", ""],
    }
}

fn dashu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ≫(°∀°)⊃ﾉ", "    ⊂⊃", "", ""],
        (Action::Talk, _) => &["", "ﾉ≫(°∀°)⊃", "    ⊂⊃", "", ""],
        (Action::Play, 0) => &["", " ≫(°▽°)⊃♪", "    ⊂⊃", "", ""],
        (Action::Play, _) => &["", "♪≫(°▽°)⊃", "     ⊂⊃", "", ""],
        (Action::Train, 0) => &["", " ≫≫(°益°)⊃!!", "      ⊂⊃", "", ""],
        (Action::Train, _) => &["", " ≫(°益°)⊃", "    ⊂⊃", "", ""],
        (Action::Relax, 0) => &["", " ≫(°_°)⊃～", "    ⊂⊃", "", ""],
        (Action::Relax, _) => &["", " ≫(° °)⊃～z", "    ⊂⊃", "", ""],
    }
}

// 18. グルグル - Spiral spinning creature
fn guruguru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " @(・▽・)@♪", "   ～～～", "", ""],
        (MoodLevel::High, _) => &["", "♪@(・▽・)@", "   ～～～", "", ""],
        (MoodLevel::Normal, 0) => &["", " @(・ω・)@", "   ～～～", "", ""],
        (MoodLevel::Normal, _) => &["", "  @(・ω・)@", "    ～～～", "", ""],
        (MoodLevel::Low, 0) => &["", " @(・_・)@", "   ～～～", "", ""],
        (MoodLevel::Low, _) => &["", " @(・ ・)@", "   ～～～", "", ""],
    }
}

fn guruguru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " @(・ω・)@ﾉ", "   ～～～", "", ""],
        (Action::Talk, _) => &["", "ﾉ@(・ω・)@", "   ～～～", "", ""],
        (Action::Play, 0) => &["", " @(・▽・)@♪", "   ～～～", "", ""],
        (Action::Play, _) => &["", "♪@(・▽・)@", "    ～～～", "", ""],
        (Action::Train, 0) => &["", " @(・益・)@!!", "   ～～～", "", ""],
        (Action::Train, _) => &["", " @(・益・)@", "   ～～～", "", ""],
        (Action::Relax, 0) => &["", " @(・_・)@～", "   ～～～", "", ""],
        (Action::Relax, _) => &["", " @(・ ・)@～z", "   ～～～", "", ""],
    }
}

// --- Normal type ---

// 19. ペタ - Flat, spread out on ground
fn peta_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (´・▽・`)♪", "  ＝＝＝＝", "", ""],
        (MoodLevel::High, _) => &["", " ♪(´・▽・`)", "  ＝＝＝＝", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (´・ω・`)", "  ＝＝＝＝", "", ""],
        (MoodLevel::Normal, _) => &["", "   (´・ω・`)", "   ＝＝＝＝", "", ""],
        (MoodLevel::Low, 0) => &["", "  (´・_・`)", "  ＝＝＝＝", "", ""],
        (MoodLevel::Low, _) => &["", "  (´・ ・`)", "  ＝＝＝＝", "", ""],
    }
}

fn peta_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (´・ω・`)ﾉ", "  ＝＝＝＝", "", ""],
        (Action::Talk, _) => &["", " ﾉ(´・ω・`)", "  ＝＝＝＝", "", ""],
        (Action::Play, 0) => &["", "  (´・▽・`)♪", "  ＝＝＝＝", "", ""],
        (Action::Play, _) => &["", " ♪(´・▽・`)", "   ＝＝＝＝", "", ""],
        (Action::Train, 0) => &["", "  (´・益・`)9", "  ＝＝＝＝", "", ""],
        (Action::Train, _) => &["", " 9(´・益・`)", "  ＝＝＝＝", "", ""],
        (Action::Relax, 0) => &["", "  (´・_・`)～", "  ＝＝＝＝", "", ""],
        (Action::Relax, _) => &["", "  (´・ ・`)～z", "  ＝＝＝＝", "", ""],
    }
}

// 20. ノホホ - Carefree, relaxed posture
fn nohoho_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)ﾉ♪", "   / \\", "", ""],
        (MoodLevel::High, _) => &["", " ♪ﾉ(˘▽˘)", "    / \\", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘ω˘)ﾉ", "   / \\", "", ""],
        (MoodLevel::Normal, _) => &["", "   (˘ω˘)ﾉ", "    / \\", "", ""],
        (MoodLevel::Low, 0) => &["", "  (˘_˘)", "   / \\", "", ""],
        (MoodLevel::Low, _) => &["", "  (˘ ˘)", "   / \\", "", ""],
    }
}

fn nohoho_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘ω˘)ﾉ", "   / \\", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘ω˘)", "   / \\", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)ﾉ♪", "   / \\", "", ""],
        (Action::Play, _) => &["", " ♪ﾉ(˘▽˘)", "    / \\", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9", "   / \\", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "   / \\", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)～", "   / \\", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)～z", "   / \\", "", ""],
    }
}

// 21. マジメ - Serious, rigid square posture
fn majime_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  [・▽・]!", "   || ||", "", ""],
        (MoodLevel::High, _) => &["", " ![・▽・]", "   || ||", "", ""],
        (MoodLevel::Normal, 0) => &["", "  [・_・]", "   || ||", "", ""],
        (MoodLevel::Normal, _) => &["", "   [・_・]", "    || ||", "", ""],
        (MoodLevel::Low, 0) => &["", "  [￣_￣]", "   || ||", "", ""],
        (MoodLevel::Low, _) => &["", "  [￣ ￣]", "   || ||", "", ""],
    }
}

fn majime_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  [・_・]ﾉ", "   || ||", "", ""],
        (Action::Talk, _) => &["", " ﾉ[・_・]", "   || ||", "", ""],
        (Action::Play, 0) => &["", "  [・▽・]♪", "   || ||", "", ""],
        (Action::Play, _) => &["", " ♪[・▽・]", "    || ||", "", ""],
        (Action::Train, 0) => &["", "  [・益・]9!", "   || ||", "", ""],
        (Action::Train, _) => &["", " 9[・益・]", "   || ||", "", ""],
        (Action::Relax, 0) => &["", "  [・_・]～", "   || ||", "", ""],
        (Action::Relax, _) => &["", "  [・ ・]～z", "   || ||", "", ""],
    }
}

// 22. フツウ - The most ordinary creature
fn futsuu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (・▽・)ﾉ!", "   ∪ ∪", "", ""],
        (MoodLevel::High, _) => &["", " !ﾉ(・▽・)", "    ∪ ∪", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (・ω・)", "   ∪ ∪", "", ""],
        (MoodLevel::Normal, _) => &["", "   (・ω・)", "    ∪ ∪", "", ""],
        (MoodLevel::Low, 0) => &["", "  (・_・)", "   ∪ ∪", "", ""],
        (MoodLevel::Low, _) => &["", "  (・ ・)", "   ∪ ∪", "", ""],
    }
}

fn futsuu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (・ω・)ﾉ", "   ∪ ∪", "", ""],
        (Action::Talk, _) => &["", " ﾉ(・ω・)", "   ∪ ∪", "", ""],
        (Action::Play, 0) => &["", "  (・▽・)♪", "   ∪ ∪", "", ""],
        (Action::Play, _) => &["", " ♪(・▽・)", "    ∪ ∪", "", ""],
        (Action::Train, 0) => &["", "  (・益・)9", "   ∪ ∪", "", ""],
        (Action::Train, _) => &["", " 9(・益・)", "   ∪ ∪", "", ""],
        (Action::Relax, 0) => &["", "  (・_・)～", "   ∪ ∪", "", ""],
        (Action::Relax, _) => &["", "  (・ ・)～z", "   ∪ ∪", "", ""],
    }
}

// 23. ナミナミ - Wavy, fluid body
fn naminami_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)♪", "  ～～～～", "", ""],
        (MoodLevel::High, _) => &["", " ♪(˘▽˘)", "  ～～～～", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘ω˘)", "  ～～～～", "", ""],
        (MoodLevel::Normal, _) => &["", "   (˘ω˘)", "   ～～～～", "", ""],
        (MoodLevel::Low, 0) => &["", "  (˘_˘)", "  ～～～～", "", ""],
        (MoodLevel::Low, _) => &["", "  (˘ ˘)", "  ～～～～", "", ""],
    }
}

fn naminami_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘ω˘)ﾉ", "  ～～～～", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘ω˘)", "  ～～～～", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)♪", "  ～～～～", "", ""],
        (Action::Play, _) => &["", " ♪(˘▽˘)", "   ～～～～", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9", "  ～～～～", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "  ～～～～", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)～", "  ～～～～", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)～z", "  ～～～～", "", ""],
    }
}

// 24. テキトー - Slouchy, casual creature
fn tekitoo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (´▽`)ﾉ♪", "   _/\\_", "", ""],
        (MoodLevel::High, _) => &["", " ♪ﾉ(´▽`)", "    _/\\_", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (´ω`)>", "   _/\\_", "", ""],
        (MoodLevel::Normal, _) => &["", "   (´ω`)>", "    _/\\_", "", ""],
        (MoodLevel::Low, 0) => &["", "  (´_`)", "   _/\\_", "", ""],
        (MoodLevel::Low, _) => &["", "  (´ `)", "   _/\\_", "", ""],
    }
}

fn tekitoo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (´ω`)>ﾉ", "   _/\\_", "", ""],
        (Action::Talk, _) => &["", " ﾉ(´ω`)>", "   _/\\_", "", ""],
        (Action::Play, 0) => &["", "  (´▽`)>♪", "   _/\\_", "", ""],
        (Action::Play, _) => &["", " ♪(´▽`)>", "    _/\\_", "", ""],
        (Action::Train, 0) => &["", "  (´益`)>9", "   _/\\_", "", ""],
        (Action::Train, _) => &["", " 9(´益`)>", "   _/\\_", "", ""],
        (Action::Relax, 0) => &["", "  (´_`)>～", "   _/\\_", "", ""],
        (Action::Relax, _) => &["", "  (´ `)>～z", "   _/\\_", "", ""],
    }
}

// --- Wild type ---

// 25. メダマ - Giant eyeball creature
fn medama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ◎(⊙▽⊙)◎!", "   ┃┃┃┃", "", ""],
        (MoodLevel::High, _) => &["", "!◎(⊙▽⊙)◎", "   ┃┃┃┃", "", ""],
        (MoodLevel::Normal, 0) => &["", " ◎(⊙_⊙)◎", "   ┃┃┃┃", "", ""],
        (MoodLevel::Normal, _) => &["", "  ◎(⊙_⊙)◎", "    ┃┃┃┃", "", ""],
        (MoodLevel::Low, 0) => &["", " ◎(⊙ ⊙)◎", "   ┃┃┃┃", "", ""],
        (MoodLevel::Low, _) => &["", " ◎(- -)◎", "   ┃┃┃┃", "", ""],
    }
}

fn medama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ◎(⊙_⊙)◎ﾉ", "   ┃┃┃┃", "", ""],
        (Action::Talk, _) => &["", "ﾉ◎(⊙_⊙)◎", "   ┃┃┃┃", "", ""],
        (Action::Play, 0) => &["", " ◎(⊙▽⊙)◎♪", "   ┃┃┃┃", "", ""],
        (Action::Play, _) => &["", "♪◎(⊙▽⊙)◎", "    ┃┃┃┃", "", ""],
        (Action::Train, 0) => &["", " ◎(⊙益⊙)◎!!", "   ┃┃┃┃", "", ""],
        (Action::Train, _) => &["", " ◎(⊙益⊙)◎", "   ┃┃┃┃", "", ""],
        (Action::Relax, 0) => &["", " ◎(⊙_⊙)◎～", "   ┃┃┃┃", "", ""],
        (Action::Relax, _) => &["", " ◎(- -)◎～z", "   ┃┃┃┃", "", ""],
    }
}

// 26. ケモノ - Feral beast with fangs
fn kemono_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " 牙(⊙▽⊙)牙!", "   爪爪爪", "", ""],
        (MoodLevel::High, _) => &["", "!牙(⊙▽⊙)牙", "   爪爪爪", "", ""],
        (MoodLevel::Normal, 0) => &["", " 牙(⊙ω⊙)牙", "   爪爪爪", "", ""],
        (MoodLevel::Normal, _) => &["", "  牙(⊙ω⊙)牙", "    爪爪爪", "", ""],
        (MoodLevel::Low, 0) => &["", " 牙(⊙_⊙)牙", "   爪爪爪", "", ""],
        (MoodLevel::Low, _) => &["", " 牙(- -)牙", "   爪爪爪", "", ""],
    }
}

fn kemono_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " 牙(⊙ω⊙)牙ﾉ", "   爪爪爪", "", ""],
        (Action::Talk, _) => &["", "ﾉ牙(⊙ω⊙)牙", "   爪爪爪", "", ""],
        (Action::Play, 0) => &["", " 牙(⊙▽⊙)牙♪", "   爪爪爪", "", ""],
        (Action::Play, _) => &["", "♪牙(⊙▽⊙)牙", "    爪爪爪", "", ""],
        (Action::Train, 0) => &["", " 牙(⊙益⊙)牙!!", "   爪爪爪", "", ""],
        (Action::Train, _) => &["", " 牙(⊙益⊙)牙", "   爪爪爪", "", ""],
        (Action::Relax, 0) => &["", " 牙(⊙_⊙)牙～", "   爪爪爪", "", ""],
        (Action::Relax, _) => &["", " 牙(- -)牙～z", "   爪爪爪", "", ""],
    }
}

// 27. ヌシ - Imposing boss creature with crown
fn nushi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " 王(⊙▽⊙)王!", "  ╠████╣", "", ""],
        (MoodLevel::High, _) => &["", "!王(⊙▽⊙)王", "  ╠████╣", "", ""],
        (MoodLevel::Normal, 0) => &["", " 王(⊙益⊙)王", "  ╠████╣", "", ""],
        (MoodLevel::Normal, _) => &["", "  王(⊙益⊙)王", "   ╠████╣", "", ""],
        (MoodLevel::Low, 0) => &["", " 王(⊙_⊙)王", "  ╠████╣", "", ""],
        (MoodLevel::Low, _) => &["", " 王(- -)王", "  ╠████╣", "", ""],
    }
}

fn nushi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " 王(⊙益⊙)王ﾉ", "  ╠████╣", "", ""],
        (Action::Talk, _) => &["", "ﾉ王(⊙益⊙)王", "  ╠████╣", "", ""],
        (Action::Play, 0) => &["", " 王(⊙▽⊙)王♪", "  ╠████╣", "", ""],
        (Action::Play, _) => &["", "♪王(⊙▽⊙)王", "   ╠████╣", "", ""],
        (Action::Train, 0) => &["", " 王(⊙益⊙)王!!", "  ╠████╣", "", ""],
        (Action::Train, _) => &["", " 王(⊙益⊙)王", "  ╠████╣", "", ""],
        (Action::Relax, 0) => &["", " 王(⊙_⊙)王～", "  ╠████╣", "", ""],
        (Action::Relax, _) => &["", " 王(- -)王～z", "  ╠████╣", "", ""],
    }
}

// 28. カゲ - Shadowy, barely visible creature
fn kage_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ░(⊙▽⊙)░!", "   ▓▓▓▓", "", ""],
        (MoodLevel::High, _) => &["", "!░(⊙▽⊙)░", "   ▓▓▓▓", "", ""],
        (MoodLevel::Normal, 0) => &["", " ░(⊙_⊙)░", "   ▓▓▓▓", "", ""],
        (MoodLevel::Normal, _) => &["", "  ░(⊙_⊙)░", "    ▓▓▓▓", "", ""],
        (MoodLevel::Low, 0) => &["", " ░(- -)░", "   ▓▓▓▓", "", ""],
        (MoodLevel::Low, _) => &["", " ░(  )░", "   ▓▓▓▓", "", ""],
    }
}

fn kage_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ░(⊙_⊙)░ﾉ", "   ▓▓▓▓", "", ""],
        (Action::Talk, _) => &["", "ﾉ░(⊙_⊙)░", "   ▓▓▓▓", "", ""],
        (Action::Play, 0) => &["", " ░(⊙▽⊙)░♪", "   ▓▓▓▓", "", ""],
        (Action::Play, _) => &["", "♪░(⊙▽⊙)░", "    ▓▓▓▓", "", ""],
        (Action::Train, 0) => &["", " ░(⊙益⊙)░!!", "   ▓▓▓▓", "", ""],
        (Action::Train, _) => &["", " ░(⊙益⊙)░", "   ▓▓▓▓", "", ""],
        (Action::Relax, 0) => &["", " ░(⊙_⊙)░～", "   ▓▓▓▓", "", ""],
        (Action::Relax, _) => &["", " ░(- -)░～z", "   ▓▓▓▓", "", ""],
    }
}

// 29. ザワザワ - Eerie creature with many appendages
fn zawazawa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ψ(⊙▽⊙)ψ!", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::High, _) => &["", "!ψ(⊙▽⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, 0) => &["", " ψ(⊙_⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, _) => &["", "  ψ(⊙_⊙)ψ", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, 0) => &["", " ψ(- -)ψ", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, _) => &["", " ψ(  )ψ", "  ∫∫∫∫∫", "", ""],
    }
}

fn zawazawa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ψ(⊙_⊙)ψﾉ", "  ∫∫∫∫∫", "", ""],
        (Action::Talk, _) => &["", "ﾉψ(⊙_⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (Action::Play, 0) => &["", " ψ(⊙▽⊙)ψ♪", "  ∫∫∫∫∫", "", ""],
        (Action::Play, _) => &["", "♪ψ(⊙▽⊙)ψ", "   ∫∫∫∫∫", "", ""],
        (Action::Train, 0) => &["", " ψ(⊙益⊙)ψ!!", "  ∫∫∫∫∫", "", ""],
        (Action::Train, _) => &["", " ψ(⊙益⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (Action::Relax, 0) => &["", " ψ(⊙_⊙)ψ～", "  ∫∫∫∫∫", "", ""],
        (Action::Relax, _) => &["", " ψ(- -)ψ～z", "  ∫∫∫∫∫", "", ""],
    }
}

// 30. ヒトダマ - Ghost fire, floating will-o-wisp
fn hitodama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " *(⊙▽⊙)*♪", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::High, _) => &["", "♪*(⊙▽⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Normal, 0) => &["", " *(⊙o⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Normal, _) => &["", "  *(⊙o⊙)*", "    ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Low, 0) => &["", " *(- -)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Low, _) => &["", " *(  )*", "   ﾟ.ﾟ.ﾟ", "", ""],
    }
}

fn hitodama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " *(⊙o⊙)*ﾉ", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Talk, _) => &["", "ﾉ*(⊙o⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Play, 0) => &["", " *(⊙▽⊙)*♪", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Play, _) => &["", "♪*(⊙▽⊙)*", "    ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Train, 0) => &["", " *(⊙益⊙)*!!", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Train, _) => &["", " *(⊙益⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Relax, 0) => &["", " *(⊙_⊙)*～", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Relax, _) => &["", " *(- -)*～z", "   ﾟ.ﾟ.ﾟ", "", ""],
    }
}

// =====================================================================
// Composable Art System for Stage 3+ (unique visuals per species)
// =====================================================================

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
