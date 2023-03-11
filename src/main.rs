use anyhow::Result;
use clap::Parser;
use std::io::Write;
use xdiff::{
    cli::{Action, Args, RunArgs},
    DiffConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(args) => run(args).await?,
        _ => panic!("Not implementd"),
    }

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    println!("{:?}", args);

    let config_file = args.config.unwrap_or_else(|| "./xdiff.yaml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config
        .get_profile(&args.profile)
        .ok_or_else(|| anyhow::anyhow!("Profile {} not found", args.profile))?;

    let extra_args = args.extra_params.into();
    let output = profile.diff(extra_args).await?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", output)?;

    Ok(())
}
