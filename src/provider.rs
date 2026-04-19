use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Anthropic,
    OpenAI,
    Google,
    Groq,
    Mistral,
    Cohere,
    DeepSeek,
    Xai,
    OpenRouter,
}

pub struct ProviderInfo {
    pub provider: Provider,
    pub display_name: &'static str,
    pub id: &'static str,
    pub env_vars: &'static [&'static str],
    pub default_model: &'static str,
    pub endpoint: &'static str,
}

pub const PROVIDERS: &[ProviderInfo] = &[
    ProviderInfo {
        provider: Provider::Anthropic,
        display_name: "Anthropic",
        id: "anthropic",
        env_vars: &[
            "ANTHROPIC_API_KEY",
            "CLAUDE_API_KEY",
            "CLAUDE_KEY",
            "ANTHROPIC_KEY",
        ],
        default_model: "claude-sonnet-4-6",
        endpoint: "https://api.anthropic.com/v1/messages",
    },
    ProviderInfo {
        provider: Provider::OpenAI,
        display_name: "OpenAI",
        id: "openai",
        env_vars: &[
            "OPENAI_API_KEY",
            "CHATGPT_API_KEY",
            "CHATGPT_KEY",
            "OPENAI_KEY",
            "GPT_API_KEY",
            "GPT_KEY",
        ],
        default_model: "gpt-4o",
        endpoint: "https://api.openai.com/v1/chat/completions",
    },
    ProviderInfo {
        provider: Provider::Google,
        display_name: "Google",
        id: "google",
        env_vars: &[
            "GEMINI_API_KEY",
            "GOOGLE_API_KEY",
            "GOOGLE_AI_API_KEY",
            "GOOGLE_GENAI_API_KEY",
            "GEMINI_KEY",
        ],
        default_model: "gemini-2.5-flash",
        endpoint: "https://generativelanguage.googleapis.com/v1beta/models",
    },
    ProviderInfo {
        provider: Provider::Groq,
        display_name: "Groq",
        id: "groq",
        env_vars: &["GROQ_API_KEY", "GROQ_KEY"],
        default_model: "llama-3.3-70b-versatile",
        endpoint: "https://api.groq.com/openai/v1/chat/completions",
    },
    ProviderInfo {
        provider: Provider::Mistral,
        display_name: "Mistral",
        id: "mistral",
        env_vars: &["MISTRAL_API_KEY", "MISTRAL_KEY"],
        default_model: "mistral-large-latest",
        endpoint: "https://api.mistral.ai/v1/chat/completions",
    },
    ProviderInfo {
        provider: Provider::Cohere,
        display_name: "Cohere",
        id: "cohere",
        env_vars: &["COHERE_API_KEY", "CO_API_KEY", "COHERE_KEY"],
        default_model: "command-r-plus",
        endpoint: "https://api.cohere.com/v2/chat",
    },
    ProviderInfo {
        provider: Provider::DeepSeek,
        display_name: "DeepSeek",
        id: "deepseek",
        env_vars: &["DEEPSEEK_API_KEY", "DEEPSEEK_KEY"],
        default_model: "deepseek-chat",
        endpoint: "https://api.deepseek.com/v1/chat/completions",
    },
    ProviderInfo {
        provider: Provider::Xai,
        display_name: "xAI",
        id: "xai",
        env_vars: &["XAI_API_KEY", "GROK_API_KEY", "GROK_KEY", "XAI_KEY"],
        default_model: "grok-2-latest",
        endpoint: "https://api.x.ai/v1/chat/completions",
    },
    ProviderInfo {
        provider: Provider::OpenRouter,
        display_name: "OpenRouter",
        id: "openrouter",
        env_vars: &["OPENROUTER_API_KEY", "OPENROUTER_KEY"],
        default_model: "anthropic/claude-sonnet-4-6",
        endpoint: "https://openrouter.ai/api/v1/chat/completions",
    },
];

pub struct Detected {
    pub info: &'static ProviderInfo,
    pub api_key: String,
    pub env_var: &'static str,
}

fn first_match(info: &'static ProviderInfo) -> Option<Detected> {
    for env_var in info.env_vars {
        if let Ok(val) = std::env::var(env_var)
            && !val.trim().is_empty()
        {
            return Some(Detected {
                info,
                api_key: val,
                env_var,
            });
        }
    }
    None
}

pub fn detect() -> Result<Detected> {
    for info in PROVIDERS {
        if let Some(d) = first_match(info) {
            return Ok(d);
        }
    }
    let mut msg = String::from("no AI provider API key found in environment. Scanned:\n");
    for info in PROVIDERS {
        msg.push_str(&format!(
            "  {:<11} {}\n",
            format!("{}:", info.display_name),
            info.env_vars.join(", ")
        ));
    }
    msg.push_str("\nSet one of these env vars, or pass --provider <name> if your key uses a non-standard name.");
    bail!("{msg}")
}

pub fn detect_for(provider: Provider) -> Result<Detected> {
    let info = info_for(provider);
    if let Some(d) = first_match(info) {
        return Ok(d);
    }
    bail!(
        "no API key found for {} in any of: {}",
        info.display_name,
        info.env_vars.join(", ")
    )
}

pub fn info_for(provider: Provider) -> &'static ProviderInfo {
    PROVIDERS
        .iter()
        .find(|i| i.provider == provider)
        .expect("provider must exist in PROVIDERS table")
}

pub fn from_id(id: &str) -> Result<Provider> {
    for info in PROVIDERS {
        if info.id.eq_ignore_ascii_case(id) {
            return Ok(info.provider);
        }
    }
    let known: Vec<&str> = PROVIDERS.iter().map(|i| i.id).collect();
    bail!("unknown provider '{id}'. Known: {}", known.join(", "))
}
