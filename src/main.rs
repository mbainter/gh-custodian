use anyhow::{Ok, Result, bail};
use clap::Parser;
use log::debug;
use octorust::{Client, auth::Credentials};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Policy {
    organization: String,
    repository_targets: Vec<String>,
    allowed_repo_roles: Vec<String>,
    authorized_individuals: Vec<IndividualAccessPolicy>,
    authorized_repo_admins: Vec<AuthorizationPolicy>,
    authorized_repo_maintainers: Vec<AuthorizationPolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum IndividualAccessPolicy {
    Role { name: String },
}

#[derive(Debug, Deserialize)]
enum TargetType {
    User,
    Team,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AuthorizationPolicy {
    CustomProperty {
        property: String,
        target_type: TargetType,
        role: String,
    },
}

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
    let policy_data = fs::read_to_string(args.policy.unwrap()).expect("Error reading policy file");
    let policy: Policy = toml::from_str(&policy_data)?;
    debug!("policy: {:?}", policy);
    println!("org: {}", policy.organization);

    /*
    let gh = create_client().unwrap();

    let repo = gh.repos().get("mbainter", "gh-custodian").await?;

    dbg!(repo);

    let rulesets = gh.repos().get_org_rulesets("litmus", 10, 1, "").await?;

    dbg!(rulesets);
    */

    Ok(())
}
