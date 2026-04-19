# Providers

Developerpod supports nine AI providers out of the box. At startup it scans your environment for an API key in priority order; **first key found wins**. **53 env var names supported** across all providers.

> The table below mirrors `src/provider.rs`. Pushes that touch that file rebuild this site automatically, but if you ever spot drift, the source is authoritative.

## Supported providers

| # | Provider   | `--provider` id | Default model                        | Endpoint                                                              |
|---|------------|-----------------|--------------------------------------|-----------------------------------------------------------------------|
| 1 | Anthropic  | `anthropic`     | `claude-sonnet-4-6`                  | `api.anthropic.com/v1/messages`                                       |
| 2 | OpenAI     | `openai`        | `gpt-5.4`                            | `api.openai.com/v1/chat/completions`                                  |
| 3 | Google     | `google`        | `gemini-2.5-flash`                   | `generativelanguage.googleapis.com/v1beta/models`                     |
| 4 | Groq       | `groq`          | `llama-3.3-70b-versatile`            | `api.groq.com/openai/v1/chat/completions`                             |
| 5 | Mistral    | `mistral`       | `mistral-large-latest`               | `api.mistral.ai/v1/chat/completions`                                  |
| 6 | Cohere     | `cohere`        | `command-a-03-2025`                  | `api.cohere.com/v2/chat`                                              |
| 7 | DeepSeek   | `deepseek`      | `deepseek-chat`                      | `api.deepseek.com/v1/chat/completions`                                |
| 8 | xAI        | `xai`           | `grok-4-1-fast-non-reasoning`        | `api.x.ai/v1/chat/completions`                                        |
| 9 | OpenRouter | `openrouter`    | `anthropic/claude-sonnet-4.6`        | `openrouter.ai/api/v1/chat/completions`                               |

## How structured output is requested

Each provider has its own way to ask for JSON-schema-shaped output. Developerpod dispatches per provider and translates your `[output].schema` into that provider's idiom:

| Provider              | Mechanism                                                                  |
|-----------------------|----------------------------------------------------------------------------|
| Anthropic             | Forced `tool_use` with a tool named `emit_result`                          |
| OpenAI / Groq /<br>DeepSeek / xAI /<br>OpenRouter | `response_format: { type: "json_schema", json_schema: { strict: true, … } }` |
| Google                | `generationConfig.responseMimeType = "application/json"` + `responseSchema` |
| Mistral               | `response_format: { type: "json_object" }` with the schema inlined into the system prompt |
| Cohere v2             | `response_format: { type: "json_object", json_schema: { … } }`             |

## Detection priority

1. **Provider order is fixed** (the table above). If both Anthropic and OpenAI keys are set, Anthropic wins.
2. **Within a provider**, env var names are tried in the order shown on the [Environment Variables](./reference/env-vars.md) page. Canonical names come first, then community conventions, then short forms, then `*_API_TOKEN` variants.
3. **Empty values** (whitespace-only) are treated as unset.

## Override flags

Force a specific provider, model, or both — useful for switching between providers you have keys for, or for trying a non-default model.

```sh
developerpod repo-mood --provider openai
developerpod repo-mood --provider google --model gemini-2.5-pro
developerpod repo-mood --model claude-opus-4-7   # uses your detected provider
```

If `--provider` is set, only that provider's env vars are scanned; the run errors out if none are present.

## When no key is found

Developerpod exits with a list of every variable it scanned, grouped by provider, and a hint about `--provider`:

```text
Error: no AI provider API key found in environment. Scanned:
  Anthropic:  ANTHROPIC_API_KEY, CLAUDE_API_KEY, ANTHROPIC_KEY, …
  OpenAI:     OPENAI_API_KEY, CHATGPT_API_KEY, OPENAI_KEY, …
  …

Set one of these env vars, or pass --provider <name> if your key uses a non-standard name.
```

The full list lives in [Environment Variables](./reference/env-vars.md).
