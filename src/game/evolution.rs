use rand::seq::SliceRandom;
use rand::Rng;

use crate::save::schema::PetData;

// --- Evolution timing constants ---
// Stage1→2: 6〜12時間（360〜720 ticks）
const STAGE2_TICKS_MIN: u64 = 360;
const STAGE2_TICKS_RANGE: u64 = 361; // 0..=360 offset
// Stage2→3: 24〜48時間（1440〜2880 ticks）
const STAGE3_TICKS_MIN: u64 = 1440;
const STAGE3_TICKS_RANGE: u64 = 1441; // 0..=1440 offset
const STAGE4_INTERVAL: u64 = 1440; // Check every 24 hours
const STAGE4_CHANCE: f64 = 0.25; // 25%

// --- Evolution types ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvoType {
    Chikara,
    Odayaka,
    Bouken,
    Normal,
    Wild,
}

// --- Stage2 species definition ---
pub struct Stage2Species {
    pub name: &'static str,
    pub evo_type: EvoType,
    pub standard_weight: f64,
    pub voice_type: VoiceType,
}

// --- Body type category ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    Marukko,     // まるっこ系: round and small
    Nagai,       // ながい系: vertically long / snake-like
    Hiroi,       // ひろい系: horizontally wide / flat
    Togari,      // とがり系: pointy / spiky
    Fuwafuwa,    // ふわふわ系: fluffy / feathery
    Karadanashi, // からだなし系: floating face/eyes only
    Nagare,      // ながれ系: ghost / slime
    AshiOoi,     // あし多い系: many limbs
    ChisaiOokii, // ちいさい+おおきい系: tiny body with big decoration
    Noppo,       // のっぽ系: tall and thin
}

// --- Stage3 species definition ---
pub struct Stage3Species {
    pub name: &'static str,
    pub allowed_from: &'static [&'static str], // Stage2 ancestors
    pub vector: [f64; 5],                      // [chikara, odayaka, bouken, nakayoshi, frequency]
    pub standard_weight: f64,
    pub voice_type: VoiceType,
    pub body_type: BodyType,
}

/// Look up the body type for a species by name (Stage3 only; returns None for other stages).
pub fn get_body_type(species: &str) -> Option<BodyType> {
    STAGE3_SPECIES
        .iter()
        .find(|s| s.name == species)
        .map(|s| s.body_type)
}

// --- Stage4 species definition ---
pub struct Stage4Species {
    pub name: &'static str,
    pub allowed_from: &'static [&'static str], // Stage3 ancestors
    pub standard_weight: f64,
    pub voice_type: VoiceType,
}

// --- Voice types ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceType {
    Tameguchi, // Casual
    Keigo,     // Polite
    Gal,       // Gal
    Oyaji,     // Old man
    Tetsugaku, // Philosopher
    Taiiku,    // Athletic
    Negative,  // Negative
    Tennen,    // Airhead
    Mukuchi,   // Taciturn
    Kajou,     // Excessive
    Kansai,    // Kansai dialect
    Kogo,      // Archaic
}

