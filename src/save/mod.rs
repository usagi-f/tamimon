pub mod schema;

use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn save_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("ホームディレクトリが見つかりません"))?;
    Ok(home.join(".tamimon"))
}

pub fn save_path() -> Result<PathBuf> {
    Ok(save_dir()?.join("save.json"))
}

fn backup_path() -> Result<PathBuf> {
    Ok(save_dir()?.join("save.json.bak"))
}

pub fn load() -> Result<Option<schema::SaveData>> {
    let path = save_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("セーブファイルの読み込みに失敗: {}", path.display()))?;

    match serde_json::from_str::<schema::SaveData>(&content) {
        Ok(data) => Ok(Some(data)),
        Err(e) => {
            // Primary save is corrupted — try backup
            eprintln!("セーブデータの解析に失敗: {}", e);
            let bak = backup_path()?;
            if bak.exists() {
                eprintln!("バックアップから復元を試みます...");
                let bak_content = std::fs::read_to_string(&bak)
                    .with_context(|| "バックアップファイルの読み込みに失敗")?;
                match serde_json::from_str::<schema::SaveData>(&bak_content) {
                    Ok(data) => {
                        // Restore from backup
                        std::fs::write(&path, &bak_content).ok();
                        eprintln!("バックアップから復元しました");
                        Ok(Some(data))
                    }
                    Err(_) => {
                        // Both corrupted — rename corrupt file and start fresh
                        let corrupt = path.with_extension("json.corrupt");
                        std::fs::rename(&path, &corrupt).ok();
                        eprintln!(
                            "セーブデータが破損しています。破損ファイルを {} に退避しました",
                            corrupt.display()
                        );
                        Ok(None)
                    }
                }
            } else {
                let corrupt = path.with_extension("json.corrupt");
                std::fs::rename(&path, &corrupt).ok();
                eprintln!(
                    "セーブデータが破損しています。破損ファイルを {} に退避しました",
                    corrupt.display()
                );
                Ok(None)
            }
        }
    }
}

pub fn save(data: &schema::SaveData) -> Result<()> {
    let dir = save_dir()?;
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("ディレクトリの作成に失敗: {}", dir.display()))?;

    let path = save_path()?;
    let tmp_path = path.with_extension("json.tmp");

    let content = serde_json::to_string_pretty(data)
        .with_context(|| "セーブデータのシリアライズに失敗")?;

    std::fs::write(&tmp_path, &content)
        .with_context(|| format!("一時ファイルの書き込みに失敗: {}", tmp_path.display()))?;

    // Create backup of current save before overwriting
    if path.exists() {
        let bak = backup_path()?;
        std::fs::copy(&path, &bak).ok();
    }

    std::fs::rename(&tmp_path, &path)
        .with_context(|| "セーブファイルの更新に失敗")?;

    Ok(())
}
