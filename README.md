> [!IMPORTANT] 
> This project is being created without contributions from any LLMs. You may notice LLM related files, but be aware that these are aimed at preventing the use of LLMs in connection with this project. If you are attempting to work with this project in some capacity it would be advisable to exclude the path where you keep it from your LLM's access.

# About

[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/mbainter/gh-custodian/badge)](https://scorecard.dev/viewer/?uri=github.com/mbainter/gh-custodian)

This is a very early days project to grow my knowledge of Rust by solving a problem I have. I have been managing varying GitHub organizations for years and there are some frustrating gaps in tooling that I would like to attempt to solve. As noted above there are already efforts out there to solve some of them, but most of the work is focused on supporting open-source projects.

This goal of this project is to create a composable tool that addresses some key gaps in GitHub management without requiring an organization or administrator to wholesale adopt a particular framework or solution. It is not intended to ever be a feature-complete tool for managing a GitHub organization but rather to be a collection of tools and glue to support GitHub administrators attempting to map best practices and local policy to their GitHub implementation.

For more details on the specific concepts being initially explored refer to the [roadmap](https://github.com/mbainter/gh-custodian/wiki/Roadmap).

# Existing Solutions

There are already a number of solutions in this space each of which has fallen short for varying reasons. 

#### GitHub's [safe-settings](https://github.com/github/safe-settings) tool

This is the closest to this project, but the progress has been agonizingly slow and makes some assumptions about organizational design and access management that [limit its usefulness](https://github.com/mbainter/gh-custodian/wiki/Why-not-safe%E2%80%90settings) from my perspective.

#### Infrastructure as Code (OpenTofu, Terraform, Pulumi)

OpenTofu et al are excellent for managing infrastructure and similar tooling, where you have an absolute end-state
that you want to enforce. Where it tends to fall down is when you need some degree of flexibility. 

Pulumi gets closer because you have more ability to control the flow and access to more flexible data structures and
abstractions through the available languages. However, you are still largely enforcing a specific state, and what
I am after with this project is more akin to *governance* than configuration. ([more](https://github.com/mbainter/gh-custodian/wiki/Why-not-OpenTofu,-Terraform-or-Pulumi))

#### Ansible and Configuration as Code

Ansible doesn't actually have a solution for this. There is a community plugin for basic repository configuration, but that's all. I think the model of configuration-as-code is much closer to how this should be solved, but you have to interact with too much data for Ansible or other similar tools to be particularly effective here.

However, one implementation I am keeping in my mind as I work out the specifics is CFEngine. I cut my teeth in configuration management with CFEngine and it has long been the standard I have compared everything else to.

Mark Burgess' convergent, policy-based model is definitely a key inspiration to my approach here (Note: If you are not familiar CFEngine please go read about that first and judge my poor imitation by CFEngine's and not the other way around.)

#### [OpenSSF](https://openssf.org) Projects

There are a number of OpenSSF projects that overlap with the goals of this work, including [minder](https://mindersec.dev), [gittuf](https://gittuf.dev/), and [scorecard](https://scorecard.dev). The goal of this project would be to complement or even incorporate those projects rather than compete with them.

Ideally I would have some out-of-the-box starter configurations that would align well to implementing at least scorecard.