// ===== Stage2: 30 species (5 types × 6 each) =====
pub const STAGE2_SPECIES: &[Stage2Species] = &[
    // --- Chikara type (6 species) ---
    Stage2Species {
        name: "ドタン",
        evo_type: EvoType::Chikara,
        standard_weight: 25.0,
        voice_type: VoiceType::Taiiku,
    },
    Stage2Species {
        name: "ガシャ",
        evo_type: EvoType::Chikara,
        standard_weight: 30.0,
        voice_type: VoiceType::Tameguchi,
    },
    Stage2Species {
        name: "ズンズン",
        evo_type: EvoType::Chikara,
        standard_weight: 35.0,
        voice_type: VoiceType::Oyaji,
    },
    Stage2Species {
        name: "デカオ",
        evo_type: EvoType::Chikara,
        standard_weight: 40.0,
        voice_type: VoiceType::Kajou,
    },
    Stage2Species {
        name: "ゴツモリ",
        evo_type: EvoType::Chikara,
        standard_weight: 28.0,
        voice_type: VoiceType::Kansai,
    },
    Stage2Species {
        name: "ドンガメ",
        evo_type: EvoType::Chikara,
        standard_weight: 32.0,
        voice_type: VoiceType::Kogo,
    },
    // --- Odayaka type (6 species) ---
    Stage2Species {
        name: "ヒョロン",
        evo_type: EvoType::Odayaka,
        standard_weight: 12.0,
        voice_type: VoiceType::Keigo,
    },
    Stage2Species {
        name: "フワモン",
        evo_type: EvoType::Odayaka,
        standard_weight: 8.0,
        voice_type: VoiceType::Tennen,
    },
    Stage2Species {
        name: "ユラリ",
        evo_type: EvoType::Odayaka,
        standard_weight: 10.0,
        voice_type: VoiceType::Tetsugaku,
    },
    Stage2Species {
        name: "ネムタ",
        evo_type: EvoType::Odayaka,
        standard_weight: 15.0,
        voice_type: VoiceType::Mukuchi,
    },
    Stage2Species {
        name: "ポワン",
        evo_type: EvoType::Odayaka,
        standard_weight: 11.0,
        voice_type: VoiceType::Tennen,
    },
    Stage2Species {
        name: "ホワモコ",
        evo_type: EvoType::Odayaka,
        standard_weight: 14.0,
        voice_type: VoiceType::Keigo,
    },
    // --- Bouken type (6 species) ---
    Stage2Species {
        name: "クルル",
        evo_type: EvoType::Bouken,
        standard_weight: 18.0,
        voice_type: VoiceType::Gal,
    },
    Stage2Species {
        name: "トゲたろう",
        evo_type: EvoType::Bouken,
        standard_weight: 20.0,
        voice_type: VoiceType::Tameguchi,
    },
    Stage2Species {
        name: "ハネオ",
        evo_type: EvoType::Bouken,
        standard_weight: 15.0,
        voice_type: VoiceType::Kajou,
    },
    Stage2Species {
        name: "ビョーン",
        evo_type: EvoType::Bouken,
        standard_weight: 16.0,
        voice_type: VoiceType::Kansai,
    },
    Stage2Species {
        name: "ダッシュ",
        evo_type: EvoType::Bouken,
        standard_weight: 17.0,
        voice_type: VoiceType::Taiiku,
    },
    Stage2Species {
        name: "グルグル",
        evo_type: EvoType::Bouken,
        standard_weight: 14.0,
        voice_type: VoiceType::Tennen,
    },
    // --- Normal type (6 species) ---
    Stage2Species {
        name: "ペタ",
        evo_type: EvoType::Normal,
        standard_weight: 20.0,
        voice_type: VoiceType::Keigo,
    },
    Stage2Species {
        name: "ノホホ",
        evo_type: EvoType::Normal,
        standard_weight: 18.0,
        voice_type: VoiceType::Tennen,
    },
    Stage2Species {
        name: "マジメ",
        evo_type: EvoType::Normal,
        standard_weight: 22.0,
        voice_type: VoiceType::Keigo,
    },
    Stage2Species {
        name: "フツウ",
        evo_type: EvoType::Normal,
        standard_weight: 19.0,
        voice_type: VoiceType::Tameguchi,
    },
    Stage2Species {
        name: "ナミナミ",
        evo_type: EvoType::Normal,
        standard_weight: 17.0,
        voice_type: VoiceType::Kansai,
    },
    Stage2Species {
        name: "テキトー",
        evo_type: EvoType::Normal,
        standard_weight: 21.0,
        voice_type: VoiceType::Oyaji,
    },
    // --- Wild type (6 species) ---
    Stage2Species {
        name: "メダマ",
        evo_type: EvoType::Wild,
        standard_weight: 8.0,
        voice_type: VoiceType::Mukuchi,
    },
    Stage2Species {
        name: "ケモノ",
        evo_type: EvoType::Wild,
        standard_weight: 25.0,
        voice_type: VoiceType::Tameguchi,
    },
    Stage2Species {
        name: "ヌシ",
        evo_type: EvoType::Wild,
        standard_weight: 50.0,
        voice_type: VoiceType::Kogo,
    },
    Stage2Species {
        name: "カゲ",
        evo_type: EvoType::Wild,
        standard_weight: 5.0,
        voice_type: VoiceType::Mukuchi,
    },
    Stage2Species {
        name: "ザワザワ",
        evo_type: EvoType::Wild,
        standard_weight: 15.0,
        voice_type: VoiceType::Tetsugaku,
    },
    Stage2Species {
        name: "ヒトダマ",
        evo_type: EvoType::Wild,
        standard_weight: 3.0,
        voice_type: VoiceType::Negative,
    },
];

