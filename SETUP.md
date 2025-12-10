# Demo Setup Guide

This document walks through setting up the repository state **before** the demo. The goal is to create a realistic merge conflict scenario that showcases jj's advantages.

## Prerequisites

- Rust toolchain (`rustup`)
- jj installed (`cargo install jj-cli` or via package manager)
- git installed
- GitHub CLI (`gh`) for PR creation (optional)

## Overview of What We're Building

A simple Rust CLI that prints a greeting. We'll create divergent histories:

```
main:    [init] -> [add clap] -> [add --user with clap]
                \
feature:         -> [parse --json manually] -> [format output as JSON]
```

The conflict arises because `main` refactors to clap while `feature` adds manual arg parsing.

---

## Step 1: Initialize the Project

```bash
# Create new Rust project
cargo init jj-conflict-demo
cd jj-conflict-demo

# Initialize as colocated jj+git repo
jj git init --colocate
```

## Step 2: Create Initial Commit on Main

Create a minimal CLI with hardcoded output:

**src/main.rs**
```rust
fn main() {
    println!("Hello, world!");
}
```

**Cargo.toml** (should already exist, just verify):
```toml
[package]
name = "jj-conflict-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Commit this baseline:

```bash
jj describe -m "Initial commit: basic hello world CLI"
jj new  # Create new empty change to work from
jj bookmark set main -r @-  # Point main at the initial commit
```

## Step 3: Create Feature Branch (2 changesets)

### Changeset 1: Parse --json flag manually

```bash
jj new main -m "Add manual --json flag parsing"
jj bookmark create feature
```

**src/main.rs**
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_output = args.iter().any(|arg| arg == "--json");

    if json_output {
        println!("{{\"message\": \"Hello, world!\"}}");
    } else {
        println!("Hello, world!");
    }
}
```

```bash
jj describe -m "Add manual --json flag parsing"
```

### Changeset 2: Improve JSON formatting

```bash
jj new -m "Improve JSON output formatting"
```

**src/main.rs**
```rust
use std::env;

struct Output {
    message: String,
}

impl Output {
    fn to_json(&self) -> String {
        format!("{{\"message\": \"{}\"}}", self.message)
    }

    fn to_text(&self) -> String {
        self.message.clone()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_output = args.iter().any(|arg| arg == "--json");

    let output = Output {
        message: "Hello, world!".to_string(),
    };

    if json_output {
        println!("{}", output.to_json());
    } else {
        println!("{}", output.to_text());
    }
}
```

```bash
jj describe -m "Refactor: extract Output struct with formatting methods"
```

Update the feature bookmark:

```bash
jj bookmark set feature
```

## Step 4: Advance Main with Clap

### Changeset 1: Add clap dependency

```bash
jj new main -m "Add clap for argument parsing"
```

**Cargo.toml**
```toml
[package]
name = "jj-conflict-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
```

**src/main.rs**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A friendly greeting CLI")]
struct Args {
    // No arguments yet, just setting up clap
}

fn main() {
    let _args = Args::parse();
    println!("Hello, world!");
}
```

```bash
jj describe -m "Add clap for CLI argument parsing"
jj bookmark set main
```

### Changeset 2: Add --user flag with clap

```bash
jj new -m "Add --user flag"
```

**src/main.rs**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A friendly greeting CLI")]
struct Args {
    /// Name of the user to greet
    #[arg(short, long, default_value = "world")]
    user: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.user);
}
```

```bash
jj describe -m "Add --user flag to customize greeting"
jj bookmark set main
```

## Step 5: Push to GitHub for PR Scenario

```bash
# Create the GitHub repo (if not exists)
gh repo create jj-conflict-demo --public --source=. --push

# Push both branches
jj git push --bookmark main
jj git push --bookmark feature

# Create PR
gh pr create --base main --head feature --title "Add JSON output support" --body "Adds --json flag for JSON-formatted output"
```

## Step 6: Verify the Setup

Check that we have the expected divergent history:

```bash
jj log
```

Expected output (approximately):
```
@  qpvuntsm  [email]  [timestamp]  feature
│  Refactor: extract Output struct with formatting methods
○  kkmpptxz  [email]  [timestamp]
│  Add manual --json flag parsing
│ ○  zskvmrvl  [email]  [timestamp]  main
│ │  Add --user flag to customize greeting
│ ○  yostqsxw  [email]  [timestamp]
├─╯  Add clap for CLI argument parsing
○  rlvkpnrz  [email]  [timestamp]
│  Initial commit: basic hello world CLI
◆  zzzzzzzz  root()
```

Verify both branches work independently:

```bash
# Test main
jj new main
cargo run -- --user "Developer"  # Should print "Hello, Developer!"
jj abandon @

# Test feature
jj new feature
cargo run -- --json  # Should print JSON
jj abandon @
```

## Step 7: Reset to Demo Starting Point

Before the demo, make sure you're in a clean state:

```bash
jj new main  # Start fresh working copy
```

---

## What Conflicts Will Occur

When rebasing `feature` onto `main`, both commits will conflict:

1. **First commit** (manual --json parsing): Conflicts with clap setup in main.rs
2. **Second commit** (Output struct): Also conflicts because it builds on the manual parsing

This creates the perfect scenario to demonstrate:
- jj's per-commit conflict markers
- Ability to continue working with unresolved conflicts
- Clean conflict resolution workflow
- The `jj undo` safety net

---

## Optional: Create a "Checkpoint" Branch

If you want to quickly reset between demo runs:

```bash
jj bookmark create demo-start -r main
```

To reset:
```bash
jj new demo-start
jj bookmark set main -r demo-start
# Re-push if needed
```
