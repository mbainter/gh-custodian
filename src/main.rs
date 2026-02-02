use anyhow::{Ok, Result, bail};
use clap::Parser;
use log::debug;
use octorust::{Client, auth::Credentials};
use std::env;
use std::path::PathBuf;

/// GitHub policy-based management for enterprises and organizations
/// https://github.com/mbainter/gh-custodian
#[derive(Parser, Debug)]
struct Args {
    /// The path to the policy file
    policy: Option<PathBuf>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn create_client() -> Result<Client> {
    let token = env::var("GITHUB_TOKEN");

    if token.is_err() {
        bail!("GITHUB_TOKEN must be set");
    }

    let github = Client::new("gh-custodian", Credentials::Token(token.unwrap()))?;

    Ok(github)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::Builder::new().filter_level(args.verbosity.into()).init();

    debug!("policy: {:?}", args.policy);

    let github = create_client()?;

    let repo = github.repos().get("mbainter", "gh-custodian").await.unwrap();

    dbg!(repo);

    Ok(())
}