// ===== Stage3: 80 species (5 types × 16 each) =====
// vector: [chikara, odayaka, bouken, nakayoshi, frequency(action_count)]
pub const STAGE3_SPECIES: &[Stage3Species] = &[
    // --- Evolved from Chikara type (16 species) ---
    Stage3Species {
        name: "ドドン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [9.0, 1.0, 3.0, 4.0, 6.0],
        standard_weight: 80.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "タワーン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [7.0, 4.0, 2.0, 8.0, 8.0],
        standard_weight: 60.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Noppo,
    },
    Stage3Species {
        name: "ゴウケン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [10.0, 0.0, 2.0, 3.0, 5.0],
        standard_weight: 90.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "テッカイ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [8.0, 2.0, 4.0, 2.0, 4.0],
        standard_weight: 100.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ブンブン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [8.0, 1.0, 5.0, 5.0, 7.0],
        standard_weight: 55.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::AshiOoi,
    },
    Stage3Species {
        name: "ガンテツ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [9.0, 3.0, 1.0, 6.0, 3.0],
        standard_weight: 120.0,
        voice_type: VoiceType::Oyaji,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ドスコイ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [7.0, 2.0, 1.0, 7.0, 9.0],
        standard_weight: 150.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::Hiroi,
    },
    Stage3Species {
        name: "バリバリ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [8.0, 0.0, 6.0, 3.0, 8.0],
        standard_weight: 65.0,
        voice_type: VoiceType::Kajou,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "メガトン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [10.0, 1.0, 1.0, 5.0, 5.0],
        standard_weight: 200.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "グランド",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [7.0, 3.0, 3.0, 7.0, 7.0],
        standard_weight: 85.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Hiroi,
    },
    Stage3Species {
        name: "イカヅチ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [9.0, 0.0, 5.0, 2.0, 6.0],
        standard_weight: 70.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "ゴリラン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [8.0, 2.0, 3.0, 8.0, 4.0],
        standard_weight: 95.0,
        voice_type: VoiceType::Oyaji,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ダイガン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [10.0, 2.0, 2.0, 1.0, 3.0],
        standard_weight: 130.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "カチワリ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [9.0, 1.0, 4.0, 4.0, 9.0],
        standard_weight: 68.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "マッスル",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [10.0, 0.0, 3.0, 6.0, 10.0],
        standard_weight: 78.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "イワオ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [7.0, 5.0, 1.0, 5.0, 1.0],
        standard_weight: 160.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ゴロゴロ",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [7.0, 3.0, 2.0, 4.0, 5.0],
        standard_weight: 110.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "テツジン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [10.0, 1.0, 2.0, 3.0, 4.0],
        standard_weight: 85.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Noppo,
    },
    Stage3Species {
        name: "ドゴン",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [8.0, 0.0, 5.0, 2.0, 8.0],
        standard_weight: 75.0,
        voice_type: VoiceType::Kajou,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "バンカー",
        allowed_from: &[
            "ドタン",
            "ガシャ",
            "ズンズン",
            "デカオ",
            "ゴツモリ",
            "ドンガメ",
        ],
        vector: [9.0, 2.0, 3.0, 1.0, 3.0],
        standard_weight: 140.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Hiroi,
    },
    // --- Evolved from Odayaka type (16 species) ---
    Stage3Species {
        name: "ながれもん",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 7.0, 7.0, 2.0, 1.0],
        standard_weight: 20.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "フワリン",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 9.0, 2.0, 7.0, 6.0],
        standard_weight: 10.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Fuwafuwa,
    },
    Stage3Species {
        name: "モコモコ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [2.0, 10.0, 1.0, 8.0, 7.0],
        standard_weight: 25.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Fuwafuwa,
    },
    Stage3Species {
        name: "ネンネ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [0.0, 9.0, 1.0, 5.0, 3.0],
        standard_weight: 30.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ポヨン",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 7.0, 4.0, 6.0, 5.0],
        standard_weight: 18.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "スヤスヤ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [0.0, 10.0, 0.0, 4.0, 1.0],
        standard_weight: 35.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "カスミ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 8.0, 3.0, 2.0, 4.0],
        standard_weight: 5.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ノドカ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [2.0, 8.0, 2.0, 9.0, 8.0],
        standard_weight: 22.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ユメミ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 7.0, 3.0, 6.0, 4.0],
        standard_weight: 15.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ボンヤリ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [0.0, 8.0, 2.0, 3.0, 2.0],
        standard_weight: 28.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "コロリン",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [2.0, 9.0, 2.0, 7.0, 6.0],
        standard_weight: 16.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ムニャ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 10.0, 1.0, 4.0, 3.0],
        standard_weight: 32.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "マッタリ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 8.0, 1.0, 8.0, 9.0],
        standard_weight: 26.0,
        voice_type: VoiceType::Oyaji,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ホワワ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [0.0, 9.0, 3.0, 5.0, 5.0],
        standard_weight: 12.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Fuwafuwa,
    },
    Stage3Species {
        name: "シズカ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [2.0, 9.0, 1.0, 6.0, 4.0],
        standard_weight: 24.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ソヨカゼ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [0.0, 7.0, 5.0, 3.0, 4.0],
        standard_weight: 8.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ヒラタ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 9.0, 2.0, 5.0, 3.0],
        standard_weight: 18.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Hiroi,
    },
    Stage3Species {
        name: "モグモグ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [2.0, 8.0, 2.0, 7.0, 7.0],
        standard_weight: 22.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "トロン",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [0.0, 10.0, 1.0, 3.0, 2.0],
        standard_weight: 28.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ユッタリ",
        allowed_from: &[
            "ヒョロン",
            "フワモン",
            "ユラリ",
            "ネムタ",
            "ポワン",
            "ホワモコ",
        ],
        vector: [1.0, 8.0, 3.0, 6.0, 4.0],
        standard_weight: 20.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Marukko,
    },
    // --- Evolved from Bouken type (16 species) ---
    Stage3Species {
        name: "ガニ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [3.0, 1.0, 9.0, 4.0, 6.0],
        standard_weight: 30.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::AshiOoi,
    },
    Stage3Species {
        name: "トビオ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [2.0, 2.0, 10.0, 5.0, 7.0],
        standard_weight: 25.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::ChisaiOokii,
    },
    Stage3Species {
        name: "マルマル",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [4.0, 3.0, 8.0, 6.0, 5.0],
        standard_weight: 35.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ハヤテ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [5.0, 0.0, 10.0, 3.0, 8.0],
        standard_weight: 22.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::Nagai,
    },
    Stage3Species {
        name: "グルグルン",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [2.0, 3.0, 8.0, 7.0, 6.0],
        standard_weight: 20.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "カゼノコ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [1.0, 4.0, 9.0, 5.0, 4.0],
        standard_weight: 15.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ドカーン",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [6.0, 0.0, 9.0, 2.0, 7.0],
        standard_weight: 45.0,
        voice_type: VoiceType::Kajou,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "スイスイ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [2.0, 5.0, 8.0, 6.0, 5.0],
        standard_weight: 18.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "サスライ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [3.0, 2.0, 10.0, 1.0, 3.0],
        standard_weight: 28.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Nagai,
    },
    Stage3Species {
        name: "ピカッ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [4.0, 1.0, 8.0, 8.0, 9.0],
        standard_weight: 16.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "バサバサ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [5.0, 2.0, 9.0, 4.0, 6.0],
        standard_weight: 32.0,
        voice_type: VoiceType::Oyaji,
        body_type: BodyType::ChisaiOokii,
    },
    Stage3Species {
        name: "ウロチョロ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [2.0, 3.0, 7.0, 5.0, 8.0],
        standard_weight: 14.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::AshiOoi,
    },
    Stage3Species {
        name: "ゴーゴー",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [6.0, 1.0, 8.0, 3.0, 9.0],
        standard_weight: 40.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "クモノス",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [3.0, 4.0, 7.0, 4.0, 4.0],
        standard_weight: 12.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::AshiOoi,
    },
    Stage3Species {
        name: "ホシゾラ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [1.0, 5.0, 9.0, 7.0, 5.0],
        standard_weight: 10.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "ブッチギリ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [7.0, 0.0, 10.0, 2.0, 10.0],
        standard_weight: 38.0,
        voice_type: VoiceType::Kajou,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "ワタリ",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [2.0, 4.0, 8.0, 2.0, 2.0],
        standard_weight: 24.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::ChisaiOokii,
    },
    Stage3Species {
        name: "ヒュー",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [3.0, 1.0, 9.0, 5.0, 7.0],
        standard_weight: 12.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::Nagai,
    },
    Stage3Species {
        name: "タンケン",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [4.0, 2.0, 9.0, 6.0, 5.0],
        standard_weight: 20.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ジェット",
        allowed_from: &[
            "クルル",
            "トゲたろう",
            "ハネオ",
            "ビョーン",
            "ダッシュ",
            "グルグル",
        ],
        vector: [5.0, 0.0, 10.0, 1.0, 8.0],
        standard_weight: 26.0,
        voice_type: VoiceType::Taiiku,
        body_type: BodyType::Togari,
    },
    // --- Evolved from Normal type (16 species) ---
    Stage3Species {
        name: "ノーマル",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [5.0, 5.0, 5.0, 5.0, 5.0],
        standard_weight: 40.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ヘイボン",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 5.0, 4.0, 6.0, 6.0],
        standard_weight: 35.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "タソガレ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [3.0, 6.0, 4.0, 4.0, 3.0],
        standard_weight: 30.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "ニッコリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 4.0, 4.0, 8.0, 8.0],
        standard_weight: 32.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ダラーン",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [3.0, 7.0, 3.0, 5.0, 2.0],
        standard_weight: 45.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "キッチリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [6.0, 4.0, 5.0, 6.0, 7.0],
        standard_weight: 38.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ボチボチ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 5.0, 5.0, 4.0, 4.0],
        standard_weight: 36.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "マアマア",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [5.0, 4.0, 4.0, 5.0, 5.0],
        standard_weight: 42.0,
        voice_type: VoiceType::Oyaji,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "フニャ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [3.0, 6.0, 3.0, 7.0, 6.0],
        standard_weight: 28.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Fuwafuwa,
    },
    Stage3Species {
        name: "テンテン",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [5.0, 3.0, 6.0, 5.0, 7.0],
        standard_weight: 34.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "ナァナァ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 5.0, 4.0, 3.0, 3.0],
        standard_weight: 39.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ポツリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [3.0, 4.0, 5.0, 4.0, 4.0],
        standard_weight: 26.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ソレナリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [5.0, 5.0, 5.0, 6.0, 6.0],
        standard_weight: 37.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ウンウン",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 5.0, 3.0, 7.0, 7.0],
        standard_weight: 33.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "チャッカリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [5.0, 3.0, 6.0, 6.0, 8.0],
        standard_weight: 31.0,
        voice_type: VoiceType::Gal,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ヌルリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [3.0, 5.0, 5.0, 5.0, 3.0],
        standard_weight: 34.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ヤレヤレ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 6.0, 3.0, 4.0, 3.0],
        standard_weight: 38.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ドッコイ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [5.0, 4.0, 4.0, 5.0, 5.0],
        standard_weight: 33.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "パッパ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [4.0, 4.0, 5.0, 5.0, 7.0],
        standard_weight: 28.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "オットリ",
        allowed_from: &["ペタ", "ノホホ", "マジメ", "フツウ", "ナミナミ", "テキトー"],
        vector: [3.0, 7.0, 2.0, 6.0, 4.0],
        standard_weight: 42.0,
        voice_type: VoiceType::Keigo,
        body_type: BodyType::Marukko,
    },
    // --- Evolved from Wild type (16 species) ---
    Stage3Species {
        name: "ヤミノメ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [3.0, 3.0, 5.0, 1.0, 1.0],
        standard_weight: 15.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "オオヌシ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [7.0, 2.0, 4.0, 2.0, 1.0],
        standard_weight: 100.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Noppo,
    },
    Stage3Species {
        name: "バケモノ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [5.0, 1.0, 7.0, 1.0, 2.0],
        standard_weight: 60.0,
        voice_type: VoiceType::Kajou,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ユウレイ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [1.0, 5.0, 4.0, 2.0, 1.0],
        standard_weight: 3.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ヤセイジ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [6.0, 1.0, 6.0, 0.0, 0.0],
        standard_weight: 45.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "シンエン",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [2.0, 6.0, 3.0, 1.0, 1.0],
        standard_weight: 20.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "ノラクロ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [4.0, 3.0, 6.0, 3.0, 2.0],
        standard_weight: 35.0,
        voice_type: VoiceType::Kansai,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "モノノケ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [3.0, 4.0, 5.0, 2.0, 1.0],
        standard_weight: 10.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "クライ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [2.0, 5.0, 3.0, 1.0, 1.0],
        standard_weight: 8.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "アヤシイ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [4.0, 2.0, 6.0, 3.0, 3.0],
        standard_weight: 25.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ムジナ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [5.0, 3.0, 5.0, 2.0, 2.0],
        standard_weight: 30.0,
        voice_type: VoiceType::Tameguchi,
        body_type: BodyType::Marukko,
    },
    Stage3Species {
        name: "ヌエ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [6.0, 2.0, 5.0, 1.0, 1.0],
        standard_weight: 55.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::ChisaiOokii,
    },
    Stage3Species {
        name: "フルエ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [2.0, 3.0, 4.0, 4.0, 3.0],
        standard_weight: 12.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ケダマ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [4.0, 4.0, 4.0, 2.0, 2.0],
        standard_weight: 22.0,
        voice_type: VoiceType::Tennen,
        body_type: BodyType::Fuwafuwa,
    },
    Stage3Species {
        name: "ジゴク",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [7.0, 1.0, 5.0, 0.0, 0.0],
        standard_weight: 70.0,
        voice_type: VoiceType::Kogo,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "ムゲン",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [3.0, 5.0, 5.0, 2.0, 1.0],
        standard_weight: 1.0,
        voice_type: VoiceType::Tetsugaku,
        body_type: BodyType::Karadanashi,
    },
    Stage3Species {
        name: "カマイタチ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [4.0, 2.0, 7.0, 1.0, 2.0],
        standard_weight: 8.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Togari,
    },
    Stage3Species {
        name: "ドロドロ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [3.0, 4.0, 4.0, 2.0, 1.0],
        standard_weight: 25.0,
        voice_type: VoiceType::Negative,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "ヒノタマ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [2.0, 3.0, 6.0, 3.0, 3.0],
        standard_weight: 5.0,
        voice_type: VoiceType::Kajou,
        body_type: BodyType::Nagare,
    },
    Stage3Species {
        name: "シノビ",
        allowed_from: &["メダマ", "ケモノ", "ヌシ", "カゲ", "ザワザワ", "ヒトダマ"],
        vector: [5.0, 2.0, 6.0, 1.0, 1.0],
        standard_weight: 20.0,
        voice_type: VoiceType::Mukuchi,
        body_type: BodyType::Marukko,
    },
];

