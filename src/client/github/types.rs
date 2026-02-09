use serde::{Deserialize, Serialize};

/// Parameters for listing organization rulesets
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOrganizationRulesetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<String>,
}

/// A set of rules to apply when specified conditions are met.
/// Repository ruleset
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationRuleset {
    /// The ID of the ruleset
    pub id: i64,
    /// The name of the ruleset
    pub name: String,
    /// The target of the ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<OrganizationRulesetTarget>,
    /// The type of the source of the ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<RulesetSourceType>,
    /// The name of the source
    pub source: String,
    /// The enforcement level of the ruleset. `evaluate` allows admins to test rules before enforcing them. Admins can view insights on the Rule Insights page (`evaluate` is only available with GitHub Enterprise).
    pub enforcement: Option<RulesetEnforcement>,
    /// The actors that can bypass the rules in this ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_actors: Option<Vec<RepositoryRulesetBypassActor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<OrganizationRulesetLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<OrganizationRulesetConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum OrganizationRulesetTarget {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "rename")]
    Push,
    #[serde(rename = "repository")]
    Repository,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum RulesetSourceType {
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "enterprise")]
    Enterprise,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum RulesetEnforcement {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "evaluate")]
    Evaluate,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OrganizationRulesetLinks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "octorust::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "octorust::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

/// An actor that can bypass rules in a ruleset
/// Repository Ruleset Bypass Actor
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct RepositoryRulesetBypassActor {
    /// The ID of the actor that can bypass a ruleset. Required for `Integration`, `RepositoryRole`, and `Team` actor types. If `actor_type` is `OrganizationAdmin`, `actor_id` is ignored. If `actor_type` is `DeployKey`, this should be null. `OrganizationAdmin` is not applicable for personal repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<serde_json::Value>,
    /// The type of actor that can bypass a ruleset.
    pub actor_type: Option<ActorType>,
    /// When the specified actor can bypass the ruleset. `pull_request` means that an actor can only bypass rules on pull requests. `pull_request` is not applicable for the `DeployKey` actor type. Also, `pull_request` is only applicable to branch rulesets. When `bypass_mode` is `exempt`, rules will not be run for that actor and a bypass audit entry will not be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_mode: Option<BypassMode>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ActorType {
    Integration,
    OrganizationAdmin,
    RepositoryRole,
    Team,
    DeployKey,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum BypassMode {
    #[serde(rename = "always")]
    #[default]
    Always,
    #[serde(rename = "pull_request")]
    PullRequest,
    #[serde(rename = "exempt")]
    Exempt,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrganizationRulesetConditions {
    RepositoryRulesetRefnameCondition,
    OrganizationRulesetRepositoryIdAndRefnameCondition,
    OrganizationRulesetRepositoryNameAndRefnameCondition,
    OrganizationRulesetRepositoryPropertyAndRefnameCondition,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrganizationRulesetRepositoryNameAndRefnameCondition {
    OrganizationRuleRepositoryNameCondition,
    RepositoryRulesetRefnameCondition,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrganizationRulesetRepositoryIdAndRefnameCondition {
    OrganizationRulesetRepositoryIdCondition,
    RepositoryRulesetRefnameCondition,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrganizationRulesetRepositoryPropertyAndRefnameCondition {
    OrganizationRulesetRepositoryPropertyCondition,
    RepositoryRulesetRefnameCondition,
}

/// Parameters for a repository ruleset ref name condition
/// Repository ruleset conditions for ref names
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RepositoryRulesetRefnameCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<Refname>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Refname {
    /// Array of ref names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~DEFAULT_BRANCH` to include the default branch or `~ALL` to include all branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Array of ref names or patterns to exclude. The condition will not pass if any of these patterns match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrganizationRuleRepositoryNameCondition {
    /// Array of repository names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~ALL` to include all repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Array of repository names or patterns to exclude. The condition will not pass if any of these patterns match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// Whether renaming of target repositories is prevented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrganizationRulesetRepositoryIdCondition {
    /// The repository IDs that the ruleset applies to. One of these IDs must match for the condition to pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_ids: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum RepositoryCustomPropertySource {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "system")]
    #[default]
    System,
}

/// Parameters for a filtering based on a Custom Property
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CustomPropertyFilter {
    /// The name of the repository property to target
    pub name: String,
    /// The values to match for the repository property
    pub property_values: Vec<String>,
    /// The source of the repository property. Defaults to 'custom' if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RepositoryCustomPropertySource>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrganizationRulesetRepositoryPropertyCondition {
    /// The repository custom properties and values to use to include the matching results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<CustomPropertyFilter>>,
    /// The repository custom properties to use to exclude matching results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<CustomPropertyFilter>>,
}
