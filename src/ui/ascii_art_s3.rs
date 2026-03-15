//! Hand-crafted ASCII art for all Stage 3 species.
//!
//! Each species has a unique silhouette distinguishable from all others.

use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

/// Returns hand-crafted idle art for a Stage 3 species, or None if not found.
pub fn get_s3_art(species: &str, mood: MoodLevel, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        // Bouken type
        "ガニ" => gani_art(mood, frame),
        "トビオ" => tobio_art(mood, frame),
        "マルマル" => marumaru_art(mood, frame),
        "ハヤテ" => hayate_art(mood, frame),
        "グルグルン" => gurugurun_art(mood, frame),
        "カゼノコ" => kazenoko_art(mood, frame),
        "ドカーン" => dokaan_art(mood, frame),
        "スイスイ" => suisui_art(mood, frame),
        "サスライ" => sasurai_art(mood, frame),
        "ピカッ" => pikat_art(mood, frame),
        "バサバサ" => basabasa_art(mood, frame),
        "ウロチョロ" => urochoro_art(mood, frame),
        "ゴーゴー" => googoo_art(mood, frame),
        "クモノス" => kumonos_art(mood, frame),
        "ホシゾラ" => hoshizora_art(mood, frame),
        "ブッチギリ" => bucchigiri_art(mood, frame),
        "ワタリ" => watari_art(mood, frame),
        "ヒュー" => hyuu_art(mood, frame),
        "タンケン" => tanken_art(mood, frame),
        "ジェット" => jetto_art(mood, frame),
        // Normal type
        "ノーマル" => noomaru_art(mood, frame),
        "ヘイボン" => heibon_art(mood, frame),
        "タソガレ" => tasogare_art(mood, frame),
        "ニッコリ" => nikkori_art(mood, frame),
        "ダラーン" => daraan_art(mood, frame),
        "キッチリ" => kicchiri_art(mood, frame),
        "ボチボチ" => bochibochi_art(mood, frame),
        "マアマア" => maamaa_art(mood, frame),
        "フニャ" => funya_art(mood, frame),
        "テンテン" => tenten_art(mood, frame),
        "ナァナァ" => naanaa_art(mood, frame),
        "ポツリ" => potsuri_art(mood, frame),
        "ソレナリ" => sorenari_art(mood, frame),
        "ウンウン" => unun_art(mood, frame),
        "チャッカリ" => chakkari_art(mood, frame),
        "ヌルリ" => nururi_art(mood, frame),
        "ヤレヤレ" => yareyare_art(mood, frame),
        "ドッコイ" => dokkoi_art(mood, frame),
        "パッパ" => pappa_art(mood, frame),
        "オットリ" => ottori_art(mood, frame),
        // Chikara type
        "ドドン" => dodon_art(mood, frame),
        "タワーン" => tawaan_art(mood, frame),
        "ゴウケン" => gouken_art(mood, frame),
        "テッカイ" => tekkai_art(mood, frame),
        "ブンブン" => bunbun_art(mood, frame),
        "ガンテツ" => gantetsu_art(mood, frame),
        "ドスコイ" => dosukoi_art(mood, frame),
        "バリバリ" => baribari_art(mood, frame),
        "メガトン" => megaton_art(mood, frame),
        "グランド" => gurando_art(mood, frame),
        "イカヅチ" => ikazuchi_art(mood, frame),
        "ゴリラン" => goriran_art(mood, frame),
        "ダイガン" => daigan_art(mood, frame),
        "ゴロゴロ" => gorogoro_art(mood, frame),
        "カチワリ" => kachiwari_art(mood, frame),
        "テツジン" => tetsujin_art(mood, frame),
        "ドゴン" => dogon_art(mood, frame),
        "バンカー" => bankaa_art(mood, frame),
        "マッスル" => massuru_art(mood, frame),
        "イワオ" => iwao_art(mood, frame),
        // Odayaka type
        "ながれもん" => nagaremon_art(mood, frame),
        "フワリン" => fuwarin_art(mood, frame),
        "モコモコ" => mokomoko_art(mood, frame),
        "ネンネ" => nenne_art(mood, frame),
        "ポヨン" => poyon_art(mood, frame),
        "スヤスヤ" => suyasuya_art(mood, frame),
        "カスミ" => kasumi_art(mood, frame),
        "ノドカ" => nodoka_art(mood, frame),
        "ユメミ" => yumemi_art(mood, frame),
        "ボンヤリ" => bonyari_art(mood, frame),
        "ヒラタ" => hirata_art(mood, frame),
        "コロリン" => kororin_art(mood, frame),
        "ムニャ" => munya_art(mood, frame),
        "マッタリ" => mattari_art(mood, frame),
        "ホワワ" => howawa_art(mood, frame),
        "シズカ" => shizuka_art(mood, frame),
        "モグモグ" => mogumogu_art(mood, frame),
        "トロン" => toron_art(mood, frame),
        "ユッタリ" => yuttari_art(mood, frame),
        "ソヨカゼ" => soyokaze_art(mood, frame),
        // Wild type
        "ヤミノメ" => yaminome_art(mood, frame),
        "オオヌシ" => oonushi_art(mood, frame),
        "バケモノ" => bakemono_art(mood, frame),
        "ユウレイ" => yuurei_art(mood, frame),
        "ヤセイジ" => yaseiji_art(mood, frame),
        "シンエン" => shinen_art(mood, frame),
        "ノラクロ" => norakuro_art(mood, frame),
        "モノノケ" => mononoke_art(mood, frame),
        "クライ" => kurai_art(mood, frame),
        "アヤシイ" => ayashii_art(mood, frame),
        "ムジナ" => mujina_art(mood, frame),
        "ヌエ" => nue_art(mood, frame),
        "カマイタチ" => kamaitachi_art(mood, frame),
        "ドロドロ" => dorodoro_art(mood, frame),
        "ヒノタマ" => hinotama_art(mood, frame),
        "フルエ" => furue_art(mood, frame),
        "ケダマ" => kedama_art(mood, frame),
        "シノビ" => shinobi_art(mood, frame),
        "ジゴク" => jigoku_art(mood, frame),
        "ムゲン" => mugen_art(mood, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

/// Returns hand-crafted action art for a Stage 3 species, or None if not found.
pub fn get_s3_action_art(species: &str, action: Action, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        // Bouken type
        "ガニ" => gani_action(action, frame),
        "トビオ" => tobio_action(action, frame),
        "マルマル" => marumaru_action(action, frame),
        "ハヤテ" => hayate_action(action, frame),
        "グルグルン" => gurugurun_action(action, frame),
        "カゼノコ" => kazenoko_action(action, frame),
        "ドカーン" => dokaan_action(action, frame),
        "スイスイ" => suisui_action(action, frame),
        "サスライ" => sasurai_action(action, frame),
        "ピカッ" => pikat_action(action, frame),
        "バサバサ" => basabasa_action(action, frame),
        "ウロチョロ" => urochoro_action(action, frame),
        "ゴーゴー" => googoo_action(action, frame),
        "クモノス" => kumonos_action(action, frame),
        "ホシゾラ" => hoshizora_action(action, frame),
        "ブッチギリ" => bucchigiri_action(action, frame),
        "ワタリ" => watari_action(action, frame),
        "ヒュー" => hyuu_action(action, frame),
        "タンケン" => tanken_action(action, frame),
        "ジェット" => jetto_action(action, frame),
        // Normal type
        "ノーマル" => noomaru_action(action, frame),
        "ヘイボン" => heibon_action(action, frame),
        "タソガレ" => tasogare_action(action, frame),
        "ニッコリ" => nikkori_action(action, frame),
        "ダラーン" => daraan_action(action, frame),
        "キッチリ" => kicchiri_action(action, frame),
        "ボチボチ" => bochibochi_action(action, frame),
        "マアマア" => maamaa_action(action, frame),
        "フニャ" => funya_action(action, frame),
        "テンテン" => tenten_action(action, frame),
        "ナァナァ" => naanaa_action(action, frame),
        "ポツリ" => potsuri_action(action, frame),
        "ソレナリ" => sorenari_action(action, frame),
        "ウンウン" => unun_action(action, frame),
        "チャッカリ" => chakkari_action(action, frame),
        "ヌルリ" => nururi_action(action, frame),
        "ヤレヤレ" => yareyare_action(action, frame),
        "ドッコイ" => dokkoi_action(action, frame),
        "パッパ" => pappa_action(action, frame),
        "オットリ" => ottori_action(action, frame),
        // Chikara type
        "ドドン" => dodon_action(action, frame),
        "タワーン" => tawaan_action(action, frame),
        "ゴウケン" => gouken_action(action, frame),
        "テッカイ" => tekkai_action(action, frame),
        "ブンブン" => bunbun_action(action, frame),
        "ガンテツ" => gantetsu_action(action, frame),
        "ドスコイ" => dosukoi_action(action, frame),
        "バリバリ" => baribari_action(action, frame),
        "メガトン" => megaton_action(action, frame),
        "グランド" => gurando_action(action, frame),
        "イカヅチ" => ikazuchi_action(action, frame),
        "ゴリラン" => goriran_action(action, frame),
        "ダイガン" => daigan_action(action, frame),
        "ゴロゴロ" => gorogoro_action(action, frame),
        "カチワリ" => kachiwari_action(action, frame),
        "テツジン" => tetsujin_action(action, frame),
        "ドゴン" => dogon_action(action, frame),
        "バンカー" => bankaa_action(action, frame),
        "マッスル" => massuru_action(action, frame),
        "イワオ" => iwao_action(action, frame),
        // Odayaka type
        "ながれもん" => nagaremon_action(action, frame),
        "フワリン" => fuwarin_action(action, frame),
        "モコモコ" => mokomoko_action(action, frame),
        "ネンネ" => nenne_action(action, frame),
        "ポヨン" => poyon_action(action, frame),
        "スヤスヤ" => suyasuya_action(action, frame),
        "カスミ" => kasumi_action(action, frame),
        "ノドカ" => nodoka_action(action, frame),
        "ユメミ" => yumemi_action(action, frame),
        "ボンヤリ" => bonyari_action(action, frame),
        "ヒラタ" => hirata_action(action, frame),
        "コロリン" => kororin_action(action, frame),
        "ムニャ" => munya_action(action, frame),
        "マッタリ" => mattari_action(action, frame),
        "ホワワ" => howawa_action(action, frame),
        "シズカ" => shizuka_action(action, frame),
        "モグモグ" => mogumogu_action(action, frame),
        "トロン" => toron_action(action, frame),
        "ユッタリ" => yuttari_action(action, frame),
        "ソヨカゼ" => soyokaze_action(action, frame),
        // Wild type
        "ヤミノメ" => yaminome_action(action, frame),
        "オオヌシ" => oonushi_action(action, frame),
        "バケモノ" => bakemono_action(action, frame),
        "ユウレイ" => yuurei_action(action, frame),
        "ヤセイジ" => yaseiji_action(action, frame),
        "シンエン" => shinen_action(action, frame),
        "ノラクロ" => norakuro_action(action, frame),
        "モノノケ" => mononoke_action(action, frame),
        "クライ" => kurai_action(action, frame),
        "アヤシイ" => ayashii_action(action, frame),
        "ムジナ" => mujina_action(action, frame),
        "ヌエ" => nue_action(action, frame),
        "カマイタチ" => kamaitachi_action(action, frame),
        "ドロドロ" => dorodoro_action(action, frame),
        "ヒノタマ" => hinotama_action(action, frame),
        "フルエ" => furue_action(action, frame),
        "ケダマ" => kedama_action(action, frame),
        "シノビ" => shinobi_action(action, frame),
        "ジゴク" => jigoku_action(action, frame),
        "ムゲン" => mugen_action(action, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

// ============================================================
// CHIKARA TYPE Stage 3 Species
// ============================================================

// --- ドドン (dodon) - Giant drum/thunder ---
fn dodon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╔══╦══╗", " ▓(▽_▽)▓!", "  ╚═╩═╩═╝", "", ""],
        (MoodLevel::High, _) => &["  ╔══╦══╗", " ▓(▽_▽)▓♪", "  ╚═╩═╩═╝", "", ""],
        (MoodLevel::Normal, 0) => &["  ╔══╦══╗", " ▓(・_・)▓", "  ╚═╩═╩═╝", "", ""],
        (MoodLevel::Normal, _) => &["  ╔══╦══╗", " ▓(・ ・)▓", "  ╚═╩═╩═╝", "", ""],
        (MoodLevel::Low, 0) => &["  ╔══╦══╗", " ▓(￣_￣)▓", "  ╚═╩═╩═╝", "", ""],
        (MoodLevel::Low, _) => &["  ╔══╦══╗", " ▓(￣ ￣)▓", "  ╚═╩═╩═╝", "", ""],
    }
}
fn dodon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╔══╦══╗", " ﾉ(・_・)▓", "  ╚═╩═╩═╝", "", ""],
        (Action::Talk, _) => &["  ╔══╦══╗", " ▓(・_・)ﾉ", "  ╚═╩═╩═╝", "", ""],
        (Action::Play, 0) => &[" ♪╔══╦══╗", " ▓(▽_▽)▓", "  ╚═╩═╩═╝", "", ""],
        (Action::Play, _) => &["  ╔══╦══╗♪", " ▓(▽_▽)▓", "  ╚═╩═╩═╝", "", ""],
        (Action::Train, 0) => &["  ╔══╦══╗!!", " ▓(益_益)▓", " ╚══╩══╩╝", "", ""],
        (Action::Train, _) => &["!!╔══╦══╗", " ▓(益_益)▓", " ╚══╩══╩╝", "", ""],
        (Action::Relax, 0) => &["  ╔══╦══╗～", " ▓(－_－)▓", "  ╚═╩═╩═╝", "", ""],
        (Action::Relax, _) => &["  ╔══╦══╗", " ▓(－_－)▓zzZ", "  ╚═╩═╩═╝", "", ""],
    }
}

// --- タワーン (tawaan) - Tower-like tall ---
fn tawaan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   ┃▲▲▲┃", "  ┃(▽_▽)┃!", "   ┗━━━┛", "", ""],
        (MoodLevel::High, _) => &["   ┃▲▲▲┃", "  ┃(▽_▽)┃♪", "   ┗━━━┛", "", ""],
        (MoodLevel::Normal, 0) => &["   ┃▲▲▲┃", "  ┃(・_・)┃", "   ┗━━━┛", "", ""],
        (MoodLevel::Normal, _) => &["   ┃▲▲▲┃", "  ┃(・ ・)┃", "   ┗━━━┛", "", ""],
        (MoodLevel::Low, 0) => &["   ┃▲▲▲┃", "  ┃(￣_￣)┃", "   ┗━━━┛", "", ""],
        (MoodLevel::Low, _) => &["   ┃▲▲▲┃", "  ┃(￣ ￣)┃", "   ┗━━━┛", "", ""],
    }
}
fn tawaan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ┃▲▲▲┃", " ﾉ┃(・_・)┃", "   ┗━━━┛", "", ""],
        (Action::Talk, _) => &["   ┃▲▲▲┃", "  ┃(・_・)┃ﾉ", "   ┗━━━┛", "", ""],
        (Action::Play, 0) => &["  ♪┃▲▲▲┃", "  ┃(▽_▽)┃", "   ┗━━━┛", "", ""],
        (Action::Play, _) => &["   ┃▲▲▲┃♪", "  ┃(▽_▽)┃", "   ┗━━━┛", "", ""],
        (Action::Train, 0) => &["   ┃▲▲▲┃!!", "  ┃(益_益)┃", "  ┗━━━━┛", "", ""],
        (Action::Train, _) => &["!!┃▲▲▲┃", "  ┃(益_益)┃", "  ┗━━━━┛", "", ""],
        (Action::Relax, 0) => &["   ┃▲▲▲┃～", "  ┃(－_－)┃", "   ┗━━━┛", "", ""],
        (Action::Relax, _) => &["   ┃▲▲▲┃", "  ┃(－_－)┃zzZ", "   ┗━━━┛", "", ""],
    }
}

// --- ゴウケン (gouken) - Strong fist ---
fn gouken_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ＊█████＊", " █(▽_▽)█!", "  ▀█████▀", "", ""],
        (MoodLevel::High, _) => &[" ＊█████＊", " █(▽_▽)█♪", "  ▀█████▀", "", ""],
        (MoodLevel::Normal, 0) => &[" ＊█████＊", " █(・_・)█", "  ▀█████▀", "", ""],
        (MoodLevel::Normal, _) => &[" ＊█████＊", " █(・ ・)█", "  ▀█████▀", "", ""],
        (MoodLevel::Low, 0) => &[" ＊█████＊", " █(￣_￣)█", "  ▀█████▀", "", ""],
        (MoodLevel::Low, _) => &[" ＊█████＊", " █(￣ ￣)█", "  ▀█████▀", "", ""],
    }
}
fn gouken_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ＊█████＊", " ﾉ(・_・)█", "  ▀█████▀", "", ""],
        (Action::Talk, _) => &[" ＊█████＊", " █(・_・)ﾉ", "  ▀█████▀", "", ""],
        (Action::Play, 0) => &[" ♪█████＊", " █(▽_▽)█", "  ▀█████▀", "", ""],
        (Action::Play, _) => &[" ＊█████♪", " █(▽_▽)█", "  ▀█████▀", "", ""],
        (Action::Train, 0) => &[" ＊█████＊!!", " █(益_益)█", " ▀███████▀", "", ""],
        (Action::Train, _) => &["!!＊█████＊", " █(益_益)█", " ▀███████▀", "", ""],
        (Action::Relax, 0) => &[" ＊█████＊～", " █(－_－)█", "  ▀█████▀", "", ""],
        (Action::Relax, _) => &[" ＊█████＊", " █(－_－)█zzZ", "  ▀█████▀", "", ""],
    }
}

// --- テッカイ (tekkai) - Iron armor ---
fn tekkai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ▓▓╬╬╬▓▓", " ▓(▽_▽)▓!", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::High, _) => &[" ▓▓╬╬╬▓▓", " ▓(▽_▽)▓♪", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Normal, 0) => &[" ▓▓╬╬╬▓▓", " ▓(・_・)▓", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Normal, _) => &[" ▓▓╬╬╬▓▓", " ▓(・ ・)▓", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Low, 0) => &[" ▓▓╬╬╬▓▓", " ▓(￣_￣)▓", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Low, _) => &[" ▓▓╬╬╬▓▓", " ▓(￣ ￣)▓", " ▓▓▓▓▓▓▓", "", ""],
    }
}
fn tekkai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ▓▓╬╬╬▓▓", " ﾉ(・_・)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Talk, _) => &[" ▓▓╬╬╬▓▓", " ▓(・_・)ﾉ", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Play, 0) => &[" ♪▓╬╬╬▓▓", " ▓(▽_▽)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Play, _) => &[" ▓▓╬╬╬▓♪", " ▓(▽_▽)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Train, 0) => &[" ▓▓╬╬╬▓▓!!", " ▓(益_益)▓", " ▓▓▓▓▓▓▓▓", "", ""],
        (Action::Train, _) => &["!!▓▓╬╬╬▓▓", " ▓(益_益)▓", " ▓▓▓▓▓▓▓▓", "", ""],
        (Action::Relax, 0) => &[" ▓▓╬╬╬▓▓～", " ▓(－_－)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Relax, _) => &[" ▓▓╬╬╬▓▓", " ▓(－_－)▓zzZ", " ▓▓▓▓▓▓▓", "", ""],
    }
}

// --- ブンブン (bunbun) - Buzzing/swinging ---
fn bunbun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ≈≈≈≈≈≈", " ≈(▽_▽)≈!", "  ≈≈≈≈≈≈", "", ""],
        (MoodLevel::High, _) => &["  ≈≈≈≈≈≈", " ≈(▽_▽)≈♪", "  ≈≈≈≈≈≈", "", ""],
        (MoodLevel::Normal, 0) => &["  ≈≈≈≈≈≈", " ≈(・_・)≈", "  ≈≈≈≈≈≈", "", ""],
        (MoodLevel::Normal, _) => &["  ≈≈≈≈≈≈", " ≈(・ ・)≈", "  ≈≈≈≈≈≈", "", ""],
        (MoodLevel::Low, 0) => &["  ≈≈≈≈≈≈", " ≈(￣_￣)≈", "  ≈≈≈≈≈≈", "", ""],
        (MoodLevel::Low, _) => &["  ≈≈≈≈≈≈", " ≈(￣ ￣)≈", "  ≈≈≈≈≈≈", "", ""],
    }
}
fn bunbun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ≈≈≈≈≈≈", " ﾉ(・_・)≈", "  ≈≈≈≈≈≈", "", ""],
        (Action::Talk, _) => &["  ≈≈≈≈≈≈", " ≈(・_・)ﾉ", "  ≈≈≈≈≈≈", "", ""],
        (Action::Play, 0) => &[" ♪≈≈≈≈≈≈", " ≈(▽_▽)≈", "  ≈≈≈≈≈≈", "", ""],
        (Action::Play, _) => &["  ≈≈≈≈≈≈♪", " ≈(▽_▽)≈", "  ≈≈≈≈≈≈", "", ""],
        (Action::Train, 0) => &["  ≈≈≈≈≈≈!!", " ≈(益_益)≈", "  ≈≈≈≈≈≈≈", "", ""],
        (Action::Train, _) => &["!!≈≈≈≈≈≈", " ≈(益_益)≈", "  ≈≈≈≈≈≈≈", "", ""],
        (Action::Relax, 0) => &["  ≈≈≈≈≈≈～", " ≈(－_－)≈", "  ≈≈≈≈≈≈", "", ""],
        (Action::Relax, _) => &["  ≈≈≈≈≈≈", " ≈(－_－)≈zzZ", "  ≈≈≈≈≈≈", "", ""],
    }
}

// --- ガンテツ (gantetsu) - Iron wall ---
fn gantetsu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ■■■■■■", " ■(▽_▽)■!", " ■■■■■■", "", ""],
        (MoodLevel::High, _) => &[" ■■■■■■", " ■(▽_▽)■♪", " ■■■■■■", "", ""],
        (MoodLevel::Normal, 0) => &[" ■■■■■■", " ■(・_・)■", " ■■■■■■", "", ""],
        (MoodLevel::Normal, _) => &[" ■■■■■■", " ■(・ ・)■", " ■■■■■■", "", ""],
        (MoodLevel::Low, 0) => &[" ■■■■■■", " ■(￣_￣)■", " ■■■■■■", "", ""],
        (MoodLevel::Low, _) => &[" ■■■■■■", " ■(￣ ￣)■", " ■■■■■■", "", ""],
    }
}
fn gantetsu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ■■■■■■", " ﾉ(・_・)■", " ■■■■■■", "", ""],
        (Action::Talk, _) => &[" ■■■■■■", " ■(・_・)ﾉ", " ■■■■■■", "", ""],
        (Action::Play, 0) => &[" ♪■■■■■", " ■(▽_▽)■", " ■■■■■■", "", ""],
        (Action::Play, _) => &[" ■■■■■♪", " ■(▽_▽)■", " ■■■■■■", "", ""],
        (Action::Train, 0) => &[" ■■■■■■!!", " ■(益_益)■", " ■■■■■■■", "", ""],
        (Action::Train, _) => &["!!■■■■■■", " ■(益_益)■", " ■■■■■■■", "", ""],
        (Action::Relax, 0) => &[" ■■■■■■～", " ■(－_－)■", " ■■■■■■", "", ""],
        (Action::Relax, _) => &[" ■■■■■■", " ■(－_－)■zzZ", " ■■■■■■", "", ""],
    }
}

// --- ドスコイ (dosukoi) - Sumo wrestler ---
fn dosukoi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ○━━━○", " ／(▽_▽)＼!", " ○○○○○○", "", ""],
        (MoodLevel::High, _) => &["  ○━━━○", " ／(▽_▽)＼♪", " ○○○○○○", "", ""],
        (MoodLevel::Normal, 0) => &["  ○━━━○", " ／(・_・)＼", " ○○○○○○", "", ""],
        (MoodLevel::Normal, _) => &["  ○━━━○", " ／(・ ・)＼", " ○○○○○○", "", ""],
        (MoodLevel::Low, 0) => &["  ○━━━○", " ／(￣_￣)＼", " ○○○○○○", "", ""],
        (MoodLevel::Low, _) => &["  ○━━━○", " ／(￣ ￣)＼", " ○○○○○○", "", ""],
    }
}
fn dosukoi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ○━━━○", " ﾉ(・_・)＼", " ○○○○○○", "", ""],
        (Action::Talk, _) => &["  ○━━━○", " ／(・_・)ﾉ", " ○○○○○○", "", ""],
        (Action::Play, 0) => &[" ♪○━━━○", " ／(▽_▽)＼", " ○○○○○○", "", ""],
        (Action::Play, _) => &["  ○━━━○♪", " ／(▽_▽)＼", " ○○○○○○", "", ""],
        (Action::Train, 0) => &["  ○━━━○!!", " ／(益_益)＼", " ○○○○○○○", "", ""],
        (Action::Train, _) => &["!!○━━━○", " ／(益_益)＼", " ○○○○○○○", "", ""],
        (Action::Relax, 0) => &["  ○━━━○～", " ／(－_－)＼", " ○○○○○○", "", ""],
        (Action::Relax, _) => &["  ○━━━○", " ／(－_－)＼zzZ", " ○○○○○○", "", ""],
    }
}

// --- バリバリ (baribari) - Crackling energy ---
fn baribari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＊╠═╬═╣＊", " ｢(▽_▽)｣!", "  ╠═════╣", "", ""],
        (MoodLevel::High, _) => &["  ＊╠═╬═╣＊", " ｢(▽_▽)｣♪", "  ╠═════╣", "", ""],
        (MoodLevel::Normal, 0) => &["  ＊╠═╬═╣＊", " ｢(・_・)｣", "  ╠═════╣", "", ""],
        (MoodLevel::Normal, _) => &["  ＊╠═╬═╣＊", " ｢(・ ・)｣", "  ╠═════╣", "", ""],
        (MoodLevel::Low, 0) => &["  ＊╠═╬═╣＊", " ｢(￣_￣)｣", "  ╠═════╣", "", ""],
        (MoodLevel::Low, _) => &["  ＊╠═╬═╣＊", " ｢(￣ ￣)｣", "  ╠═════╣", "", ""],
    }
}
fn baribari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＊╠═╬═╣＊", " ﾉ(・_・)｣", "  ╠═════╣", "", ""],
        (Action::Talk, _) => &["  ＊╠═╬═╣＊", " ｢(・_・)ﾉ", "  ╠═════╣", "", ""],
        (Action::Play, 0) => &[" ♪＊╠═╬═╣", " ｢(▽_▽)｣", "  ╠═════╣", "", ""],
        (Action::Play, _) => &["  ╠═╬═╣＊♪", " ｢(▽_▽)｣", "  ╠═════╣", "", ""],
        (Action::Train, 0) => &["  ＊╠═╬═╣＊!!", " ｢(益_益)｣", " ╠══════╣", "", ""],
        (Action::Train, _) => &["!!＊╠═╬═╣＊", " ｢(益_益)｣", " ╠══════╣", "", ""],
        (Action::Relax, 0) => &["  ＊╠═╬═╣＊～", " ｢(－_－)｣", "  ╠═════╣", "", ""],
        (Action::Relax, _) => &["  ＊╠═╬═╣＊", " ｢(－_－)｣zzZ", "  ╠═════╣", "", ""],
    }
}

