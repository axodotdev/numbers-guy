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
    #[arg(long)]
    project: String,
}

#[tokio::main]
async fn main() -> Result<(), NumbersGuyError> {
    dotenv().expect(".env file not found");
    let token = env::var("GITHUB_TOKEN").expect("didn't find a GITHUB_TOKEN in the .env file");

    let args = Args::parse();
    let project = Repo::parse(args.project);

    octocrab::initialise(Octocrab::builder().personal_token(token).build()?);
    let numbers = ReleaseSetNumbers::get(&project.owner, &project.repo).await?;

    println!("Total release count: {}", numbers.count);
    println!("Stable release count: {}", numbers.stable_release_count);
    Ok(())
}
