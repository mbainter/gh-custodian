use anyhow::{Ok, Result, bail};
use clap::Parser;
use log::debug;
use octocrate::{GitHubAPI, APIConfig, PersonalAccessToken};
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::Builder::new().filter_level(args.verbosity.into()).init();

    debug!("policy: {:?}", args.policy);

    let token = env::var("GITHUB_TOKEN");

    if token.is_err() {
        bail!("GITHUB_TOKEN must be set");
    }

    let pat = PersonalAccessToken::new(token.unwrap());
    let config = APIConfig::with_token(pat).shared();

    let gh = GitHubAPI::new(&config);

    // let repo = gh.repos.get("mbainter", "gh-custodian").send().await?;
    // dbg!(repo);

    let ruleset = gh.repos.get_org_ruleset("litmus", 670245i32).send().await?;
    dbg!(ruleset);

    Ok(())
}
