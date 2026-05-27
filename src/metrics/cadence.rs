use chrono::{DateTime, TimeDelta, Utc};

use crate::{git::kit::GRepo, metrics::GitMetric};
#[derive(Debug, Default)]
pub struct CadenceMetric {
    pub commits_per_second: f32,
}

impl CadenceMetric {
    pub fn new() -> Self {
        CadenceMetric::default()
    }

    pub fn author_commits_per_second(repo: &GRepo, email: String) {
        todo!("Need to impl")
    }
}

impl GitMetric for CadenceMetric {
    type Output = CadenceMetric;

    fn calculate(&self, repo: &GRepo) -> Result<Self::Output, git2::Error> {
        let commit_dates: Vec<DateTime<Utc>> = repo
            .iter_commits()?
            .filter_map(|commit| DateTime::from_timestamp_secs(commit.time().seconds()))
            .collect();

        let commits_per_second = commits_per_second(&commit_dates);

        Ok(CadenceMetric { commits_per_second })
    }
}

fn commits_per_second(xs: &[DateTime<Utc>]) -> f32 {
    match telescope_time(&xs) {
        Some(delta) => {
            let seconds_avg = delta.as_seconds_f32();
            if seconds_avg > 0.0 {
                1.0 / seconds_avg
            } else {
                0.0
            }
        }
        None => 0.0,
    }
}

//https://en.wikipedia.org/wiki/Telescoping_series
fn telescope_time(datetimes: &[DateTime<Utc>]) -> Option<TimeDelta> {
    if datetimes.len() < 2 {
        return None;
    }

    // the middle dates all cancel when summing over their differences as pairs
    // and we are left with the first and last only
    let total_duration = *datetimes.first()? - *datetimes.last()?;
    let count = (datetimes.len() - 1) as i32;

    total_duration.checked_div(count)
}
