# sntry

[![CI](https://github.com/d6e/sntry/actions/workflows/ci.yml/badge.svg)](https://github.com/d6e/sntry/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line tool for managing Sentry issues.

## Installation

### From source

```bash
cargo install --path .
```

### Build from source

```bash
git clone https://github.com/d6e/sntry
cd sntry
cargo build --release
```

The binary will be at `target/release/sntry`.

## Configuration

### Authentication

Set your Sentry auth token via environment variable (recommended):

```bash
export SENTRY_AUTH_TOKEN="sntrys_..."
export SENTRY_ORG="your-organization"
```

Or create a config file:

```bash
sntry config init
# Edit ~/.config/sntry/config.toml
```

Config file format (`~/.config/sntry/config.toml`):

```toml
default_org = "your-organization"
server_url = "https://sentry.io"  # or your self-hosted URL
auth_token = "sntrys_..."
default_project = "your-project"
```

**Priority order**: CLI flags > environment variables > config file > defaults

### Self-hosted Sentry

For self-hosted Sentry instances:

```bash
sntry --server https://sentry.yourcompany.com issues list
```

Or set in config:

```toml
server_url = "https://sentry.yourcompany.com"
```

## Usage

### List Issues

```bash
# List all unresolved issues
sntry issues list

# Filter by project
sntry issues list --project backend

# Filter by status
sntry issues list --status resolved

# Custom search query
sntry issues list --query "is:unresolved level:error"

# JSON output (for scripting)
sntry issues list --output json

# Fetch all pages
sntry issues list --all --limit 100
```

### View Issue Details

```bash
sntry issues view 1234567890
sntry issues view PROJ-123

# JSON output
sntry issues view 1234567890 --output json
```

### Resolve Issues

```bash
# Resolve a single issue
sntry issues resolve 1234567890

# Resolve multiple issues
sntry issues resolve 1234567890 1234567891

# Resolve in specific release
sntry issues resolve 1234567890 --in-release 1.2.3

# Resolve in next release
sntry issues resolve 1234567890 --in-next-release
```

### Unresolve Issues

```bash
sntry issues unresolve 1234567890
```

### Assign Issues

```bash
# Assign to user
sntry issues assign 1234567890 --to user@example.com

# Assign to team
sntry issues assign 1234567890 --to team:backend

# Unassign
sntry issues assign 1234567890 --unassign
```

### Ignore Issues

```bash
# Ignore indefinitely
sntry issues ignore 1234567890

# Ignore for 24 hours (1440 minutes)
sntry issues ignore 1234567890 --duration 1440

# Ignore until 100 more events
sntry issues ignore 1234567890 --count 100

# Ignore until escalating
sntry issues ignore 1234567890 --until-escalating
```

### Delete Issues

```bash
# Delete with confirmation prompt
sntry issues delete 1234567890

# Skip confirmation
sntry issues delete 1234567890 --confirm

# Delete multiple
sntry issues delete 1234567890 1234567891 --confirm
```

### Merge Issues

```bash
# Merge issues into a primary issue
sntry issues merge 1234567890 1234567891 1234567892
```

### Configuration Management

```bash
# Create config file
sntry config init

# Show current config
sntry config show

# Set config values
sntry config set default_org my-org
sntry config set auth_token sntrys_...
```

## Global Options

```
--server <URL>     Sentry server URL (default: https://sentry.io)
--org <ORG>        Organization slug
--token <TOKEN>    Auth token
-v, --verbose      Enable verbose output (shows API requests)
-h, --help         Print help
-V, --version      Print version
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `SENTRY_AUTH_TOKEN` | Authentication token |
| `SENTRY_ORG` | Default organization slug |
| `SENTRY_SERVER_URL` | Sentry server URL |
| `SENTRY_PROJECT` | Default project slug |

## Examples

### Scripting with JSON output

```bash
# Get all issue IDs
sntry issues list --output json | jq -r '.[].id'

# Count issues by status
sntry issues list --all --output json | jq 'group_by(.status) | map({status: .[0].status, count: length})'

# Resolve all issues matching a query
sntry issues list --query "is:unresolved browser:Chrome" --output json | \
  jq -r '.[].id' | \
  xargs sntry issues resolve
```

### Verbose mode for debugging

```bash
sntry -v issues list
# [verbose] Server: https://sentry.io/
# [verbose] Organization: my-org
# [verbose] GET https://sentry.io/api/0/organizations/my-org/issues/...
# [verbose] Response: 200 OK
```

## License

MIT
