use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Compact,
}

#[derive(Parser)]
#[command(name = "sentry-cli")]
#[command(about = "CLI tool for managing Sentry issues", long_about = None)]
#[command(version)]
#[command(after_help = "EXAMPLES:
    sentry issues list --project myproject
    sentry issues view ISSUE-123
    sentry issues resolve ISSUE-123
    sentry config show")]
pub struct Cli {
    /// Sentry server URL (default: https://sentry.io)
    #[arg(long, global = true)]
    pub server: Option<String>,

    /// Organization slug
    #[arg(long, short, global = true)]
    pub org: Option<String>,

    /// Auth token (overrides env var and config)
    #[arg(long, global = true)]
    pub token: Option<String>,

    /// Output format (table, json, compact)
    #[arg(long, short = 'O', global = true, value_enum, default_value = "table")]
    pub format: OutputFormat,

    /// Suppress success messages
    #[arg(long, short, global = true)]
    pub quiet: bool,

    /// Enable verbose output (show error chains)
    #[arg(long, short, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage Sentry issues
    #[command(
        alias = "i",
        after_help = "EXAMPLES:
    sentry issues list --project myproject
    sentry issues list --status unresolved --limit 50
    sentry issues view ISSUE-123
    sentry issues resolve ISSUE-123 ISSUE-456"
    )]
    Issues {
        #[command(subcommand)]
        command: IssuesCommands,
    },
    /// Manage releases
    #[command(
        alias = "r",
        after_help = "EXAMPLES:
    sentry releases list
    sentry releases list --limit 50
    sentry releases view v0.90.1"
    )]
    Releases {
        #[command(subcommand)]
        command: ReleasesCommands,
    },
    /// View error events
    #[command(
        alias = "e",
        after_help = "EXAMPLES:
    sentry events list ISSUE-123
    sentry events list ISSUE-123 --limit 50
    sentry events view abc123def --project myproject"
    )]
    Events {
        #[command(subcommand)]
        command: EventsCommands,
    },
    /// Manage CLI configuration
    #[command(
        alias = "cfg",
        after_help = "EXAMPLES:
    sentry config init
    sentry config show
    sentry config set default_org myorg"
    )]
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Generate shell completions
    #[command(after_help = "EXAMPLES:
    sentry completions bash > ~/.bash_completion.d/sentry
    sentry completions zsh > ~/.zfunc/_sentry
    sentry completions fish > ~/.config/fish/completions/sentry.fish")]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum IssuesCommands {
    /// List issues with optional filtering
    #[command(
        alias = "ls",
        after_help = "EXAMPLES:
    sentry issues list
    sentry issues list --project myproject --status unresolved
    sentry issues list --query \"is:unresolved\" --limit 100"
    )]
    List {
        /// Filter by project slug(s), comma-separated
        #[arg(long, short)]
        project: Option<String>,

        /// Filter by status: unresolved, resolved, ignored
        #[arg(long, short)]
        status: Option<String>,

        /// Sentry search query string
        #[arg(long)]
        query: Option<String>,

        /// Sort by: date, new, freq, user
        #[arg(long, default_value = "date")]
        sort: String,

        /// Maximum number of results per page
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Fetch all pages (may be slow for large result sets)
        #[arg(long)]
        all: bool,
    },

    /// View detailed issue information
    #[command(
        alias = "show",
        alias = "v",
        after_help = "EXAMPLES:
    sentry issues view ISSUE-123
    sentry issues view 12345678"
    )]
    View {
        /// Issue ID or short ID
        issue_id: String,
    },

    /// Resolve one or more issues
    #[command(
        alias = "r",
        after_help = "EXAMPLES:
    sentry issues resolve ISSUE-123
    sentry issues resolve ISSUE-123 ISSUE-456 --in-next-release"
    )]
    Resolve {
        /// Issue ID(s) to resolve
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// Mark resolved in specific release
        #[arg(long)]
        in_release: Option<String>,

        /// Mark resolved in next release
        #[arg(long)]
        in_next_release: bool,
    },

    /// Unresolve one or more issues
    #[command(after_help = "EXAMPLES:
    sentry issues unresolve ISSUE-123")]
    Unresolve {
        /// Issue ID(s) to unresolve
        #[arg(required = true)]
        issue_ids: Vec<String>,
    },

    /// Assign issue(s) to a user or team
    #[command(
        alias = "a",
        after_help = "EXAMPLES:
    sentry issues assign ISSUE-123 --to user@example.com
    sentry issues assign ISSUE-123 --to team:backend
    sentry issues assign ISSUE-123 --unassign"
    )]
    Assign {
        /// Issue ID(s) to assign
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// User email or team slug (prefix with "team:")
        #[arg(long)]
        to: Option<String>,

        /// Remove assignment instead
        #[arg(long)]
        unassign: bool,
    },

    /// Ignore issue(s)
    #[command(after_help = "EXAMPLES:
    sentry issues ignore ISSUE-123 --duration 60
    sentry issues ignore ISSUE-123 --count 100
    sentry issues ignore ISSUE-123 --until-escalating")]
    Ignore {
        /// Issue ID(s) to ignore
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// Ignore for N minutes
        #[arg(long)]
        duration: Option<u64>,

        /// Ignore until N more events
        #[arg(long)]
        count: Option<u64>,

        /// Ignore until escalating
        #[arg(long)]
        until_escalating: bool,
    },

    /// Delete issue(s)
    #[command(after_help = "EXAMPLES:
    sentry issues delete ISSUE-123 --confirm")]
    Delete {
        /// Issue ID(s) to delete
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// Skip confirmation prompt
        #[arg(long)]
        confirm: bool,
    },

    /// Merge multiple issues into one
    #[command(after_help = "EXAMPLES:
    sentry issues merge ISSUE-123 ISSUE-456 ISSUE-789")]
    Merge {
        /// Primary issue ID (issues will be merged into this one)
        primary_id: String,

        /// Other issue IDs to merge
        #[arg(required = true)]
        other_ids: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Create default config file
    #[command(after_help = "EXAMPLES:
    sentry config init")]
    Init,

    /// Display current configuration
    #[command(after_help = "EXAMPLES:
    sentry config show")]
    Show,

    /// Set a configuration value
    #[command(after_help = "EXAMPLES:
    sentry config set default_org myorg
    sentry config set auth_token sk-...")]
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
}

