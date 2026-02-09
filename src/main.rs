mod client;
use anyhow::{Ok, Result, bail};
use clap::Parser;
use client::github::orgs::OrgsMgmt;
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

    let client = Client::new("gh-custodian", Credentials::Token(token.unwrap()))?;

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::Builder::new().filter_level(args.verbosity.into()).init();

    debug!("policy: {:?}", args.policy);

    let gh = create_client().unwrap();

    let repo = gh.repos().get("mbainter", "gh-custodian").await?;

    dbg!(repo);

    let rulesets = gh.orgs().list_rulesets("litmus").await?;

    dbg!(rulesets);

    Ok(())
}
