> [!NOTE] A brief note about LLMs
> This project is being created without contributions from any LLMs. You may notice LLM related files, but be aware that these are aimed at preventing the use of LLMs in connection with this project. If you are attempting to work with this project in some capacity it would be advisable to exclude the path where you keep it from your LLM's access.

# About

This is a very early days project to grow my knowledge of Rust by solving a problem I have. I have been managing varying GitHub organizations for years and there are some frustrating gaps in tooling that I would like to attempt to solve. As noted above there are already efforts out there to solve some of them, but most of the work is focused on supporting open-source projects.

This goal of this project is to create a composable tool that addresses some key gaps in GitHub management without requiring an organization or administrator to wholesale adopt a particular framework or solution. It is not intended to ever be a feature-complete tool for managing a GitHub organization but rather to be a collection of tools and glue to support GitHub administrators attempting to map best practices and local policy to their GitHub implementation.

For more details on the specific concepts being initially explored refer to the [roadmap](https://github.com/mbainter/gh-custodian/wiki/Roadmap).

# Existing Solutions

There are already a number of solutions in this space each of which has fallen short for varying reasons. 

#### [safe-settings](https://github.com/github/safe-settings)

This is the closest to this project, but the progress has been agonizingly slow and makes some assumptions about organizational design and access management that limit its usefulness. The use of yaml also limits its flexibility and efficiency without diving into generated configurations.

#### Infrastructure as Code (Terraform, OpenTofu, Pulumi)

One of the biggest challenges for solving this is that while GitHub provides an expansive API it has had a number of significant version changes, and GitHub is not great at ensuring comparable functionality between versions. It is not uncommon to need to work with multiple versions of the API to accomplish a given task. There are even tasks that require leveraging both the GraphQL API and the REST API to accomplish effectively. This presents a significant hurdle for IaC providers.

A lot of this could be done with Terraform, and I have gone down this route multiple times over the years. The way the API is designed creates cyclical reference challenges and large numbers of resources that quickly becomes unwieldy and error-prone. There are also challenges when trying to balance policy and flexibility.

There are situations where you want to allow the team managing a repository to have a degree of control over the rules and configuration but still ensure that certain requirements are always present. This mix requires a policy validation approach to avoid configuration drift nightmares.

Pulumi gets a lot closer because you can leverage data structures more flexbily and powerfully in code than you can in Terraform's DSL. It also makes dealing with some of the circular references easier. Unfortunately it still falls prey to the API problems in the first paragraph.

#### [OpenSSF](https://openssf.org) Projects

There are a number of OpenSSF projects that overlap with the goals of this work, including [minder](https://mindersec.dev), [gittuf](https://gittuf.dev/), and [scorecard](https://scorecard.dev). The goal of this project would be to complement or even incorporate those projects.

