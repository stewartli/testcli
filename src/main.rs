use dialoguer::{self, theme::ColorfulTheme, Confirm, Input, MultiSelect};
use color_eyre::{eyre::{eyre, WrapErr}, Result};

const OUTPUT: [&str; 5] = ["pdf", "docx", "html", "pptx", "revealjs"];

fn main() -> Result<()>{
    // set up
    color_eyre::install()?;
    let ctheme = ColorfulTheme::default();

    // define path
    let cwd_path = std::env::current_dir()?;
    let qmd_dir_path = cwd_path.join("example");
    let qmd_file_path = qmd_dir_path.join("index.qmd");

    // need type annotation
    let title: String = Input::with_theme(&ctheme)
        .with_prompt("title")
        .interact_text()?;

    let date: String = Input::with_theme(&ctheme)
        .with_prompt("date")
        .default("today".into())
        .interact_text()?;
    
    let output = MultiSelect::with_theme(&ctheme)
        .with_prompt("output")
        .items(&OUTPUT)
        .interact()?
        .into_iter()
        .map(|index| OUTPUT[index]);

    let ctx = output
        .map(|out| format!("\n - {}", out))
        .collect::<String>();
    
    // no whitespace
    let ctx_fmt = format!(
        r#"---
title: "{}"
date: {}
format: {}
---
    "#, title, date, ctx);

    // proceed
    if !Confirm::with_theme(&ctheme).with_prompt("ready now?").interact()?{
        return Err(eyre!("quit now ..."));
    }

    // write out
    std::fs::create_dir_all(qmd_dir_path)?;
    std::fs::write(&qmd_file_path, ctx_fmt)
        .wrap_err_with(|| format!("could not write to {}", qmd_file_path.display()))?;

    Ok(())
}
