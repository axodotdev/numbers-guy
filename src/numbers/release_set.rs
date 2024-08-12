use crate::error::NumbersGuyError;
use octocrab::models::repos::Release;

pub struct ReleaseSetNumbers {
    pub total: usize,
    pub dist_release_count: usize,
    pub stable_release_count: usize,
    pub asset_count: usize,
    pub download_count: i64,
    pub first_release_date: String,
    pub last_release_date: String,
}

impl ReleaseSetNumbers {
    pub async fn get(owner: &str, repo: &str) -> Result<Self, NumbersGuyError> {
        let octocrab = octocrab::instance();
        let page = octocrab.repos(owner, repo).releases().list().send().await?;

        let all = octocrab.all_pages::<Release>(page).await?;

        let total = all.len();
        let mut dist_release_count = 0;
        let mut stable_release_count = 0;
        let mut asset_count = 0;
        let mut download_count = 0;
        let mut dist_releases = vec![];

        for release in all {
            let mut is_dist_release = false;
            for asset in &release.assets {
                if asset.name == "dist-manifest.json" {
                    is_dist_release = true;
                }
            }
            if is_dist_release {
                dist_releases.push(release.clone());
                dist_release_count += 1;
                if !release.prerelease {
                    stable_release_count += 1;
                }
                asset_count += release.assets.len();
                for asset in release.assets {
                    if asset.name != "dist-manifest.json" {
                        download_count += asset.download_count;
                    }
                }
            }
        }

        let last_release_date = dist_releases
            .first()
            .unwrap()
            .created_at
            .unwrap()
            .to_string();
        let first_release_date = dist_releases
            .last()
            .unwrap()
            .created_at
            .unwrap()
            .to_string();

        Ok(Self {
            total,
            dist_release_count,
            stable_release_count,
            asset_count,
            download_count,
            first_release_date,
            last_release_date,
        })
    }
}
