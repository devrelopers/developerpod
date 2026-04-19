# repo-mood

Read the current vibe of a git repo. Two gatherers — recent commits and the README — feed a single prompt that returns a one-line mood, supporting evidence, and a one-liner.

## The kcup

```toml
{{#include ../../../examples/repo-mood.kcup.toml}}
```

## How it breaks down

**Identity**

```toml
name = "repo-mood"
description = "Read the current vibe of a git repo"
```

`name` matches the lookup path: `developerpod repo-mood` resolves to `./repo-mood.kcup.toml` or `./examples/repo-mood.kcup.toml`. `description` is printed on each run.

**Gatherers**

```toml
[[gather]]
id = "commits"
shell = "git log --oneline -20"

[[gather]]
id = "readme"
file = "README.md"
optional = true
```

The `commits` gatherer runs the `git log` command via `sh -c` and captures stdout. The `readme` gatherer reads `README.md` as text; `optional = true` means a missing README produces an empty string instead of an error — which is what you want for a tool you might point at any repo.

**Prompt**

```toml
[prompt]
system = "You read repo signals and return the current mood."
user = """
Recent commits:
{{commits}}

README:
{{readme}}
"""
```

`{{commits}}` and `{{readme}}` are replaced with the gathered values before the call. The system message sets the framing; the user message hands over the labeled context.

**Output schema**

```toml
[output]
schema = { mood = "string", evidence = "string", one_liner = "string" }
```

Three string fields. Whichever provider is detected, developerpod converts this into that provider's structured-output mechanism (forced tool use, `response_format`, `responseSchema`, etc.) so the model returns exactly these fields.

## Run it

```sh
developerpod repo-mood
```

## Sample output

```text
▶ brewing with Anthropic (claude-sonnet-4-6) — key from ANTHROPIC_API_KEY
▶ loading examples/repo-mood.kcup.toml
▶ pod repo-mood — Read the current vibe of a git repo (2 gatherers)
▣ repo-mood
  evidence: Fresh scaffold landed in rapid succession — initial commit,
            provider auto-detection across 9 providers, model refresh, env-var
            expansion. README is thorough and self-described as v0.1 with a
            clear missing-features list.
  mood: energized early-stage
  one_liner: Fresh scaffold with real ambition — the foundation is solid and the roadmap is already visible in what's missing.
```
