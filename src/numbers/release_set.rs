use crate::error::NumbersGuyError;
use octocrab::models::repos::Release;

pub struct ReleaseSetNumbers {
    pub count: usize,
    pub stable_release_count: usize,
}

impl ReleaseSetNumbers {
    pub async fn get(owner: &str, repo: &str) -> Result<Self, NumbersGuyError> {
        let octocrab = octocrab::instance();
        let page = octocrab.repos(owner, repo).releases().list().send().await?;

        let all = octocrab.all_pages::<Release>(page).await?;

        let count = all.len();
        let stable_release_count = all
            .into_iter()
            .filter(|rel| !rel.prerelease)
            .collect::<Vec<Release>>()
            .len();

        Ok(Self {
            count,
            stable_release_count,
        })
    }
}
