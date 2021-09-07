use anyhow::Result;
use chrono::{prelude::Utc, DateTime, Duration};

pub fn time_diff(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<String> {
    let duration: Duration = end - start;
    let mut diff: Vec<String> = vec![];

    let weeks = duration.num_weeks();
    let days = duration.num_days() - (weeks * 7);
    let hours = duration.num_hours() - (duration.num_days() * 24);
    let mins = duration.num_minutes() - (duration.num_hours() * 60);
    let seconds = duration.num_seconds() - (duration.num_minutes() * 60);

    if weeks > 0 {
        diff.push(format!("{}w", weeks));
    }
    if days > 0 {
        diff.push(format!("{}d", days));
    }
    if hours > 0 {
        diff.push(format!("{}h", hours));
    }
    if mins > 0 {
        diff.push(format!("{}m", mins));
    }
    if seconds > 0 {
        diff.push(format!("{}s", seconds));
    }

    Ok(diff.join(", "))
}