#[derive(Subcommand)]
pub enum ReleasesCommands {
    /// List releases
    #[command(
        alias = "ls",
        after_help = "EXAMPLES:
    sentry releases list
    sentry releases list --limit 50
    sentry releases list --all"
    )]
    List {
        /// Sentry search query string
        #[arg(long)]
        query: Option<String>,

        /// Maximum number of results per page
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Fetch all pages (may be slow for large result sets)
        #[arg(long)]
        all: bool,
    },

    /// View detailed release information
    #[command(
        alias = "show",
        alias = "v",
        after_help = "EXAMPLES:
    sentry releases view v0.90.1
    sentry releases view 1.0.0"
    )]
    View {
        /// Release version
        version: String,
    },
}

#[derive(Subcommand)]
pub enum EventsCommands {
    /// List events for an issue
    #[command(
        alias = "ls",
        after_help = "EXAMPLES:
    sentry events list ISSUE-123
    sentry events list ISSUE-123 --limit 50
    sentry events list ISSUE-123 --all"
    )]
    List {
        /// Issue ID or short ID
        issue_id: String,

        /// Maximum number of results per page
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Fetch all pages (may be slow for large result sets)
        #[arg(long)]
        all: bool,
    },

    /// View detailed event information
    #[command(
        alias = "show",
        alias = "v",
        after_help = "EXAMPLES:
    sentry events view abc123def --project myproject"
    )]
    View {
        /// Event ID
        event_id: String,

        /// Project slug (required for fetching individual events)
        #[arg(long, short)]
        project: String,
    },
}
