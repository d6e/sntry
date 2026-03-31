mod api;
mod cli;
mod config;
mod error;
mod output;

use std::error::Error;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::args::{Cli, Commands, ConfigCommands, EventsCommands, IssuesCommands, ReleasesCommands};
use cli::commands::{config as config_cmd, events, issues, releases};
use config::load_config;
use output::print_error;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = run().await {
        print_error(&e.to_string());

        // Show error chain if verbose flag was passed
        if std::env::args().any(|arg| arg == "--verbose" || arg == "-v") {
            let mut source = e.source();
            while let Some(cause) = source {
                eprintln!("Caused by: {cause}");
                source = cause.source();
            }
        }

        std::process::exit(1);
    }
}

async fn run() -> error::Result<()> {
    let cli = Cli::parse();
    let config = load_config();

    // Set global output format and quiet mode
    output::set_format(cli.format);
    output::set_quiet(cli.quiet);

    match cli.command {
        Commands::Issues { command } => {
            let client = api::SentryClient::new(
                &config,
                cli.org.as_deref(),
                cli.server.as_deref(),
                cli.token.as_deref(),
                cli.verbose,
            )?;

            match command {
                IssuesCommands::List {
                    project,
                    status,
                    query,
                    sort,
                    limit,
                    all,
                    environment,
                    period,
                    start,
                    end,
                } => {
                    let options = issues::ListOptions {
                        project,
                        status,
                        query,
                        sort: sort.to_string(),
                        limit,
                        all,
                        environment,
                        period,
                        start,
                        end,
                    };
                    issues::list_issues(&client, options).await?;
                }
                IssuesCommands::View { issue_id } => {
                    issues::view_issue(&client, &issue_id).await?;
                }
                IssuesCommands::Resolve {
                    issue_ids,
                    in_release,
                    in_next_release,
                } => {
                    issues::resolve_issues(&client, issue_ids, in_release, in_next_release).await?;
                }
                IssuesCommands::Unresolve { issue_ids } => {
                    issues::unresolve_issues(&client, issue_ids).await?;
                }
                IssuesCommands::Assign {
                    issue_ids,
                    to,
                    unassign,
                } => {
                    issues::assign_issues(&client, issue_ids, to, unassign).await?;
                }
                IssuesCommands::Ignore {
                    issue_ids,
                    duration,
                    count,
                    until_escalating,
                } => {
                    issues::ignore_issues(&client, issue_ids, duration, count, until_escalating)
                        .await?;
                }
                IssuesCommands::Delete { issue_ids, confirm } => {
                    issues::delete_issues(&client, issue_ids, confirm).await?;
                }
                IssuesCommands::Merge {
                    primary_id,
                    other_ids,
                } => {
                    issues::merge_issues(&client, primary_id, other_ids).await?;
                }
                IssuesCommands::Comment {
                    issue_id,
                    text,
                    list,
                } => {
                    if list || text.is_none() {
                        issues::list_issue_comments(&client, &issue_id).await?;
                    } else {
                        issues::comment_issue(&client, &issue_id, &text.unwrap()).await?;
                    }
                }
                IssuesCommands::Tags { issue_id, tag_key } => {
                    issues::get_issue_tag(&client, &issue_id, &tag_key).await?;
                }
                IssuesCommands::Links { issue_id } => {
                    issues::list_external_issues(&client, &issue_id).await?;
                }
                IssuesCommands::Link {
                    issue_ids,
                    url,
                    project,
                    identifier,
                    integration,
                } => {
                    issues::link_issues(&client, issue_ids, &url, project, identifier, &integration)
                        .await?;
                }
            }
        }
        Commands::Releases { command } => {
            let client = api::SentryClient::new(
                &config,
                cli.org.as_deref(),
                cli.server.as_deref(),
                cli.token.as_deref(),
                cli.verbose,
            )?;

            match command {
                ReleasesCommands::List { query, limit, all } => {
                    let options = releases::ListOptions { query, limit, all };
                    releases::list_releases(&client, options).await?;
                }
                ReleasesCommands::View { version } => {
                    releases::view_release(&client, &version).await?;
                }
            }
        }
        Commands::Events { command } => {
            let client = api::SentryClient::new(
                &config,
                cli.org.as_deref(),
                cli.server.as_deref(),
                cli.token.as_deref(),
                cli.verbose,
            )?;

            match command {
                EventsCommands::List { issue_id, limit, all } => {
                    let options = events::ListOptions { issue_id, limit, all };
                    events::list_events(&client, options).await?;
                }
                EventsCommands::View { event_id, project } => {
                    events::view_event(&client, &event_id, &project).await?;
                }
            }
        }
        Commands::Config { command } => match command {
            ConfigCommands::Init => {
                config_cmd::init_config()?;
            }
            ConfigCommands::Show => {
                config_cmd::show_config()?;
            }
            ConfigCommands::Set { key, value } => {
                config_cmd::set_config(&key, &value)?;
            }
        },
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "sentry", &mut std::io::stdout());
        }
    }

    Ok(())
}
