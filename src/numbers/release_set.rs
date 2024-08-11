use crate::error::NumbersGuyError;
use octocrab::models::repos::Release;

pub struct ReleaseSetNumbers {
    pub count: usize,
    pub stable_release_count: usize,
    pub asset_count: usize,
    pub download_count: i64,
}

impl ReleaseSetNumbers {
    pub async fn get(owner: &str, repo: &str) -> Result<Self, NumbersGuyError> {
        let octocrab = octocrab::instance();
        let page = octocrab.repos(owner, repo).releases().list().send().await?;

        let all = octocrab.all_pages::<Release>(page).await?;

        let count = all.len();
        let mut stable_release_count = 0;
        let mut asset_count = 0;
        let mut download_count = 0;

        for release in all {
            if !release.prerelease {
                stable_release_count += 1;
            }
            asset_count += release.assets.len();
            for asset in release.assets {
                download_count += asset.download_count;
            }
        }

        Ok(Self {
            count,
            stable_release_count,
            asset_count,
            download_count,
        })
    }
}