// --- メガトン (megaton) - Mega heavy ---
fn megaton_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ▄▄███▄▄", " ▌(▽_▽)▐!", " ▀▀███▀▀", "", ""],
        (MoodLevel::High, _) => &[" ▄▄███▄▄", " ▌(▽_▽)▐♪", " ▀▀███▀▀", "", ""],
        (MoodLevel::Normal, 0) => &[" ▄▄███▄▄", " ▌(・_・)▐", " ▀▀███▀▀", "", ""],
        (MoodLevel::Normal, _) => &[" ▄▄███▄▄", " ▌(・ ・)▐", " ▀▀███▀▀", "", ""],
        (MoodLevel::Low, 0) => &[" ▄▄███▄▄", " ▌(￣_￣)▐", " ▀▀███▀▀", "", ""],
        (MoodLevel::Low, _) => &[" ▄▄███▄▄", " ▌(￣ ￣)▐", " ▀▀███▀▀", "", ""],
    }
}
fn megaton_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ▄▄███▄▄", " ﾉ(・_・)▐", " ▀▀███▀▀", "", ""],
        (Action::Talk, _) => &[" ▄▄███▄▄", " ▌(・_・)ﾉ", " ▀▀███▀▀", "", ""],
        (Action::Play, 0) => &[" ♪▄███▄▄", " ▌(▽_▽)▐", " ▀▀███▀▀", "", ""],
        (Action::Play, _) => &[" ▄▄███▄♪", " ▌(▽_▽)▐", " ▀▀███▀▀", "", ""],
        (Action::Train, 0) => &[" ▄▄███▄▄!!", " ▌(益_益)▐", " ▀▀█████▀▀", "", ""],
        (Action::Train, _) => &["!!▄▄███▄▄", " ▌(益_益)▐", " ▀▀█████▀▀", "", ""],
        (Action::Relax, 0) => &[" ▄▄███▄▄～", " ▌(－_－)▐", " ▀▀███▀▀", "", ""],
        (Action::Relax, _) => &[" ▄▄███▄▄", " ▌(－_－)▐zzZ", " ▀▀███▀▀", "", ""],
    }
}

// --- グランド (gurando) - Grand/earth ---
fn gurando_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＿▓▓▓＿", " ▓(▽_▽)▓!", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::High, _) => &["  ＿▓▓▓＿", " ▓(▽_▽)▓♪", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Normal, 0) => &["  ＿▓▓▓＿", " ▓(・_・)▓", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Normal, _) => &["  ＿▓▓▓＿", " ▓(・ ・)▓", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Low, 0) => &["  ＿▓▓▓＿", " ▓(￣_￣)▓", " ▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Low, _) => &["  ＿▓▓▓＿", " ▓(￣ ￣)▓", " ▓▓▓▓▓▓▓", "", ""],
    }
}
fn gurando_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＿▓▓▓＿", " ﾉ(・_・)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Talk, _) => &["  ＿▓▓▓＿", " ▓(・_・)ﾉ", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Play, 0) => &[" ♪＿▓▓▓＿", " ▓(▽_▽)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Play, _) => &["  ＿▓▓▓＿♪", " ▓(▽_▽)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Train, 0) => &["  ＿▓▓▓＿!!", " ▓(益_益)▓", " ▓▓▓▓▓▓▓▓", "", ""],
        (Action::Train, _) => &["!!＿▓▓▓＿", " ▓(益_益)▓", " ▓▓▓▓▓▓▓▓", "", ""],
        (Action::Relax, 0) => &["  ＿▓▓▓＿～", " ▓(－_－)▓", " ▓▓▓▓▓▓▓", "", ""],
        (Action::Relax, _) => &["  ＿▓▓▓＿", " ▓(－_－)▓zzZ", " ▓▓▓▓▓▓▓", "", ""],
    }
}

// --- イカヅチ (ikazuchi) - Lightning ---
fn ikazuchi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ⚡╔══╗⚡", " ｛(▽_▽)｝!", "  ⚡╚══╝⚡", "", ""],
        (MoodLevel::High, _) => &["  ⚡╔══╗⚡", " ｛(▽_▽)｝♪", "  ⚡╚══╝⚡", "", ""],
        (MoodLevel::Normal, 0) => &["  ⚡╔══╗⚡", " ｛(・_・)｝", "  ⚡╚══╝⚡", "", ""],
        (MoodLevel::Normal, _) => &["  ⚡╔══╗⚡", " ｛(・ ・)｝", "  ⚡╚══╝⚡", "", ""],
        (MoodLevel::Low, 0) => &["  ⚡╔══╗⚡", " ｛(￣_￣)｝", "  ⚡╚══╝⚡", "", ""],
        (MoodLevel::Low, _) => &["  ⚡╔══╗⚡", " ｛(￣ ￣)｝", "  ⚡╚══╝⚡", "", ""],
    }
}
fn ikazuchi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ⚡╔══╗⚡", " ﾉ(・_・)｝", "  ⚡╚══╝⚡", "", ""],
        (Action::Talk, _) => &["  ⚡╔══╗⚡", " ｛(・_・)ﾉ", "  ⚡╚══╝⚡", "", ""],
        (Action::Play, 0) => &[" ♪⚡╔══╗⚡", " ｛(▽_▽)｝", "  ⚡╚══╝⚡", "", ""],
        (Action::Play, _) => &["  ⚡╔══╗⚡♪", " ｛(▽_▽)｝", "  ⚡╚══╝⚡", "", ""],
        (Action::Train, 0) => &["  ⚡╔══╗⚡!!", " ｛(益_益)｝", " ⚡╚════╝⚡", "", ""],
        (Action::Train, _) => &["!!⚡╔══╗⚡", " ｛(益_益)｝", " ⚡╚════╝⚡", "", ""],
        (Action::Relax, 0) => &["  ⚡╔══╗⚡～", " ｛(－_－)｝", "  ⚡╚══╝⚡", "", ""],
        (Action::Relax, _) => &["  ⚡╔══╗⚡", " ｛(－_－)｝zzZ", "  ⚡╚══╝⚡", "", ""],
    }
}

// --- ゴリラン (goriran) - Gorilla-like ---
fn goriran_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ＊＊▲▲▲＊＊", " ｜(▽_▽)｜!", "  ＿━━━＿", "", ""],
        (MoodLevel::High, _) => &[" ＊＊▲▲▲＊＊", " ｜(▽_▽)｜♪", "  ＿━━━＿", "", ""],
        (MoodLevel::Normal, 0) => &[" ＊＊▲▲▲＊＊", " ｜(・_・)｜", "  ＿━━━＿", "", ""],
        (MoodLevel::Normal, _) => &[" ＊＊▲▲▲＊＊", " ｜(・ ・)｜", "  ＿━━━＿", "", ""],
        (MoodLevel::Low, 0) => &[" ＊＊▲▲▲＊＊", " ｜(￣_￣)｜", "  ＿━━━＿", "", ""],
        (MoodLevel::Low, _) => &[" ＊＊▲▲▲＊＊", " ｜(￣ ￣)｜", "  ＿━━━＿", "", ""],
    }
}
fn goriran_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ＊＊▲▲▲＊＊", " ﾉ(・_・)｜", "  ＿━━━＿", "", ""],
        (Action::Talk, _) => &[" ＊＊▲▲▲＊＊", " ｜(・_・)ﾉ", "  ＿━━━＿", "", ""],
        (Action::Play, 0) => &[" ♪＊▲▲▲＊＊", " ｜(▽_▽)｜", "  ＿━━━＿", "", ""],
        (Action::Play, _) => &[" ＊＊▲▲▲＊♪", " ｜(▽_▽)｜", "  ＿━━━＿", "", ""],
        (Action::Train, 0) => &[" ＊＊▲▲▲＊＊!!", " ｜(益_益)｜", "  ＿━━━━＿", "", ""],
        (Action::Train, _) => &["!!＊＊▲▲▲＊＊", " ｜(益_益)｜", "  ＿━━━━＿", "", ""],
        (Action::Relax, 0) => &[" ＊＊▲▲▲＊＊～", " ｜(－_－)｜", "  ＿━━━＿", "", ""],
        (Action::Relax, _) => &[" ＊＊▲▲▲＊＊", " ｜(－_－)｜zzZ", "  ＿━━━＿", "", ""],
    }
}

// --- ダイガン (daigan) - Great boulder ---
fn daigan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ◆◆◆◆◆◆", " ◆(▽_▽)◆!", " ◆◆◆◆◆◆", "", ""],
        (MoodLevel::High, _) => &[" ◆◆◆◆◆◆", " ◆(▽_▽)◆♪", " ◆◆◆◆◆◆", "", ""],
        (MoodLevel::Normal, 0) => &[" ◆◆◆◆◆◆", " ◆(・_・)◆", " ◆◆◆◆◆◆", "", ""],
        (MoodLevel::Normal, _) => &[" ◆◆◆◆◆◆", " ◆(・ ・)◆", " ◆◆◆◆◆◆", "", ""],
        (MoodLevel::Low, 0) => &[" ◆◆◆◆◆◆", " ◆(￣_￣)◆", " ◆◆◆◆◆◆", "", ""],
        (MoodLevel::Low, _) => &[" ◆◆◆◆◆◆", " ◆(￣ ￣)◆", " ◆◆◆◆◆◆", "", ""],
    }
}
fn daigan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ◆◆◆◆◆◆", " ﾉ(・_・)◆", " ◆◆◆◆◆◆", "", ""],
        (Action::Talk, _) => &[" ◆◆◆◆◆◆", " ◆(・_・)ﾉ", " ◆◆◆◆◆◆", "", ""],
        (Action::Play, 0) => &[" ♪◆◆◆◆◆", " ◆(▽_▽)◆", " ◆◆◆◆◆◆", "", ""],
        (Action::Play, _) => &[" ◆◆◆◆◆♪", " ◆(▽_▽)◆", " ◆◆◆◆◆◆", "", ""],
        (Action::Train, 0) => &[" ◆◆◆◆◆◆!!", " ◆(益_益)◆", " ◆◆◆◆◆◆◆", "", ""],
        (Action::Train, _) => &["!!◆◆◆◆◆◆", " ◆(益_益)◆", " ◆◆◆◆◆◆◆", "", ""],
        (Action::Relax, 0) => &[" ◆◆◆◆◆◆～", " ◆(－_－)◆", " ◆◆◆◆◆◆", "", ""],
        (Action::Relax, _) => &[" ◆◆◆◆◆◆", " ◆(－_－)◆zzZ", " ◆◆◆◆◆◆", "", ""],
    }
}

// --- ゴロゴロ (gorogoro) - Rolling thunder ---
fn gorogoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ●━●━●━", " ●(▽_▽)●!", "  ●━●━●", "", ""],
        (MoodLevel::High, _) => &["  ━●━●━●", " ●(▽_▽)●♪", "  ●━●━●", "", ""],
        (MoodLevel::Normal, 0) => &["  ●━●━●━", " ●(・_・)●", "  ●━●━●", "", ""],
        (MoodLevel::Normal, _) => &["  ━●━●━●", " ●(・ ・)●", "  ●━●━●", "", ""],
        (MoodLevel::Low, 0) => &["  ●━●━●━", " ●(￣_￣)●", "  ●━●━●", "", ""],
        (MoodLevel::Low, _) => &["  ━●━●━●", " ●(￣ ￣)●", "  ●━●━●", "", ""],
    }
}
fn gorogoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ●━●━●━", " ﾉ(・_・)●", "  ●━●━●", "", ""],
        (Action::Talk, _) => &["  ━●━●━●", " ●(・_・)ﾉ", "  ●━●━●", "", ""],
        (Action::Play, 0) => &[" ♪●━●━●", " ●(▽_▽)●", "  ●━●━●", "", ""],
        (Action::Play, _) => &["  ●━●━●♪", " ●(▽_▽)●", "  ●━●━●", "", ""],
        (Action::Train, 0) => &["  ●━●━●━!!", " ●(益_益)●", "  ●━●━●━", "", ""],
        (Action::Train, _) => &["!!━●━●━●", " ●(益_益)●", "  ━●━●━●", "", ""],
        (Action::Relax, 0) => &["  ●━●━●━～", " ●(－_－)●", "  ●━●━●", "", ""],
        (Action::Relax, _) => &["  ●━●━●━", " ●(－_－)●zzZ", "  ●━●━●", "", ""],
    }
}

// --- カチワリ (kachiwari) - Splitting/cracking ---
fn kachiwari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╱▓▓▓╲", " ╱(▽_▽)╲!", "  ╲▓▓▓╱", "", ""],
        (MoodLevel::High, _) => &["  ╱▓▓▓╲", " ╱(▽_▽)╲♪", "  ╲▓▓▓╱", "", ""],
        (MoodLevel::Normal, 0) => &["  ╱▓▓▓╲", " ╱(・_・)╲", "  ╲▓▓▓╱", "", ""],
        (MoodLevel::Normal, _) => &["  ╱▓▓▓╲", " ╱(・ ・)╲", "  ╲▓▓▓╱", "", ""],
        (MoodLevel::Low, 0) => &["  ╱▓▓▓╲", " ╱(￣_￣)╲", "  ╲▓▓▓╱", "", ""],
        (MoodLevel::Low, _) => &["  ╱▓▓▓╲", " ╱(￣ ￣)╲", "  ╲▓▓▓╱", "", ""],
    }
}
fn kachiwari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╱▓▓▓╲", " ﾉ(・_・)╲", "  ╲▓▓▓╱", "", ""],
        (Action::Talk, _) => &["  ╱▓▓▓╲", " ╱(・_・)ﾉ", "  ╲▓▓▓╱", "", ""],
        (Action::Play, 0) => &[" ♪╱▓▓▓╲", " ╱(▽_▽)╲", "  ╲▓▓▓╱", "", ""],
        (Action::Play, _) => &["  ╱▓▓▓╲♪", " ╱(▽_▽)╲", "  ╲▓▓▓╱", "", ""],
        (Action::Train, 0) => &["  ╱▓▓▓╲!!", " ╱(益_益)╲", "  ╲▓▓▓▓╱", "", ""],
        (Action::Train, _) => &["!!╱▓▓▓╲", " ╱(益_益)╲", "  ╲▓▓▓▓╱", "", ""],
        (Action::Relax, 0) => &["  ╱▓▓▓╲～", " ╱(－_－)╲", "  ╲▓▓▓╱", "", ""],
        (Action::Relax, _) => &["  ╱▓▓▓╲", " ╱(－_－)╲zzZ", "  ╲▓▓▓╱", "", ""],
    }
}

// --- テツジン (tetsujin) - Iron man ---
fn tetsujin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ┏━╦━╦━┓", " ┃(▽_▽)┃!", " ┗━╩━╩━┛", "", ""],
        (MoodLevel::High, _) => &[" ┏━╦━╦━┓", " ┃(▽_▽)┃♪", " ┗━╩━╩━┛", "", ""],
        (MoodLevel::Normal, 0) => &[" ┏━╦━╦━┓", " ┃(・_・)┃", " ┗━╩━╩━┛", "", ""],
        (MoodLevel::Normal, _) => &[" ┏━╦━╦━┓", " ┃(・ ・)┃", " ┗━╩━╩━┛", "", ""],
        (MoodLevel::Low, 0) => &[" ┏━╦━╦━┓", " ┃(￣_￣)┃", " ┗━╩━╩━┛", "", ""],
        (MoodLevel::Low, _) => &[" ┏━╦━╦━┓", " ┃(￣ ￣)┃", " ┗━╩━╩━┛", "", ""],
    }
}
fn tetsujin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ┏━╦━╦━┓", " ﾉ(・_・)┃", " ┗━╩━╩━┛", "", ""],
        (Action::Talk, _) => &[" ┏━╦━╦━┓", " ┃(・_・)ﾉ", " ┗━╩━╩━┛", "", ""],
        (Action::Play, 0) => &[" ♪┏━╦━╦━┓", " ┃(▽_▽)┃", " ┗━╩━╩━┛", "", ""],
        (Action::Play, _) => &[" ┏━╦━╦━┓♪", " ┃(▽_▽)┃", " ┗━╩━╩━┛", "", ""],
        (Action::Train, 0) => &[" ┏━╦━╦━┓!!", " ┃(益_益)┃", " ┗━╩━╩━╩┛", "", ""],
        (Action::Train, _) => &["!!┏━╦━╦━┓", " ┃(益_益)┃", " ┗━╩━╩━╩┛", "", ""],
        (Action::Relax, 0) => &[" ┏━╦━╦━┓～", " ┃(－_－)┃", " ┗━╩━╩━┛", "", ""],
        (Action::Relax, _) => &[" ┏━╦━╦━┓", " ┃(－_－)┃zzZ", " ┗━╩━╩━┛", "", ""],
    }
}

// --- ドゴン (dogon) - Deep impact ---
fn dogon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╠╬╬╬╬╣", " ╠(▽_▽)╣!", "  ╚╩╩╩╩╝", "", ""],
        (MoodLevel::High, _) => &["  ╠╬╬╬╬╣", " ╠(▽_▽)╣♪", "  ╚╩╩╩╩╝", "", ""],
        (MoodLevel::Normal, 0) => &["  ╠╬╬╬╬╣", " ╠(・_・)╣", "  ╚╩╩╩╩╝", "", ""],
        (MoodLevel::Normal, _) => &["  ╠╬╬╬╬╣", " ╠(・ ・)╣", "  ╚╩╩╩╩╝", "", ""],
        (MoodLevel::Low, 0) => &["  ╠╬╬╬╬╣", " ╠(￣_￣)╣", "  ╚╩╩╩╩╝", "", ""],
        (MoodLevel::Low, _) => &["  ╠╬╬╬╬╣", " ╠(￣ ￣)╣", "  ╚╩╩╩╩╝", "", ""],
    }
}
fn dogon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╠╬╬╬╬╣", " ﾉ(・_・)╣", "  ╚╩╩╩╩╝", "", ""],
        (Action::Talk, _) => &["  ╠╬╬╬╬╣", " ╠(・_・)ﾉ", "  ╚╩╩╩╩╝", "", ""],
        (Action::Play, 0) => &[" ♪╠╬╬╬╬╣", " ╠(▽_▽)╣", "  ╚╩╩╩╩╝", "", ""],
        (Action::Play, _) => &["  ╠╬╬╬╬╣♪", " ╠(▽_▽)╣", "  ╚╩╩╩╩╝", "", ""],
        (Action::Train, 0) => &["  ╠╬╬╬╬╣!!", " ╠(益_益)╣", " ╚╩╩╩╩╩╝", "", ""],
        (Action::Train, _) => &["!!╠╬╬╬╬╣", " ╠(益_益)╣", " ╚╩╩╩╩╩╝", "", ""],
        (Action::Relax, 0) => &["  ╠╬╬╬╬╣～", " ╠(－_－)╣", "  ╚╩╩╩╩╝", "", ""],
        (Action::Relax, _) => &["  ╠╬╬╬╬╣", " ╠(－_－)╣zzZ", "  ╚╩╩╩╩╝", "", ""],
    }
}

// --- バンカー (bankaa) - Bunker ---
fn bankaa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ╔▓╦▓╦▓╗", " ║(▽_▽)║!", " ╚▓╩▓╩▓╝", "", ""],
        (MoodLevel::High, _) => &[" ╔▓╦▓╦▓╗", " ║(▽_▽)║♪", " ╚▓╩▓╩▓╝", "", ""],
        (MoodLevel::Normal, 0) => &[" ╔▓╦▓╦▓╗", " ║(・_・)║", " ╚▓╩▓╩▓╝", "", ""],
        (MoodLevel::Normal, _) => &[" ╔▓╦▓╦▓╗", " ║(・ ・)║", " ╚▓╩▓╩▓╝", "", ""],
        (MoodLevel::Low, 0) => &[" ╔▓╦▓╦▓╗", " ║(￣_￣)║", " ╚▓╩▓╩▓╝", "", ""],
        (MoodLevel::Low, _) => &[" ╔▓╦▓╦▓╗", " ║(￣ ￣)║", " ╚▓╩▓╩▓╝", "", ""],
    }
}
fn bankaa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ╔▓╦▓╦▓╗", " ﾉ(・_・)║", " ╚▓╩▓╩▓╝", "", ""],
        (Action::Talk, _) => &[" ╔▓╦▓╦▓╗", " ║(・_・)ﾉ", " ╚▓╩▓╩▓╝", "", ""],
        (Action::Play, 0) => &[" ♪╔▓╦▓╦▓╗", " ║(▽_▽)║", " ╚▓╩▓╩▓╝", "", ""],
        (Action::Play, _) => &[" ╔▓╦▓╦▓╗♪", " ║(▽_▽)║", " ╚▓╩▓╩▓╝", "", ""],
        (Action::Train, 0) => &[" ╔▓╦▓╦▓╗!!", " ║(益_益)║", " ╚▓╩▓╩▓╩╝", "", ""],
        (Action::Train, _) => &["!!╔▓╦▓╦▓╗", " ║(益_益)║", " ╚▓╩▓╩▓╩╝", "", ""],
        (Action::Relax, 0) => &[" ╔▓╦▓╦▓╗～", " ║(－_－)║", " ╚▓╩▓╩▓╝", "", ""],
        (Action::Relax, _) => &[" ╔▓╦▓╦▓╗", " ║(－_－)║zzZ", " ╚▓╩▓╩▓╝", "", ""],
    }
}

// --- マッスル (massuru) - Muscle ---
fn massuru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ﾉ）▓▓▓（ﾉ", "  ）(▽_▽)(!", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (MoodLevel::High, _) => &[" ﾉ）▓▓▓（ﾉ", "  ）(▽_▽)(♪", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (MoodLevel::Normal, 0) => &[" ﾉ）▓▓▓（ﾉ", "  ）(・_・)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (MoodLevel::Normal, _) => &[" ﾉ）▓▓▓（ﾉ", "  ）(・ ・)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (MoodLevel::Low, 0) => &[" ﾉ）▓▓▓（ﾉ", "  ）(￣_￣)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (MoodLevel::Low, _) => &[" ﾉ）▓▓▓（ﾉ", "  ）(￣ ￣)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
    }
}
fn massuru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ﾉ）▓▓▓（ﾉ", " ﾉ）(・_・)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (Action::Talk, _) => &[" ﾉ）▓▓▓（ﾉ", "  ）(・_・)(ﾉ", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (Action::Play, 0) => &[" ♪）▓▓▓（ﾉ", "  ）(▽_▽)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (Action::Play, _) => &[" ﾉ）▓▓▓（ﾉ♪", "  ）(▽_▽)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (Action::Train, 0) => &[" ﾉ）▓▓▓（ﾉ!!", "  ）(益_益)(", "  ﾉ▓▓▓▓▓ﾉ", "", ""],
        (Action::Train, _) => &["!!ﾉ）▓▓▓（ﾉ", "  ）(益_益)(", "  ﾉ▓▓▓▓▓ﾉ", "", ""],
        (Action::Relax, 0) => &[" ﾉ）▓▓▓（ﾉ～", "  ）(－_－)(", "  ﾉ▓▓▓▓ﾉ", "", ""],
        (Action::Relax, _) => &[" ﾉ）▓▓▓（ﾉ", "  ）(－_－)(zzZ", "  ﾉ▓▓▓▓ﾉ", "", ""],
    }
}

// --- イワオ (iwao) - Great rock ---
fn iwao_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ☆▓▓▓▓▓☆", " ▓(▽_▽)▓!", " ☆▓▓▓▓▓☆", "", ""],
        (MoodLevel::High, _) => &[" ☆▓▓▓▓▓☆", " ▓(▽_▽)▓♪", " ☆▓▓▓▓▓☆", "", ""],
        (MoodLevel::Normal, 0) => &[" ☆▓▓▓▓▓☆", " ▓(・_・)▓", " ☆▓▓▓▓▓☆", "", ""],
        (MoodLevel::Normal, _) => &[" ☆▓▓▓▓▓☆", " ▓(・ ・)▓", " ☆▓▓▓▓▓☆", "", ""],
        (MoodLevel::Low, 0) => &[" ☆▓▓▓▓▓☆", " ▓(￣_￣)▓", " ☆▓▓▓▓▓☆", "", ""],
        (MoodLevel::Low, _) => &[" ☆▓▓▓▓▓☆", " ▓(￣ ￣)▓", " ☆▓▓▓▓▓☆", "", ""],
    }
}
fn iwao_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ☆▓▓▓▓▓☆", " ﾉ(・_・)▓", " ☆▓▓▓▓▓☆", "", ""],
        (Action::Talk, _) => &[" ☆▓▓▓▓▓☆", " ▓(・_・)ﾉ", " ☆▓▓▓▓▓☆", "", ""],
        (Action::Play, 0) => &[" ♪☆▓▓▓▓▓", " ▓(▽_▽)▓", " ☆▓▓▓▓▓☆", "", ""],
        (Action::Play, _) => &[" ☆▓▓▓▓▓☆♪", " ▓(▽_▽)▓", " ☆▓▓▓▓▓☆", "", ""],
        (Action::Train, 0) => &[" ☆▓▓▓▓▓☆!!", " ▓(益_益)▓", " ☆▓▓▓▓▓▓☆", "", ""],
        (Action::Train, _) => &["!!☆▓▓▓▓▓☆", " ▓(益_益)▓", " ☆▓▓▓▓▓▓☆", "", ""],
        (Action::Relax, 0) => &[" ☆▓▓▓▓▓☆～", " ▓(－_－)▓", " ☆▓▓▓▓▓☆", "", ""],
        (Action::Relax, _) => &[" ☆▓▓▓▓▓☆", " ▓(－_－)▓zzZ", " ☆▓▓▓▓▓☆", "", ""],
    }
}

// ============================================================
// BOUKEN TYPE Stage 3 Species
// ============================================================

// --- ガニ (gani) - Crab-like armored, pincers ---
fn gani_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＝╦╦╦╦═", " ﾉ(▽ ▽)ﾉ!", "  凵～～凵", "", ""],
        (MoodLevel::High, _) => &["   ＝╦╦╦╦═", " ﾉ(▽ ▽)ﾉ♪", "  凵～～凵", "", ""],
        (MoodLevel::Normal, 0) => &["  ＝╦╦╦╦═", " ﾉ(・ ・)ﾉ", "  凵～～凵", "", ""],
        (MoodLevel::Normal, _) => &["   ＝╦╦╦╦═", " ﾉ(・ ・)ﾉ", "   凵～～凵", "", ""],
        (MoodLevel::Low, 0) => &["  ＝╦╦╦╦═", " ﾉ(￣_￣)ﾉ", "  凵～～凵", "", ""],
        (MoodLevel::Low, _) => &["  ＝╦╦╦╦═", " ﾉ(￣ ￣)ﾉ", "  凵～～凵", "", ""],
    }
}
fn gani_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＝╦╦╦╦═", " ﾉ(・o・) ", "  凵～～凵", "", ""],
        (Action::Talk, _) => &["  ＝╦╦╦╦═", "  (・o・)ﾉ", "  凵～～凵", "", ""],
        (Action::Play, 0) => &[" ♪＝╦╦╦╦═", " ﾉ(▽ ▽)ﾉ", "  凵～～凵", "", ""],
        (Action::Play, _) => &["  ＝╦╦╦╦═♪", " ﾉ(▽ ▽)ﾉ", "  凵～～凵", "", ""],
        (Action::Train, 0) => &["  ＝╦╦╦╦═!!", " ﾉ(益 益)ﾉ", " 凵～～～～凵", "", ""],
        (Action::Train, _) => &["!!＝╦╦╦╦═", " ﾉ(益 益)ﾉ", " 凵～～～～凵", "", ""],
        (Action::Relax, 0) => &["  ＝╦╦╦╦═～", "  (－ －) ", "  凵～～凵", "", ""],
        (Action::Relax, _) => &["  ＝╦╦╦╦═", "  (－ －)zzZ", "  凵～～凵", "", ""],
    }
}

