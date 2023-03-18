use anyhow::{Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use std::{io::Write, vec};
use xdiff::{
    cli::{Action, Args, RunArgs},
    DiffConfig, DiffProfile, ExtraArgs, RequestProfile, ResponseProfile,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(args) => run(args).await?,
        Action::Parse => parse().await?,
        _ => panic!("Not implementd"),
    }

    Ok(())
}

async fn parse() -> Result<()> {
    let theme = ColorfulTheme::default();
    let url1: String = Input::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;
    let url2: String = Input::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;
    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;

    let profile_name: String = Input::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    let res = req1.send(&ExtraArgs::default()).await?;

    let headers = res.get_header_keys();

    let choosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select headers to skip")
        .items(&headers)
        .interact()?;

    let skip_headers = choosen.iter().map(|i| headers[*i].to_string()).collect();

    let res = ResponseProfile::new(skip_headers, vec![]);
    let profile = DiffProfile::new(req1, req2, res);
    let config = DiffConfig::new(vec![(profile_name, profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "---\n{}", result)?;
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
