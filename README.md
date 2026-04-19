# developerpod

developerpod is a small CLI machine that runs **kcups** — single-file TOML pods that
declare what context to gather (shell commands, files) and what to ask a model about
that context. The machine loads a kcup, runs each gatherer, interpolates the captured
values into a prompt template, auto-detects an AI provider from your environment,
calls that provider's API with a structured-output request derived from the pod's
declared schema, validates the response, and pretty-prints the result. Pods are
portable: drop a `*.kcup.toml` into a repo and any developerpod machine can brew it.

## Install

Clone the repo and build with cargo:

```sh
git clone https://github.com/devrelopers/developerpod
cd developerpod
cargo build --release
# binary at ./target/release/developerpod
```

Set any one of the supported provider API keys (see **Providers** below) — the
machine auto-detects which one is present:

```sh
export ANTHROPIC_API_KEY=sk-ant-...
# or OPENAI_API_KEY, GEMINI_API_KEY, GROQ_API_KEY, …
```

## Usage

Run a kcup by name. The machine looks for `./<name>.kcup.toml` first, then
`./examples/<name>.kcup.toml`.

```sh
developerpod repo-mood
developerpod repo-mood --provider openai          # force a provider
developerpod repo-mood --provider google --model gemini-1.5-pro  # force model too
```

On startup, developerpod prints which provider, model, and env var it picked up,
e.g. `▶ brewing with Anthropic (claude-sonnet-4-6) — key from ANTHROPIC_API_KEY`.

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

## Providers

developerpod scans your environment at startup and uses the first provider whose
API key is set. Detection order is fixed; the first match wins. Override with
`--provider <id>` and/or `--model <name>`.

| # | Provider   | `--provider` id | Default model                        | Env vars scanned (in order)                                                                                                                                              |
|---|------------|-----------------|--------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1 | Anthropic  | `anthropic`     | `claude-sonnet-4-6`                  | `ANTHROPIC_API_KEY`, `CLAUDE_API_KEY`, `ANTHROPIC_KEY`, `CLAUDE_KEY`, `ANTHROPIC_API_TOKEN`, `CLAUDE_API_TOKEN`                                                          |
| 2 | OpenAI     | `openai`        | `gpt-5.4`                            | `OPENAI_API_KEY`, `CHATGPT_API_KEY`, `OPENAI_KEY`, `CHATGPT_KEY`, `GPT_API_KEY`, `GPT_KEY`, `OPENAI_API_TOKEN`, `OPENAI_TOKEN`, `CHATGPT_API_TOKEN`                      |
| 3 | Google     | `google`        | `gemini-2.5-flash`                   | `GEMINI_API_KEY`, `GOOGLE_API_KEY`, `GOOGLE_GENERATIVE_AI_API_KEY`, `GOOGLE_AI_API_KEY`, `GOOGLE_GENAI_API_KEY`, `GOOGLE_GEMINI_API_KEY`, `GEMINI_KEY`                   |
| 4 | Groq       | `groq`          | `llama-3.3-70b-versatile`            | `GROQ_API_KEY`, `GROQ_KEY`, `GROQ_API_TOKEN`                                                                                                                             |
| 5 | Mistral    | `mistral`       | `mistral-large-latest`               | `MISTRAL_API_KEY`, `MISTRALAI_API_KEY`, `MISTRAL_KEY`, `MISTRAL_API_TOKEN`                                                                                               |
| 6 | Cohere     | `cohere`        | `command-a-03-2025`                  | `COHERE_API_KEY`, `CO_API_KEY`, `COHERE_KEY`, `CO_KEY`, `COHERE_API_TOKEN`                                                                                               |
| 7 | DeepSeek   | `deepseek`      | `deepseek-chat`                      | `DEEPSEEK_API_KEY`, `DEEPSEEK_KEY`, `DEEPSEEK_API_TOKEN`, `DEEPSEEK_TOKEN`                                                                                               |
| 8 | xAI        | `xai`           | `grok-4-1-fast-non-reasoning`        | `XAI_API_KEY`, `GROK_API_KEY`, `XAI_KEY`, `GROK_KEY`, `XAI_API_TOKEN`, `GROK_API_TOKEN`                                                                                  |
| 9 | OpenRouter | `openrouter`    | `anthropic/claude-sonnet-4.6`        | `OPENROUTER_API_KEY`, `OPEN_ROUTER_API_KEY`, `OPENROUTER_KEY`, `OPENROUTER_API_TOKEN`                                                                                    |

The list mixes official names, community conventions (`CLAUDE_API_KEY`, `CHATGPT_API_KEY`, `GROK_API_KEY`), tool-specific defaults (e.g. `GOOGLE_GENERATIVE_AI_API_KEY` from the Vercel AI SDK, `CO_API_KEY` from the Cohere SDK), and `*_API_TOKEN` / `*_TOKEN` variants. If your key uses a name not on this list, set it under any of the listed names or pass `--provider <id>` and any one of them.

Structured output is requested in each provider's native idiom: Anthropic forced
tool use, OpenAI/Groq/DeepSeek/xAI/OpenRouter `response_format: json_schema`,
Google `generationConfig.responseSchema`, Mistral `json_object` (with the schema
inlined into the system prompt), and Cohere v2 `response_format: json_object`
with `json_schema`. The same `[output].schema` table in your kcup drives all of
them.

If no key is found, the machine exits and lists every env var it scanned, grouped
by provider, so you can see what to set.

## Status

v0.1 — initial scaffold. Supports `shell` and `file` gatherers, auto-detects across
9 AI providers, and ships with a bundled `repo-mood` example. No `http` gatherer,
no caching, no parallel gather, no pod registry yet.