// --- トビオ (tobio) - Flying/jumping, wings ---
fn tobio_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＼＼∧∧／／", "   (▽▽)!", "   ∨＿∨", "", ""],
        (MoodLevel::High, _) => &["   ＼＼∧∧／／", "   (▽▽)♪", "    ∨＿∨", "", ""],
        (MoodLevel::Normal, 0) => &["  ＼＼∧∧／／", "   (oo)", "   ∨＿∨", "", ""],
        (MoodLevel::Normal, _) => &["   ＼＼∧∧／／", "    (oo)", "    ∨＿∨", "", ""],
        (MoodLevel::Low, 0) => &["  ＼＼∧∧／／", "   (￣_￣)", "   ∨＿∨", "", ""],
        (MoodLevel::Low, _) => &["  ＼＼∧∧／／", "   (￣ ￣)", "   ∨＿∨", "", ""],
    }
}
fn tobio_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＼＼∧∧／／", "  ﾉ(oωo) ", "   ∨＿∨", "", ""],
        (Action::Talk, _) => &["  ＼＼∧∧／／", "   (oωo)ﾉ", "   ∨＿∨", "", ""],
        (Action::Play, 0) => &[" ♪＼＼∧∧／／", "   (▽▽)", "   ∨＿∨", "", ""],
        (Action::Play, _) => &["  ＼＼∧∧／／♪", "   (▽▽)", "   ∨＿∨", "", ""],
        (Action::Train, 0) => &["  ＼＼∧∧／／!!", "   (益益)", "  ∨＿＿＿∨", "", ""],
        (Action::Train, _) => &["!!＼＼∧∧／／", "   (益益)", "  ∨＿＿＿∨", "", ""],
        (Action::Relax, 0) => &["  ＼＼∧∧／／～", "   (－－)", "   ∨＿∨", "", ""],
        (Action::Relax, _) => &["  ＼＼∧∧／／", "   (－－)zzZ", "   ∨＿∨", "", ""],
    }
}

// --- マルマル (marumaru) - Round rolling ball ---
fn marumaru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ○○○○", " (（▽▽）)!", " ○○○○○", "", ""],
        (MoodLevel::High, _) => &["   ○○○○", "  (（▽▽）)♪", "  ○○○○○", "", ""],
        (MoodLevel::Normal, 0) => &["  ○○○○", " (（・・）)", " ○○○○○", "", ""],
        (MoodLevel::Normal, _) => &["   ○○○○", "  (（・・）)", "  ○○○○○", "", ""],
        (MoodLevel::Low, 0) => &["  ○○○○", " (（￣_￣）)", " ○○○○○", "", ""],
        (MoodLevel::Low, _) => &["  ○○○○", " (（￣ ￣）)", " ○○○○○", "", ""],
    }
}
fn marumaru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ○○○○", " ﾉ（・o・）", " ○○○○○", "", ""],
        (Action::Talk, _) => &["  ○○○○", "  （・o・）ﾉ", " ○○○○○", "", ""],
        (Action::Play, 0) => &[" ♪○○○○", " (（▽▽）)", " ○○○○○", "", ""],
        (Action::Play, _) => &["  ○○○○♪", " (（▽▽）)", " ○○○○○", "", ""],
        (Action::Train, 0) => &["  ○○○○!!", " (（益益）)", " ○○○○○○", "", ""],
        (Action::Train, _) => &["!!○○○○", " (（益益）)", " ○○○○○○", "", ""],
        (Action::Relax, 0) => &["  ○○○○～", " (（－－）)", " ○○○○○", "", ""],
        (Action::Relax, _) => &["  ○○○○", " (（－－）)zzZ", " ○○○○○", "", ""],
    }
}

// --- ハヤテ (hayate) - Gale wind, speed lines ---
fn hayate_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ≫≫▷▷", " ＝(▽ω▽)＞!", "  ≫≫＿＿≫", "", ""],
        (MoodLevel::High, _) => &["   ≫≫▷▷", "  ＝(▽ω▽)＞♪", "   ≫≫＿＿≫", "", ""],
        (MoodLevel::Normal, 0) => &["  ≫≫▷▷", " ＝(・ω・)＞", "  ≫≫＿＿≫", "", ""],
        (MoodLevel::Normal, _) => &["   ≫≫▷▷", "  ＝(・ω・)＞", "   ≫≫＿＿≫", "", ""],
        (MoodLevel::Low, 0) => &["  ≫≫▷▷", " ＝(￣_￣)＞", "  ≫≫＿＿≫", "", ""],
        (MoodLevel::Low, _) => &["  ≫≫▷▷", " ＝(￣ ￣)＞", "  ≫≫＿＿≫", "", ""],
    }
}
fn hayate_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ≫≫▷▷", " ﾉ(・o・)＞", "  ≫≫＿＿≫", "", ""],
        (Action::Talk, _) => &["  ≫≫▷▷", " ＝(・o・)ﾉ", "  ≫≫＿＿≫", "", ""],
        (Action::Play, 0) => &[" ♪≫≫▷▷", " ＝(▽ω▽)＞", "  ≫≫＿＿≫", "", ""],
        (Action::Play, _) => &["  ≫≫▷▷♪", " ＝(▽ω▽)＞", "  ≫≫＿＿≫", "", ""],
        (Action::Train, 0) => &["  ≫≫▷▷!!", " ＝(益益)＞＞", " ≫≫≫＿＿≫≫", "", ""],
        (Action::Train, _) => &["!!≫≫▷▷", " ＝＝(益益)＞＞", " ≫≫≫＿＿≫≫", "", ""],
        (Action::Relax, 0) => &["  ≫≫▷▷～", " ＝(－ω－) ", "  ≫≫＿＿≫", "", ""],
        (Action::Relax, _) => &["  ≫≫▷▷", " ＝(－ω－)zzZ", "  ≫≫＿＿≫", "", ""],
    }
}

// --- グルグルン (gurugurun) - Spinning vortex ---
fn gurugurun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＠＠＠", " ＠(▽▽)＠!", "  ＠＠＠", "", ""],
        (MoodLevel::High, _) => &["   ＠＠＠", "  ＠(▽▽)＠♪", "   ＠＠＠", "", ""],
        (MoodLevel::Normal, 0) => &["  ＠＠＠", " ＠(・・)＠", "  ＠＠＠", "", ""],
        (MoodLevel::Normal, _) => &["   ＠＠＠", "  ＠(・・)＠", "   ＠＠＠", "", ""],
        (MoodLevel::Low, 0) => &["  ＠＠＠", " ＠(￣_￣)＠", "  ＠＠＠", "", ""],
        (MoodLevel::Low, _) => &["  ＠＠＠", " ＠(￣ ￣)＠", "  ＠＠＠", "", ""],
    }
}
fn gurugurun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＠＠＠", " ﾉ(・o・)＠", "  ＠＠＠", "", ""],
        (Action::Talk, _) => &["  ＠＠＠", " ＠(・o・)ﾉ", "  ＠＠＠", "", ""],
        (Action::Play, 0) => &[" ♪＠＠＠", " ＠(▽▽)＠", "  ＠＠＠", "", ""],
        (Action::Play, _) => &["  ＠＠＠♪", " ＠(▽▽)＠", "  ＠＠＠", "", ""],
        (Action::Train, 0) => &["  ＠＠＠!!", " ＠(益益)＠", " ＠＠＠＠", "", ""],
        (Action::Train, _) => &["!!＠＠＠", " ＠(益益)＠", " ＠＠＠＠", "", ""],
        (Action::Relax, 0) => &["  ＠＠＠～", " ＠(－－)＠", "  ＠＠＠", "", ""],
        (Action::Relax, _) => &["  ＠＠＠", " ＠(－－)＠zzZ", "  ＠＠＠", "", ""],
    }
}

// --- カゼノコ (kazenoko) - Wind child, breezy ---
fn kazenoko_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～彡彡～", "  (▽‿▽)~!", "  ﾉ＿＿ﾉ~", "", ""],
        (MoodLevel::High, _) => &["   ～彡彡～", "   (▽‿▽)~♪", "   ﾉ＿＿ﾉ~", "", ""],
        (MoodLevel::Normal, 0) => &["  ～彡彡～", "  (・‿・)~", "  ﾉ＿＿ﾉ~", "", ""],
        (MoodLevel::Normal, _) => &["   ～彡彡～", "   (・‿・)~", "   ﾉ＿＿ﾉ~", "", ""],
        (MoodLevel::Low, 0) => &["  ～彡彡～", "  (￣_￣)~", "  ﾉ＿＿ﾉ ", "", ""],
        (MoodLevel::Low, _) => &["  ～彡彡～", "  (￣ ￣)~", "  ﾉ＿＿ﾉ ", "", ""],
    }
}
fn kazenoko_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～彡彡～", " ﾉ(・o・)~", "  ﾉ＿＿ﾉ~", "", ""],
        (Action::Talk, _) => &["  ～彡彡～", "  (・o・)ﾉ~", "  ﾉ＿＿ﾉ~", "", ""],
        (Action::Play, 0) => &[" ♪～彡彡～", "  (▽‿▽)~", "  ﾉ＿＿ﾉ~", "", ""],
        (Action::Play, _) => &["  ～彡彡～♪", "  (▽‿▽)~", "  ﾉ＿＿ﾉ~", "", ""],
        (Action::Train, 0) => &["  ～彡彡～!!", "  (益益)~!", "  ﾉ＿＿＿ﾉ~", "", ""],
        (Action::Train, _) => &["!!～彡彡～", "  (益益)~!", "  ﾉ＿＿＿ﾉ~", "", ""],
        (Action::Relax, 0) => &["  ～彡彡～～", "  (－‿－)~", "  ﾉ＿＿ﾉ ", "", ""],
        (Action::Relax, _) => &["  ～彡彡～", "  (－‿－)zzZ", "  ﾉ＿＿ﾉ ", "", ""],
    }
}

// --- ドカーン (dokaan) - Explosion, blast marks ---
fn dokaan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＊※＊※", " ※(▽Д▽)※!", "  ＊＿※＿＊", "", ""],
        (MoodLevel::High, _) => &["   ＊※＊※", "  ※(▽Д▽)※♪", "   ＊＿※＿＊", "", ""],
        (MoodLevel::Normal, 0) => &["  ＊※＊※", " ※(・Д・)※", "  ＊＿※＿＊", "", ""],
        (MoodLevel::Normal, _) => &["   ＊※＊※", "  ※(・Д・)※", "   ＊＿※＿＊", "", ""],
        (MoodLevel::Low, 0) => &["  ＊※＊※", " ※(￣_￣)※", "  ＊＿※＿＊", "", ""],
        (MoodLevel::Low, _) => &["  ＊※＊※", " ※(￣ ￣)※", "  ＊＿※＿＊", "", ""],
    }
}
fn dokaan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＊※＊※", " ﾉ(・Д・)※", "  ＊＿※＿＊", "", ""],
        (Action::Talk, _) => &["  ＊※＊※", " ※(・Д・)ﾉ", "  ＊＿※＿＊", "", ""],
        (Action::Play, 0) => &[" ♪＊※＊※", " ※(▽Д▽)※", "  ＊＿※＿＊", "", ""],
        (Action::Play, _) => &["  ＊※＊※♪", " ※(▽Д▽)※", "  ＊＿※＿＊", "", ""],
        (Action::Train, 0) => &["  ＊※＊※!!", " ※(益Д益)※", " ＊＿※＿※＿＊", "", ""],
        (Action::Train, _) => &["!!＊※＊※", " ※(益Д益)※", " ＊＿※＿※＿＊", "", ""],
        (Action::Relax, 0) => &["  ＊※＊※～", " ※(－Д－)※", "  ＊＿※＿＊", "", ""],
        (Action::Relax, _) => &["  ＊※＊※", " ※(－Д－)zzZ", "  ＊＿※＿＊", "", ""],
    }
}

// --- スイスイ (suisui) - Swimming/gliding fish-like ---
fn suisui_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＞＜＞＜", " ＜(▽.▽)＞>!", "  ＜＿＞＿＞", "", ""],
        (MoodLevel::High, _) => &["   ＞＜＞＜", "  ＜(▽.▽)＞>♪", "   ＜＿＞＿＞", "", ""],
        (MoodLevel::Normal, 0) => &["  ＞＜＞＜", " ＜(・.・)＞>", "  ＜＿＞＿＞", "", ""],
        (MoodLevel::Normal, _) => &["   ＞＜＞＜", "  ＜(・.・)＞>", "   ＜＿＞＿＞", "", ""],
        (MoodLevel::Low, 0) => &["  ＞＜＞＜", " ＜(￣_￣)＞>", "  ＜＿＞＿＞", "", ""],
        (MoodLevel::Low, _) => &["  ＞＜＞＜", " ＜(￣ ￣)＞>", "  ＜＿＞＿＞", "", ""],
    }
}
fn suisui_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＞＜＞＜", " ﾉ(・o・)＞>", "  ＜＿＞＿＞", "", ""],
        (Action::Talk, _) => &["  ＞＜＞＜", " ＜(・o・)ﾉ>", "  ＜＿＞＿＞", "", ""],
        (Action::Play, 0) => &[" ♪＞＜＞＜", " ＜(▽.▽)＞>", "  ＜＿＞＿＞", "", ""],
        (Action::Play, _) => &["  ＞＜＞＜♪", " ＜(▽.▽)＞>", "  ＜＿＞＿＞", "", ""],
        (Action::Train, 0) => &["  ＞＜＞＜!!", " ＜(益.益)＞>", " ＜＿＞＿＞＿＞", "", ""],
        (Action::Train, _) => &["!!＞＜＞＜", " ＜(益.益)＞>", " ＜＿＞＿＞＿＞", "", ""],
        (Action::Relax, 0) => &["  ＞＜＞＜～", " ＜(－.－)＞>", "  ＜＿＞＿＞", "", ""],
        (Action::Relax, _) => &["  ＞＜＞＜", " ＜(－.－)zzZ", "  ＜＿＞＿＞", "", ""],
    }
}

// --- サスライ (sasurai) - Wanderer, hat/cloak ---
fn sasurai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ▓▓▓▓▓", " ▕(▽_▽)▏!", "  ▕▓▓▓▏", "", ""],
        (MoodLevel::High, _) => &["   ▓▓▓▓▓", "  ▕(▽_▽)▏♪", "   ▕▓▓▓▏", "", ""],
        (MoodLevel::Normal, 0) => &["  ▓▓▓▓▓", " ▕(・_・)▏", "  ▕▓▓▓▏", "", ""],
        (MoodLevel::Normal, _) => &["   ▓▓▓▓▓", "  ▕(・_・)▏", "   ▕▓▓▓▏", "", ""],
        (MoodLevel::Low, 0) => &["  ▓▓▓▓▓", " ▕(￣_￣)▏", "  ▕▓▓▓▏", "", ""],
        (MoodLevel::Low, _) => &["  ▓▓▓▓▓", " ▕(￣ ￣)▏", "  ▕▓▓▓▏", "", ""],
    }
}
fn sasurai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ▓▓▓▓▓", " ﾉ(・_・)▏", "  ▕▓▓▓▏", "", ""],
        (Action::Talk, _) => &["  ▓▓▓▓▓", " ▕(・_・)ﾉ", "  ▕▓▓▓▏", "", ""],
        (Action::Play, 0) => &[" ♪▓▓▓▓▓", " ▕(▽_▽)▏", "  ▕▓▓▓▏", "", ""],
        (Action::Play, _) => &["  ▓▓▓▓▓♪", " ▕(▽_▽)▏", "  ▕▓▓▓▏", "", ""],
        (Action::Train, 0) => &["  ▓▓▓▓▓!!", " ▕(益_益)▏", " ▕▓▓▓▓▓▏", "", ""],
        (Action::Train, _) => &["!!▓▓▓▓▓", " ▕(益_益)▏", " ▕▓▓▓▓▓▏", "", ""],
        (Action::Relax, 0) => &["  ▓▓▓▓▓～", " ▕(－_－)▏", "  ▕▓▓▓▏", "", ""],
        (Action::Relax, _) => &["  ▓▓▓▓▓", " ▕(－_－)zzZ", "  ▕▓▓▓▏", "", ""],
    }
}

// --- ピカッ (pikat) - Flash/sparkle, star burst ---
fn pikat_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ☆★☆★", " ★(▽☆▽)★!", "  ☆＿★＿☆", "", ""],
        (MoodLevel::High, _) => &["   ☆★☆★", "  ★(▽☆▽)★♪", "   ☆＿★＿☆", "", ""],
        (MoodLevel::Normal, 0) => &["  ☆★☆★", " ★(・☆・)★", "  ☆＿★＿☆", "", ""],
        (MoodLevel::Normal, _) => &["   ☆★☆★", "  ★(・☆・)★", "   ☆＿★＿☆", "", ""],
        (MoodLevel::Low, 0) => &["  ☆★☆★", " ★(￣_￣)★", "  ☆＿★＿☆", "", ""],
        (MoodLevel::Low, _) => &["  ☆★☆★", " ★(￣ ￣)★", "  ☆＿★＿☆", "", ""],
    }
}
fn pikat_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ☆★☆★", " ﾉ(・☆・)★", "  ☆＿★＿☆", "", ""],
        (Action::Talk, _) => &["  ☆★☆★", " ★(・☆・)ﾉ", "  ☆＿★＿☆", "", ""],
        (Action::Play, 0) => &[" ♪☆★☆★", " ★(▽☆▽)★", "  ☆＿★＿☆", "", ""],
        (Action::Play, _) => &["  ☆★☆★♪", " ★(▽☆▽)★", "  ☆＿★＿☆", "", ""],
        (Action::Train, 0) => &["  ☆★☆★!!", " ★(益☆益)★", " ☆＿★＿★＿☆", "", ""],
        (Action::Train, _) => &["!!☆★☆★", " ★(益☆益)★", " ☆＿★＿★＿☆", "", ""],
        (Action::Relax, 0) => &["  ☆★☆★～", " ★(－☆－)★", "  ☆＿★＿☆", "", ""],
        (Action::Relax, _) => &["  ☆★☆★", " ★(－☆－)zzZ", "  ☆＿★＿☆", "", ""],
    }
}

// --- バサバサ (basabasa) - Flapping big wings ---
fn basabasa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ＼＼|｜／／", "  彡(▽▽)彡!", "   ∪＿∪", "", ""],
        (MoodLevel::High, _) => &["  ／／|｜＼＼", "  彡(▽▽)彡♪", "    ∪＿∪", "", ""],
        (MoodLevel::Normal, 0) => &[" ＼＼|｜／／", "  彡(・・)彡", "   ∪＿∪", "", ""],
        (MoodLevel::Normal, _) => &["  ／／|｜＼＼", "   彡(・・)彡", "    ∪＿∪", "", ""],
        (MoodLevel::Low, 0) => &[" ＼＼|｜／／", "  彡(￣_￣)彡", "   ∪＿∪", "", ""],
        (MoodLevel::Low, _) => &[" ＼＼|｜／／", "  彡(￣ ￣)彡", "   ∪＿∪", "", ""],
    }
}
fn basabasa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ＼＼|｜／／", " ﾉ彡(・o・)彡", "   ∪＿∪", "", ""],
        (Action::Talk, _) => &[" ／／|｜＼＼", "  彡(・o・)ﾉ彡", "   ∪＿∪", "", ""],
        (Action::Play, 0) => &["♪＼＼|｜／／", "  彡(▽▽)彡", "   ∪＿∪", "", ""],
        (Action::Play, _) => &[" ／／|｜＼＼♪", "  彡(▽▽)彡", "   ∪＿∪", "", ""],
        (Action::Train, 0) => &[" ＼＼|｜／／!!", "  彡(益益)彡", "  ∪＿＿＿∪", "", ""],
        (Action::Train, _) => &["!!／／|｜＼＼", "  彡(益益)彡", "  ∪＿＿＿∪", "", ""],
        (Action::Relax, 0) => &[" ＼＼|｜／／～", "  彡(－－)彡", "   ∪＿∪", "", ""],
        (Action::Relax, _) => &[" ＼＼|｜／／", "  彡(－－)zzZ", "   ∪＿∪", "", ""],
    }
}

// --- ウロチョロ (urochoro) - Restless, fidgety ---
fn urochoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ,,^^,,", " ﾉ(▽o▽)ﾉ!", "  d＿b", "", ""],
        (MoodLevel::High, _) => &["    ,,^^,,", "   ﾉ(▽o▽)ﾉ♪", "    d＿b", "", ""],
        (MoodLevel::Normal, 0) => &["  ,,^^,,", " ﾉ(・o・)ﾉ", "  d＿b", "", ""],
        (MoodLevel::Normal, _) => &["    ,,^^,,", "   ﾉ(・o・)ﾉ", "    d＿b", "", ""],
        (MoodLevel::Low, 0) => &["  ,,^^,,", "  (￣_￣)", "  d＿b", "", ""],
        (MoodLevel::Low, _) => &["  ,,^^,,", "  (￣ ￣)", "  d＿b", "", ""],
    }
}
fn urochoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ,,^^,,", " ﾉ(・□・) ", "  d＿b", "", ""],
        (Action::Talk, _) => &["    ,,^^,,", "   (・□・)ﾉ", "    d＿b", "", ""],
        (Action::Play, 0) => &[" ♪,,^^,,", " ﾉ(▽o▽)ﾉ", "  d＿b", "", ""],
        (Action::Play, _) => &["    ,,^^,,♪", "   ﾉ(▽o▽)ﾉ", "    d＿b", "", ""],
        (Action::Train, 0) => &["  ,,^^,,!!", " ﾉ(益o益)ﾉ", " d＿＿＿b", "", ""],
        (Action::Train, _) => &["!!,,^^,,", " ﾉ(益o益)ﾉ", " d＿＿＿b", "", ""],
        (Action::Relax, 0) => &["  ,,^^,,～", "  (－o－) ", "  d＿b", "", ""],
        (Action::Relax, _) => &["  ,,^^,,", "  (－o－)zzZ", "  d＿b", "", ""],
    }
}

// --- ゴーゴー (googoo) - Energetic, flames ---
fn googoo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  炎炎炎", " 火(▽Д▽)火!", "  爪＿＿爪", "", ""],
        (MoodLevel::High, _) => &["   炎炎炎", "  火(▽Д▽)火♪", "   爪＿＿爪", "", ""],
        (MoodLevel::Normal, 0) => &["  炎炎炎", " 火(・Д・)火", "  爪＿＿爪", "", ""],
        (MoodLevel::Normal, _) => &["   炎炎炎", "  火(・Д・)火", "   爪＿＿爪", "", ""],
        (MoodLevel::Low, 0) => &["  炎炎炎", " 火(￣_￣)火", "  爪＿＿爪", "", ""],
        (MoodLevel::Low, _) => &["  炎炎炎", " 火(￣ ￣)火", "  爪＿＿爪", "", ""],
    }
}
fn googoo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  炎炎炎", " ﾉ(・Д・)火", "  爪＿＿爪", "", ""],
        (Action::Talk, _) => &["  炎炎炎", " 火(・Д・)ﾉ", "  爪＿＿爪", "", ""],
        (Action::Play, 0) => &[" ♪炎炎炎", " 火(▽Д▽)火", "  爪＿＿爪", "", ""],
        (Action::Play, _) => &["  炎炎炎♪", " 火(▽Д▽)火", "  爪＿＿爪", "", ""],
        (Action::Train, 0) => &["  炎炎炎!!", " 火(益Д益)火", " 爪＿＿＿＿爪", "", ""],
        (Action::Train, _) => &["!!炎炎炎", " 火(益Д益)火", " 爪＿＿＿＿爪", "", ""],
        (Action::Relax, 0) => &["  炎炎炎～", " 火(－Д－)火", "  爪＿＿爪", "", ""],
        (Action::Relax, _) => &["  炎炎炎", " 火(－Д－)zzZ", "  爪＿＿爪", "", ""],
    }
}

// --- クモノス (kumonos) - Spider web creature ---
fn kumonos_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  /|\\|/|\\", " 八(▽;;▽)八!", "  /|＿＿|\\", "", ""],
        (MoodLevel::High, _) => &["   /|\\|/|\\", "  八(▽;;▽)八♪", "   /|＿＿|\\", "", ""],
        (MoodLevel::Normal, 0) => &["  /|\\|/|\\", " 八(・;;・)八", "  /|＿＿|\\", "", ""],
        (MoodLevel::Normal, _) => &["   /|\\|/|\\", "  八(・;;・)八", "   /|＿＿|\\", "", ""],
        (MoodLevel::Low, 0) => &["  /|\\|/|\\", " 八(￣_￣)八", "  /|＿＿|\\", "", ""],
        (MoodLevel::Low, _) => &["  /|\\|/|\\", " 八(￣ ￣)八", "  /|＿＿|\\", "", ""],
    }
}
fn kumonos_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  /|\\|/|\\", " ﾉ(・;;・)八", "  /|＿＿|\\", "", ""],
        (Action::Talk, _) => &["  /|\\|/|\\", " 八(・;;・)ﾉ", "  /|＿＿|\\", "", ""],
        (Action::Play, 0) => &[" ♪/|\\|/|\\", " 八(▽;;▽)八", "  /|＿＿|\\", "", ""],
        (Action::Play, _) => &["  /|\\|/|\\♪", " 八(▽;;▽)八", "  /|＿＿|\\", "", ""],
        (Action::Train, 0) => &["  /|\\|/|\\!!", " 八(益;;益)八", " /|＿＿＿＿|\\", "", ""],
        (Action::Train, _) => &["!!/|\\|/|\\", " 八(益;;益)八", " /|＿＿＿＿|\\", "", ""],
        (Action::Relax, 0) => &["  /|\\|/|\\～", " 八(－;;－)八", "  /|＿＿|\\", "", ""],
        (Action::Relax, _) => &["  /|\\|/|\\", " 八(－;;－)zzZ", "  /|＿＿|\\", "", ""],
    }
}

