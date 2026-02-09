//! Extends octorust to support additional organization endpoints

use super::types::OrganizationRuleset;

use octorust::{ClientError, Response};

mod progenitor_support {
    use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

/// Trait for my extensions to support the organizational management endpoints of the GitHub API
pub trait OrgsMgmt {
    /// List all repository rulesets for an organization
    async fn list_rulesets(&self, org: &str) -> Result<Response<Vec<OrganizationRuleset>>, ClientError>;
}

impl OrgsMgmt for octorust::orgs::Orgs {
    async fn list_rulesets(&self, org: &str) -> Result<Response<Vec<OrganizationRuleset>>, ClientError> {
        let url = format!("/orgs/{}/rulesets", progenitor_support::encode_path(org),);

        self.get(&url).await
    }
}
