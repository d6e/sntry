use std::fmt;

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Compact,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum SortBy {
    #[default]
    Date,
    New,
    Freq,
    User,
    Inbox,
    Trends,
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortBy::Date => write!(f, "date"),
            SortBy::New => write!(f, "new"),
            SortBy::Freq => write!(f, "freq"),
            SortBy::User => write!(f, "user"),
            SortBy::Inbox => write!(f, "inbox"),
            SortBy::Trends => write!(f, "trends"),
        }
    }
}

#[derive(Parser)]
#[command(name = "sntry")]
#[command(about = "CLI tool for managing Sentry issues", long_about = None)]
#[command(version)]
#[command(after_help = "EXAMPLES:
    sntry issues list --project myproject
    sntry issues view ISSUE-123
    sntry issues resolve ISSUE-123
    sntry config show")]
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
    sntry issues list --project myproject
    sntry issues list --status unresolved --limit 50
    sntry issues view ISSUE-123
    sntry issues resolve ISSUE-123 ISSUE-456"
    )]
    Issues {
        #[command(subcommand)]
        command: IssuesCommands,
    },
    /// Manage releases
    #[command(
        alias = "r",
        after_help = "EXAMPLES:
    sntry releases list
    sntry releases list --limit 50
    sntry releases view v0.90.1"
    )]
    Releases {
        #[command(subcommand)]
        command: ReleasesCommands,
    },
    /// View error events
    #[command(
        alias = "e",
        after_help = "EXAMPLES:
    sntry events list ISSUE-123
    sntry events list ISSUE-123 --limit 50
    sntry events view abc123def --project myproject"
    )]
    Events {
        #[command(subcommand)]
        command: EventsCommands,
    },
    /// Manage CLI configuration
    #[command(
        alias = "cfg",
        after_help = "EXAMPLES:
    sntry config init
    sntry config show
    sntry config set default_org myorg"
    )]
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Generate shell completions
    #[command(after_help = "EXAMPLES:
    sntry completions bash > ~/.bash_completion.d/sntry
    sntry completions zsh > ~/.zfunc/_sntry
    sntry completions fish > ~/.config/fish/completions/sntry.fish")]
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
    sntry issues list
    sntry issues list --project myproject --status unresolved
    sntry issues list --query \"is:unresolved\" --limit 100
    sntry issues list --environment production --period 14d
    sntry issues list --start 2024-01-01T00:00:00Z --end 2024-01-31T23:59:59Z
    sntry issues list --sort user --environment production,staging"
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

        /// Sort by: date, new, freq, user, inbox, trends
        #[arg(long, value_enum, default_value = "date")]
        sort: SortBy,

        /// Maximum number of results per page
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Fetch all pages (may be slow for large result sets)
        #[arg(long)]
        all: bool,

        /// Filter by environment(s), comma-separated
        #[arg(long, short = 'e')]
        environment: Option<String>,

        /// Stats period (e.g. 24h, 14d, 30d)
        #[arg(long)]
        period: Option<String>,

        /// Start of date range (ISO-8601)
        #[arg(long, conflicts_with = "period")]
        start: Option<String>,

        /// End of date range (ISO-8601)
        #[arg(long, conflicts_with = "period", requires = "start")]
        end: Option<String>,
    },

    /// View detailed issue information
    #[command(
        alias = "show",
        alias = "v",
        after_help = "EXAMPLES:
    sntry issues view ISSUE-123
    sntry issues view 12345678"
    )]
    View {
        /// Issue ID or short ID
        issue_id: String,
    },

    /// Resolve one or more issues
    #[command(
        alias = "r",
        after_help = "EXAMPLES:
    sntry issues resolve ISSUE-123
    sntry issues resolve ISSUE-123 ISSUE-456 --in-next-release"
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
    sntry issues unresolve ISSUE-123")]
    Unresolve {
        /// Issue ID(s) to unresolve
        #[arg(required = true)]
        issue_ids: Vec<String>,
    },

    /// Assign issue(s) to a user or team
    #[command(
        alias = "a",
        after_help = "EXAMPLES:
    sntry issues assign ISSUE-123 --to user@example.com
    sntry issues assign ISSUE-123 --to team:backend
    sntry issues assign ISSUE-123 --unassign"
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
    sntry issues ignore ISSUE-123 --duration 60
    sntry issues ignore ISSUE-123 --count 100
    sntry issues ignore ISSUE-123 --until-escalating")]
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
    sntry issues delete ISSUE-123 --confirm")]
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
    sntry issues merge ISSUE-123 ISSUE-456 ISSUE-789")]
    Merge {
        /// Primary issue ID (issues will be merged into this one)
        primary_id: String,

        /// Other issue IDs to merge
        #[arg(required = true)]
        other_ids: Vec<String>,
    },

    /// Add a comment (note) to an issue
    #[command(after_help = "EXAMPLES:\n    sntry issues comment ISSUE-123 \"Tracked in PRG-456\"\n    sntry issues comment ISSUE-123 --list")]
    Comment {
        /// Issue ID
        issue_id: String,

        /// Comment text (omit to list comments)
        text: Option<String>,

        /// List comments instead of adding one
        #[arg(long)]
        list: bool,
    },

    /// View tag distribution for an issue
    #[command(
        after_help = "EXAMPLES:\n    sntry issues tags ISSUE-123 environment\n    sntry issues tags 7207273243 autoslay"
    )]
    Tags {
        /// Issue ID or short ID
        issue_id: String,
        /// Tag key to query
        tag_key: String,
    },

    /// List external issue links for an issue
    #[command(
        alias = "ls-links",
        after_help = "EXAMPLES:
    sntry issues links ISSUE-123
    sntry issues links 7207273243"
    )]
    Links {
        /// Issue ID or short ID
        issue_id: String,
    },

    /// Link issue(s) to an external issue tracker
    #[command(after_help = "EXAMPLES:
    sntry issues link 7207273243 --url https://linear.app/megacrit/issue/PRG-1234/title
    sntry issues link 123 456 --url https://linear.app/megacrit/issue/PRG-1234/title
    sntry issues link 123 --url https://example.com/issue/1 --project FOO --identifier FOO-1
    sntry issues link 123 --url https://example.com/issue/1 --project FOO --identifier FOO-1 --integration jira")]
    Link {
        /// Issue ID(s) to link
        #[arg(required = true)]
        issue_ids: Vec<String>,

        /// URL of the external issue
        #[arg(long)]
        url: String,

        /// External project key (inferred from Linear URLs)
        #[arg(long)]
        project: Option<String>,

        /// External issue identifier (inferred from Linear URLs)
        #[arg(long)]
        identifier: Option<String>,

        /// Integration app slug
        #[arg(long, default_value = "linear")]
        integration: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Create default config file
    #[command(after_help = "EXAMPLES:
    sntry config init")]
    Init,

    /// Display current configuration
    #[command(after_help = "EXAMPLES:
    sntry config show")]
    Show,

    /// Set a configuration value
    #[command(after_help = "EXAMPLES:
    sntry config set default_org myorg
    sntry config set auth_token sk-...")]
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
    sntry releases list
    sntry releases list --limit 50
    sntry releases list --all"
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
    sntry releases view v0.90.1
    sntry releases view 1.0.0"
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
    sntry events list ISSUE-123
    sntry events list ISSUE-123 --limit 50
    sntry events list ISSUE-123 --all"
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
    sntry events view abc123def --project myproject"
    )]
    View {
        /// Event ID
        event_id: String,

        /// Project slug (required for fetching individual events)
        #[arg(long, short)]
        project: String,
    },

    /// List or download event attachments (e.g. minidumps)
    #[command(
        alias = "att",
        after_help = "EXAMPLES:
    sntry events attachments abc123def --project myproject
    sntry events attachments abc123def --project myproject --download --output /tmp/crash.dmp
    sntry events attachments abc123def --project myproject --download --id 12345 --output /tmp/"
    )]
    Attachments {
        /// Event ID
        event_id: String,

        /// Project slug
        #[arg(long, short)]
        project: String,

        /// Download attachment (default: first minidump)
        #[arg(long, short)]
        download: bool,

        /// Output path (file or directory)
        #[arg(long, default_value = ".")]
        output: String,

        /// Specific attachment ID to download
        #[arg(long)]
        id: Option<String>,
    },
}