// --- ホシゾラ (hoshizora) - Starry sky, cosmic ---
fn hoshizora_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ☆.☆.☆", " ﾟ+(▽*▽)+ﾟ!", "  ☆＿.＿☆", "", ""],
        (MoodLevel::High, _) => &["   ☆.☆.☆", "  ﾟ+(▽*▽)+ﾟ♪", "   ☆＿.＿☆", "", ""],
        (MoodLevel::Normal, 0) => &["  ☆.☆.☆", " ﾟ+(・*・)+ﾟ", "  ☆＿.＿☆", "", ""],
        (MoodLevel::Normal, _) => &["   ☆.☆.☆", "  ﾟ+(・*・)+ﾟ", "   ☆＿.＿☆", "", ""],
        (MoodLevel::Low, 0) => &["  ☆.☆.☆", " ﾟ+(￣_￣)+ﾟ", "  ☆＿.＿☆", "", ""],
        (MoodLevel::Low, _) => &["  ☆.☆.☆", " ﾟ+(￣ ￣)+ﾟ", "  ☆＿.＿☆", "", ""],
    }
}
fn hoshizora_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ☆.☆.☆", " ﾉ+(・*・)+ﾟ", "  ☆＿.＿☆", "", ""],
        (Action::Talk, _) => &["  ☆.☆.☆", " ﾟ+(・*・)ﾉﾟ", "  ☆＿.＿☆", "", ""],
        (Action::Play, 0) => &[" ♪☆.☆.☆", " ﾟ+(▽*▽)+ﾟ", "  ☆＿.＿☆", "", ""],
        (Action::Play, _) => &["  ☆.☆.☆♪", " ﾟ+(▽*▽)+ﾟ", "  ☆＿.＿☆", "", ""],
        (Action::Train, 0) => &["  ☆.☆.☆!!", " ﾟ+(益*益)+ﾟ", " ☆＿.＿.＿☆", "", ""],
        (Action::Train, _) => &["!!☆.☆.☆", " ﾟ+(益*益)+ﾟ", " ☆＿.＿.＿☆", "", ""],
        (Action::Relax, 0) => &["  ☆.☆.☆～", " ﾟ+(－*－)+ﾟ", "  ☆＿.＿☆", "", ""],
        (Action::Relax, _) => &["  ☆.☆.☆", " ﾟ+(－*－)zzZ", "  ☆＿.＿☆", "", ""],
    }
}

// --- ブッチギリ (bucchigiri) - Record-breaking, extreme speed ---
fn bucchigiri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ≡≡▶▶▶", " ≡≡(▽皿▽)≡>!", "  ≡≡＿＿≡≡", "", ""],
        (MoodLevel::High, _) => &["   ≡≡▶▶▶", "  ≡≡(▽皿▽)≡>♪", "   ≡≡＿＿≡≡", "", ""],
        (MoodLevel::Normal, 0) => &["  ≡≡▶▶▶", " ≡≡(・皿・)≡>", "  ≡≡＿＿≡≡", "", ""],
        (MoodLevel::Normal, _) => &["   ≡≡▶▶▶", "  ≡≡(・皿・)≡>", "   ≡≡＿＿≡≡", "", ""],
        (MoodLevel::Low, 0) => &["  ≡≡▶▶▶", " ≡≡(￣_￣)≡>", "  ≡≡＿＿≡≡", "", ""],
        (MoodLevel::Low, _) => &["  ≡≡▶▶▶", " ≡≡(￣ ￣)≡>", "  ≡≡＿＿≡≡", "", ""],
    }
}
fn bucchigiri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ≡≡▶▶▶", " ﾉ≡(・皿・)≡>", "  ≡≡＿＿≡≡", "", ""],
        (Action::Talk, _) => &["  ≡≡▶▶▶", " ≡≡(・皿・)ﾉ>", "  ≡≡＿＿≡≡", "", ""],
        (Action::Play, 0) => &[" ♪≡≡▶▶▶", " ≡≡(▽皿▽)≡>", "  ≡≡＿＿≡≡", "", ""],
        (Action::Play, _) => &["  ≡≡▶▶▶♪", " ≡≡(▽皿▽)≡>", "  ≡≡＿＿≡≡", "", ""],
        (Action::Train, 0) => &["  ≡≡▶▶▶!!", " ≡≡(益皿益)≡>", " ≡≡＿＿＿＿≡≡", "", ""],
        (Action::Train, _) => &["!!≡≡▶▶▶", " ≡≡(益皿益)≡>", " ≡≡＿＿＿＿≡≡", "", ""],
        (Action::Relax, 0) => &["  ≡≡▶▶▶～", " ≡≡(－皿－)≡>", "  ≡≡＿＿≡≡", "", ""],
        (Action::Relax, _) => &["  ≡≡▶▶▶", " ≡≡(－皿－)zzZ", "  ≡≡＿＿≡≡", "", ""],
    }
}

// --- ワタリ (watari) - Migratory, compass marks ---
fn watari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＋N＋", " W(▽_▽)E!", "  ＋S＋", "", ""],
        (MoodLevel::High, _) => &["   ＋N＋", "  W(▽_▽)E♪", "   ＋S＋", "", ""],
        (MoodLevel::Normal, 0) => &["  ＋N＋", " W(・_・)E", "  ＋S＋", "", ""],
        (MoodLevel::Normal, _) => &["   ＋N＋", "  W(・_・)E", "   ＋S＋", "", ""],
        (MoodLevel::Low, 0) => &["  ＋N＋", " W(￣_￣)E", "  ＋S＋", "", ""],
        (MoodLevel::Low, _) => &["  ＋N＋", " W(￣ ￣)E", "  ＋S＋", "", ""],
    }
}
fn watari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＋N＋", " ﾉ(・_・)E", "  ＋S＋", "", ""],
        (Action::Talk, _) => &["  ＋N＋", " W(・_・)ﾉ", "  ＋S＋", "", ""],
        (Action::Play, 0) => &[" ♪＋N＋", " W(▽_▽)E", "  ＋S＋", "", ""],
        (Action::Play, _) => &["  ＋N＋♪", " W(▽_▽)E", "  ＋S＋", "", ""],
        (Action::Train, 0) => &["  ＋N＋!!", " W(益_益)E", " ＋＋S＋＋", "", ""],
        (Action::Train, _) => &["!!＋N＋", " W(益_益)E", " ＋＋S＋＋", "", ""],
        (Action::Relax, 0) => &["  ＋N＋～", " W(－_－)E", "  ＋S＋", "", ""],
        (Action::Relax, _) => &["  ＋N＋", " W(－_－)zzZ", "  ＋S＋", "", ""],
    }
}

// --- ヒュー (hyuu) - Whooshing wind, streaks ---
fn hyuu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～～⇒⇒", " ～(▽з▽)⇒!", "  ～～＿＿⇒", "", ""],
        (MoodLevel::High, _) => &["   ～～⇒⇒", "  ～(▽з▽)⇒♪", "   ～～＿＿⇒", "", ""],
        (MoodLevel::Normal, 0) => &["  ～～⇒⇒", " ～(・з・)⇒", "  ～～＿＿⇒", "", ""],
        (MoodLevel::Normal, _) => &["   ～～⇒⇒", "  ～(・з・)⇒", "   ～～＿＿⇒", "", ""],
        (MoodLevel::Low, 0) => &["  ～～⇒⇒", " ～(￣_￣)⇒", "  ～～＿＿⇒", "", ""],
        (MoodLevel::Low, _) => &["  ～～⇒⇒", " ～(￣ ￣)⇒", "  ～～＿＿⇒", "", ""],
    }
}
fn hyuu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～～⇒⇒", " ﾉ(・з・)⇒", "  ～～＿＿⇒", "", ""],
        (Action::Talk, _) => &["  ～～⇒⇒", " ～(・з・)ﾉ", "  ～～＿＿⇒", "", ""],
        (Action::Play, 0) => &[" ♪～～⇒⇒", " ～(▽з▽)⇒", "  ～～＿＿⇒", "", ""],
        (Action::Play, _) => &["  ～～⇒⇒♪", " ～(▽з▽)⇒", "  ～～＿＿⇒", "", ""],
        (Action::Train, 0) => &["  ～～⇒⇒!!", " ～(益з益)⇒⇒", " ～～～＿＿⇒⇒", "", ""],
        (Action::Train, _) => &["!!～～⇒⇒", " ～～(益з益)⇒", " ～～～＿＿⇒⇒", "", ""],
        (Action::Relax, 0) => &["  ～～⇒⇒～", " ～(－з－)  ", "  ～～＿＿⇒", "", ""],
        (Action::Relax, _) => &["  ～～⇒⇒", " ～(－з－)zzZ", "  ～～＿＿⇒", "", ""],
    }
}

// --- タンケン (tanken) - Explorer, map/compass ---
fn tanken_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ◇◆◇◆", " ◆(▽◇▽)◆!", "  ◆＿◇＿◆", "", ""],
        (MoodLevel::High, _) => &["   ◇◆◇◆", "  ◆(▽◇▽)◆♪", "   ◆＿◇＿◆", "", ""],
        (MoodLevel::Normal, 0) => &["  ◇◆◇◆", " ◆(・◇・)◆", "  ◆＿◇＿◆", "", ""],
        (MoodLevel::Normal, _) => &["   ◇◆◇◆", "  ◆(・◇・)◆", "   ◆＿◇＿◆", "", ""],
        (MoodLevel::Low, 0) => &["  ◇◆◇◆", " ◆(￣_￣)◆", "  ◆＿◇＿◆", "", ""],
        (MoodLevel::Low, _) => &["  ◇◆◇◆", " ◆(￣ ￣)◆", "  ◆＿◇＿◆", "", ""],
    }
}
fn tanken_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ◇◆◇◆", " ﾉ(・◇・)◆", "  ◆＿◇＿◆", "", ""],
        (Action::Talk, _) => &["  ◇◆◇◆", " ◆(・◇・)ﾉ", "  ◆＿◇＿◆", "", ""],
        (Action::Play, 0) => &[" ♪◇◆◇◆", " ◆(▽◇▽)◆", "  ◆＿◇＿◆", "", ""],
        (Action::Play, _) => &["  ◇◆◇◆♪", " ◆(▽◇▽)◆", "  ◆＿◇＿◆", "", ""],
        (Action::Train, 0) => &["  ◇◆◇◆!!", " ◆(益◇益)◆", " ◆＿◇＿◇＿◆", "", ""],
        (Action::Train, _) => &["!!◇◆◇◆", " ◆(益◇益)◆", " ◆＿◇＿◇＿◆", "", ""],
        (Action::Relax, 0) => &["  ◇◆◇◆～", " ◆(－◇－)◆", "  ◆＿◇＿◆", "", ""],
        (Action::Relax, _) => &["  ◇◆◇◆", " ◆(－◇－)zzZ", "  ◆＿◇＿◆", "", ""],
    }
}

// --- ジェット (jetto) - Jet propulsion, exhaust ---
fn jetto_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ▷▷▷▷▷", " ▷▷(▽△▽)▷!", "  ▷＿△＿▷", "", ""],
        (MoodLevel::High, _) => &["   ▷▷▷▷▷", "  ▷▷(▽△▽)▷♪", "   ▷＿△＿▷", "", ""],
        (MoodLevel::Normal, 0) => &["  ▷▷▷▷▷", " ▷▷(・△・)▷", "  ▷＿△＿▷", "", ""],
        (MoodLevel::Normal, _) => &["   ▷▷▷▷▷", "  ▷▷(・△・)▷", "   ▷＿△＿▷", "", ""],
        (MoodLevel::Low, 0) => &["  ▷▷▷▷▷", " ▷▷(￣_￣)▷", "  ▷＿△＿▷", "", ""],
        (MoodLevel::Low, _) => &["  ▷▷▷▷▷", " ▷▷(￣ ￣)▷", "  ▷＿△＿▷", "", ""],
    }
}
fn jetto_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ▷▷▷▷▷", " ﾉ▷(・△・)▷", "  ▷＿△＿▷", "", ""],
        (Action::Talk, _) => &["  ▷▷▷▷▷", " ▷▷(・△・)ﾉ", "  ▷＿△＿▷", "", ""],
        (Action::Play, 0) => &[" ♪▷▷▷▷▷", " ▷▷(▽△▽)▷", "  ▷＿△＿▷", "", ""],
        (Action::Play, _) => &["  ▷▷▷▷▷♪", " ▷▷(▽△▽)▷", "  ▷＿△＿▷", "", ""],
        (Action::Train, 0) => &["  ▷▷▷▷▷!!", " ▷▷(益△益)▷▷", " ▷＿△＿△＿▷", "", ""],
        (Action::Train, _) => &["!!▷▷▷▷▷", " ▷▷(益△益)▷▷", " ▷＿△＿△＿▷", "", ""],
        (Action::Relax, 0) => &["  ▷▷▷▷▷～", " ▷▷(－△－)  ", "  ▷＿△＿▷", "", ""],
        (Action::Relax, _) => &["  ▷▷▷▷▷", " ▷▷(－△－)zzZ", "  ▷＿△＿▷", "", ""],
    }
}

// ============================================================
// NORMAL TYPE Stage 3 Species
// ============================================================

// --- ノーマル (noomaru) - Standard, perfectly average ---
fn noomaru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＿＿＿", " |(▽_▽)|!", "  |＿＿|", "", ""],
        (MoodLevel::High, _) => &["   ＿＿＿", "  |(▽_▽)|♪", "   |＿＿|", "", ""],
        (MoodLevel::Normal, 0) => &["  ＿＿＿", " |(・_・)|", "  |＿＿|", "", ""],
        (MoodLevel::Normal, _) => &["   ＿＿＿", "  |(・_・)|", "   |＿＿|", "", ""],
        (MoodLevel::Low, 0) => &["  ＿＿＿", " |(￣_￣)|", "  |＿＿|", "", ""],
        (MoodLevel::Low, _) => &["  ＿＿＿", " |(￣ ￣)|", "  |＿＿|", "", ""],
    }
}
fn noomaru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＿＿＿", " ﾉ(・_・)|", "  |＿＿|", "", ""],
        (Action::Talk, _) => &["  ＿＿＿", " |(・_・)ﾉ", "  |＿＿|", "", ""],
        (Action::Play, 0) => &[" ♪＿＿＿", " |(▽_▽)|", "  |＿＿|", "", ""],
        (Action::Play, _) => &["  ＿＿＿♪", " |(▽_▽)|", "  |＿＿|", "", ""],
        (Action::Train, 0) => &["  ＿＿＿!!", " |(益_益)|", " |＿＿＿＿|", "", ""],
        (Action::Train, _) => &["!!＿＿＿", " |(益_益)|", " |＿＿＿＿|", "", ""],
        (Action::Relax, 0) => &["  ＿＿＿～", " |(－_－)|", "  |＿＿|", "", ""],
        (Action::Relax, _) => &["  ＿＿＿", " |(－_－)zzZ", "  |＿＿|", "", ""],
    }
}

// --- ヘイボン (heibon) - Average/mediocre ---
fn heibon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ーーー", " ｜(▽ｰ▽)｜!", "  ｜＿｜", "", ""],
        (MoodLevel::High, _) => &["   ーーー", "  ｜(▽ｰ▽)｜♪", "   ｜＿｜", "", ""],
        (MoodLevel::Normal, 0) => &["  ーーー", " ｜(・ｰ・)｜", "  ｜＿｜", "", ""],
        (MoodLevel::Normal, _) => &["   ーーー", "  ｜(・ｰ・)｜", "   ｜＿｜", "", ""],
        (MoodLevel::Low, 0) => &["  ーーー", " ｜(￣_￣)｜", "  ｜＿｜", "", ""],
        (MoodLevel::Low, _) => &["  ーーー", " ｜(￣ ￣)｜", "  ｜＿｜", "", ""],
    }
}
fn heibon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ーーー", " ﾉ(・ｰ・)｜", "  ｜＿｜", "", ""],
        (Action::Talk, _) => &["  ーーー", " ｜(・ｰ・)ﾉ", "  ｜＿｜", "", ""],
        (Action::Play, 0) => &[" ♪ーーー", " ｜(▽ｰ▽)｜", "  ｜＿｜", "", ""],
        (Action::Play, _) => &["  ーーー♪", " ｜(▽ｰ▽)｜", "  ｜＿｜", "", ""],
        (Action::Train, 0) => &["  ーーー!!", " ｜(益ｰ益)｜", " ｜＿＿＿｜", "", ""],
        (Action::Train, _) => &["!!ーーー", " ｜(益ｰ益)｜", " ｜＿＿＿｜", "", ""],
        (Action::Relax, 0) => &["  ーーー～", " ｜(－ｰ－)｜", "  ｜＿｜", "", ""],
        (Action::Relax, _) => &["  ーーー", " ｜(－ｰ－)zzZ", "  ｜＿｜", "", ""],
    }
}

// --- タソガレ (tasogare) - Twilight/melancholy ---
fn tasogare_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  .:*:.:*:", " ﾟ(▽ ▽)ﾟ!", "  .:＿＿:.", "", ""],
        (MoodLevel::High, _) => &["   .:*:.:*:", "  ﾟ(▽ ▽)ﾟ♪", "   .:＿＿:.", "", ""],
        (MoodLevel::Normal, 0) => &["  .:*:.:*:", " ﾟ(・ ・)ﾟ", "  .:＿＿:.", "", ""],
        (MoodLevel::Normal, _) => &["   .:*:.:*:", "  ﾟ(・ ・)ﾟ", "   .:＿＿:.", "", ""],
        (MoodLevel::Low, 0) => &["  .:*:.:*:", " ﾟ(￣_￣)ﾟ", "  .:＿＿:.", "", ""],
        (MoodLevel::Low, _) => &["  .:*:.:*:", " ﾟ(￣ ￣)ﾟ", "  .:＿＿:.", "", ""],
    }
}
fn tasogare_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  .:*:.:*:", " ﾉ(・ ・)ﾟ", "  .:＿＿:.", "", ""],
        (Action::Talk, _) => &["  .:*:.:*:", " ﾟ(・ ・)ﾉ", "  .:＿＿:.", "", ""],
        (Action::Play, 0) => &[" ♪.:*:.:*:", " ﾟ(▽ ▽)ﾟ", "  .:＿＿:.", "", ""],
        (Action::Play, _) => &["  .:*:.:*:♪", " ﾟ(▽ ▽)ﾟ", "  .:＿＿:.", "", ""],
        (Action::Train, 0) => &["  .:*:.:*:!!", " ﾟ(益 益)ﾟ", " .:＿＿＿＿:.", "", ""],
        (Action::Train, _) => &["!!.:*:.:*:", " ﾟ(益 益)ﾟ", " .:＿＿＿＿:.", "", ""],
        (Action::Relax, 0) => &["  .:*:.:*:～", " ﾟ(－ －)ﾟ", "  .:＿＿:.", "", ""],
        (Action::Relax, _) => &["  .:*:.:*:", " ﾟ(－ －)zzZ", "  .:＿＿:.", "", ""],
    }
}

// --- ニッコリ (nikkori) - Always smiling, flowers ---
fn nikkori_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ✿❀✿", " ❀(▽‿▽)❀!", "  ❀＿＿❀", "", ""],
        (MoodLevel::High, _) => &["   ✿❀✿", "  ❀(▽‿▽)❀♪", "   ❀＿＿❀", "", ""],
        (MoodLevel::Normal, 0) => &["  ✿❀✿", " ❀(・‿・)❀", "  ❀＿＿❀", "", ""],
        (MoodLevel::Normal, _) => &["   ✿❀✿", "  ❀(・‿・)❀", "   ❀＿＿❀", "", ""],
        (MoodLevel::Low, 0) => &["  ✿❀✿", " ❀(￣_￣)❀", "  ❀＿＿❀", "", ""],
        (MoodLevel::Low, _) => &["  ✿❀✿", " ❀(￣ ￣)❀", "  ❀＿＿❀", "", ""],
    }
}
fn nikkori_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ✿❀✿", " ﾉ(・‿・)❀", "  ❀＿＿❀", "", ""],
        (Action::Talk, _) => &["  ✿❀✿", " ❀(・‿・)ﾉ", "  ❀＿＿❀", "", ""],
        (Action::Play, 0) => &[" ♪✿❀✿", " ❀(▽‿▽)❀", "  ❀＿＿❀", "", ""],
        (Action::Play, _) => &["  ✿❀✿♪", " ❀(▽‿▽)❀", "  ❀＿＿❀", "", ""],
        (Action::Train, 0) => &["  ✿❀✿!!", " ❀(益‿益)❀", " ❀＿＿＿＿❀", "", ""],
        (Action::Train, _) => &["!!✿❀✿", " ❀(益‿益)❀", " ❀＿＿＿＿❀", "", ""],
        (Action::Relax, 0) => &["  ✿❀✿～", " ❀(－‿－)❀", "  ❀＿＿❀", "", ""],
        (Action::Relax, _) => &["  ✿❀✿", " ❀(－‿－)zzZ", "  ❀＿＿❀", "", ""],
    }
}

// --- ダラーン (daraan) - Lazy/droopy ---
fn daraan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～～～～", " ~(▽～▽)~!", "  ~＿～＿~", "", ""],
        (MoodLevel::High, _) => &["   ～～～～", "  ~(▽～▽)~♪", "   ~＿～＿~", "", ""],
        (MoodLevel::Normal, 0) => &["  ～～～～", " ~(・～・)~", "  ~＿～＿~", "", ""],
        (MoodLevel::Normal, _) => &["   ～～～～", "  ~(・～・)~", "   ~＿～＿~", "", ""],
        (MoodLevel::Low, 0) => &["  ～～～～", " ~(￣_￣)~", "  ~＿～＿~", "", ""],
        (MoodLevel::Low, _) => &["  ～～～～", " ~(￣ ￣)~", "  ~＿～＿~", "", ""],
    }
}
fn daraan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～～～～", " ﾉ(・～・)~", "  ~＿～＿~", "", ""],
        (Action::Talk, _) => &["  ～～～～", " ~(・～・)ﾉ", "  ~＿～＿~", "", ""],
        (Action::Play, 0) => &[" ♪～～～～", " ~(▽～▽)~", "  ~＿～＿~", "", ""],
        (Action::Play, _) => &["  ～～～～♪", " ~(▽～▽)~", "  ~＿～＿~", "", ""],
        (Action::Train, 0) => &["  ～～～～!!", " ~(益～益)~", " ~＿～＿～＿~", "", ""],
        (Action::Train, _) => &["!!～～～～", " ~(益～益)~", " ~＿～＿～＿~", "", ""],
        (Action::Relax, 0) => &["  ～～～～～", " ~(－～－)~", "  ~＿～＿~", "", ""],
        (Action::Relax, _) => &["  ～～～～", " ~(－～－)zzZ", "  ~＿～＿~", "", ""],
    }
}

// --- キッチリ (kicchiri) - Precise/neat, geometric ---
fn kicchiri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ┌┬┬┬┐", " ├(▽┃▽)┤!", "  └┴┴┴┘", "", ""],
        (MoodLevel::High, _) => &["   ┌┬┬┬┐", "  ├(▽┃▽)┤♪", "   └┴┴┴┘", "", ""],
        (MoodLevel::Normal, 0) => &["  ┌┬┬┬┐", " ├(・┃・)┤", "  └┴┴┴┘", "", ""],
        (MoodLevel::Normal, _) => &["   ┌┬┬┬┐", "  ├(・┃・)┤", "   └┴┴┴┘", "", ""],
        (MoodLevel::Low, 0) => &["  ┌┬┬┬┐", " ├(￣_￣)┤", "  └┴┴┴┘", "", ""],
        (MoodLevel::Low, _) => &["  ┌┬┬┬┐", " ├(￣ ￣)┤", "  └┴┴┴┘", "", ""],
    }
}
fn kicchiri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ┌┬┬┬┐", " ﾉ(・┃・)┤", "  └┴┴┴┘", "", ""],
        (Action::Talk, _) => &["  ┌┬┬┬┐", " ├(・┃・)ﾉ", "  └┴┴┴┘", "", ""],
        (Action::Play, 0) => &[" ♪┌┬┬┬┐", " ├(▽┃▽)┤", "  └┴┴┴┘", "", ""],
        (Action::Play, _) => &["  ┌┬┬┬┐♪", " ├(▽┃▽)┤", "  └┴┴┴┘", "", ""],
        (Action::Train, 0) => &["  ┌┬┬┬┐!!", " ├(益┃益)┤", " └┴┴┴┴┴┘", "", ""],
        (Action::Train, _) => &["!!┌┬┬┬┐", " ├(益┃益)┤", " └┴┴┴┴┴┘", "", ""],
        (Action::Relax, 0) => &["  ┌┬┬┬┐～", " ├(－┃－)┤", "  └┴┴┴┘", "", ""],
        (Action::Relax, _) => &["  ┌┬┬┬┐", " ├(－┃－)zzZ", "  └┴┴┴┘", "", ""],
    }
}

// --- ボチボチ (bochibochi) - Bit by bit, round ---
fn bochibochi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  。。。", " ○(▽。▽)○!", "  ○＿＿○", "", ""],
        (MoodLevel::High, _) => &["   。。。", "  ○(▽。▽)○♪", "   ○＿＿○", "", ""],
        (MoodLevel::Normal, 0) => &["  。。。", " ○(・。・)○", "  ○＿＿○", "", ""],
        (MoodLevel::Normal, _) => &["   。。。", "  ○(・。・)○", "   ○＿＿○", "", ""],
        (MoodLevel::Low, 0) => &["  。。。", " ○(￣_￣)○", "  ○＿＿○", "", ""],
        (MoodLevel::Low, _) => &["  。。。", " ○(￣ ￣)○", "  ○＿＿○", "", ""],
    }
}
fn bochibochi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  。。。", " ﾉ(・。・)○", "  ○＿＿○", "", ""],
        (Action::Talk, _) => &["  。。。", " ○(・。・)ﾉ", "  ○＿＿○", "", ""],
        (Action::Play, 0) => &[" ♪。。。", " ○(▽。▽)○", "  ○＿＿○", "", ""],
        (Action::Play, _) => &["  。。。♪", " ○(▽。▽)○", "  ○＿＿○", "", ""],
        (Action::Train, 0) => &["  。。。!!", " ○(益。益)○", " ○＿＿＿＿○", "", ""],
        (Action::Train, _) => &["!!。。。", " ○(益。益)○", " ○＿＿＿＿○", "", ""],
        (Action::Relax, 0) => &["  。。。～", " ○(－。－)○", "  ○＿＿○", "", ""],
        (Action::Relax, _) => &["  。。。", " ○(－。－)zzZ", "  ○＿＿○", "", ""],
    }
}

