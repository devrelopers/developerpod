# The Kcup Format

A kcup is a single TOML file named `*.kcup.toml`. Four parts: identity, gatherers, prompt, output schema.

## Top-level fields

| Field         | Type   | Required | Description                                       |
|---------------|--------|----------|---------------------------------------------------|
| `name`        | string | yes      | Identifier shown in logs and as the output title. |
| `description` | string | no       | One-line summary printed on each run.             |

## `[[gather]]` blocks

Zero or more gather blocks run before the model call. Each captures one piece of context and binds it to an `id` you can interpolate into the prompt.

| Field      | Type    | Required | Description                                                                 |
|------------|---------|----------|-----------------------------------------------------------------------------|
| `id`       | string  | yes      | The handle used in the prompt as `{{id}}`.                                  |
| `shell`    | string  | one of   | Shell command run via `sh -c`. Captures stdout, trimmed.                    |
| `file`     | string  | one of   | File path read as UTF-8.                                                    |
| `optional` | boolean | no       | If `true` and `file` cannot be read, the value is empty instead of failing. |

Exactly one of `shell` or `file` must be set per block. A `shell` gatherer fails the run if the command exits non-zero (its stderr is shown). A `file` gatherer fails if the file is missing, unless `optional = true`.

## `[prompt]` block

| Field    | Type   | Required | Description                                                                 |
|----------|--------|----------|-----------------------------------------------------------------------------|
| `system` | string | yes      | System message — sets behavior, voice, output rules.                        |
| `user`   | string | yes      | User message template. `{{id}}` placeholders are replaced with gathered values before the call. |

Interpolation is plain string replace. Unknown placeholders are left as-is.

## `[output]` block

| Field    | Type  | Required | Description                                                                 |
|----------|-------|----------|-----------------------------------------------------------------------------|
| `schema` | table | yes      | Map of field name → JSON-schema type. Drives structured output across all providers and post-call validation. |

### Supported schema types

The validator recognizes these strings (from `src/output.rs`):

| Type      | Validator check                                |
|-----------|------------------------------------------------|
| `string`  | JSON string                                    |
| `number`  | JSON number (int or float)                     |
| `integer` | JSON integer (signed or unsigned 64-bit)       |
| `boolean` | JSON boolean                                   |
| `array`   | JSON array (element types are not constrained) |
| `object`  | JSON object (nested keys are not constrained)  |

Any other type string is passed through to the provider untouched but the local validator will accept any value for it. Stick to the six above unless you know what you're doing.

Every field in `schema` is treated as required. The model is asked to return all of them.

## Minimal example

```toml
name = "echo"
description = "Round-trip a single string through the model."

[prompt]
system = "You repeat the user's message back, lowercased."
user = "Hello, world!"

[output]
schema = { result = "string" }
```

No gatherers, one input, one output field.

## Maximal example

```toml
name = "release-notes"
description = "Draft release notes from the diff and merged PRs since the last tag."

[[gather]]
id = "last_tag"
shell = "git describe --tags --abbrev=0"

[[gather]]
id = "commits"
shell = "git log $(git describe --tags --abbrev=0)..HEAD --pretty=format:'%h %s'"

[[gather]]
id = "diff_stat"
shell = "git diff --stat $(git describe --tags --abbrev=0)..HEAD"

[[gather]]
id = "changelog"
file = "CHANGELOG.md"
optional = true

[prompt]
system = """
You write release notes in the style of crates.io changelogs.
Group changes into: Features, Fixes, Internal. Be specific. Cite commit shas.
"""
user = """
Last tag: {{last_tag}}

Commits since:
{{commits}}

Diff stat:
{{diff_stat}}

Existing CHANGELOG (for tone reference):
{{changelog}}
"""

[output]
schema = { version = "string", features = "array", fixes = "array", internal = "array", summary = "string" }
```

Four gatherers (one optional file), a multi-line prompt, and a richer schema.
