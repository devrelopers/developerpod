mod brew;
mod gather;
mod output;
mod pod;
mod provider;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(
    name = "developerpod",
    about = "Run declarative AI-backed kcups (TOML pods that gather context and ask a model)",
    version
)]
struct Cli {
    /// Name of the kcup to run (looks up <name>.kcup.toml or examples/<name>.kcup.toml)
    kcup: String,

    /// Force a specific provider (anthropic, openai, google, groq, mistral, cohere, deepseek, xai, openrouter)
    #[arg(long)]
    provider: Option<String>,

    /// Override the model name for the chosen provider
    #[arg(long)]
    model: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let detected = match cli.provider.as_deref() {
        Some(name) => provider::detect_for(provider::from_id(name)?)?,
        None => provider::detect()?,
    };
    let model = cli
        .model
        .as_deref()
        .unwrap_or(detected.info.default_model);

    eprintln!(
        "{} {} ({}) — key from {}",
        "▶ brewing with".dimmed(),
        detected.info.display_name.bold(),
        model.cyan(),
        detected.env_var.yellow()
    );

    let path = pod::resolve_pod_path(&cli.kcup)?;
    eprintln!("{} {}", "▶ loading".dimmed(), path.display());
    let pod = pod::load_pod(&path)?;

    eprintln!(
        "{} {} — {} ({} gatherers)",
        "▶ pod".dimmed(),
        pod.name.bold(),
        pod.description.dimmed(),
        pod.gather.len()
    );
    let gathered = gather::run_all(&pod.gather)?;

    let result = brew::brew(&pod, &gathered, &detected, model)?;
    output::validate(&result, &pod.output.schema)?;
    output::pretty_print(&pod.name, &result);

    Ok(())
}
