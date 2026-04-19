# Writing Your Own Kcup

The whole point of developerpod is that a useful dev tool can be a single TOML file. Here's how to write one.

## 1. Pick the smallest useful question

A good kcup answers one specific question that needs a model in the loop — interpretation, summarization, judgment, classification — using context that's awkward to assemble by hand. If you can answer it with a regex, you don't need a kcup.

Some prompts that work well:

- "What's the actual change in this diff, ignoring the rename noise?"
- "Which of these failing tests are flaky vs. real?"
- "Draft a release-notes paragraph from these commits."
- "Read this README and tell me what the project actually does."

## 2. Start with one shell gatherer

Resist the urge to gather everything up front. Get one signal flowing first.

```toml
name = "scratch"
description = "Scratch pad — replace me"

[[gather]]
id = "input"
shell = "git log --oneline -10"

[prompt]
system = "You summarize git logs in one sentence."
user = "{{input}}"

[output]
schema = { summary = "string" }
```

Save as `scratch.kcup.toml` and run `developerpod scratch`. If that works, you have the loop end-to-end.

## 3. Add the gatherers you actually need

Each gatherer is either a `shell` command or a `file` read. Add them one at a time and reference them in the prompt with `{{id}}`:

```toml
[[gather]]
id = "diff"
shell = "git diff HEAD~1"

[[gather]]
id = "readme"
file = "README.md"
optional = true       # don't fail the run if it's missing
```

Mark anything that might not exist as `optional = true`.

## 4. Tighten the prompt

Once context is flowing, tune the prompt:

- **System message**: behavior, voice, format constraints. Keep it short.
- **User template**: the actual data, with labels so the model knows what each chunk is.

```toml
[prompt]
system = "You read git diffs and identify the smallest accurate description of the change."
user = """
Diff:
{{diff}}

README (for project context):
{{readme}}
"""
```

## 5. Declare a real schema

The schema does double duty: it tells the provider what shape to return (so you don't get prose when you wanted a list), and it's checked locally before printing.

```toml
[output]
schema = { headline = "string", details = "string", risk_level = "string" }
```

Use `array` if you genuinely want a list, `object` for nested data, `boolean` for yes/no questions. The full type list is in [The Kcup Format](./kcup-format.md#supported-schema-types).

## 6. Iterate

Run, read the output, adjust the prompt. The fastest improvements usually come from:

- Adding a missing piece of context as another gatherer.
- Renaming schema fields so the model knows what each one means.
- Tightening the system message — "be terse", "cite shas", "avoid hedging", whatever you actually want.

You're done when the output is the thing you would have written yourself.

## Kcups vs. agent skills

Kcups look superficially similar to **agent skills** (Claude Code skills, OpenAI Assistants instructions, etc.) — both are file-based ways to package "how to use a model for a thing" without writing application code. The difference is in *who reads the file* and *when the work happens*.

|                          | **Kcup**                                                                 | **Agent skill**                                                          |
|--------------------------|--------------------------------------------------------------------------|--------------------------------------------------------------------------|
| Read by                  | The `developerpod` CLI (deterministic).                                  | The agent (an LLM) deciding what to do next.                             |
| Invoked                  | Explicitly: `developerpod <name>`.                                       | Implicitly, when the agent judges the skill is relevant to the request. |
| Shape                    | TOML with declared sections and a typed output schema.                   | Markdown prose describing capability, triggers, and guidance.            |
| Control flow             | One pass: gather → prompt → structured output → exit.                    | Multi-turn loop: the agent picks tools, branches, asks follow-ups.       |
| Side effects             | None. Kcups gather and print; they don't take actions.                   | Whatever the agent's tools allow — file edits, shell, API calls, etc.    |
| State across calls       | None.                                                                    | The agent carries conversation state.                                    |
| Runtime requirement      | The `developerpod` binary + an API key.                                  | An agent harness (Claude Code, Claude API agent loop, Cursor, etc.).     |
| Output                   | A JSON object matching a declared schema, pretty-printed.                | Whatever the agent decides to say or do next.                            |

**Reach for an agent skill when** the task needs judgment about *what to do next* — choosing among tools, deciding when something's done, recovering from errors, handling a back-and-forth.

**Reach for a kcup when** the task is "look at this specific context, give me back this specific shape" and you want to call it from a Makefile, a git hook, CI, or a one-line shell alias. Kcups are scripts, not assistants.

The two compose: a kcup makes a great pre-step that hands a structured result to an agent, and an agent can absolutely shell out to `developerpod <name>` mid-loop when it needs an opinion on something concrete.

## When *not* to write a kcup

- The answer doesn't need a model. Use a shell script.
- You need to take an action based on the output, not just read it. Kcups print; they don't act. Pipe the JSON elsewhere if you need to.
- The context exceeds what you want to send to a model. Kcups gather everything before the call — they don't paginate or retrieve.
- The right tool is an agent skill (see above).
