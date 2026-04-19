# Environment Variables

The full list of environment variables developerpod scans, grouped by provider, in priority order. The first non-empty value wins.

> Mirrored from `src/provider.rs`. Pushes that touch that file rebuild this site.

## Anthropic

```
ANTHROPIC_API_KEY
CLAUDE_API_KEY
ANTHROPIC_KEY
CLAUDE_KEY
ANTHROPIC_API_TOKEN
CLAUDE_API_TOKEN
```

## OpenAI

```
OPENAI_API_KEY
CHATGPT_API_KEY
OPENAI_KEY
CHATGPT_KEY
GPT_API_KEY
GPT_KEY
OPENAI_API_TOKEN
OPENAI_TOKEN
CHATGPT_API_TOKEN
```

## Google

```
GEMINI_API_KEY
GOOGLE_API_KEY
GOOGLE_GENERATIVE_AI_API_KEY     # Vercel AI SDK default
GOOGLE_AI_API_KEY
GOOGLE_GENAI_API_KEY
GOOGLE_GEMINI_API_KEY
GEMINI_KEY
```

## Groq

```
GROQ_API_KEY
GROQ_KEY
GROQ_API_TOKEN
```

## Mistral

```
MISTRAL_API_KEY
MISTRALAI_API_KEY
MISTRAL_KEY
MISTRAL_API_TOKEN
```

## Cohere

```
COHERE_API_KEY
CO_API_KEY                       # Cohere SDK default
COHERE_KEY
CO_KEY
COHERE_API_TOKEN
```

## DeepSeek

```
DEEPSEEK_API_KEY
DEEPSEEK_KEY
DEEPSEEK_API_TOKEN
DEEPSEEK_TOKEN
```

## xAI

```
XAI_API_KEY
GROK_API_KEY
XAI_KEY
GROK_KEY
XAI_API_TOKEN
GROK_API_TOKEN
```

## OpenRouter

```
OPENROUTER_API_KEY
OPEN_ROUTER_API_KEY
OPENROUTER_KEY
OPENROUTER_API_TOKEN
```

## Notes

- **Empty values count as unset.** Whitespace-only values are skipped, so `export OPENAI_API_KEY=""` won't accidentally select OpenAI.
- **Provider order is fixed.** If both Anthropic and OpenAI keys are set, Anthropic wins. To override, pass `--provider`.
- **Within a provider**, the canonical name is first, then community conventions, then short forms, then `*_API_TOKEN` / `*_TOKEN` variants.
- **Missing your name?** Open an issue or PR. The list is meant to be generous — if a sensible convention exists in the wild and we don't catch it, that's a bug.
