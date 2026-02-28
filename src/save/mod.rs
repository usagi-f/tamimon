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

pub fn load() -> Result<Option<schema::SaveData>> {
    let path = save_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("セーブファイルの読み込みに失敗: {}", path.display()))?;
    let data: schema::SaveData = serde_json::from_str(&content)
        .with_context(|| "セーブデータの解析に失敗しました")?;
    Ok(Some(data))
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

    std::fs::rename(&tmp_path, &path)
        .with_context(|| "セーブファイルの更新に失敗")?;

    Ok(())
}