// --- マアマア (maamaa) - So-so, shrugging ---
fn maamaa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＝＝＝", " ┐(▽へ▽)┌!", "  ＿||＿", "", ""],
        (MoodLevel::High, _) => &["   ＝＝＝", "  ┐(▽へ▽)┌♪", "   ＿||＿", "", ""],
        (MoodLevel::Normal, 0) => &["  ＝＝＝", " ┐(・へ・)┌", "  ＿||＿", "", ""],
        (MoodLevel::Normal, _) => &["   ＝＝＝", "  ┐(・へ・)┌", "   ＿||＿", "", ""],
        (MoodLevel::Low, 0) => &["  ＝＝＝", " ┐(￣_￣)┌", "  ＿||＿", "", ""],
        (MoodLevel::Low, _) => &["  ＝＝＝", " ┐(￣ ￣)┌", "  ＿||＿", "", ""],
    }
}
fn maamaa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＝＝＝", " ﾉ(・へ・)┌", "  ＿||＿", "", ""],
        (Action::Talk, _) => &["  ＝＝＝", " ┐(・へ・)ﾉ", "  ＿||＿", "", ""],
        (Action::Play, 0) => &[" ♪＝＝＝", " ┐(▽へ▽)┌", "  ＿||＿", "", ""],
        (Action::Play, _) => &["  ＝＝＝♪", " ┐(▽へ▽)┌", "  ＿||＿", "", ""],
        (Action::Train, 0) => &["  ＝＝＝!!", " ┐(益へ益)┌", " ＿||||＿", "", ""],
        (Action::Train, _) => &["!!＝＝＝", " ┐(益へ益)┌", " ＿||||＿", "", ""],
        (Action::Relax, 0) => &["  ＝＝＝～", " ┐(－へ－)┌", "  ＿||＿", "", ""],
        (Action::Relax, _) => &["  ＝＝＝", " ┐(－へ－)zzZ", "  ＿||＿", "", ""],
    }
}

// --- フニャ (funya) - Soft/bendy, wobbly ---
fn funya_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～∽～∽", " ∽(▽ω▽)∽!", "  ∽＿∽＿∽", "", ""],
        (MoodLevel::High, _) => &["   ～∽～∽", "  ∽(▽ω▽)∽♪", "   ∽＿∽＿∽", "", ""],
        (MoodLevel::Normal, 0) => &["  ～∽～∽", " ∽(・ω・)∽", "  ∽＿∽＿∽", "", ""],
        (MoodLevel::Normal, _) => &["   ～∽～∽", "  ∽(・ω・)∽", "   ∽＿∽＿∽", "", ""],
        (MoodLevel::Low, 0) => &["  ～∽～∽", " ∽(￣_￣)∽", "  ∽＿∽＿∽", "", ""],
        (MoodLevel::Low, _) => &["  ～∽～∽", " ∽(￣ ￣)∽", "  ∽＿∽＿∽", "", ""],
    }
}
fn funya_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～∽～∽", " ﾉ(・ω・)∽", "  ∽＿∽＿∽", "", ""],
        (Action::Talk, _) => &["  ～∽～∽", " ∽(・ω・)ﾉ", "  ∽＿∽＿∽", "", ""],
        (Action::Play, 0) => &[" ♪～∽～∽", " ∽(▽ω▽)∽", "  ∽＿∽＿∽", "", ""],
        (Action::Play, _) => &["  ～∽～∽♪", " ∽(▽ω▽)∽", "  ∽＿∽＿∽", "", ""],
        (Action::Train, 0) => &["  ～∽～∽!!", " ∽(益ω益)∽", " ∽＿∽＿∽＿∽", "", ""],
        (Action::Train, _) => &["!!～∽～∽", " ∽(益ω益)∽", " ∽＿∽＿∽＿∽", "", ""],
        (Action::Relax, 0) => &["  ～∽～∽～", " ∽(－ω－)∽", "  ∽＿∽＿∽", "", ""],
        (Action::Relax, _) => &["  ～∽～∽", " ∽(－ω－)zzZ", "  ∽＿∽＿∽", "", ""],
    }
}

// --- テンテン (tenten) - Dotted/spotted ---
fn tenten_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ・:・:・", " :(▽:▽):.!", "  :・＿・:", "", ""],
        (MoodLevel::High, _) => &["   ・:・:・", "  :(▽:▽):.♪", "   :・＿・:", "", ""],
        (MoodLevel::Normal, 0) => &["  ・:・:・", " :(・:・):.", "  :・＿・:", "", ""],
        (MoodLevel::Normal, _) => &["   ・:・:・", "  :(・:・):.", "   :・＿・:", "", ""],
        (MoodLevel::Low, 0) => &["  ・:・:・", " :(￣_￣):.", "  :・＿・:", "", ""],
        (MoodLevel::Low, _) => &["  ・:・:・", " :(￣ ￣):.", "  :・＿・:", "", ""],
    }
}
fn tenten_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ・:・:・", " ﾉ(・:・):.", "  :・＿・:", "", ""],
        (Action::Talk, _) => &["  ・:・:・", " :(・:・)ﾉ.", "  :・＿・:", "", ""],
        (Action::Play, 0) => &[" ♪・:・:・", " :(▽:▽):.", "  :・＿・:", "", ""],
        (Action::Play, _) => &["  ・:・:・♪", " :(▽:▽):.", "  :・＿・:", "", ""],
        (Action::Train, 0) => &["  ・:・:・!!", " :(益:益):.", " :・＿・＿・:", "", ""],
        (Action::Train, _) => &["!!・:・:・", " :(益:益):.", " :・＿・＿・:", "", ""],
        (Action::Relax, 0) => &["  ・:・:・～", " :(－:－):.", "  :・＿・:", "", ""],
        (Action::Relax, _) => &["  ・:・:・", " :(－:－)zzZ", "  :・＿・:", "", ""],
    }
}

// --- ナァナァ (naanaa) - Casual, laid-back ---
fn naanaa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～＿～＿", " ＿(▽ v▽)＿!", "  ＿/＿＿\\＿", "", ""],
        (MoodLevel::High, _) => &["   ～＿～＿", "  ＿(▽ v▽)＿♪", "   ＿/＿＿\\＿", "", ""],
        (MoodLevel::Normal, 0) => &["  ～＿～＿", " ＿(・ v・)＿", "  ＿/＿＿\\＿", "", ""],
        (MoodLevel::Normal, _) => &["   ～＿～＿", "  ＿(・ v・)＿", "   ＿/＿＿\\＿", "", ""],
        (MoodLevel::Low, 0) => &["  ～＿～＿", " ＿(￣_￣)＿", "  ＿/＿＿\\＿", "", ""],
        (MoodLevel::Low, _) => &["  ～＿～＿", " ＿(￣ ￣)＿", "  ＿/＿＿\\＿", "", ""],
    }
}
fn naanaa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～＿～＿", " ﾉ(・ v・)＿", "  ＿/＿＿\\＿", "", ""],
        (Action::Talk, _) => &["  ～＿～＿", " ＿(・ v・)ﾉ", "  ＿/＿＿\\＿", "", ""],
        (Action::Play, 0) => &[" ♪～＿～＿", " ＿(▽ v▽)＿", "  ＿/＿＿\\＿", "", ""],
        (Action::Play, _) => &["  ～＿～＿♪", " ＿(▽ v▽)＿", "  ＿/＿＿\\＿", "", ""],
        (Action::Train, 0) => &["  ～＿～＿!!", " ＿(益 v益)＿", " ＿/＿＿＿＿\\＿", "", ""],
        (Action::Train, _) => &["!!～＿～＿", " ＿(益 v益)＿", " ＿/＿＿＿＿\\＿", "", ""],
        (Action::Relax, 0) => &["  ～＿～＿～", " ＿(－ v－)＿", "  ＿/＿＿\\＿", "", ""],
        (Action::Relax, _) => &["  ～＿～＿", " ＿(－ v－)zzZ", "  ＿/＿＿\\＿", "", ""],
    }
}

// --- ポツリ (potsuri) - Solitary/quiet, small ---
fn potsuri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   .", "  (▽▽)!", "  u＿u", "", ""],
        (MoodLevel::High, _) => &["    .", "   (▽▽)♪", "   u＿u", "", ""],
        (MoodLevel::Normal, 0) => &["   .", "  (・・)", "  u＿u", "", ""],
        (MoodLevel::Normal, _) => &["    .", "   (・・)", "   u＿u", "", ""],
        (MoodLevel::Low, 0) => &["   .", "  (￣_￣)", "  u＿u", "", ""],
        (MoodLevel::Low, _) => &["   .", "  (￣ ￣)", "  u＿u", "", ""],
    }
}
fn potsuri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   .", " ﾉ(・・)", "  u＿u", "", ""],
        (Action::Talk, _) => &["   .", "  (・・)ﾉ", "  u＿u", "", ""],
        (Action::Play, 0) => &["  ♪.", "  (▽▽)", "  u＿u", "", ""],
        (Action::Play, _) => &["   .♪", "  (▽▽)", "  u＿u", "", ""],
        (Action::Train, 0) => &["   .!!", "  (益益)", " u＿＿u", "", ""],
        (Action::Train, _) => &["!!  .", "  (益益)", " u＿＿u", "", ""],
        (Action::Relax, 0) => &["   .～", "  (－－)", "  u＿u", "", ""],
        (Action::Relax, _) => &["   .", "  (－－)zzZ", "  u＿u", "", ""],
    }
}

// --- ソレナリ (sorenari) - Adequate, moderate ---
fn sorenari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ∩＝＝∩", " ‖(▽∀▽)‖!", "  ‖＿＿‖", "", ""],
        (MoodLevel::High, _) => &["   ∩＝＝∩", "  ‖(▽∀▽)‖♪", "   ‖＿＿‖", "", ""],
        (MoodLevel::Normal, 0) => &["  ∩＝＝∩", " ‖(・∀・)‖", "  ‖＿＿‖", "", ""],
        (MoodLevel::Normal, _) => &["   ∩＝＝∩", "  ‖(・∀・)‖", "   ‖＿＿‖", "", ""],
        (MoodLevel::Low, 0) => &["  ∩＝＝∩", " ‖(￣_￣)‖", "  ‖＿＿‖", "", ""],
        (MoodLevel::Low, _) => &["  ∩＝＝∩", " ‖(￣ ￣)‖", "  ‖＿＿‖", "", ""],
    }
}
fn sorenari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ∩＝＝∩", " ﾉ(・∀・)‖", "  ‖＿＿‖", "", ""],
        (Action::Talk, _) => &["  ∩＝＝∩", " ‖(・∀・)ﾉ", "  ‖＿＿‖", "", ""],
        (Action::Play, 0) => &[" ♪∩＝＝∩", " ‖(▽∀▽)‖", "  ‖＿＿‖", "", ""],
        (Action::Play, _) => &["  ∩＝＝∩♪", " ‖(▽∀▽)‖", "  ‖＿＿‖", "", ""],
        (Action::Train, 0) => &["  ∩＝＝∩!!", " ‖(益∀益)‖", " ‖＿＿＿＿‖", "", ""],
        (Action::Train, _) => &["!!∩＝＝∩", " ‖(益∀益)‖", " ‖＿＿＿＿‖", "", ""],
        (Action::Relax, 0) => &["  ∩＝＝∩～", " ‖(－∀－)‖", "  ‖＿＿‖", "", ""],
        (Action::Relax, _) => &["  ∩＝＝∩", " ‖(－∀－)zzZ", "  ‖＿＿‖", "", ""],
    }
}

// --- ウンウン (unun) - Nodding, round ---
fn unun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ◎◎◎", " ◎(▽◎▽)◎!", "  ◎＿＿◎", "", ""],
        (MoodLevel::High, _) => &["   ◎◎◎", "  ◎(▽◎▽)◎♪", "   ◎＿＿◎", "", ""],
        (MoodLevel::Normal, 0) => &["  ◎◎◎", " ◎(・◎・)◎", "  ◎＿＿◎", "", ""],
        (MoodLevel::Normal, _) => &["   ◎◎◎", "  ◎(・◎・)◎", "   ◎＿＿◎", "", ""],
        (MoodLevel::Low, 0) => &["  ◎◎◎", " ◎(￣_￣)◎", "  ◎＿＿◎", "", ""],
        (MoodLevel::Low, _) => &["  ◎◎◎", " ◎(￣ ￣)◎", "  ◎＿＿◎", "", ""],
    }
}
fn unun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ◎◎◎", " ﾉ(・◎・)◎", "  ◎＿＿◎", "", ""],
        (Action::Talk, _) => &["  ◎◎◎", " ◎(・◎・)ﾉ", "  ◎＿＿◎", "", ""],
        (Action::Play, 0) => &[" ♪◎◎◎", " ◎(▽◎▽)◎", "  ◎＿＿◎", "", ""],
        (Action::Play, _) => &["  ◎◎◎♪", " ◎(▽◎▽)◎", "  ◎＿＿◎", "", ""],
        (Action::Train, 0) => &["  ◎◎◎!!", " ◎(益◎益)◎", " ◎＿＿＿＿◎", "", ""],
        (Action::Train, _) => &["!!◎◎◎", " ◎(益◎益)◎", " ◎＿＿＿＿◎", "", ""],
        (Action::Relax, 0) => &["  ◎◎◎～", " ◎(－◎－)◎", "  ◎＿＿◎", "", ""],
        (Action::Relax, _) => &["  ◎◎◎", " ◎(－◎－)zzZ", "  ◎＿＿◎", "", ""],
    }
}

// --- チャッカリ (chakkari) - Shrewd/clever ---
fn chakkari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  △▽△▽", " ▷(▽ｪ▽)◁!", "  ▽△＿△▽", "", ""],
        (MoodLevel::High, _) => &["   △▽△▽", "  ▷(▽ｪ▽)◁♪", "   ▽△＿△▽", "", ""],
        (MoodLevel::Normal, 0) => &["  △▽△▽", " ▷(¬ｪ¬)◁", "  ▽△＿△▽", "", ""],
        (MoodLevel::Normal, _) => &["   △▽△▽", "  ▷(¬ｪ¬)◁", "   ▽△＿△▽", "", ""],
        (MoodLevel::Low, 0) => &["  △▽△▽", " ▷(￣_￣)◁", "  ▽△＿△▽", "", ""],
        (MoodLevel::Low, _) => &["  △▽△▽", " ▷(￣ ￣)◁", "  ▽△＿△▽", "", ""],
    }
}
fn chakkari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  △▽△▽", " ﾉ(¬ｪ¬)◁", "  ▽△＿△▽", "", ""],
        (Action::Talk, _) => &["  △▽△▽", " ▷(¬ｪ¬)ﾉ", "  ▽△＿△▽", "", ""],
        (Action::Play, 0) => &[" ♪△▽△▽", " ▷(▽ｪ▽)◁", "  ▽△＿△▽", "", ""],
        (Action::Play, _) => &["  △▽△▽♪", " ▷(▽ｪ▽)◁", "  ▽△＿△▽", "", ""],
        (Action::Train, 0) => &["  △▽△▽!!", " ▷(益ｪ益)◁", " ▽△＿＿＿△▽", "", ""],
        (Action::Train, _) => &["!!△▽△▽", " ▷(益ｪ益)◁", " ▽△＿＿＿△▽", "", ""],
        (Action::Relax, 0) => &["  △▽△▽～", " ▷(－ｪ－)◁", "  ▽△＿△▽", "", ""],
        (Action::Relax, _) => &["  △▽△▽", " ▷(－ｪ－)zzZ", "  ▽△＿△▽", "", ""],
    }
}

// --- ヌルリ (nururi) - Slimy/slippery ---
fn nururi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ;,:;,:;", " ;,(▽~▽),;!", "  ;,＿~＿,;", "", ""],
        (MoodLevel::High, _) => &["   ;,:;,:;", "  ;,(▽~▽),;♪", "   ;,＿~＿,;", "", ""],
        (MoodLevel::Normal, 0) => &["  ;,:;,:;", " ;,(・~・),;", "  ;,＿~＿,;", "", ""],
        (MoodLevel::Normal, _) => &["   ;,:;,:;", "  ;,(・~・),;", "   ;,＿~＿,;", "", ""],
        (MoodLevel::Low, 0) => &["  ;,:;,:;", " ;,(￣_￣),;", "  ;,＿~＿,;", "", ""],
        (MoodLevel::Low, _) => &["  ;,:;,:;", " ;,(￣ ￣),;", "  ;,＿~＿,;", "", ""],
    }
}
fn nururi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ;,:;,:;", " ﾉ(・~・),;", "  ;,＿~＿,;", "", ""],
        (Action::Talk, _) => &["  ;,:;,:;", " ;,(・~・)ﾉ;", "  ;,＿~＿,;", "", ""],
        (Action::Play, 0) => &[" ♪;,:;,:;", " ;,(▽~▽),;", "  ;,＿~＿,;", "", ""],
        (Action::Play, _) => &["  ;,:;,:;♪", " ;,(▽~▽),;", "  ;,＿~＿,;", "", ""],
        (Action::Train, 0) => &["  ;,:;,:;!!", " ;,(益~益),;", " ;,＿~＿~＿,;", "", ""],
        (Action::Train, _) => &["!!;,:;,:;", " ;,(益~益),;", " ;,＿~＿~＿,;", "", ""],
        (Action::Relax, 0) => &["  ;,:;,:;～", " ;,(－~－),;", "  ;,＿~＿,;", "", ""],
        (Action::Relax, _) => &["  ;,:;,:;", " ;,(－~－)zzZ", "  ;,＿~＿,;", "", ""],
    }
}

// --- ヤレヤレ (yareyare) - Exasperated, sweat drops ---
fn yareyare_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ；；；", " ；(▽A▽)；!", "  ；＿＿；", "", ""],
        (MoodLevel::High, _) => &["   ；；；", "  ；(▽A▽)；♪", "   ；＿＿；", "", ""],
        (MoodLevel::Normal, 0) => &["  ；；；", " ；(・A・)；", "  ；＿＿；", "", ""],
        (MoodLevel::Normal, _) => &["   ；；；", "  ；(・A・)；", "   ；＿＿；", "", ""],
        (MoodLevel::Low, 0) => &["  ；；；", " ；(￣_￣)；", "  ；＿＿；", "", ""],
        (MoodLevel::Low, _) => &["  ；；；", " ；(￣ ￣)；", "  ；＿＿；", "", ""],
    }
}
fn yareyare_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ；；；", " ﾉ(・A・)；", "  ；＿＿；", "", ""],
        (Action::Talk, _) => &["  ；；；", " ；(・A・)ﾉ", "  ；＿＿；", "", ""],
        (Action::Play, 0) => &[" ♪；；；", " ；(▽A▽)；", "  ；＿＿；", "", ""],
        (Action::Play, _) => &["  ；；；♪", " ；(▽A▽)；", "  ；＿＿；", "", ""],
        (Action::Train, 0) => &["  ；；；!!", " ；(益A益)；", " ；＿＿＿＿；", "", ""],
        (Action::Train, _) => &["!!；；；", " ；(益A益)；", " ；＿＿＿＿；", "", ""],
        (Action::Relax, 0) => &["  ；；；～", " ；(－A－)；", "  ；＿＿；", "", ""],
        (Action::Relax, _) => &["  ；；；", " ；(－A－)zzZ", "  ；＿＿；", "", ""],
    }
}

// --- ドッコイ (dokkoi) - Heave-ho, sturdy ---
fn dokkoi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ▄▄▄▄", " ▌(▽皿▽)▐!", "  ▀▀▀▀", "", ""],
        (MoodLevel::High, _) => &["   ▄▄▄▄", "  ▌(▽皿▽)▐♪", "   ▀▀▀▀", "", ""],
        (MoodLevel::Normal, 0) => &["  ▄▄▄▄", " ▌(・皿・)▐", "  ▀▀▀▀", "", ""],
        (MoodLevel::Normal, _) => &["   ▄▄▄▄", "  ▌(・皿・)▐", "   ▀▀▀▀", "", ""],
        (MoodLevel::Low, 0) => &["  ▄▄▄▄", " ▌(￣_￣)▐", "  ▀▀▀▀", "", ""],
        (MoodLevel::Low, _) => &["  ▄▄▄▄", " ▌(￣ ￣)▐", "  ▀▀▀▀", "", ""],
    }
}
fn dokkoi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ▄▄▄▄", " ﾉ(・皿・)▐", "  ▀▀▀▀", "", ""],
        (Action::Talk, _) => &["  ▄▄▄▄", " ▌(・皿・)ﾉ", "  ▀▀▀▀", "", ""],
        (Action::Play, 0) => &[" ♪▄▄▄▄", " ▌(▽皿▽)▐", "  ▀▀▀▀", "", ""],
        (Action::Play, _) => &["  ▄▄▄▄♪", " ▌(▽皿▽)▐", "  ▀▀▀▀", "", ""],
        (Action::Train, 0) => &["  ▄▄▄▄!!", " ▌(益皿益)▐", " ▀▀▀▀▀▀", "", ""],
        (Action::Train, _) => &["!!▄▄▄▄", " ▌(益皿益)▐", " ▀▀▀▀▀▀", "", ""],
        (Action::Relax, 0) => &["  ▄▄▄▄～", " ▌(－皿－)▐", "  ▀▀▀▀", "", ""],
        (Action::Relax, _) => &["  ▄▄▄▄", " ▌(－皿－)zzZ", "  ▀▀▀▀", "", ""],
    }
}

// --- パッパ (pappa) - Quick/brisk, light ---
fn pappa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(▽v▽)ﾊﾟ!", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (MoodLevel::High, _) => &["   ﾊﾟﾊﾟﾊﾟ", "  ﾊﾟ(▽v▽)ﾊﾟ♪", "   ﾊﾟ＿ﾊﾟ", "", ""],
        (MoodLevel::Normal, 0) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(・v・)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (MoodLevel::Normal, _) => &["   ﾊﾟﾊﾟﾊﾟ", "  ﾊﾟ(・v・)ﾊﾟ", "   ﾊﾟ＿ﾊﾟ", "", ""],
        (MoodLevel::Low, 0) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(￣_￣)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (MoodLevel::Low, _) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(￣ ￣)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
    }
}
fn pappa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾉ(・v・)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (Action::Talk, _) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(・v・)ﾉ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (Action::Play, 0) => &[" ♪ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(▽v▽)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (Action::Play, _) => &["  ﾊﾟﾊﾟﾊﾟ♪", " ﾊﾟ(▽v▽)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (Action::Train, 0) => &["  ﾊﾟﾊﾟﾊﾟ!!", " ﾊﾟ(益v益)ﾊﾟ", " ﾊﾟ＿＿＿ﾊﾟ", "", ""],
        (Action::Train, _) => &["!!ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(益v益)ﾊﾟ", " ﾊﾟ＿＿＿ﾊﾟ", "", ""],
        (Action::Relax, 0) => &["  ﾊﾟﾊﾟﾊﾟ～", " ﾊﾟ(－v－)ﾊﾟ", "  ﾊﾟ＿ﾊﾟ", "", ""],
        (Action::Relax, _) => &["  ﾊﾟﾊﾟﾊﾟ", " ﾊﾟ(－v－)zzZ", "  ﾊﾟ＿ﾊﾟ", "", ""],
    }
}

// --- オットリ (ottori) - Gentle/mild, soft curves ---
fn ottori_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～。～。", " 。(▽u▽)。!", "  。～＿～。", "", ""],
        (MoodLevel::High, _) => &["   ～。～。", "  。(▽u▽)。♪", "   。～＿～。", "", ""],
        (MoodLevel::Normal, 0) => &["  ～。～。", " 。(・u・)。", "  。～＿～。", "", ""],
        (MoodLevel::Normal, _) => &["   ～。～。", "  。(・u・)。", "   。～＿～。", "", ""],
        (MoodLevel::Low, 0) => &["  ～。～。", " 。(￣_￣)。", "  。～＿～。", "", ""],
        (MoodLevel::Low, _) => &["  ～。～。", " 。(￣ ￣)。", "  。～＿～。", "", ""],
    }
}
fn ottori_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～。～。", " ﾉ(・u・)。", "  。～＿～。", "", ""],
        (Action::Talk, _) => &["  ～。～。", " 。(・u・)ﾉ", "  。～＿～。", "", ""],
        (Action::Play, 0) => &[" ♪～。～。", " 。(▽u▽)。", "  。～＿～。", "", ""],
        (Action::Play, _) => &["  ～。～。♪", " 。(▽u▽)。", "  。～＿～。", "", ""],
        (Action::Train, 0) => &["  ～。～。!!", " 。(益u益)。", " 。～＿＿＿～。", "", ""],
        (Action::Train, _) => &["!!～。～。", " 。(益u益)。", " 。～＿＿＿～。", "", ""],
        (Action::Relax, 0) => &["  ～。～。～", " 。(－u－)。", "  。～＿～。", "", ""],
        (Action::Relax, _) => &["  ～。～。", " 。(－u－)zzZ", "  。～＿～。", "", ""],
    }
}

// ============================================================
// ODAYAKA TYPE Stage 3 Species
// ============================================================

// --- ながれもん (nagaremon) - Drifting thing, cloud-stream silhouette ---
fn nagaremon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～☁～☁", " ～(▽ᵕ▽)～!", "  ～～＿～～", "", ""],
        (MoodLevel::High, _) => &["   ☁～☁～", "  ～(▽ᵕ▽)～♪", "   ～～＿～～", "", ""],
        (MoodLevel::Normal, 0) => &["  ～☁～☁", " ～(・ᵕ・)～", "  ～～＿～～", "", ""],
        (MoodLevel::Normal, _) => &["   ☁～☁～", "  ～(˘ᵕ˘)～", "   ～～＿～～", "", ""],
        (MoodLevel::Low, 0) => &["  ～☁～☁", " ～(￣_￣)～", "  ～～＿～～", "", ""],
        (MoodLevel::Low, _) => &["  ～☁～☁", " ～(￣ ￣)～", "  ～～＿～～", "", ""],
    }
}
fn nagaremon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～☁～☁", " ﾉ(・ᵕ・)～", "  ～～＿～～", "", ""],
        (Action::Talk, _) => &["  ～☁～☁", " ～(・ᵕ・)ﾉ", "  ～～＿～～", "", ""],
        (Action::Play, 0) => &[" ♪～☁～☁", " ～(▽ᵕ▽)～", "  ～～＿～～", "", ""],
        (Action::Play, _) => &["  ～☁～☁♪", " ～(▽ᵕ▽)～", "  ～～＿～～", "", ""],
        (Action::Train, 0) => &["  ～☁～☁!!", " ～(益ᵕ益)～", " ～～＿＿＿～～", "", ""],
        (Action::Train, _) => &["!!～☁～☁", " ～(益ᵕ益)～", " ～～＿＿＿～～", "", ""],
        (Action::Relax, 0) => &["  ～☁～☁～", " ～(－ᵕ－)～", "  ～～＿～～", "", ""],
        (Action::Relax, _) => &["  ～☁～☁", " ～(－ᵕ－)zzZ", "  ～～＿～～", "", ""],
    }
}