// ===== Stage4: Mutations (8 species) =====
pub const STAGE4_SPECIES: &[Stage4Species] = &[
    Stage4Species {
        name: "ゲンソウ",
        allowed_from: &[
            "ドドン",
            "タワーン",
            "ゴウケン",
            "テッカイ",
            "ブンブン",
            "ガンテツ",
            "ドスコイ",
            "バリバリ",
            "メガトン",
            "グランド",
            "イカヅチ",
            "ゴリラン",
            "ダイガン",
            "カチワリ",
            "マッスル",
            "イワオ",
            "ゴロゴロ",
            "テツジン",
            "ドゴン",
            "バンカー",
        ],
        standard_weight: 250.0,
        voice_type: VoiceType::Kogo,
    },
    Stage4Species {
        name: "エーテル",
        allowed_from: &[
            "ながれもん",
            "フワリン",
            "モコモコ",
            "ネンネ",
            "ポヨン",
            "スヤスヤ",
            "カスミ",
            "ノドカ",
            "ユメミ",
            "ボンヤリ",
            "コロリン",
            "ムニャ",
            "マッタリ",
            "ホワワ",
            "シズカ",
            "ソヨカゼ",
            "ヒラタ",
            "モグモグ",
            "トロン",
            "ユッタリ",
        ],
        standard_weight: 0.5,
        voice_type: VoiceType::Tetsugaku,
    },
    Stage4Species {
        name: "カイザー",
        allowed_from: &[
            "ガニ",
            "トビオ",
            "マルマル",
            "ハヤテ",
            "グルグルン",
            "カゼノコ",
            "ドカーン",
            "スイスイ",
            "サスライ",
            "ピカッ",
            "バサバサ",
            "ウロチョロ",
            "ゴーゴー",
            "クモノス",
            "ホシゾラ",
            "ブッチギリ",
            "ワタリ",
            "ヒュー",
            "タンケン",
            "ジェット",
        ],
        standard_weight: 80.0,
        voice_type: VoiceType::Kajou,
    },
    Stage4Species {
        name: "ハクチュウ",
        allowed_from: &[
            "ノーマル",
            "ヘイボン",
            "タソガレ",
            "ニッコリ",
            "ダラーン",
            "キッチリ",
            "ボチボチ",
            "マアマア",
            "フニャ",
            "テンテン",
            "ナァナァ",
            "ポツリ",
            "ソレナリ",
            "ウンウン",
            "チャッカリ",
            "ヌルリ",
            "ヤレヤレ",
            "ドッコイ",
            "パッパ",
            "オットリ",
        ],
        standard_weight: 50.0,
        voice_type: VoiceType::Tennen,
    },
    Stage4Species {
        name: "コンゲン",
        allowed_from: &[
            "ヤミノメ",
            "オオヌシ",
            "バケモノ",
            "ユウレイ",
            "ヤセイジ",
            "シンエン",
            "ノラクロ",
            "モノノケ",
            "クライ",
            "アヤシイ",
            "ムジナ",
            "ヌエ",
            "フルエ",
            "ケダマ",
            "ジゴク",
            "ムゲン",
            "カマイタチ",
            "ドロドロ",
            "ヒノタマ",
            "シノビ",
        ],
        standard_weight: 300.0,
        voice_type: VoiceType::Mukuchi,
    },
    Stage4Species {
        name: "キセキ",
        allowed_from: &[
            "ドドン",
            "フワリン",
            "トビオ",
            "ニッコリ",
            "ヤミノメ",
            "マッスル",
            "モコモコ",
            "ハヤテ",
            "キッチリ",
            "オオヌシ",
        ],
        standard_weight: 42.0,
        voice_type: VoiceType::Gal,
    },
    Stage4Species {
        name: "ムゲンダイ",
        allowed_from: &[
            "メガトン",
            "スヤスヤ",
            "ブッチギリ",
            "ダラーン",
            "ジゴク",
            "イワオ",
            "ネンネ",
            "サスライ",
            "ヌルリ",
            "ムゲン",
        ],
        standard_weight: 999.0,
        voice_type: VoiceType::Tetsugaku,
    },
    Stage4Species {
        name: "ナナシ",
        allowed_from: &[
            "カスミ",
            "ソヨカゼ",
            "クモノス",
            "ポツリ",
            "フルエ",
            "ボンヤリ",
            "カゼノコ",
            "ナァナァ",
            "クライ",
            "ケダマ",
        ],
        standard_weight: 7.0,
        voice_type: VoiceType::Mukuchi,
    },
];

