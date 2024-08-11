use crate::numbers::release_set::ReleaseSetNumbers;
use crate::repo::Repo;
use clap::Parser;
use dotenvy::dotenv;
use error::NumbersGuyError;
use octocrab::Octocrab;
use std::env;

mod error;
mod numbers;
mod repo;

#[derive(Debug, Parser)]
pub struct Args {
    /// A comma-separated list of projects in repo-slug format: `owner/repo`
    #[arg(long, value_delimiter = ',')]
    projects: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), NumbersGuyError> {
    dotenv().expect(".env file not found");
    let token = env::var("GITHUB_TOKEN").expect("didn't find a GITHUB_TOKEN in the .env file");
    octocrab::initialise(Octocrab::builder().personal_token(token).build()?);

    let args = Args::parse();
    for project in args.projects {
        let project = Repo::parse(project);

        let numbers = ReleaseSetNumbers::get(&project.owner, &project.repo).await?;

        println!("PROJECT: {}/{}", project.owner, project.repo);
        println!("Total release count: {}", numbers.count);
        println!("Stable release count: {}", numbers.stable_release_count);
        println!("Asset count: {}", numbers.asset_count);
        println!("Download count: {}", numbers.download_count);
    }
    Ok(())
}