// --- フワリン (fuwarin) - Floaty, feather-puff silhouette ---
fn fuwarin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  °ﾟ°ﾟ°", " ﾟ(▽ω▽)ﾟ!", "  °ﾟ～ﾟ°", "", ""],
        (MoodLevel::High, _) => &["   °ﾟ°ﾟ°", "  ﾟ(▽ω▽)ﾟ♪", "   °ﾟ～ﾟ°", "", ""],
        (MoodLevel::Normal, 0) => &["  °ﾟ°ﾟ°", " ﾟ(・ω・)ﾟ", "  °ﾟ～ﾟ°", "", ""],
        (MoodLevel::Normal, _) => &["   °ﾟ°ﾟ°", "  ﾟ(˘ω˘)ﾟ", "   °ﾟ～ﾟ°", "", ""],
        (MoodLevel::Low, 0) => &["  °ﾟ°ﾟ°", " ﾟ(￣_￣)ﾟ", "  °ﾟ～ﾟ°", "", ""],
        (MoodLevel::Low, _) => &["  °ﾟ°ﾟ°", " ﾟ(￣ ￣)ﾟ", "  °ﾟ～ﾟ°", "", ""],
    }
}
fn fuwarin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  °ﾟ°ﾟ°", " ﾉ(・ω・)ﾟ", "  °ﾟ～ﾟ°", "", ""],
        (Action::Talk, _) => &["  °ﾟ°ﾟ°", " ﾟ(・ω・)ﾉ", "  °ﾟ～ﾟ°", "", ""],
        (Action::Play, 0) => &[" ♪°ﾟ°ﾟ°", " ﾟ(▽ω▽)ﾟ", "  °ﾟ～ﾟ°", "", ""],
        (Action::Play, _) => &["  °ﾟ°ﾟ°♪", " ﾟ(▽ω▽)ﾟ", "  °ﾟ～ﾟ°", "", ""],
        (Action::Train, 0) => &["  °ﾟ°ﾟ°!!", " ﾟ(益ω益)ﾟ", " °ﾟ～～～ﾟ°", "", ""],
        (Action::Train, _) => &["!!°ﾟ°ﾟ°", " ﾟ(益ω益)ﾟ", " °ﾟ～～～ﾟ°", "", ""],
        (Action::Relax, 0) => &["  °ﾟ°ﾟ°～", " ﾟ(－ω－)ﾟ", "  °ﾟ～ﾟ°", "", ""],
        (Action::Relax, _) => &["  °ﾟ°ﾟ°", " ﾟ(－ω－)zzZ", "  °ﾟ～ﾟ°", "", ""],
    }
}

// --- モコモコ (mokomoko) - Fluffy, wool-puff silhouette ---
fn mokomoko_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ∩∩∩∩", " ∩(▽ᴗ▽)∩!", "  ∪∪∪∪", "", ""],
        (MoodLevel::High, _) => &["   ∩∩∩∩", "  ∩(▽ᴗ▽)∩♪", "   ∪∪∪∪", "", ""],
        (MoodLevel::Normal, 0) => &["  ∩∩∩∩", " ∩(・ᴗ・)∩", "  ∪∪∪∪", "", ""],
        (MoodLevel::Normal, _) => &["   ∩∩∩∩", "  ∩(˘ᴗ˘)∩", "   ∪∪∪∪", "", ""],
        (MoodLevel::Low, 0) => &["  ∩∩∩∩", " ∩(￣_￣)∩", "  ∪∪∪∪", "", ""],
        (MoodLevel::Low, _) => &["  ∩∩∩∩", " ∩(￣ ￣)∩", "  ∪∪∪∪", "", ""],
    }
}
fn mokomoko_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ∩∩∩∩", " ﾉ(・ᴗ・)∩", "  ∪∪∪∪", "", ""],
        (Action::Talk, _) => &["  ∩∩∩∩", " ∩(・ᴗ・)ﾉ", "  ∪∪∪∪", "", ""],
        (Action::Play, 0) => &[" ♪∩∩∩∩", " ∩(▽ᴗ▽)∩", "  ∪∪∪∪", "", ""],
        (Action::Play, _) => &["  ∩∩∩∩♪", " ∩(▽ᴗ▽)∩", "  ∪∪∪∪", "", ""],
        (Action::Train, 0) => &["  ∩∩∩∩!!", " ∩(益ᴗ益)∩", " ∪∪∪∪∪∪", "", ""],
        (Action::Train, _) => &["!!∩∩∩∩", " ∩(益ᴗ益)∩", " ∪∪∪∪∪∪", "", ""],
        (Action::Relax, 0) => &["  ∩∩∩∩～", " ∩(－ᴗ－)∩", "  ∪∪∪∪", "", ""],
        (Action::Relax, _) => &["  ∩∩∩∩", " ∩(－ᴗ－)zzZ", "  ∪∪∪∪", "", ""],
    }
}

// --- ネンネ (nenne) - Sleepy baby, bonnet silhouette ---
fn nenne_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ♥∩∩♥", " ♥(▽ᵕ▽)♥!", "  ♥～＿～♥", "", ""],
        (MoodLevel::High, _) => &["   ♥∩∩♥", "  ♥(▽ᵕ▽)♥♪", "   ♥～＿～♥", "", ""],
        (MoodLevel::Normal, 0) => &["  ♥∩∩♥", " ♥(˶ᵕ˶)♥", "  ♥～＿～♥", "", ""],
        (MoodLevel::Normal, _) => &["   ♥∩∩♥", "  ♥(˘ᵕ˘)♥", "   ♥～＿～♥", "", ""],
        (MoodLevel::Low, 0) => &["  ♥∩∩♥", " ♥(￣_￣)♥", "  ♥～＿～♥", "", ""],
        (MoodLevel::Low, _) => &["  ♥∩∩♥", " ♥(￣ ￣)♥", "  ♥～＿～♥", "", ""],
    }
}
fn nenne_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ♥∩∩♥", " ﾉ(˶ᵕ˶)♥", "  ♥～＿～♥", "", ""],
        (Action::Talk, _) => &["  ♥∩∩♥", " ♥(˶ᵕ˶)ﾉ", "  ♥～＿～♥", "", ""],
        (Action::Play, 0) => &[" ♪♥∩∩♥", " ♥(▽ᵕ▽)♥", "  ♥～＿～♥", "", ""],
        (Action::Play, _) => &["  ♥∩∩♥♪", " ♥(▽ᵕ▽)♥", "  ♥～＿～♥", "", ""],
        (Action::Train, 0) => &["  ♥∩∩♥!!", " ♥(益ᵕ益)♥", " ♥～＿＿＿～♥", "", ""],
        (Action::Train, _) => &["!!♥∩∩♥", " ♥(益ᵕ益)♥", " ♥～＿＿＿～♥", "", ""],
        (Action::Relax, 0) => &["  ♥∩∩♥～", " ♥(－ᵕ－)♥", "  ♥～＿～♥", "", ""],
        (Action::Relax, _) => &["  ♥∩∩♥", " ♥(－ᵕ－)zzZ", "  ♥～＿～♥", "", ""],
    }
}

// --- ポヨン (poyon) - Bouncy soft, jelly silhouette ---
fn poyon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ○ぷ○", " ぷ(▽◡▽)ぷ!", "  ○ぷ～ぷ○", "", ""],
        (MoodLevel::High, _) => &["   ○ぷ○", "  ぷ(▽◡▽)ぷ♪", "   ○ぷ～ぷ○", "", ""],
        (MoodLevel::Normal, 0) => &["  ○ぷ○", " ぷ(・◡・)ぷ", "  ○ぷ～ぷ○", "", ""],
        (MoodLevel::Normal, _) => &["   ○ぷ○", "  ぷ(˘◡˘)ぷ", "   ○ぷ～ぷ○", "", ""],
        (MoodLevel::Low, 0) => &["  ○ぷ○", " ぷ(￣_￣)ぷ", "  ○ぷ～ぷ○", "", ""],
        (MoodLevel::Low, _) => &["  ○ぷ○", " ぷ(￣ ￣)ぷ", "  ○ぷ～ぷ○", "", ""],
    }
}
fn poyon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ○ぷ○", " ﾉ(・◡・)ぷ", "  ○ぷ～ぷ○", "", ""],
        (Action::Talk, _) => &["  ○ぷ○", " ぷ(・◡・)ﾉ", "  ○ぷ～ぷ○", "", ""],
        (Action::Play, 0) => &[" ♪○ぷ○", " ぷ(▽◡▽)ぷ", "  ○ぷ～ぷ○", "", ""],
        (Action::Play, _) => &["  ○ぷ○♪", " ぷ(▽◡▽)ぷ", "  ○ぷ～ぷ○", "", ""],
        (Action::Train, 0) => &["  ○ぷ○!!", " ぷ(益◡益)ぷ", " ○ぷ～～～ぷ○", "", ""],
        (Action::Train, _) => &["!!○ぷ○", " ぷ(益◡益)ぷ", " ○ぷ～～～ぷ○", "", ""],
        (Action::Relax, 0) => &["  ○ぷ○～", " ぷ(－◡－)ぷ", "  ○ぷ～ぷ○", "", ""],
        (Action::Relax, _) => &["  ○ぷ○", " ぷ(－◡－)zzZ", "  ○ぷ～ぷ○", "", ""],
    }
}

// --- スヤスヤ (suyasuya) - Peacefully sleeping, pillow silhouette ---
fn suyasuya_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  zzz☆", " ☆(▽‿▽)☆!", "  ～～～～", "", ""],
        (MoodLevel::High, _) => &["   zzz☆", "  ☆(▽‿▽)☆♪", "   ～～～～", "", ""],
        (MoodLevel::Normal, 0) => &["  zzz☆", " ☆(˘‿˘)☆", "  ～～～～", "", ""],
        (MoodLevel::Normal, _) => &["   zzz☆", "  ☆(˘‿˘)☆", "   ～～～～", "", ""],
        (MoodLevel::Low, 0) => &["  zzz☆", " ☆(￣_￣)☆", "  ～～～～", "", ""],
        (MoodLevel::Low, _) => &["  zzz☆", " ☆(￣ ￣)☆", "  ～～～～", "", ""],
    }
}
fn suyasuya_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  zzz☆", " ﾉ(˘‿˘)☆", "  ～～～～", "", ""],
        (Action::Talk, _) => &["  zzz☆", " ☆(˘‿˘)ﾉ", "  ～～～～", "", ""],
        (Action::Play, 0) => &[" ♪zzz☆", " ☆(▽‿▽)☆", "  ～～～～", "", ""],
        (Action::Play, _) => &["  zzz☆♪", " ☆(▽‿▽)☆", "  ～～～～", "", ""],
        (Action::Train, 0) => &["  zzz☆!!", " ☆(益‿益)☆", " ～～～～～～", "", ""],
        (Action::Train, _) => &["!!zzz☆", " ☆(益‿益)☆", " ～～～～～～", "", ""],
        (Action::Relax, 0) => &["  zzz☆～", " ☆(－‿－)☆", "  ～～～～", "", ""],
        (Action::Relax, _) => &["  zzz☆", " ☆(－‿－)zzZ", "  ～～～～", "", ""],
    }
}

// --- カスミ (kasumi) - Mist/haze, wispy silhouette ---
fn kasumi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ···☁·", " ·(▽˵▽)·!", "  ·～·～·", "", ""],
        (MoodLevel::High, _) => &["   ···☁·", "  ·(▽˵▽)·♪", "   ·～·～·", "", ""],
        (MoodLevel::Normal, 0) => &["  ···☁·", " ·(˵·˵)·", "  ·～·～·", "", ""],
        (MoodLevel::Normal, _) => &["   ···☁·", "  ·(˵˘˵)·", "   ·～·～·", "", ""],
        (MoodLevel::Low, 0) => &["  ···☁·", " ·(￣_￣)·", "  ·～·～·", "", ""],
        (MoodLevel::Low, _) => &["  ···☁·", " ·(￣ ￣)·", "  ·～·～·", "", ""],
    }
}
fn kasumi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ···☁·", " ﾉ(˵·˵)·", "  ·～·～·", "", ""],
        (Action::Talk, _) => &["  ···☁·", " ·(˵·˵)ﾉ", "  ·～·～·", "", ""],
        (Action::Play, 0) => &[" ♪···☁·", " ·(▽˵▽)·", "  ·～·～·", "", ""],
        (Action::Play, _) => &["  ···☁·♪", " ·(▽˵▽)·", "  ·～·～·", "", ""],
        (Action::Train, 0) => &["  ···☁·!!", " ·(益˵益)·", " ·～·～·～·", "", ""],
        (Action::Train, _) => &["!!···☁·", " ·(益˵益)·", " ·～·～·～·", "", ""],
        (Action::Relax, 0) => &["  ···☁·～", " ·(－˵－)·", "  ·～·～·", "", ""],
        (Action::Relax, _) => &["  ···☁·", " ·(－˵－)zzZ", "  ·～·～·", "", ""],
    }
}

// --- ノドカ (nodoka) - Peaceful, meadow-flower silhouette ---
fn nodoka_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ✿✿✿✿", " ✿(▽◠▽)✿!", "  ✿～＿～✿", "", ""],
        (MoodLevel::High, _) => &["   ✿✿✿✿", "  ✿(▽◠▽)✿♪", "   ✿～＿～✿", "", ""],
        (MoodLevel::Normal, 0) => &["  ✿✿✿✿", " ✿(・◠・)✿", "  ✿～＿～✿", "", ""],
        (MoodLevel::Normal, _) => &["   ✿✿✿✿", "  ✿(˘◠˘)✿", "   ✿～＿～✿", "", ""],
        (MoodLevel::Low, 0) => &["  ✿✿✿✿", " ✿(￣_￣)✿", "  ✿～＿～✿", "", ""],
        (MoodLevel::Low, _) => &["  ✿✿✿✿", " ✿(￣ ￣)✿", "  ✿～＿～✿", "", ""],
    }
}
fn nodoka_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ✿✿✿✿", " ﾉ(・◠・)✿", "  ✿～＿～✿", "", ""],
        (Action::Talk, _) => &["  ✿✿✿✿", " ✿(・◠・)ﾉ", "  ✿～＿～✿", "", ""],
        (Action::Play, 0) => &[" ♪✿✿✿✿", " ✿(▽◠▽)✿", "  ✿～＿～✿", "", ""],
        (Action::Play, _) => &["  ✿✿✿✿♪", " ✿(▽◠▽)✿", "  ✿～＿～✿", "", ""],
        (Action::Train, 0) => &["  ✿✿✿✿!!", " ✿(益◠益)✿", " ✿～＿＿＿～✿", "", ""],
        (Action::Train, _) => &["!!✿✿✿✿", " ✿(益◠益)✿", " ✿～＿＿＿～✿", "", ""],
        (Action::Relax, 0) => &["  ✿✿✿✿～", " ✿(－◠－)✿", "  ✿～＿～✿", "", ""],
        (Action::Relax, _) => &["  ✿✿✿✿", " ✿(－◠－)zzZ", "  ✿～＿～✿", "", ""],
    }
}

// --- ユメミ (yumemi) - Dreamer, star-bubble silhouette ---
fn yumemi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ☆·:*☆", " ·(▽˶▽)·!", "  ·*:·☆·", "", ""],
        (MoodLevel::High, _) => &["   ☆·:*☆", "  ·(▽˶▽)·♪", "   ·*:·☆·", "", ""],
        (MoodLevel::Normal, 0) => &["  ☆·:*☆", " ·(˶·˶)·", "  ·*:·☆·", "", ""],
        (MoodLevel::Normal, _) => &["   ☆·:*☆", "  ·(˶˘˶)·", "   ·*:·☆·", "", ""],
        (MoodLevel::Low, 0) => &["  ☆·:*☆", " ·(￣_￣)·", "  ·*:·☆·", "", ""],
        (MoodLevel::Low, _) => &["  ☆·:*☆", " ·(￣ ￣)·", "  ·*:·☆·", "", ""],
    }
}
fn yumemi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ☆·:*☆", " ﾉ(˶·˶)·", "  ·*:·☆·", "", ""],
        (Action::Talk, _) => &["  ☆·:*☆", " ·(˶·˶)ﾉ", "  ·*:·☆·", "", ""],
        (Action::Play, 0) => &[" ♪☆·:*☆", " ·(▽˶▽)·", "  ·*:·☆·", "", ""],
        (Action::Play, _) => &["  ☆·:*☆♪", " ·(▽˶▽)·", "  ·*:·☆·", "", ""],
        (Action::Train, 0) => &["  ☆·:*☆!!", " ·(益˶益)·", " ·*:·☆☆·:*·", "", ""],
        (Action::Train, _) => &["!!☆·:*☆", " ·(益˶益)·", " ·*:·☆☆·:*·", "", ""],
        (Action::Relax, 0) => &["  ☆·:*☆～", " ·(－˶－)·", "  ·*:·☆·", "", ""],
        (Action::Relax, _) => &["  ☆·:*☆", " ·(－˶－)zzZ", "  ·*:·☆·", "", ""],
    }
}

// --- ボンヤリ (bonyari) - Absent-minded, hazy oval silhouette ---
fn bonyari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ○○○○", " ○(▽｀▽)○!", "  ○～○～○", "", ""],
        (MoodLevel::High, _) => &["   ○○○○", "  ○(▽｀▽)○♪", "   ○～○～○", "", ""],
        (MoodLevel::Normal, 0) => &["  ○○○○", " ○(´｀)○", "  ○～○～○", "", ""],
        (MoodLevel::Normal, _) => &["   ○○○○", "  ○(｀｀)○", "   ○～○～○", "", ""],
        (MoodLevel::Low, 0) => &["  ○○○○", " ○(￣_￣)○", "  ○～○～○", "", ""],
        (MoodLevel::Low, _) => &["  ○○○○", " ○(￣ ￣)○", "  ○～○～○", "", ""],
    }
}
fn bonyari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ○○○○", " ﾉ(´｀)○", "  ○～○～○", "", ""],
        (Action::Talk, _) => &["  ○○○○", " ○(´｀)ﾉ", "  ○～○～○", "", ""],
        (Action::Play, 0) => &[" ♪○○○○", " ○(▽｀▽)○", "  ○～○～○", "", ""],
        (Action::Play, _) => &["  ○○○○♪", " ○(▽｀▽)○", "  ○～○～○", "", ""],
        (Action::Train, 0) => &["  ○○○○!!", " ○(益｀益)○", " ○～○～○～○", "", ""],
        (Action::Train, _) => &["!!○○○○", " ○(益｀益)○", " ○～○～○～○", "", ""],
        (Action::Relax, 0) => &["  ○○○○～", " ○(－｀－)○", "  ○～○～○", "", ""],
        (Action::Relax, _) => &["  ○○○○", " ○(－｀－)zzZ", "  ○～○～○", "", ""],
    }
}

// --- ヒラタ (hirata) - Flat/wide, low-spread silhouette ---
fn hirata_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ＿＿＿＿＿", " (▽ ▽ ▽)!", " ＿＿＿＿＿＿", "", ""],
        (MoodLevel::High, _) => &["   ＿＿＿＿＿", "  (▽ ▽ ▽)♪", "  ＿＿＿＿＿＿", "", ""],
        (MoodLevel::Normal, 0) => &["  ＿＿＿＿＿", " (・ ・ ・)", " ＿＿＿＿＿＿", "", ""],
        (MoodLevel::Normal, _) => &["   ＿＿＿＿＿", "  (˘ ˘ ˘)", "  ＿＿＿＿＿＿", "", ""],
        (MoodLevel::Low, 0) => &["  ＿＿＿＿＿", " (￣_￣_￣)", " ＿＿＿＿＿＿", "", ""],
        (MoodLevel::Low, _) => &["  ＿＿＿＿＿", " (￣ ￣ ￣)", " ＿＿＿＿＿＿", "", ""],
    }
}
fn hirata_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ＿＿＿＿＿", " ﾉ(・ ・ ・)", " ＿＿＿＿＿＿", "", ""],
        (Action::Talk, _) => &["  ＿＿＿＿＿", " (・ ・ ・)ﾉ", " ＿＿＿＿＿＿", "", ""],
        (Action::Play, 0) => &[" ♪＿＿＿＿＿", " (▽ ▽ ▽)", " ＿＿＿＿＿＿", "", ""],
        (Action::Play, _) => &["  ＿＿＿＿＿♪", " (▽ ▽ ▽)", " ＿＿＿＿＿＿", "", ""],
        (Action::Train, 0) => &["  ＿＿＿＿＿!!", " (益 益 益)", " ＿＿＿＿＿＿＿", "", ""],
        (Action::Train, _) => &["!!＿＿＿＿＿", " (益 益 益)", " ＿＿＿＿＿＿＿", "", ""],
        (Action::Relax, 0) => &["  ＿＿＿＿＿～", " (－ ˘ －)", " ＿＿＿＿＿＿", "", ""],
        (Action::Relax, _) => &["  ＿＿＿＿＿", " (－ ˘ －)zzZ", " ＿＿＿＿＿＿", "", ""],
    }
}

// --- コロリン (kororin) - Tumbling round, ring-roll silhouette ---
fn kororin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ◎◎◎", " ◎(▽◉▽)◎!", "  ◎◎◎◎", "", ""],
        (MoodLevel::High, _) => &["   ◎◎◎", "  ◎(▽◉▽)◎♪", "   ◎◎◎◎", "", ""],
        (MoodLevel::Normal, 0) => &["  ◎◎◎", " ◎(・◉・)◎", "  ◎◎◎◎", "", ""],
        (MoodLevel::Normal, _) => &["   ◎◎◎", "  ◎(˘◉˘)◎", "   ◎◎◎◎", "", ""],
        (MoodLevel::Low, 0) => &["  ◎◎◎", " ◎(￣_￣)◎", "  ◎◎◎◎", "", ""],
        (MoodLevel::Low, _) => &["  ◎◎◎", " ◎(￣ ￣)◎", "  ◎◎◎◎", "", ""],
    }
}
fn kororin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ◎◎◎", " ﾉ(・◉・)◎", "  ◎◎◎◎", "", ""],
        (Action::Talk, _) => &["  ◎◎◎", " ◎(・◉・)ﾉ", "  ◎◎◎◎", "", ""],
        (Action::Play, 0) => &[" ♪◎◎◎", " ◎(▽◉▽)◎", "  ◎◎◎◎", "", ""],
        (Action::Play, _) => &["  ◎◎◎♪", " ◎(▽◉▽)◎", "  ◎◎◎◎", "", ""],
        (Action::Train, 0) => &["  ◎◎◎!!", " ◎(益◉益)◎", " ◎◎◎◎◎◎", "", ""],
        (Action::Train, _) => &["!!◎◎◎", " ◎(益◉益)◎", " ◎◎◎◎◎◎", "", ""],
        (Action::Relax, 0) => &["  ◎◎◎～", " ◎(－◉－)◎", "  ◎◎◎◎", "", ""],
        (Action::Relax, _) => &["  ◎◎◎", " ◎(－◉－)zzZ", "  ◎◎◎◎", "", ""],
    }
}

// --- ムニャ (munya) - Mumbling sleepy, droopy silhouette ---
fn munya_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  zz∩zz", " z(▽ᴖ▽)z!", "  ～～～～", "", ""],
        (MoodLevel::High, _) => &["   zz∩zz", "  z(▽ᴖ▽)z♪", "   ～～～～", "", ""],
        (MoodLevel::Normal, 0) => &["  zz∩zz", " z(˘ᴖ˘)z", "  ～～～～", "", ""],
        (MoodLevel::Normal, _) => &["   zz∩zz", "  z(˘ᴖ˘)z", "   ～～～～", "", ""],
        (MoodLevel::Low, 0) => &["  zz∩zz", " z(￣_￣)z", "  ～～～～", "", ""],
        (MoodLevel::Low, _) => &["  zz∩zz", " z(￣ ￣)z", "  ～～～～", "", ""],
    }
}
fn munya_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  zz∩zz", " ﾉ(˘ᴖ˘)z", "  ～～～～", "", ""],
        (Action::Talk, _) => &["  zz∩zz", " z(˘ᴖ˘)ﾉ", "  ～～～～", "", ""],
        (Action::Play, 0) => &[" ♪zz∩zz", " z(▽ᴖ▽)z", "  ～～～～", "", ""],
        (Action::Play, _) => &["  zz∩zz♪", " z(▽ᴖ▽)z", "  ～～～～", "", ""],
        (Action::Train, 0) => &["  zz∩zz!!", " z(益ᴖ益)z", " ～～～～～～", "", ""],
        (Action::Train, _) => &["!!zz∩zz", " z(益ᴖ益)z", " ～～～～～～", "", ""],
        (Action::Relax, 0) => &["  zz∩zz～", " z(－ᴖ－)z", "  ～～～～", "", ""],
        (Action::Relax, _) => &["  zz∩zz", " z(－ᴖ－)zzZ", "  ～～～～", "", ""],
    }
}

// --- マッタリ (mattari) - Relaxed, hammock silhouette ---
fn mattari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～♪～♪", " ♪(▽‿▽)♪!", "  ～＿＿～", "", ""],
        (MoodLevel::High, _) => &["   ～♪～♪", "  ♪(▽‿▽)♪♪", "   ～＿＿～", "", ""],
        (MoodLevel::Normal, 0) => &["  ～♪～♪", " ♪(˘‿˘)♪", "  ～＿＿～", "", ""],
        (MoodLevel::Normal, _) => &["   ～♪～♪", "  ♪(˘‿˘)♪", "   ～＿＿～", "", ""],
        (MoodLevel::Low, 0) => &["  ～♪～♪", " ♪(￣_￣)♪", "  ～＿＿～", "", ""],
        (MoodLevel::Low, _) => &["  ～♪～♪", " ♪(￣ ￣)♪", "  ～＿＿～", "", ""],
    }
}
fn mattari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～♪～♪", " ﾉ(˘‿˘)♪", "  ～＿＿～", "", ""],
        (Action::Talk, _) => &["  ～♪～♪", " ♪(˘‿˘)ﾉ", "  ～＿＿～", "", ""],
        (Action::Play, 0) => &[" ♪～♪～♪", " ♪(▽‿▽)♪", "  ～＿＿～", "", ""],
        (Action::Play, _) => &["  ～♪～♪♪", " ♪(▽‿▽)♪", "  ～＿＿～", "", ""],
        (Action::Train, 0) => &["  ～♪～♪!!", " ♪(益‿益)♪", " ～＿＿＿＿～", "", ""],
        (Action::Train, _) => &["!!～♪～♪", " ♪(益‿益)♪", " ～＿＿＿＿～", "", ""],
        (Action::Relax, 0) => &["  ～♪～♪～", " ♪(－‿－)♪", "  ～＿＿～", "", ""],
        (Action::Relax, _) => &["  ～♪～♪", " ♪(－‿－)zzZ", "  ～＿＿～", "", ""],
    }
}

