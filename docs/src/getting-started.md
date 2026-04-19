# Getting Started

## Install

```sh
cargo install developerpod
```

This puts a `developerpod` binary on your `$PATH`.

## Set an API key

Developerpod auto-detects which provider to use by scanning your environment. Set any one of the supported keys — for example:

```sh
export ANTHROPIC_API_KEY=sk-ant-...
# or OPENAI_API_KEY, GEMINI_API_KEY, GROQ_API_KEY, MISTRAL_API_KEY,
#    COHERE_API_KEY, DEEPSEEK_API_KEY, XAI_API_KEY, OPENROUTER_API_KEY,
#    or any of ~50 other accepted aliases — see the Providers chapter.
```

The first key found wins, in priority order. If none are set, developerpod prints the full list of every variable it scanned, grouped by provider, and exits.

## Run a kcup

The repo ships with a couple of example kcups. The simplest is `repo-mood`:

```sh
git clone https://github.com/DevRelopers/developerpod
cd developerpod
developerpod repo-mood
```

The machine looks for `./<name>.kcup.toml` first, then `./examples/<name>.kcup.toml`, so this picks up `examples/repo-mood.kcup.toml`. You'll see something like:

```text
▶ brewing with Anthropic (claude-sonnet-4-6) — key from ANTHROPIC_API_KEY
▶ loading examples/repo-mood.kcup.toml
▶ pod repo-mood — Read the current vibe of a git repo (2 gatherers)
▣ repo-mood
  evidence: …
  mood: …
  one_liner: …
```

That's the full loop: gather → interpolate → call model → validate → print.

## Next steps

- Browse the [example kcups](./examples/repo-mood.md) to see what's possible.
- Read [The Kcup Format](./kcup-format.md) for the TOML schema.
- Write your own — see [Writing Your Own Kcup](./writing-your-own.md).
