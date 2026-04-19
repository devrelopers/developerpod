mod brew;
mod gather;
mod output;
mod pod;

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = pod::resolve_pod_path(&cli.kcup)?;
    eprintln!("{} {}", "▶ loading".dimmed(), path.display());
    let pod = pod::load_pod(&path)?;

    eprintln!(
        "{} {} — {} ({} gatherers)",
        "▶ brewing".dimmed(),
        pod.name.bold(),
        pod.description.dimmed(),
        pod.gather.len()
    );
    let gathered = gather::run_all(&pod.gather)?;

    let result = brew::brew(&pod, &gathered)?;
    output::validate(&result, &pod.output.schema)?;
    output::pretty_print(&pod.name, &result);

    Ok(())
}