// --- ホワワ (howawa) - Fluffy warm, cotton-ball silhouette ---
fn howawa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ふわふわ", " ふ(▽ᵕ▽)ふ!", "  ふわ＿わふ", "", ""],
        (MoodLevel::High, _) => &["   ふわふわ", "  ふ(▽ᵕ▽)ふ♪", "   ふわ＿わふ", "", ""],
        (MoodLevel::Normal, 0) => &["  ふわふわ", " ふ(˶ᵕ˶)ふ", "  ふわ＿わふ", "", ""],
        (MoodLevel::Normal, _) => &["   ふわふわ", "  ふ(˘ᵕ˘)ふ", "   ふわ＿わふ", "", ""],
        (MoodLevel::Low, 0) => &["  ふわふわ", " ふ(￣_￣)ふ", "  ふわ＿わふ", "", ""],
        (MoodLevel::Low, _) => &["  ふわふわ", " ふ(￣ ￣)ふ", "  ふわ＿わふ", "", ""],
    }
}
fn howawa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ふわふわ", " ﾉ(˶ᵕ˶)ふ", "  ふわ＿わふ", "", ""],
        (Action::Talk, _) => &["  ふわふわ", " ふ(˶ᵕ˶)ﾉ", "  ふわ＿わふ", "", ""],
        (Action::Play, 0) => &[" ♪ふわふわ", " ふ(▽ᵕ▽)ふ", "  ふわ＿わふ", "", ""],
        (Action::Play, _) => &["  ふわふわ♪", " ふ(▽ᵕ▽)ふ", "  ふわ＿わふ", "", ""],
        (Action::Train, 0) => &["  ふわふわ!!", " ふ(益ᵕ益)ふ", " ふわ＿＿＿わふ", "", ""],
        (Action::Train, _) => &["!!ふわふわ", " ふ(益ᵕ益)ふ", " ふわ＿＿＿わふ", "", ""],
        (Action::Relax, 0) => &["  ふわふわ～", " ふ(－ᵕ－)ふ", "  ふわ＿わふ", "", ""],
        (Action::Relax, _) => &["  ふわふわ", " ふ(－ᵕ－)zzZ", "  ふわ＿わふ", "", ""],
    }
}

// --- シズカ (shizuka) - Quiet, still-water silhouette ---
fn shizuka_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ─═─═─", " ─(▽‥▽)─!", "  ─═─═──", "", ""],
        (MoodLevel::High, _) => &["   ─═─═─", "  ─(▽‥▽)─♪", "   ─═─═──", "", ""],
        (MoodLevel::Normal, 0) => &["  ─═─═─", " ─(˘‥˘)─", "  ─═─═──", "", ""],
        (MoodLevel::Normal, _) => &["   ─═─═─", "  ─(・‥・)─", "   ─═─═──", "", ""],
        (MoodLevel::Low, 0) => &["  ─═─═─", " ─(￣_￣)─", "  ─═─═──", "", ""],
        (MoodLevel::Low, _) => &["  ─═─═─", " ─(￣ ￣)─", "  ─═─═──", "", ""],
    }
}
fn shizuka_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ─═─═─", " ﾉ(˘‥˘)─", "  ─═─═──", "", ""],
        (Action::Talk, _) => &["  ─═─═─", " ─(˘‥˘)ﾉ", "  ─═─═──", "", ""],
        (Action::Play, 0) => &[" ♪─═─═─", " ─(▽‥▽)─", "  ─═─═──", "", ""],
        (Action::Play, _) => &["  ─═─═─♪", " ─(▽‥▽)─", "  ─═─═──", "", ""],
        (Action::Train, 0) => &["  ─═─═─!!", " ─(益‥益)─", " ─═─═─═──", "", ""],
        (Action::Train, _) => &["!!─═─═─", " ─(益‥益)─", " ─═─═─═──", "", ""],
        (Action::Relax, 0) => &["  ─═─═─～", " ─(－‥－)─", "  ─═─═──", "", ""],
        (Action::Relax, _) => &["  ─═─═─", " ─(－‥－)zzZ", "  ─═─═──", "", ""],
    }
}

// --- モグモグ (mogumogu) - Munching, cheek-puff silhouette ---
fn mogumogu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  もぐもぐ", " も(▽ᗜ▽)も!", "  も～～も", "", ""],
        (MoodLevel::High, _) => &["   もぐもぐ", "  も(▽ᗜ▽)も♪", "   も～～も", "", ""],
        (MoodLevel::Normal, 0) => &["  もぐもぐ", " も(˘ᗜ˘)も", "  も～～も", "", ""],
        (MoodLevel::Normal, _) => &["   もぐもぐ", "  も(・ᗜ・)も", "   も～～も", "", ""],
        (MoodLevel::Low, 0) => &["  もぐもぐ", " も(￣_￣)も", "  も～～も", "", ""],
        (MoodLevel::Low, _) => &["  もぐもぐ", " も(￣ ￣)も", "  も～～も", "", ""],
    }
}
fn mogumogu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  もぐもぐ", " ﾉ(˘ᗜ˘)も", "  も～～も", "", ""],
        (Action::Talk, _) => &["  もぐもぐ", " も(˘ᗜ˘)ﾉ", "  も～～も", "", ""],
        (Action::Play, 0) => &[" ♪もぐもぐ", " も(▽ᗜ▽)も", "  も～～も", "", ""],
        (Action::Play, _) => &["  もぐもぐ♪", " も(▽ᗜ▽)も", "  も～～も", "", ""],
        (Action::Train, 0) => &["  もぐもぐ!!", " も(益ᗜ益)も", " も～～～～も", "", ""],
        (Action::Train, _) => &["!!もぐもぐ", " も(益ᗜ益)も", " も～～～～も", "", ""],
        (Action::Relax, 0) => &["  もぐもぐ～", " も(－ᗜ－)も", "  も～～も", "", ""],
        (Action::Relax, _) => &["  もぐもぐ", " も(－ᗜ－)zzZ", "  も～～も", "", ""],
    }
}

// --- トロン (toron) - Drowsy, melting silhouette ---
fn toron_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  とろ～ん", " と(▽ᴗ▽)ろ!", "  ～とろ～", "", ""],
        (MoodLevel::High, _) => &["   とろ～ん", "  と(▽ᴗ▽)ろ♪", "   ～とろ～", "", ""],
        (MoodLevel::Normal, 0) => &["  とろ～ん", " と(˘ᴗ˘)ろ", "  ～とろ～", "", ""],
        (MoodLevel::Normal, _) => &["   とろ～ん", "  と(˘ᴗ˘)ろ", "   ～とろ～", "", ""],
        (MoodLevel::Low, 0) => &["  とろ～ん", " と(￣_￣)ろ", "  ～とろ～", "", ""],
        (MoodLevel::Low, _) => &["  とろ～ん", " と(￣ ￣)ろ", "  ～とろ～", "", ""],
    }
}
fn toron_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  とろ～ん", " ﾉ(˘ᴗ˘)ろ", "  ～とろ～", "", ""],
        (Action::Talk, _) => &["  とろ～ん", " と(˘ᴗ˘)ﾉ", "  ～とろ～", "", ""],
        (Action::Play, 0) => &[" ♪とろ～ん", " と(▽ᴗ▽)ろ", "  ～とろ～", "", ""],
        (Action::Play, _) => &["  とろ～ん♪", " と(▽ᴗ▽)ろ", "  ～とろ～", "", ""],
        (Action::Train, 0) => &["  とろ～ん!!", " と(益ᴗ益)ろ", " ～とろとろ～", "", ""],
        (Action::Train, _) => &["!!とろ～ん", " と(益ᴗ益)ろ", " ～とろとろ～", "", ""],
        (Action::Relax, 0) => &["  とろ～ん～", " と(－ᴗ－)ろ", "  ～とろ～", "", ""],
        (Action::Relax, _) => &["  とろ～ん", " と(－ᴗ－)zzZ", "  ～とろ～", "", ""],
    }
}

// --- ユッタリ (yuttari) - Leisurely, wide-base silhouette ---
fn yuttari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ゆ～たり", " ゆ(▽ᵔ▽)り!", "  ゆ～＿～り", "", ""],
        (MoodLevel::High, _) => &["   ゆ～たり", "  ゆ(▽ᵔ▽)り♪", "   ゆ～＿～り", "", ""],
        (MoodLevel::Normal, 0) => &["  ゆ～たり", " ゆ(˘ᵔ˘)り", "  ゆ～＿～り", "", ""],
        (MoodLevel::Normal, _) => &["   ゆ～たり", "  ゆ(・ᵔ・)り", "   ゆ～＿～り", "", ""],
        (MoodLevel::Low, 0) => &["  ゆ～たり", " ゆ(￣_￣)り", "  ゆ～＿～り", "", ""],
        (MoodLevel::Low, _) => &["  ゆ～たり", " ゆ(￣ ￣)り", "  ゆ～＿～り", "", ""],
    }
}
fn yuttari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ゆ～たり", " ﾉ(˘ᵔ˘)り", "  ゆ～＿～り", "", ""],
        (Action::Talk, _) => &["  ゆ～たり", " ゆ(˘ᵔ˘)ﾉ", "  ゆ～＿～り", "", ""],
        (Action::Play, 0) => &[" ♪ゆ～たり", " ゆ(▽ᵔ▽)り", "  ゆ～＿～り", "", ""],
        (Action::Play, _) => &["  ゆ～たり♪", " ゆ(▽ᵔ▽)り", "  ゆ～＿～り", "", ""],
        (Action::Train, 0) => &["  ゆ～たり!!", " ゆ(益ᵔ益)り", " ゆ～＿＿＿～り", "", ""],
        (Action::Train, _) => &["!!ゆ～たり", " ゆ(益ᵔ益)り", " ゆ～＿＿＿～り", "", ""],
        (Action::Relax, 0) => &["  ゆ～たり～", " ゆ(－ᵔ－)り", "  ゆ～＿～り", "", ""],
        (Action::Relax, _) => &["  ゆ～たり", " ゆ(－ᵔ－)zzZ", "  ゆ～＿～り", "", ""],
    }
}

// --- ソヨカゼ (soyokaze) - Gentle breeze, leaf-drift silhouette ---
fn soyokaze_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ～♪～♪～", " ♪(▽˃▽)～!", "  ～～＿～～", "", ""],
        (MoodLevel::High, _) => &["   ～♪～♪～", "  ～(▽˃▽)♪♪", "   ～～＿～～", "", ""],
        (MoodLevel::Normal, 0) => &["  ～♪～♪～", " ♪(˘˃˘)～", "  ～～＿～～", "", ""],
        (MoodLevel::Normal, _) => &["   ～♪～♪～", "  ～(・˃・)♪", "   ～～＿～～", "", ""],
        (MoodLevel::Low, 0) => &["  ～♪～♪～", " ♪(￣_￣)～", "  ～～＿～～", "", ""],
        (MoodLevel::Low, _) => &["  ～♪～♪～", " ♪(￣ ￣)～", "  ～～＿～～", "", ""],
    }
}
fn soyokaze_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ～♪～♪～", " ﾉ(˘˃˘)～", "  ～～＿～～", "", ""],
        (Action::Talk, _) => &["  ～♪～♪～", " ～(˘˃˘)ﾉ", "  ～～＿～～", "", ""],
        (Action::Play, 0) => &[" ♪～♪～♪～", " ♪(▽˃▽)～", "  ～～＿～～", "", ""],
        (Action::Play, _) => &["  ～♪～♪～♪", " ～(▽˃▽)♪", "  ～～＿～～", "", ""],
        (Action::Train, 0) => &["  ～♪～♪～!!", " ♪(益˃益)～", " ～～＿＿＿～～", "", ""],
        (Action::Train, _) => &["!!～♪～♪～", " ～(益˃益)♪", " ～～＿＿＿～～", "", ""],
        (Action::Relax, 0) => &["  ～♪～♪～～", " ♪(－˃－)～", "  ～～＿～～", "", ""],
        (Action::Relax, _) => &["  ～♪～♪～", " ♪(－˃－)zzZ", "  ～～＿～～", "", ""],
    }
}

// ============================================================
// WILD TYPE Stage 3 Species
// ============================================================

// --- ヤミノメ (yaminome) - Dark eye ---
fn yaminome_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ◉≪≪≪≪◉", " ψ(▽_▽)ψ!", "  ‡━━━‡", "", ""],
        (MoodLevel::High, _) => &["  ◉≪≪≪≪◉", "  ψ(▽_▽)ψ♪", "   ‡━━━‡", "", ""],
        (MoodLevel::Normal, 0) => &[" ◉≪≪≪≪◉", " ψ(◉_◉)ψ", "  ‡━━━‡", "", ""],
        (MoodLevel::Normal, _) => &["  ◉≪≪≪≪◉", "  ψ(⊙_⊙)ψ", "   ‡━━━‡", "", ""],
        (MoodLevel::Low, 0) => &[" ◉≪≪≪≪◉", " ψ(￣_￣)ψ", "  ‡━━━‡", "", ""],
        (MoodLevel::Low, _) => &[" ◉≪≪≪≪◉", " ψ(￣ ￣)ψ", "  ‡━━━‡", "", ""],
    }
}
fn yaminome_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ◉≪≪≪≪◉", " ﾉ(◉_◉)ψ", "  ‡━━━‡", "", ""],
        (Action::Talk, _) => &[" ◉≪≪≪≪◉", " ψ(◉_◉)ﾉ", "  ‡━━━‡", "", ""],
        (Action::Play, 0) => &["♪◉≪≪≪≪◉", " ψ(▽_▽)ψ", "  ‡━━━‡", "", ""],
        (Action::Play, _) => &[" ◉≪≪≪≪◉♪", " ψ(▽_▽)ψ", "  ‡━━━‡", "", ""],
        (Action::Train, 0) => &[" ◉≪≪≪≪◉!!", " ψ(益_益)ψ", " ‡━━━━━‡", "", ""],
        (Action::Train, _) => &["!!◉≪≪≪≪◉", " ψ(益_益)ψ", " ‡━━━━━‡", "", ""],
        (Action::Relax, 0) => &[" ◉≪≪≪≪◉～", " ψ(－_－)ψ", "  ‡━━━‡", "", ""],
        (Action::Relax, _) => &[" ◉≪≪≪≪◉", " ψ(－_－)zzZ", "  ‡━━━‡", "", ""],
    }
}

// --- オオヌシ (oonushi) - Great lord/boss ---
fn oonushi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ▓▓╬╬▓▓", " ▓(▽皿▽)▓!", " ▓▓╩╩▓▓", "", ""],
        (MoodLevel::High, _) => &["  ▓▓╬╬▓▓", "  ▓(▽皿▽)▓♪", "  ▓▓╩╩▓▓", "", ""],
        (MoodLevel::Normal, 0) => &[" ▓▓╬╬▓▓", " ▓(⊙皿⊙)▓", " ▓▓╩╩▓▓", "", ""],
        (MoodLevel::Normal, _) => &["  ▓▓╬╬▓▓", "  ▓(・皿・)▓", "  ▓▓╩╩▓▓", "", ""],
        (MoodLevel::Low, 0) => &[" ▓▓╬╬▓▓", " ▓(￣_￣)▓", " ▓▓╩╩▓▓", "", ""],
        (MoodLevel::Low, _) => &[" ▓▓╬╬▓▓", " ▓(￣ ￣)▓", " ▓▓╩╩▓▓", "", ""],
    }
}
fn oonushi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ▓▓╬╬▓▓", " ﾉ(⊙皿⊙)▓", " ▓▓╩╩▓▓", "", ""],
        (Action::Talk, _) => &[" ▓▓╬╬▓▓", " ▓(⊙皿⊙)ﾉ", " ▓▓╩╩▓▓", "", ""],
        (Action::Play, 0) => &["♪▓▓╬╬▓▓", " ▓(▽皿▽)▓", " ▓▓╩╩▓▓", "", ""],
        (Action::Play, _) => &[" ▓▓╬╬▓▓♪", " ▓(▽皿▽)▓", " ▓▓╩╩▓▓", "", ""],
        (Action::Train, 0) => &[" ▓▓╬╬▓▓!!", " ▓(益皿益)▓", " ▓▓╩╩╩╩▓▓", "", ""],
        (Action::Train, _) => &["!!▓▓╬╬▓▓", " ▓(益皿益)▓", " ▓▓╩╩╩╩▓▓", "", ""],
        (Action::Relax, 0) => &[" ▓▓╬╬▓▓～", " ▓(－皿－)▓", " ▓▓╩╩▓▓", "", ""],
        (Action::Relax, _) => &[" ▓▓╬╬▓▓", " ▓(－皿－)zzZ", " ▓▓╩╩▓▓", "", ""],
    }
}

// --- バケモノ (bakemono) - Monster ---
fn bakemono_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" †×⌇×⌇×†", " ×(▽益▽)×!", "  ×⌇⌇⌇×", "", ""],
        (MoodLevel::High, _) => &["  †×⌇×⌇×†", "  ×(▽益▽)×♪", "   ×⌇⌇⌇×", "", ""],
        (MoodLevel::Normal, 0) => &[" †×⌇×⌇×†", " ×(⊙益⊙)×", "  ×⌇⌇⌇×", "", ""],
        (MoodLevel::Normal, _) => &["  †×⌇×⌇×†", "  ×(・益・)×", "   ×⌇⌇⌇×", "", ""],
        (MoodLevel::Low, 0) => &[" †×⌇×⌇×†", " ×(￣_￣)×", "  ×⌇⌇⌇×", "", ""],
        (MoodLevel::Low, _) => &[" †×⌇×⌇×†", " ×(￣ ￣)×", "  ×⌇⌇⌇×", "", ""],
    }
}
fn bakemono_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" †×⌇×⌇×†", " ﾉ(⊙益⊙)×", "  ×⌇⌇⌇×", "", ""],
        (Action::Talk, _) => &[" †×⌇×⌇×†", " ×(⊙益⊙)ﾉ", "  ×⌇⌇⌇×", "", ""],
        (Action::Play, 0) => &["♪†×⌇×⌇×†", " ×(▽益▽)×", "  ×⌇⌇⌇×", "", ""],
        (Action::Play, _) => &[" †×⌇×⌇×†♪", " ×(▽益▽)×", "  ×⌇⌇⌇×", "", ""],
        (Action::Train, 0) => &[" †×⌇×⌇×†!!", " ×(益益益)×", " ×⌇⌇⌇⌇⌇×", "", ""],
        (Action::Train, _) => &["!!†×⌇×⌇×†", " ×(益益益)×", " ×⌇⌇⌇⌇⌇×", "", ""],
        (Action::Relax, 0) => &[" †×⌇×⌇×†～", " ×(－益－)×", "  ×⌇⌇⌇×", "", ""],
        (Action::Relax, _) => &[" †×⌇×⌇×†", " ×(－益－)zzZ", "  ×⌇⌇⌇×", "", ""],
    }
}

// --- ユウレイ (yuurei) - Ghost ---
fn yuurei_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ░░░░░", " ░(▽△▽)░!", "  ░〜〜〜", "", ""],
        (MoodLevel::High, _) => &["   ░░░░░", "  ░(▽△▽)░♪", "   ░〜〜〜", "", ""],
        (MoodLevel::Normal, 0) => &["  ░░░░░", " ░(⊙△⊙)░", "  ░〜〜〜", "", ""],
        (MoodLevel::Normal, _) => &["   ░░░░░", "  ░(・△・)░", "   ░〜〜〜", "", ""],
        (MoodLevel::Low, 0) => &["  ░░░░░", " ░(￣_￣)░", "  ░〜〜〜", "", ""],
        (MoodLevel::Low, _) => &["  ░░░░░", " ░(￣ ￣)░", "  ░〜〜〜", "", ""],
    }
}
fn yuurei_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ░░░░░", " ﾉ(⊙△⊙)░", "  ░〜〜〜", "", ""],
        (Action::Talk, _) => &["  ░░░░░", " ░(⊙△⊙)ﾉ", "  ░〜〜〜", "", ""],
        (Action::Play, 0) => &[" ♪░░░░░", " ░(▽△▽)░", "  ░〜〜〜", "", ""],
        (Action::Play, _) => &["  ░░░░░♪", " ░(▽△▽)░", "  ░〜〜〜", "", ""],
        (Action::Train, 0) => &["  ░░░░░!!", " ░(益△益)░", " ░〜〜〜〜〜", "", ""],
        (Action::Train, _) => &["!!░░░░░", " ░(益△益)░", " ░〜〜〜〜〜", "", ""],
        (Action::Relax, 0) => &["  ░░░░░～", " ░(－△－)░", "  ░〜〜〜", "", ""],
        (Action::Relax, _) => &["  ░░░░░", " ░(－△－)zzZ", "  ░〜〜〜", "", ""],
    }
}

// --- ヤセイジ (yaseiji) - Wild child ---
fn yaseiji_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ꝋ≫≪ꝋ≫≪", " ≪(▽ω▽)≫!", "  ꝋ┻┻ꝋ", "", ""],
        (MoodLevel::High, _) => &["  ꝋ≫≪ꝋ≫≪", "  ≪(▽ω▽)≫♪", "   ꝋ┻┻ꝋ", "", ""],
        (MoodLevel::Normal, 0) => &[" ꝋ≫≪ꝋ≫≪", " ≪(⊙ω⊙)≫", "  ꝋ┻┻ꝋ", "", ""],
        (MoodLevel::Normal, _) => &["  ꝋ≫≪ꝋ≫≪", "  ≪(・ω・)≫", "   ꝋ┻┻ꝋ", "", ""],
        (MoodLevel::Low, 0) => &[" ꝋ≫≪ꝋ≫≪", " ≪(￣_￣)≫", "  ꝋ┻┻ꝋ", "", ""],
        (MoodLevel::Low, _) => &[" ꝋ≫≪ꝋ≫≪", " ≪(￣ ￣)≫", "  ꝋ┻┻ꝋ", "", ""],
    }
}
fn yaseiji_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ꝋ≫≪ꝋ≫≪", " ﾉ(⊙ω⊙)≫", "  ꝋ┻┻ꝋ", "", ""],
        (Action::Talk, _) => &[" ꝋ≫≪ꝋ≫≪", " ≪(⊙ω⊙)ﾉ", "  ꝋ┻┻ꝋ", "", ""],
        (Action::Play, 0) => &["♪ꝋ≫≪ꝋ≫≪", " ≪(▽ω▽)≫", "  ꝋ┻┻ꝋ", "", ""],
        (Action::Play, _) => &[" ꝋ≫≪ꝋ≫≪♪", " ≪(▽ω▽)≫", "  ꝋ┻┻ꝋ", "", ""],
        (Action::Train, 0) => &[" ꝋ≫≪ꝋ≫≪!!", " ≪(益ω益)≫", " ꝋ┻┻┻┻ꝋ", "", ""],
        (Action::Train, _) => &["!!ꝋ≫≪ꝋ≫≪", " ≪(益ω益)≫", " ꝋ┻┻┻┻ꝋ", "", ""],
        (Action::Relax, 0) => &[" ꝋ≫≪ꝋ≫≪～", " ≪(－ω－)≫", "  ꝋ┻┻ꝋ", "", ""],
        (Action::Relax, _) => &[" ꝋ≫≪ꝋ≫≪", " ≪(－ω－)zzZ", "  ꝋ┻┻ꝋ", "", ""],
    }
}

// --- シンエン (shinen) - Deep abyss ---
fn shinen_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ▒▒▒▒▒▒", " ▒(▽∇▽)▒!", "  ▒▒▒▒▒", "", ""],
        (MoodLevel::High, _) => &["  ▒▒▒▒▒▒", "  ▒(▽∇▽)▒♪", "   ▒▒▒▒▒", "", ""],
        (MoodLevel::Normal, 0) => &[" ▒▒▒▒▒▒", " ▒(⊙∇⊙)▒", "  ▒▒▒▒▒", "", ""],
        (MoodLevel::Normal, _) => &["  ▒▒▒▒▒▒", "  ▒(・∇・)▒", "   ▒▒▒▒▒", "", ""],
        (MoodLevel::Low, 0) => &[" ▒▒▒▒▒▒", " ▒(￣_￣)▒", "  ▒▒▒▒▒", "", ""],
        (MoodLevel::Low, _) => &[" ▒▒▒▒▒▒", " ▒(￣ ￣)▒", "  ▒▒▒▒▒", "", ""],
    }
}
fn shinen_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ▒▒▒▒▒▒", " ﾉ(⊙∇⊙)▒", "  ▒▒▒▒▒", "", ""],
        (Action::Talk, _) => &[" ▒▒▒▒▒▒", " ▒(⊙∇⊙)ﾉ", "  ▒▒▒▒▒", "", ""],
        (Action::Play, 0) => &["♪▒▒▒▒▒▒", " ▒(▽∇▽)▒", "  ▒▒▒▒▒", "", ""],
        (Action::Play, _) => &[" ▒▒▒▒▒▒♪", " ▒(▽∇▽)▒", "  ▒▒▒▒▒", "", ""],
        (Action::Train, 0) => &[" ▒▒▒▒▒▒!!", " ▒(益∇益)▒", " ▒▒▒▒▒▒▒", "", ""],
        (Action::Train, _) => &["!!▒▒▒▒▒▒", " ▒(益∇益)▒", " ▒▒▒▒▒▒▒", "", ""],
        (Action::Relax, 0) => &[" ▒▒▒▒▒▒～", " ▒(－∇－)▒", "  ▒▒▒▒▒", "", ""],
        (Action::Relax, _) => &[" ▒▒▒▒▒▒", " ▒(－∇－)zzZ", "  ▒▒▒▒▒", "", ""],
    }
}

// --- ノラクロ (norakuro) - Stray black ---
fn norakuro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" █▀▀▀▀█", " █(▽◆▽)█!", "  █▄▄▄█", "", ""],
        (MoodLevel::High, _) => &["  █▀▀▀▀█", "  █(▽◆▽)█♪", "   █▄▄▄█", "", ""],
        (MoodLevel::Normal, 0) => &[" █▀▀▀▀█", " █(⊙◆⊙)█", "  █▄▄▄█", "", ""],
        (MoodLevel::Normal, _) => &["  █▀▀▀▀█", "  █(・◆・)█", "   █▄▄▄█", "", ""],
        (MoodLevel::Low, 0) => &[" █▀▀▀▀█", " █(￣_￣)█", "  █▄▄▄█", "", ""],
        (MoodLevel::Low, _) => &[" █▀▀▀▀█", " █(￣ ￣)█", "  █▄▄▄█", "", ""],
    }
}
fn norakuro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" █▀▀▀▀█", " ﾉ(⊙◆⊙)█", "  █▄▄▄█", "", ""],
        (Action::Talk, _) => &[" █▀▀▀▀█", " █(⊙◆⊙)ﾉ", "  █▄▄▄█", "", ""],
        (Action::Play, 0) => &["♪█▀▀▀▀█", " █(▽◆▽)█", "  █▄▄▄█", "", ""],
        (Action::Play, _) => &[" █▀▀▀▀█♪", " █(▽◆▽)█", "  █▄▄▄█", "", ""],
        (Action::Train, 0) => &[" █▀▀▀▀█!!", " █(益◆益)█", " █▄▄▄▄▄█", "", ""],
        (Action::Train, _) => &["!!█▀▀▀▀█", " █(益◆益)█", " █▄▄▄▄▄█", "", ""],
        (Action::Relax, 0) => &[" █▀▀▀▀█～", " █(－◆－)█", "  █▄▄▄█", "", ""],
        (Action::Relax, _) => &[" █▀▀▀▀█", " █(－◆－)zzZ", "  █▄▄▄█", "", ""],
    }
}