/// Evolution event result
#[allow(dead_code)]
pub struct EvolutionEvent {
    pub new_species: String,
    pub new_stage: u8,
}

/// Check for evolution (called at startup and after actions)
pub fn check_evolution(pet: &mut PetData, rng: &mut impl Rng) -> Option<EvolutionEvent> {
    match pet.stage {
        1 => check_stage1_to_2(pet, rng),
        2 => check_stage2_to_3(pet, rng),
        3 => check_stage3_to_4(pet, rng),
        _ => None,
    }
}

/// Derive a deterministic u64 from birth_timestamp for randomizing evolution timing.
/// Uses a simple hash so the threshold is stable across calls for the same pet.
fn birth_hash(pet: &PetData, salt: u64) -> u64 {
    let seed = (pet.birth_timestamp.timestamp() as u64) ^ salt;
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

/// Stage1 → Stage2 (6〜12時間後、determined by type tendency)
fn check_stage1_to_2(pet: &mut PetData, rng: &mut impl Rng) -> Option<EvolutionEvent> {
    let threshold = STAGE2_TICKS_MIN + birth_hash(pet, 0) % STAGE2_TICKS_RANGE;
    if pet.age_ticks < threshold {
        return None;
    }

    let ts = &pet.type_scores;
    let evo_type = determine_evo_type(ts.chikara, ts.odayaka, ts.bouken);

    // Random selection from matching type
    let candidates: Vec<&Stage2Species> = STAGE2_SPECIES
        .iter()
        .filter(|s| s.evo_type == evo_type)
        .collect();

    let species = candidates.choose(rng).unwrap();
    apply_evolution(pet, species.name, 2, species.standard_weight);

    Some(EvolutionEvent {
        new_species: species.name.to_string(),
        new_stage: 2,
    })
}

/// Determine EvoType from type tendency scores
fn determine_evo_type(chikara: u32, odayaka: u32, bouken: u32) -> EvoType {
    let max_score = chikara.max(odayaka).max(bouken);
    let min_score = chikara.min(odayaka).min(bouken);

    // All zero (no actions taken) → Wild type
    if max_score == 0 {
        return EvoType::Wild;
    }

    // Difference between max and min <= 3 → Normal type
    if max_score - min_score <= 3 {
        return EvoType::Normal;
    }

    // Type with highest score
    if chikara >= odayaka && chikara >= bouken {
        EvoType::Chikara
    } else if odayaka >= chikara && odayaka >= bouken {
        EvoType::Odayaka
    } else {
        EvoType::Bouken
    }
}

/// Stage2 → Stage3 (24〜48時間後、determined by cosine similarity)
fn check_stage2_to_3(pet: &mut PetData, rng: &mut impl Rng) -> Option<EvolutionEvent> {
    let threshold = STAGE3_TICKS_MIN + birth_hash(pet, 1) % STAGE3_TICKS_RANGE;
    if pet.age_ticks < threshold {
        return None;
    }

    // Build raising-style vector
    let total_actions =
        (pet.type_scores.chikara + pet.type_scores.odayaka + pet.type_scores.bouken) as f64;
    let frequency = (total_actions / 10.0).min(10.0); // Normalize

    let player_vec = [
        pet.type_scores.chikara as f64,
        pet.type_scores.odayaka as f64,
        pet.type_scores.bouken as f64,
        pet.nakayoshi / 10.0, // Scale to 0-10
        frequency,
    ];

    // Lineage constraint: only Stage3 species reachable from current Stage2
    let candidates: Vec<&Stage3Species> = STAGE3_SPECIES
        .iter()
        .filter(|s| s.allowed_from.contains(&pet.species.as_str()))
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // Rank by cosine similarity
    let mut scored: Vec<(&Stage3Species, f64)> = candidates
        .iter()
        .map(|s| (*s, cosine_similarity(&player_vec, &s.vector)))
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Weighted random selection from top 3 (for diversity)
    let top_n = scored.len().min(3);
    let weights: Vec<f64> = (0..top_n)
        .map(|i| {
            let base = scored[i].1.max(0.01);
            base * (top_n - i) as f64
        })
        .collect();

    let total_weight: f64 = weights.iter().sum();
    let mut roll = rng.gen::<f64>() * total_weight;

    let mut chosen = scored[0].0;
    for (i, w) in weights.iter().enumerate() {
        roll -= w;
        if roll <= 0.0 {
            chosen = scored[i].0;
            break;
        }
    }

    apply_evolution(pet, chosen.name, 3, chosen.standard_weight);

    Some(EvolutionEvent {
        new_species: chosen.name.to_string(),
        new_stage: 3,
    })
}

/// Stage3 → Stage4 (25% chance every 24 hours)
fn check_stage3_to_4(pet: &mut PetData, rng: &mut impl Rng) -> Option<EvolutionEvent> {
    if pet.age_ticks < STAGE3_TICKS_MIN + STAGE4_INTERVAL {
        return None;
    }

    // Calculate how many 24h intervals have passed since reaching Stage3 (approximate)
    let ticks_since_stage3 = pet.age_ticks.saturating_sub(STAGE3_TICKS_MIN);
    let total_intervals = ticks_since_stage3 / STAGE4_INTERVAL;

    // Only check intervals that haven't been checked yet
    let new_intervals = total_intervals.saturating_sub(pet.last_stage4_check);
    pet.last_stage4_check = total_intervals;

    if new_intervals == 0 {
        return None;
    }

    // Roll for each new interval: survival probability = (1 - 0.25)^new_intervals
    let survival_prob = (1.0 - STAGE4_CHANCE).powi(new_intervals as i32);
    if rng.gen::<f64>() < survival_prob {
        return None; // No mutation
    }

    // Determine mutation target
    let candidates: Vec<&Stage4Species> = STAGE4_SPECIES
        .iter()
        .filter(|s| s.allowed_from.contains(&pet.species.as_str()))
        .collect();

    if candidates.is_empty() {
        return None;
    }

    let species = candidates.choose(rng).unwrap();
    apply_evolution(pet, species.name, 4, species.standard_weight);

    Some(EvolutionEvent {
        new_species: species.name.to_string(),
        new_stage: 4,
    })
}

/// Common evolution application logic
fn apply_evolution(pet: &mut PetData, new_name: &str, new_stage: u8, standard_weight: f64) {
    pet.species = new_name.to_string();
    pet.stage = new_stage;
    pet.weight = standard_weight;
    pet.evolution_line.push(new_name.to_string());
}

/// Cosine similarity
fn cosine_similarity(a: &[f64; 5], b: &[f64; 5]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot / (mag_a * mag_b)
}

/// Unified species lookup: returns (stage, standard_weight, voice_type) for Stage2+ species.
fn find_species_info(species: &str) -> Option<(u8, f64, VoiceType)> {
    for s in STAGE2_SPECIES {
        if s.name == species {
            return Some((2, s.standard_weight, s.voice_type));
        }
    }
    for s in STAGE3_SPECIES {
        if s.name == species {
            return Some((3, s.standard_weight, s.voice_type));
        }
    }
    for s in STAGE4_SPECIES {
        if s.name == species {
            return Some((4, s.standard_weight, s.voice_type));
        }
    }
    None
}

/// Get voice type from species name
pub fn get_voice_type(species: &str) -> Option<VoiceType> {
    find_species_info(species).map(|(_, _, vt)| vt)
}

/// Get standard weight from species name (Stage2+)
pub fn get_standard_weight(species: &str) -> Option<f64> {
    find_species_info(species).map(|(_, w, _)| w)
}

/// Get stage number from species name
pub fn get_stage(species: &str) -> Option<u8> {
    find_species_info(species).map(|(stage, _, _)| stage)
}

/// List all species names (for album)
pub fn all_species_names() -> Vec<&'static str> {
    use crate::game::pet::STAGE1_SPECIES;

    let mut names = Vec::new();

    // Stage1
    for s in &STAGE1_SPECIES {
        names.push(s.name);
    }
    // Stage2
    for s in STAGE2_SPECIES {
        names.push(s.name);
    }
    // Stage3
    for s in STAGE3_SPECIES {
        names.push(s.name);
    }
    // Stage4
    for s in STAGE4_SPECIES {
        names.push(s.name);
    }

    names
}
