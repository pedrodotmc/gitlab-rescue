use crate::api_client::api_client;
use crate::app_error::Result;
use crate::gitlab_api::GitLabApi;
use crate::io::IO;
use crate::Performable;
use crate::{app_info, app_success, extract_environment, extract_token, extract_url};
use clap::ArgMatches;
use std::convert::From;
use std::env;
use urlencoding::encode;

#[derive(Debug, Clone)]
pub struct GetVariableCommand {
    /// Variable name
    name: String,
    /// Project ID or URL-encoded NAMESPACE/PROJECT_NAME
    gitlab_project: Option<String>,
    /// Group ID or URL-encoded path of the group
    gitlab_group: Option<String>,
    /// Name of GitLab CI/CD environment
    environment: String,
    /// If variable is not found in defined environment (-e option), try with "All" environment.
    from_all_if_missing: bool,
    /// GitLab URL
    url: String,
    /// GitLab API Token
    token: String,
}

impl From<&ArgMatches<'_>> for GetVariableCommand {
    fn from(argm: &ArgMatches<'_>) -> Self {
        GetVariableCommand {
            name: argm.value_of("VARIABLE_NAME").unwrap().to_owned(),
            gitlab_project: if let Some(v) = argm.value_of("project") { Some(encode(v)) } else { None },
            gitlab_group: if let Some(v) = argm.value_of("group") { Some(v.to_owned()) } else { None },
            environment: extract_environment!(argm),
            from_all_if_missing: argm.is_present("from-all-if-missing"),
            url: extract_url!(argm),
            token: extract_token!(argm),
        }
    }
}

impl GetVariableCommand {
    fn get_variable_from_group(&self) -> Result<String> {
        api_client("v4", &self.url, &self.token)
            .get_from_group(self.gitlab_group.as_ref().unwrap(), &self.name)
            .map(|g| g.value)
    }

    fn get_variable_from_project(&self, p: &str) -> Result<String> {
        let api = api_client("v4", &self.url, &self.token);
        api.get_from_project(p, &self.name, &self.environment)
            .and_then(|var| Ok(var.value))
            .or_else(|e| match self.environment != "*" && self.from_all_if_missing {
                true => api.get_from_project(p, &self.name, "*").and_then(|g| Ok(g.value)),
                _ => Err(e),
            })
    }
}

impl Performable for GetVariableCommand {
    fn get_action(self) -> IO<Result<()>> {
        IO::unit(move || match self.gitlab_project.as_ref() {
            Some(p) => {
                app_info!("Getting variable from project {}...", p);
                self.get_variable_from_project(p)
            }
            None => {
                app_info!("Getting variable from group {}...", &self.gitlab_group.as_ref().unwrap());
                self.get_variable_from_group()
            }
        })
        .map(|r| {
            r.and_then(|v| {
                app_success!("Variable obtained successfully");
                Ok(println!("{}", v))
            })
        })
    }
}