// --- モノノケ (mononoke) - Spirit/specter ---
fn mononoke_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ⌇⌇⌇⌇⌇⌇", " ⌇(▽霊▽)⌇!", "  ⌇⌇⌇⌇⌇", "", ""],
        (MoodLevel::High, _) => &["  ⌇⌇⌇⌇⌇⌇", "  ⌇(▽霊▽)⌇♪", "   ⌇⌇⌇⌇⌇", "", ""],
        (MoodLevel::Normal, 0) => &[" ⌇⌇⌇⌇⌇⌇", " ⌇(⊙霊⊙)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
        (MoodLevel::Normal, _) => &["  ⌇⌇⌇⌇⌇⌇", "  ⌇(・霊・)⌇", "   ⌇⌇⌇⌇⌇", "", ""],
        (MoodLevel::Low, 0) => &[" ⌇⌇⌇⌇⌇⌇", " ⌇(￣_￣)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
        (MoodLevel::Low, _) => &[" ⌇⌇⌇⌇⌇⌇", " ⌇(￣ ￣)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
    }
}
fn mononoke_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ⌇⌇⌇⌇⌇⌇", " ﾉ(⊙霊⊙)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
        (Action::Talk, _) => &[" ⌇⌇⌇⌇⌇⌇", " ⌇(⊙霊⊙)ﾉ", "  ⌇⌇⌇⌇⌇", "", ""],
        (Action::Play, 0) => &["♪⌇⌇⌇⌇⌇⌇", " ⌇(▽霊▽)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
        (Action::Play, _) => &[" ⌇⌇⌇⌇⌇⌇♪", " ⌇(▽霊▽)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
        (Action::Train, 0) => &[" ⌇⌇⌇⌇⌇⌇!!", " ⌇(益霊益)⌇", " ⌇⌇⌇⌇⌇⌇⌇", "", ""],
        (Action::Train, _) => &["!!⌇⌇⌇⌇⌇⌇", " ⌇(益霊益)⌇", " ⌇⌇⌇⌇⌇⌇⌇", "", ""],
        (Action::Relax, 0) => &[" ⌇⌇⌇⌇⌇⌇～", " ⌇(－霊－)⌇", "  ⌇⌇⌇⌇⌇", "", ""],
        (Action::Relax, _) => &[" ⌇⌇⌇⌇⌇⌇", " ⌇(－霊－)zzZ", "  ⌇⌇⌇⌇⌇", "", ""],
    }
}

// --- クライ (kurai) - Dark/gloomy ---
fn kurai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ☽☽☽☽☽", " ☽(▽闇▽)☽!", "  ☽━━━☽", "", ""],
        (MoodLevel::High, _) => &["  ☽☽☽☽☽", "  ☽(▽闇▽)☽♪", "   ☽━━━☽", "", ""],
        (MoodLevel::Normal, 0) => &[" ☽☽☽☽☽", " ☽(⊙闇⊙)☽", "  ☽━━━☽", "", ""],
        (MoodLevel::Normal, _) => &["  ☽☽☽☽☽", "  ☽(・闇・)☽", "   ☽━━━☽", "", ""],
        (MoodLevel::Low, 0) => &[" ☽☽☽☽☽", " ☽(￣_￣)☽", "  ☽━━━☽", "", ""],
        (MoodLevel::Low, _) => &[" ☽☽☽☽☽", " ☽(￣ ￣)☽", "  ☽━━━☽", "", ""],
    }
}
fn kurai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ☽☽☽☽☽", " ﾉ(⊙闇⊙)☽", "  ☽━━━☽", "", ""],
        (Action::Talk, _) => &[" ☽☽☽☽☽", " ☽(⊙闇⊙)ﾉ", "  ☽━━━☽", "", ""],
        (Action::Play, 0) => &["♪☽☽☽☽☽", " ☽(▽闇▽)☽", "  ☽━━━☽", "", ""],
        (Action::Play, _) => &[" ☽☽☽☽☽♪", " ☽(▽闇▽)☽", "  ☽━━━☽", "", ""],
        (Action::Train, 0) => &[" ☽☽☽☽☽!!", " ☽(益闇益)☽", " ☽━━━━━☽", "", ""],
        (Action::Train, _) => &["!!☽☽☽☽☽", " ☽(益闇益)☽", " ☽━━━━━☽", "", ""],
        (Action::Relax, 0) => &[" ☽☽☽☽☽～", " ☽(－闇－)☽", "  ☽━━━☽", "", ""],
        (Action::Relax, _) => &[" ☽☽☽☽☽", " ☽(－闇－)zzZ", "  ☽━━━☽", "", ""],
    }
}

// --- アヤシイ (ayashii) - Suspicious/mysterious ---
fn ayashii_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ？？？？？", " ？(▽？▽)？!", "  ？＿＿？", "", ""],
        (MoodLevel::High, _) => &["  ？？？？？", "  ？(▽？▽)？♪", "   ？＿＿？", "", ""],
        (MoodLevel::Normal, 0) => &[" ？？？？？", " ？(⊙？⊙)？", "  ？＿＿？", "", ""],
        (MoodLevel::Normal, _) => &["  ？？？？？", "  ？(・？・)？", "   ？＿＿？", "", ""],
        (MoodLevel::Low, 0) => &[" ？？？？？", " ？(￣_￣)？", "  ？＿＿？", "", ""],
        (MoodLevel::Low, _) => &[" ？？？？？", " ？(￣ ￣)？", "  ？＿＿？", "", ""],
    }
}
fn ayashii_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ？？？？？", " ﾉ(⊙？⊙)？", "  ？＿＿？", "", ""],
        (Action::Talk, _) => &[" ？？？？？", " ？(⊙？⊙)ﾉ", "  ？＿＿？", "", ""],
        (Action::Play, 0) => &["♪？？？？？", " ？(▽？▽)？", "  ？＿＿？", "", ""],
        (Action::Play, _) => &[" ？？？？？♪", " ？(▽？▽)？", "  ？＿＿？", "", ""],
        (Action::Train, 0) => &[" ？？？？？!!", " ？(益？益)？", " ？＿＿＿＿？", "", ""],
        (Action::Train, _) => &["!!？？？？？", " ？(益？益)？", " ？＿＿＿＿？", "", ""],
        (Action::Relax, 0) => &[" ？？？？？～", " ？(－？－)？", "  ？＿＿？", "", ""],
        (Action::Relax, _) => &[" ？？？？？", " ？(－？－)zzZ", "  ？＿＿？", "", ""],
    }
}

// --- ムジナ (mujina) - Badger/shapeshifter ---
fn mujina_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ≋≋≋≋≋≋", " ≋(▽化▽)≋!", "  ≋〓〓≋", "", ""],
        (MoodLevel::High, _) => &["  ≋≋≋≋≋≋", "  ≋(▽化▽)≋♪", "   ≋〓〓≋", "", ""],
        (MoodLevel::Normal, 0) => &[" ≋≋≋≋≋≋", " ≋(⊙化⊙)≋", "  ≋〓〓≋", "", ""],
        (MoodLevel::Normal, _) => &["  ≋≋≋≋≋≋", "  ≋(・化・)≋", "   ≋〓〓≋", "", ""],
        (MoodLevel::Low, 0) => &[" ≋≋≋≋≋≋", " ≋(￣_￣)≋", "  ≋〓〓≋", "", ""],
        (MoodLevel::Low, _) => &[" ≋≋≋≋≋≋", " ≋(￣ ￣)≋", "  ≋〓〓≋", "", ""],
    }
}
fn mujina_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ≋≋≋≋≋≋", " ﾉ(⊙化⊙)≋", "  ≋〓〓≋", "", ""],
        (Action::Talk, _) => &[" ≋≋≋≋≋≋", " ≋(⊙化⊙)ﾉ", "  ≋〓〓≋", "", ""],
        (Action::Play, 0) => &["♪≋≋≋≋≋≋", " ≋(▽化▽)≋", "  ≋〓〓≋", "", ""],
        (Action::Play, _) => &[" ≋≋≋≋≋≋♪", " ≋(▽化▽)≋", "  ≋〓〓≋", "", ""],
        (Action::Train, 0) => &[" ≋≋≋≋≋≋!!", " ≋(益化益)≋", " ≋〓〓〓〓≋", "", ""],
        (Action::Train, _) => &["!!≋≋≋≋≋≋", " ≋(益化益)≋", " ≋〓〓〓〓≋", "", ""],
        (Action::Relax, 0) => &[" ≋≋≋≋≋≋～", " ≋(－化－)≋", "  ≋〓〓≋", "", ""],
        (Action::Relax, _) => &[" ≋≋≋≋≋≋", " ≋(－化－)zzZ", "  ≋〓〓≋", "", ""],
    }
}

// --- ヌエ (nue) - Chimera monster ---
fn nue_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ⚔⚔⚔⚔⚔", " ⚔(▽鵺▽)⚔!", "  ⚔╋╋⚔", "", ""],
        (MoodLevel::High, _) => &["  ⚔⚔⚔⚔⚔", "  ⚔(▽鵺▽)⚔♪", "   ⚔╋╋⚔", "", ""],
        (MoodLevel::Normal, 0) => &[" ⚔⚔⚔⚔⚔", " ⚔(⊙鵺⊙)⚔", "  ⚔╋╋⚔", "", ""],
        (MoodLevel::Normal, _) => &["  ⚔⚔⚔⚔⚔", "  ⚔(・鵺・)⚔", "   ⚔╋╋⚔", "", ""],
        (MoodLevel::Low, 0) => &[" ⚔⚔⚔⚔⚔", " ⚔(￣_￣)⚔", "  ⚔╋╋⚔", "", ""],
        (MoodLevel::Low, _) => &[" ⚔⚔⚔⚔⚔", " ⚔(￣ ￣)⚔", "  ⚔╋╋⚔", "", ""],
    }
}
fn nue_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ⚔⚔⚔⚔⚔", " ﾉ(⊙鵺⊙)⚔", "  ⚔╋╋⚔", "", ""],
        (Action::Talk, _) => &[" ⚔⚔⚔⚔⚔", " ⚔(⊙鵺⊙)ﾉ", "  ⚔╋╋⚔", "", ""],
        (Action::Play, 0) => &["♪⚔⚔⚔⚔⚔", " ⚔(▽鵺▽)⚔", "  ⚔╋╋⚔", "", ""],
        (Action::Play, _) => &[" ⚔⚔⚔⚔⚔♪", " ⚔(▽鵺▽)⚔", "  ⚔╋╋⚔", "", ""],
        (Action::Train, 0) => &[" ⚔⚔⚔⚔⚔!!", " ⚔(益鵺益)⚔", " ⚔╋╋╋╋⚔", "", ""],
        (Action::Train, _) => &["!!⚔⚔⚔⚔⚔", " ⚔(益鵺益)⚔", " ⚔╋╋╋╋⚔", "", ""],
        (Action::Relax, 0) => &[" ⚔⚔⚔⚔⚔～", " ⚔(－鵺－)⚔", "  ⚔╋╋⚔", "", ""],
        (Action::Relax, _) => &[" ⚔⚔⚔⚔⚔", " ⚔(－鵺－)zzZ", "  ⚔╋╋⚔", "", ""],
    }
}

// --- カマイタチ (kamaitachi) - Sickle weasel ---
fn kamaitachi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" 〻〻〻〻〻", " 〻(▽刃▽)〻!", "  〻⌒⌒〻", "", ""],
        (MoodLevel::High, _) => &["  〻〻〻〻〻", "  〻(▽刃▽)〻♪", "   〻⌒⌒〻", "", ""],
        (MoodLevel::Normal, 0) => &[" 〻〻〻〻〻", " 〻(⊙刃⊙)〻", "  〻⌒⌒〻", "", ""],
        (MoodLevel::Normal, _) => &["  〻〻〻〻〻", "  〻(・刃・)〻", "   〻⌒⌒〻", "", ""],
        (MoodLevel::Low, 0) => &[" 〻〻〻〻〻", " 〻(￣_￣)〻", "  〻⌒⌒〻", "", ""],
        (MoodLevel::Low, _) => &[" 〻〻〻〻〻", " 〻(￣ ￣)〻", "  〻⌒⌒〻", "", ""],
    }
}
fn kamaitachi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" 〻〻〻〻〻", " ﾉ(⊙刃⊙)〻", "  〻⌒⌒〻", "", ""],
        (Action::Talk, _) => &[" 〻〻〻〻〻", " 〻(⊙刃⊙)ﾉ", "  〻⌒⌒〻", "", ""],
        (Action::Play, 0) => &["♪〻〻〻〻〻", " 〻(▽刃▽)〻", "  〻⌒⌒〻", "", ""],
        (Action::Play, _) => &[" 〻〻〻〻〻♪", " 〻(▽刃▽)〻", "  〻⌒⌒〻", "", ""],
        (Action::Train, 0) => &[" 〻〻〻〻〻!!", " 〻(益刃益)〻", " 〻⌒⌒⌒⌒〻", "", ""],
        (Action::Train, _) => &["!!〻〻〻〻〻", " 〻(益刃益)〻", " 〻⌒⌒⌒⌒〻", "", ""],
        (Action::Relax, 0) => &[" 〻〻〻〻〻～", " 〻(－刃－)〻", "  〻⌒⌒〻", "", ""],
        (Action::Relax, _) => &[" 〻〻〻〻〻", " 〻(－刃－)zzZ", "  〻⌒⌒〻", "", ""],
    }
}

// --- ドロドロ (dorodoro) - Muddy/melting ---
fn dorodoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ∿∿∿∿∿∿", " ∿(▽泥▽)∿!", "  ∿∿∿∿∿", "", ""],
        (MoodLevel::High, _) => &["  ∿∿∿∿∿∿", "  ∿(▽泥▽)∿♪", "   ∿∿∿∿∿", "", ""],
        (MoodLevel::Normal, 0) => &[" ∿∿∿∿∿∿", " ∿(⊙泥⊙)∿", "  ∿∿∿∿∿", "", ""],
        (MoodLevel::Normal, _) => &["  ∿∿∿∿∿∿", "  ∿(・泥・)∿", "   ∿∿∿∿∿", "", ""],
        (MoodLevel::Low, 0) => &[" ∿∿∿∿∿∿", " ∿(￣_￣)∿", "  ∿∿∿∿∿", "", ""],
        (MoodLevel::Low, _) => &[" ∿∿∿∿∿∿", " ∿(￣ ￣)∿", "  ∿∿∿∿∿", "", ""],
    }
}
fn dorodoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ∿∿∿∿∿∿", " ﾉ(⊙泥⊙)∿", "  ∿∿∿∿∿", "", ""],
        (Action::Talk, _) => &[" ∿∿∿∿∿∿", " ∿(⊙泥⊙)ﾉ", "  ∿∿∿∿∿", "", ""],
        (Action::Play, 0) => &["♪∿∿∿∿∿∿", " ∿(▽泥▽)∿", "  ∿∿∿∿∿", "", ""],
        (Action::Play, _) => &[" ∿∿∿∿∿∿♪", " ∿(▽泥▽)∿", "  ∿∿∿∿∿", "", ""],
        (Action::Train, 0) => &[" ∿∿∿∿∿∿!!", " ∿(益泥益)∿", " ∿∿∿∿∿∿∿", "", ""],
        (Action::Train, _) => &["!!∿∿∿∿∿∿", " ∿(益泥益)∿", " ∿∿∿∿∿∿∿", "", ""],
        (Action::Relax, 0) => &[" ∿∿∿∿∿∿～", " ∿(－泥－)∿", "  ∿∿∿∿∿", "", ""],
        (Action::Relax, _) => &[" ∿∿∿∿∿∿", " ∿(－泥－)zzZ", "  ∿∿∿∿∿", "", ""],
    }
}

// --- ヒノタマ (hinotama) - Will-o-wisp/fireball ---
fn hinotama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" 炎炎炎炎炎", " 炎(▽火▽)炎!", "  炎～～炎", "", ""],
        (MoodLevel::High, _) => &["  炎炎炎炎炎", "  炎(▽火▽)炎♪", "   炎～～炎", "", ""],
        (MoodLevel::Normal, 0) => &[" 炎炎炎炎炎", " 炎(⊙火⊙)炎", "  炎～～炎", "", ""],
        (MoodLevel::Normal, _) => &["  炎炎炎炎炎", "  炎(・火・)炎", "   炎～～炎", "", ""],
        (MoodLevel::Low, 0) => &[" 炎炎炎炎炎", " 炎(￣_￣)炎", "  炎～～炎", "", ""],
        (MoodLevel::Low, _) => &[" 炎炎炎炎炎", " 炎(￣ ￣)炎", "  炎～～炎", "", ""],
    }
}
fn hinotama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" 炎炎炎炎炎", " ﾉ(⊙火⊙)炎", "  炎～～炎", "", ""],
        (Action::Talk, _) => &[" 炎炎炎炎炎", " 炎(⊙火⊙)ﾉ", "  炎～～炎", "", ""],
        (Action::Play, 0) => &["♪炎炎炎炎炎", " 炎(▽火▽)炎", "  炎～～炎", "", ""],
        (Action::Play, _) => &[" 炎炎炎炎炎♪", " 炎(▽火▽)炎", "  炎～～炎", "", ""],
        (Action::Train, 0) => &[" 炎炎炎炎炎!!", " 炎(益火益)炎", " 炎～～～～炎", "", ""],
        (Action::Train, _) => &["!!炎炎炎炎炎", " 炎(益火益)炎", " 炎～～～～炎", "", ""],
        (Action::Relax, 0) => &[" 炎炎炎炎炎～", " 炎(－火－)炎", "  炎～～炎", "", ""],
        (Action::Relax, _) => &[" 炎炎炎炎炎", " 炎(－火－)zzZ", "  炎～～炎", "", ""],
    }
}

// --- フルエ (furue) - Trembling ---
fn furue_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ！！！！！", " ！(▽震▽)！!", "  ！！！！", "", ""],
        (MoodLevel::High, _) => &["  ！！！！！", "  ！(▽震▽)！♪", "   ！！！！", "", ""],
        (MoodLevel::Normal, 0) => &[" ！！！！！", " ！(⊙震⊙)！", "  ！！！！", "", ""],
        (MoodLevel::Normal, _) => &["  ！！！！！", "  ！(・震・)！", "   ！！！！", "", ""],
        (MoodLevel::Low, 0) => &[" ！！！！！", " ！(￣_￣)！", "  ！！！！", "", ""],
        (MoodLevel::Low, _) => &[" ！！！！！", " ！(￣ ￣)！", "  ！！！！", "", ""],
    }
}
fn furue_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ！！！！！", " ﾉ(⊙震⊙)！", "  ！！！！", "", ""],
        (Action::Talk, _) => &[" ！！！！！", " ！(⊙震⊙)ﾉ", "  ！！！！", "", ""],
        (Action::Play, 0) => &["♪！！！！！", " ！(▽震▽)！", "  ！！！！", "", ""],
        (Action::Play, _) => &[" ！！！！！♪", " ！(▽震▽)！", "  ！！！！", "", ""],
        (Action::Train, 0) => &[" ！！！！！!!", " ！(益震益)！", " ！！！！！！", "", ""],
        (Action::Train, _) => &["!!！！！！！", " ！(益震益)！", " ！！！！！！", "", ""],
        (Action::Relax, 0) => &[" ！！！！！～", " ！(－震－)！", "  ！！！！", "", ""],
        (Action::Relax, _) => &[" ！！！！！", " ！(－震－)zzZ", "  ！！！！", "", ""],
    }
}

// --- ケダマ (kedama) - Fur ball ---
fn kedama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(▽毛▽)ｼｬ!", "  ﾓｼｬ＿ｼｬ", "", ""],
        (MoodLevel::High, _) => &["  ﾓｼｬﾓｼｬﾓｼｬ", "  ﾓ(▽毛▽)ｼｬ♪", "   ﾓｼｬ＿ｼｬ", "", ""],
        (MoodLevel::Normal, 0) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(⊙毛⊙)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (MoodLevel::Normal, _) => &["  ﾓｼｬﾓｼｬﾓｼｬ", "  ﾓ(・毛・)ｼｬ", "   ﾓｼｬ＿ｼｬ", "", ""],
        (MoodLevel::Low, 0) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(￣_￣)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (MoodLevel::Low, _) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(￣ ￣)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
    }
}
fn kedama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾉ(⊙毛⊙)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (Action::Talk, _) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(⊙毛⊙)ﾉ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (Action::Play, 0) => &["♪ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(▽毛▽)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (Action::Play, _) => &[" ﾓｼｬﾓｼｬﾓｼｬ♪", " ﾓ(▽毛▽)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (Action::Train, 0) => &[" ﾓｼｬﾓｼｬﾓｼｬ!!", " ﾓ(益毛益)ｼｬ", " ﾓｼｬ＿＿＿ｼｬ", "", ""],
        (Action::Train, _) => &["!!ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(益毛益)ｼｬ", " ﾓｼｬ＿＿＿ｼｬ", "", ""],
        (Action::Relax, 0) => &[" ﾓｼｬﾓｼｬﾓｼｬ～", " ﾓ(－毛－)ｼｬ", "  ﾓｼｬ＿ｼｬ", "", ""],
        (Action::Relax, _) => &[" ﾓｼｬﾓｼｬﾓｼｬ", " ﾓ(－毛－)zzZ", "  ﾓｼｬ＿ｼｬ", "", ""],
    }
}

// --- シノビ (shinobi) - Ninja/stealth ---
fn shinobi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" 卍卍卍卍卍", " 卍(▽忍▽)卍!", "  卍━━卍", "", ""],
        (MoodLevel::High, _) => &["  卍卍卍卍卍", "  卍(▽忍▽)卍♪", "   卍━━卍", "", ""],
        (MoodLevel::Normal, 0) => &[" 卍卍卍卍卍", " 卍(⊙忍⊙)卍", "  卍━━卍", "", ""],
        (MoodLevel::Normal, _) => &["  卍卍卍卍卍", "  卍(・忍・)卍", "   卍━━卍", "", ""],
        (MoodLevel::Low, 0) => &[" 卍卍卍卍卍", " 卍(￣_￣)卍", "  卍━━卍", "", ""],
        (MoodLevel::Low, _) => &[" 卍卍卍卍卍", " 卍(￣ ￣)卍", "  卍━━卍", "", ""],
    }
}
fn shinobi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" 卍卍卍卍卍", " ﾉ(⊙忍⊙)卍", "  卍━━卍", "", ""],
        (Action::Talk, _) => &[" 卍卍卍卍卍", " 卍(⊙忍⊙)ﾉ", "  卍━━卍", "", ""],
        (Action::Play, 0) => &["♪卍卍卍卍卍", " 卍(▽忍▽)卍", "  卍━━卍", "", ""],
        (Action::Play, _) => &[" 卍卍卍卍卍♪", " 卍(▽忍▽)卍", "  卍━━卍", "", ""],
        (Action::Train, 0) => &[" 卍卍卍卍卍!!", " 卍(益忍益)卍", " 卍━━━━卍", "", ""],
        (Action::Train, _) => &["!!卍卍卍卍卍", " 卍(益忍益)卍", " 卍━━━━卍", "", ""],
        (Action::Relax, 0) => &[" 卍卍卍卍卍～", " 卍(－忍－)卍", "  卍━━卍", "", ""],
        (Action::Relax, _) => &[" 卍卍卍卍卍", " 卍(－忍－)zzZ", "  卍━━卍", "", ""],
    }
}

// --- ジゴク (jigoku) - Hell ---
fn jigoku_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" 獄獄獄獄獄", " 獄(▽地▽)獄!", "  獄╬╬獄", "", ""],
        (MoodLevel::High, _) => &["  獄獄獄獄獄", "  獄(▽地▽)獄♪", "   獄╬╬獄", "", ""],
        (MoodLevel::Normal, 0) => &[" 獄獄獄獄獄", " 獄(⊙地⊙)獄", "  獄╬╬獄", "", ""],
        (MoodLevel::Normal, _) => &["  獄獄獄獄獄", "  獄(・地・)獄", "   獄╬╬獄", "", ""],
        (MoodLevel::Low, 0) => &[" 獄獄獄獄獄", " 獄(￣_￣)獄", "  獄╬╬獄", "", ""],
        (MoodLevel::Low, _) => &[" 獄獄獄獄獄", " 獄(￣ ￣)獄", "  獄╬╬獄", "", ""],
    }
}
fn jigoku_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" 獄獄獄獄獄", " ﾉ(⊙地⊙)獄", "  獄╬╬獄", "", ""],
        (Action::Talk, _) => &[" 獄獄獄獄獄", " 獄(⊙地⊙)ﾉ", "  獄╬╬獄", "", ""],
        (Action::Play, 0) => &["♪獄獄獄獄獄", " 獄(▽地▽)獄", "  獄╬╬獄", "", ""],
        (Action::Play, _) => &[" 獄獄獄獄獄♪", " 獄(▽地▽)獄", "  獄╬╬獄", "", ""],
        (Action::Train, 0) => &[" 獄獄獄獄獄!!", " 獄(益地益)獄", " 獄╬╬╬╬獄", "", ""],
        (Action::Train, _) => &["!!獄獄獄獄獄", " 獄(益地益)獄", " 獄╬╬╬╬獄", "", ""],
        (Action::Relax, 0) => &[" 獄獄獄獄獄～", " 獄(－地－)獄", "  獄╬╬獄", "", ""],
        (Action::Relax, _) => &[" 獄獄獄獄獄", " 獄(－地－)zzZ", "  獄╬╬獄", "", ""],
    }
}

// --- ムゲン (mugen) - Infinite/void ---
fn mugen_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ∞∞∞∞∞∞", " ∞(▽無▽)∞!", "  ∞◎◎∞", "", ""],
        (MoodLevel::High, _) => &["  ∞∞∞∞∞∞", "  ∞(▽無▽)∞♪", "   ∞◎◎∞", "", ""],
        (MoodLevel::Normal, 0) => &[" ∞∞∞∞∞∞", " ∞(⊙無⊙)∞", "  ∞◎◎∞", "", ""],
        (MoodLevel::Normal, _) => &["  ∞∞∞∞∞∞", "  ∞(・無・)∞", "   ∞◎◎∞", "", ""],
        (MoodLevel::Low, 0) => &[" ∞∞∞∞∞∞", " ∞(￣_￣)∞", "  ∞◎◎∞", "", ""],
        (MoodLevel::Low, _) => &[" ∞∞∞∞∞∞", " ∞(￣ ￣)∞", "  ∞◎◎∞", "", ""],
    }
}
fn mugen_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ∞∞∞∞∞∞", " ﾉ(⊙無⊙)∞", "  ∞◎◎∞", "", ""],
        (Action::Talk, _) => &[" ∞∞∞∞∞∞", " ∞(⊙無⊙)ﾉ", "  ∞◎◎∞", "", ""],
        (Action::Play, 0) => &["♪∞∞∞∞∞∞", " ∞(▽無▽)∞", "  ∞◎◎∞", "", ""],
        (Action::Play, _) => &[" ∞∞∞∞∞∞♪", " ∞(▽無▽)∞", "  ∞◎◎∞", "", ""],
        (Action::Train, 0) => &[" ∞∞∞∞∞∞!!", " ∞(益無益)∞", " ∞◎◎◎◎∞", "", ""],
        (Action::Train, _) => &["!!∞∞∞∞∞∞", " ∞(益無益)∞", " ∞◎◎◎◎∞", "", ""],
        (Action::Relax, 0) => &[" ∞∞∞∞∞∞～", " ∞(－無－)∞", "  ∞◎◎∞", "", ""],
        (Action::Relax, _) => &[" ∞∞∞∞∞∞", " ∞(－無－)zzZ", "  ∞◎◎∞", "", ""],
    }
}
