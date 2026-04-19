# CLI Reference

```text
developerpod [OPTIONS] <KCUP>
```

Run a kcup. Resolves `<KCUP>` to `./<KCUP>.kcup.toml`, then `./examples/<KCUP>.kcup.toml`.

## Arguments

| Name      | Required | Description                                                                                  |
|-----------|----------|----------------------------------------------------------------------------------------------|
| `<KCUP>`  | yes      | Name of the kcup to run (without the `.kcup.toml` suffix). Resolution order described above. |

## Options

| Flag                 | Description                                                                                                |
|----------------------|------------------------------------------------------------------------------------------------------------|
| `--provider <ID>`    | Force a specific provider. Valid IDs: `anthropic`, `openai`, `google`, `groq`, `mistral`, `cohere`, `deepseek`, `xai`, `openrouter`. Skips auto-detection and only scans that provider's env vars. |
| `--model <NAME>`     | Override the model name for whichever provider is selected. Useful when a provider's default has been deprecated or when you want to try a different tier (e.g. Opus instead of Sonnet on Anthropic). |
| `-h`, `--help`       | Print help and exit.                                                                                       |
| `-V`, `--version`    | Print version and exit.                                                                                    |

## Examples

```sh
# Auto-detect provider, run the bundled example
developerpod repo-mood

# Force OpenAI even if Anthropic is also set
developerpod repo-mood --provider openai

# Force a specific model on the detected provider
developerpod repo-mood --model claude-opus-4-7

# Force both
developerpod repo-mood --provider google --model gemini-2.5-pro
```

## Exit codes

| Code | Meaning                                                                                          |
|------|--------------------------------------------------------------------------------------------------|
| 0    | Success — the model returned a valid response matching the declared schema and it was printed.   |
| 1    | Any failure — kcup not found, parse error, gatherer failure, missing API key, API error, schema validation failure. The full error chain (via `anyhow`) is printed to stderr. |

There are currently no distinct exit codes per failure type. If you need to programmatically distinguish between (for example) a missing API key and a network error, parse the stderr message — and open an issue, since that's a reasonable thing to want.

## Output streams

- **stdout**: only the final pretty-printed result.
- **stderr**: the progress lines (`▶ brewing with …`, `▶ loading …`, `▶ pod …`) and any error chain.

This means you can pipe `developerpod <name>` into other tools and only get the result, while the progress output stays visible in your terminal.
