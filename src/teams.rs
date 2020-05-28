use std::env;

use clap::ArgMatches;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::utils::{read_config_from_files, write_file};

use cio::UserConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GitHubTeamMembers {
    pub team: String,
    pub members: Vec<UserConfig>,
}

/**
 * Generate GitHub and AWS terraform configs that configure members of the
 * organization and team membership. We use terraform instead of calling out the
 * api ourselves because the diffs of the files after changes are more readable
 * than not having that functionality at all.
 *
 * This command uses the users.toml file for information.
 */
pub fn cmd_teams_run(cli_matches: &ArgMatches) {
    // Get the config.
    let config = read_config_from_files(cli_matches);

    // Get the current working directory.
    let github_path = env::current_dir().unwrap().join("terraform/github/");
    let aws_path = env::current_dir().unwrap().join("terraform/aws/");

    // Initialize handlebars.
    let handlebars = Handlebars::new();

    // Generate the members of the GitHub org file.
    let github_rendered = handlebars
        .render_template(
            &TEMPLATE_TERRAFORM_GITHUB_ORG_MEMBERSHIP,
            &config.users,
        )
        .unwrap();

    // Join it with the directory to save the files in.
    let github_file = github_path.join("generated.organization-members.tf");

    write_file(github_file, github_rendered);

    // Generate the members of the AWS org file.
    let aws_rendered = handlebars
        .render_template(&TEMPLATE_TERRAFORM_AWS_ORG_MEMBERSHIP, &config.users)
        .unwrap();

    // Join it with the directory to save the files in.
    let aws_file = aws_path.join("generated.organization-members.tf");

    write_file(aws_file, aws_rendered);

    // Generate the members of each GitHub team.
    // TODO: don't hard code these
    let teams = vec!["all", "eng", "friends-of-oxide"];
    for team in teams {
        // Build the members array.
        let mut members: Vec<UserConfig> = Default::default();
        for (_, user) in config.users.clone() {
            match user.clone().groups {
                Some(groups) => {
                    if groups.contains(&team.to_string()) {
                        members.push(user.clone());
                    }
                }
                None => continue,
            }
        }

        // Generate the members of the team file.
        let rendered = handlebars
            .render_template(
                &TEMPLATE_TERRAFORM_GITHUB_TEAM_MEMBERSHIP,
                &GitHubTeamMembers {
                    team: team.to_string(),
                    members: members,
                },
            )
            .unwrap();

        // Join it with the directory to save the files in.
        let file = github_path
            .join(format!("generated.team-members-{}.tf", team.to_string()));

        write_file(file, rendered);
    }

    // TODO: Generate files for the repositories.
    // Initialize the clients for the config.
    // let mut client = Client::new(config);
}

/// Template for terraform GitHub org membership.
static TEMPLATE_TERRAFORM_GITHUB_ORG_MEMBERSHIP: &'static str = r#"# THIS IS A GENERATED FILE, DO NOT EDIT THIS FILE DIRECTLY.

# Define the members of the organization.
{{#each this}}{{#if this.github}}
# Add @{{this.github}} to the organization.
resource "github_membership" "{{this.github}}" {
  username = "{{this.github}}"
  role     = "{{#if this.is_super_admin}}admin{{else}}member{{/if}}"
}
{{/if}}{{/each}}
"#;

/// Template for terraform GitHub team membership.
static TEMPLATE_TERRAFORM_GITHUB_TEAM_MEMBERSHIP: &'static str = r#"# THIS IS A GENERATED FILE, DO NOT EDIT THIS FILE DIRECTLY.

# Define the members of the {{this.team}} team.
{{#each this.members}}{{#if this.github}}
# Add @{{this.github}} to {{../team}}.
resource "github_team_membership" "{{../team}}-{{this.github}}" {
  team_id  = github_team.{{../team}}.id
  username = "{{this.github}}"
  role     = "{{#if this.is_super_admin}}maintainer{{else}}member{{/if}}"
}
{{/if}}{{/each}}
"#;

/// Template for terraform AWS org membership.
static TEMPLATE_TERRAFORM_AWS_ORG_MEMBERSHIP: &'static str = r#"# THIS IS A GENERATED FILE, DO NOT EDIT THIS FILE DIRECTLY.

# Define the members of the organization.
{{#each this}}{{#if this.github}}
# Add @{{this.github}} to the organization.
resource "aws_organizations_account" "{{this.username}}" {
  name  = "{{this.first_name}} {{this.last_name}}"
  email = "{{this.username}}+aws@oxidecomputer.com"

  parent_id = aws_organizations_organization.engineering.id
}
{{/if}}{{/each}}
"#;
