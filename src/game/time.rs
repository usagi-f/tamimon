use chrono::{DateTime, Utc};
use serde::Deserialize;

const MAX_ELAPSED_TICKS: u64 = 4320; // 72 hours * 60 minutes
const DRIFT_THRESHOLD_MINUTES: i64 = 10;
const API_TIMEOUT_SECS: u64 = 5;

pub enum TimeSource {
    TimeApi,
    LocalFallback,
}

pub struct TimeResult {
    pub now: DateTime<Utc>,
    pub source: TimeSource,
    pub drift_warning: Option<String>,
}

#[allow(dead_code)]
pub struct ElapsedResult {
    pub ticks: u64,
    pub capped: bool,
    pub rollback_detected: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimeApiResponse {
    date_time: String,
}

const TIME_API_URL: &str = "https://timeapi.io/api/time/current/zone?timeZone=UTC";

pub async fn fetch_current_time() -> TimeResult {
    let local_now = Utc::now();

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(API_TIMEOUT_SECS))
        .build()
    {
        Ok(c) => c,
        Err(_) => return local_fallback(local_now),
    };

    // Retry up to 2 times on network failure
    for attempt in 0..2 {
        if attempt > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        let resp = match client.get(TIME_API_URL).send().await {
            Ok(r) => r,
            Err(_) => continue,
        };

        let data = match resp.json::<TimeApiResponse>().await {
            Ok(d) => d,
            Err(_) => continue,
        };

        // timeapi.io returns datetime without timezone suffix, append Z for UTC
        let rfc3339 = format!("{}Z", data.date_time);
        if let Ok(parsed) = DateTime::parse_from_rfc3339(&rfc3339) {
            let api_utc: DateTime<Utc> = parsed.into();
            let drift = (api_utc - local_now).num_minutes().abs();
            let drift_warning = if drift >= DRIFT_THRESHOLD_MINUTES {
                Some(format!(
                    "ローカル時刻とサーバー時刻に{}分の差があります",
                    drift
                ))
            } else {
                None
            };
            return TimeResult {
                now: api_utc,
                source: TimeSource::TimeApi,
                drift_warning,
            };
        }
    }

    local_fallback(local_now)
}

fn local_fallback(now: DateTime<Utc>) -> TimeResult {
    TimeResult {
        now,
        source: TimeSource::LocalFallback,
        drift_warning: None,
    }
}

pub fn calculate_elapsed_ticks(last_check: DateTime<Utc>, now: DateTime<Utc>) -> ElapsedResult {
    let diff = now.signed_duration_since(last_check);

    if diff.num_seconds() < 0 {
        return ElapsedResult {
            ticks: 0,
            capped: false,
            rollback_detected: true,
        };
    }

    let raw_ticks = diff.num_minutes() as u64;
    let capped = raw_ticks > MAX_ELAPSED_TICKS;
    let ticks = raw_ticks.min(MAX_ELAPSED_TICKS);

    ElapsedResult {
        ticks,
        capped,
        rollback_detected: false,
    }
}

pub fn format_elapsed(birth: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let diff = now.signed_duration_since(birth);
    if diff.num_seconds() < 0 {
        return "0秒".to_string();
    }

    let total_secs = diff.num_seconds() as u64;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}日", days));
    }
    if hours > 0 || days > 0 {
        parts.push(format!("{}時間", hours));
    }
    if mins > 0 || hours > 0 || days > 0 {
        parts.push(format!("{:02}分", mins));
    }
    parts.push(format!("{:02}秒", secs));

    parts.join(" ")
}

pub fn format_elapsed_short(ticks: u64) -> String {
    let hours = ticks / 60;
    let mins = ticks % 60;
    if hours > 0 {
        format!("{}時間 {}分", hours, mins)
    } else {
        format!("{}分", mins)
    }
}
