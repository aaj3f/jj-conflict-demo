# Demo Presentation Guide

This document is your script for walking colleagues through the jj conflict resolution demo.

---

## Opening: Set the Stage (2 min)

### Talking Points

> "We've all been there: you're working on a feature branch, ready to merge, and then... conflicts. In git, this often means entering a weird detached state, fixing everything at once, hoping you don't mess up, and praying `git reflog` can save you if things go wrong."
>
> "jj approaches this completely differently. Let me show you."

### Show the Current State

```bash
jj log
```

Explain the history:
- "Here's our feature branch - it adds JSON output using manual argument parsing"
- "Meanwhile, main has moved forward - someone added clap for CLI args and a --user flag"
- "These branches have diverged. Let's see what happens when we try to rebase."

---

## Part 1: The Rebase (3 min)

### Attempt the Rebase

```bash
jj rebase -s feature -d main
```

### Key Observation: No Panic, No Blocking

> "Notice what just happened. In git, we'd be stuck in 'rebase in progress' limbo. But jj just... did it. The conflicts exist, but they're **stored in the commits themselves**, not blocking our repository."

Show the log again:

```bash
jj log
```

Point out the conflict markers in the log output (commits will show as having conflicts).

### Show Conflicts Are Data, Not State

```bash
jj status
```

> "See? We have a normal working copy. We're not in any special state. The conflicts are just... there, waiting to be resolved."

---

## Part 2: jj's Killer Feature - Keep Working (3 min)

### Demonstrate Non-Blocking Workflow

> "Here's where jj shines. In git, you'd have to stop everything and resolve conflicts before doing anything else. Watch this:"

```bash
jj new main -m "Urgent hotfix (unrelated to feature work)"
```

> "I just created a new commit off main. With unresolved conflicts sitting in my feature branch. Try doing that in git!"

```bash
jj log
```

Show that you now have:
- The conflicted feature commits
- A new clean commit for other work

```bash
# Clean up the demo commit
jj abandon @
```

---

## Part 3: Actually Resolving Conflicts (5 min)

### Navigate to First Conflicted Commit

```bash
jj log  # Find the first conflicted commit (the --json parsing one)
jj edit <commit-id>  # Or use jj next/prev
```

### Show the Conflict Markers

```bash
cat src/main.rs
```

> "jj uses different conflict markers than git. They're actually more informative - you can see the base, each side, and jj handles multi-way conflicts gracefully."

Explain the markers:
```
<<<<<<< Conflict 1 of 1
%%%%%%% Changes from base to side #1
-old line
+new line from side 1
+++++++ Contents of side #2
new line from side 2
>>>>>>>
```

### Resolve the Conflict

Edit the file to combine both approaches - use clap AND support --json:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A friendly greeting CLI")]
struct Args {
    /// Name of the user to greet
    #[arg(short, long, default_value = "world")]
    user: String,

    /// Output in JSON format
    #[arg(long)]
    json: bool,
}

fn main() {
    let args = Args::parse();

    if args.json {
        println!("{{\"message\": \"Hello, {}!\"}}", args.user);
    } else {
        println!("Hello, {}!", args.user);
    }
}
```

```bash
jj status  # Should show no more conflicts in this commit
jj describe -m "Add --json flag using clap"
```

### Move to Second Conflicted Commit

```bash
jj log  # Find the second conflicted commit
jj edit <commit-id>
```

> "Now we resolve the second commit. Notice each commit is handled independently - we're not juggling a massive diff of everything at once."

Resolve by evolving the Output struct to work with the new args:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A friendly greeting CLI")]
struct Args {
    /// Name of the user to greet
    #[arg(short, long, default_value = "world")]
    user: String,

    /// Output in JSON format
    #[arg(long)]
    json: bool,
}

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
    let args = Args::parse();

    let output = Output {
        message: format!("Hello, {}!", args.user),
    };

    if args.json {
        println!("{}", output.to_json());
    } else {
        println!("{}", output.to_text());
    }
}
```

```bash
jj status  # All conflicts resolved
jj describe -m "Refactor: extract Output struct with formatting methods"
```

---

## Part 4: The Safety Net (2 min)

### Show the Operation Log

```bash
jj op log
```

> "Every single operation in jj is recorded. This is your time machine."

### Demonstrate Undo

> "Let's say I don't like how I resolved those conflicts. In git, I'd have to figure out reflog, maybe force-push, and hope for the best. In jj:"

```bash
jj undo
```

> "That's it. The entire rebase is undone. My feature branch is back to its pre-rebase state."

```bash
jj log  # Show it's back to diverged state
```

### Redo for the Demo

```bash
jj op restore <operation-id>  # Restore to post-resolution state
```

---

## Part 5: Comparison with Git (2 min)

### Side-by-Side Talking Points

| Situation | Git | jj |
|-----------|-----|-----|
| Mid-rebase, need to do something else | Stash everything, abort rebase, do thing, restart rebase | Just `jj new` and work elsewhere |
| Made a mistake during conflict resolution | Hope reflog has it, `reset --hard`, start over | `jj undo` |
| Want to see what conflicts look like before resolving | You're already in it, no preview | `jj rebase` then inspect at leisure |
| Complex multi-commit rebase | All conflicts dumped on you at once (unless interactive) | Each commit handled independently |
| Conflict markers | Basic `<<<` / `===` / `>>>` | Shows base + both sides, handles multi-way |

---

## Bonus Demos (If Time Permits)

### Bonus 1: Splitting Commits

```bash
jj split  # Interactively split current commit
```

> "Ever made a commit that should have been two? In git, this is `rebase -i` with `edit`, painful. In jj, one command."

### Bonus 2: Working Copy Is Always a Commit

```bash
jj status
jj log
```

> "Notice there's no 'staged' vs 'unstaged'. Your working copy IS a commit. Always. This eliminates an entire class of 'I forgot to add that file' bugs."

### Bonus 3: Squash Up

```bash
jj squash  # Squash current commit into parent
```

> "Like `git commit --amend` but for any commit, not just HEAD."

---

## Closing (1 min)

### Key Takeaways

1. **Conflicts are data, not state** - You're never "stuck" in a rebase
2. **Every operation is undoable** - `jj undo` is your safety net
3. **Git-compatible** - This is a real git repo, push/pull works normally
4. **Mental model is simpler** - Working copy = commit, no staging area confusion

### Call to Action

> "jj is production-ready and git-compatible. You can start using it today on existing repos with `jj git init --colocate`. Your teammates using git won't even notice."

### Resources

- [jj GitHub](https://github.com/martinvonz/jj)
- [jj Tutorial](https://martinvonz.github.io/jj/latest/tutorial/)
- [Steve Klabnik's "jj init"](https://steveklabnik.com/writing/jj-init)

---

## Troubleshooting

### If Conflicts Don't Appear as Expected

```bash
jj log -r 'all()'  # See all commits
jj log -r 'conflicts()'  # See only conflicted commits
```

### If You Need to Reset Mid-Demo

```bash
jj op log  # Find the operation before you started
jj op restore <op-id>
```

### If Git State Gets Confused

```bash
jj git export  # Sync jj state to git
```
