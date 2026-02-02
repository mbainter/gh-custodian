use assert_cmd::cargo::*;
use predicates::prelude::*;
use std::env;

#[test]
fn github_token_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("gh-custodian");

    cmd.env_remove("GITHUB_TOKEN");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("GITHUB_TOKEN must be set"));

    Ok(())
}
