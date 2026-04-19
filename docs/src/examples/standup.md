# standup

Generate a standup report from your recent git activity. Five shell gatherers (yesterday's commits, today's commits, current branch, uncommitted changes, recently active branches) feed a prompt that returns the three classic standup fields: yesterday, today, blockers.

## The kcup

```toml
{{#include ../../../examples/standup.kcup.toml}}
```

## How it breaks down

**Gatherers**

```toml
[[gather]]
id = "yesterday_commits"
shell = "git log --since='yesterday.midnight' --until='today.midnight' --author=\"$(git config user.email)\" --pretty=format:'%h %s' --all"
```

`--all` picks up commits across every branch (so work-in-progress branches you never merged still show up). `--author="$(git config user.email)"` filters to your own commits using whatever email your local git is configured with — no hardcoding. Note the escaped double quotes around the `$(...)` so the variable expansion survives into the shell.

```toml
[[gather]]
id = "today_commits"
shell = "git log --since='today.midnight' --author=\"$(git config user.email)\" --pretty=format:'%h %s' --all"
```

Same pattern, narrowed to today.

```toml
[[gather]]
id = "current_branch"
shell = "git rev-parse --abbrev-ref HEAD"

[[gather]]
id = "status"
shell = "git status --short"

[[gather]]
id = "recent_branches"
shell = "git for-each-ref --sort=-committerdate --count=5 --format='%(refname:short) %(committerdate:relative)' refs/heads/"
```

`current_branch` and `status` together describe what you're sitting on right now — useful for the "today / blockers" framing. `recent_branches` lists the five most recently touched branches so the model can spot context-switches you might want to mention.

**Prompt + schema**

```toml
[prompt]
system = """
You generate concise standup reports from git activity. Be factual and specific — reference actual commit messages and files. Do not invent work that isn't evidenced in the data. If yesterday had no commits, say so. Keep each field to 1-3 short sentences.
"""

[output]
schema = { yesterday = "string", today = "string", blockers = "string" }
```

The system message is doing real work here: it explicitly forbids hallucination ("Do not invent work that isn't evidenced in the data") and constrains length (1–3 short sentences per field). Without those constraints the model tends to pad.

## Run it

```sh
developerpod standup
```

## Sample output

```text
▶ brewing with Anthropic (claude-sonnet-4-6) — key from ANTHROPIC_API_KEY
▶ loading examples/standup.kcup.toml
▶ pod standup — Generate a standup report from recent git activity (5 gatherers)
▣ standup
  blockers: None — current branch is clean and pushed; no uncommitted work in flight.
  today:    Started on the docs site scaffold (mdBook + GitHub Actions deploy).
  yesterday: Shipped 0.2.0 to crates.io. Auto-detect rewrite landed (9 providers,
             53 env var names) and per-provider default models refreshed to
             current April 2026 IDs.
```

## Variations to try

- **Add merged PRs**: `gh pr list --author @me --state merged --limit 5 --json title,mergedAt` — feed it as a sixth gatherer if you have `gh` installed.
- **Cross-repo**: replace the `git log` calls with a script that loops over a list of repo paths.
- **Slack-ready**: change the system message to "format the result as a single Slack-ready message under 200 words" and collapse the schema to `{ message = "string" }`.
