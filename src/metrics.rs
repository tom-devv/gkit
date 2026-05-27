use crate::git::kit::GRepo;

pub mod cadence;

pub trait GitMetric {
    type Output;

    fn calculate(&self, repo: &GRepo) -> Result<Self::Output, git2::Error>;
}
