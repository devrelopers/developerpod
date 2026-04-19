# developerpod

developerpod is a small CLI machine that runs **kcups** — single-file TOML pods that
declare what context to gather (shell commands, files) and what to ask a model about
that context. The machine loads a kcup, runs each gatherer, interpolates the captured
values into a prompt template, calls Anthropic's Messages API with a structured-output
tool definition derived from the pod's declared schema, validates the response, and
pretty-prints the result. Pods are portable: drop a `*.kcup.toml` into a repo and any
developerpod machine can brew it.

## Install

Clone the repo and build with cargo:

```sh
git clone https://github.com/devrelopers/developerpod
cd developerpod
cargo build --release
# binary at ./target/release/developerpod
```

Set your Anthropic API key:

```sh
export ANTHROPIC_API_KEY=sk-ant-...
```

## Usage

Run a kcup by name. The machine looks for `./<name>.kcup.toml` first, then
`./examples/<name>.kcup.toml`.

```sh
developerpod repo-mood
```

A kcup looks like this (`examples/repo-mood.kcup.toml`):

```toml
name = "repo-mood"
description = "Read the current vibe of a git repo"

[[gather]]
id = "commits"
shell = "git log --oneline -20"

[[gather]]
id = "readme"
file = "README.md"
optional = true

[prompt]
system = "You read repo signals and return the current mood."
user = """
Recent commits:
{{commits}}

README:
{{readme}}
"""

[output]
schema = { mood = "string", evidence = "string", one_liner = "string" }
```

Each `[[gather]]` block produces a value bound to its `id`; `{{id}}` placeholders in
the prompt template are replaced before the call. The `[output].schema` table is
turned into a JSON Schema and supplied to the model as a forced tool call, so the
response always matches the shape you asked for.

## Status

v0.1 — initial scaffold. Supports `shell` and `file` gatherers, structured output
via Anthropic tool use against `claude-sonnet-4-6`, and a single bundled `repo-mood`
example. No `http` gatherer, no caching, no parallel gather, no pod registry yet.
