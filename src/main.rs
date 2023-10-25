use std::{error::Error, path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use clipboard::ClipboardProvider;
use komodo::{
    code::{stress::run_stress, Question},
    expand::FileExpand,
};

#[derive(Parser, Debug)]
#[command(
    name = "komodo",
    version = "0.1.0",
    about = "Komodo CLI for Rust Competitive Programming"
)]
struct KomodoCli {
    #[command(subcommand)]
    cmd: KomodoCliCommands,
}

#[derive(Subcommand, Debug)]
enum KomodoCliCommands {
    /// Expands a solution file into a submission file, saves to clipboard by default
    Expand {
        /// Solution file to expand
        #[arg(short, long, default_value = "a")]
        question: String,

        /// Output file, defaults to clipboard
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Runs main of src/code/{question}.rs
    Code {
        /// Solution file to run
        #[arg(short, long, default_value = "a")]
        question: String,
    },
    /// Stress tests a solution with a brute force solution
    Stress {
        /// Solution file to stress test
        #[arg(short, long, default_value = "a")]
        question: String,
    },
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = KomodoCli::parse();
    match args.cmd {
        KomodoCliCommands::Expand { question, output } => {
            let question = Question::from_str(&question).unwrap();
            let expanded = FileExpand::expand_file(question.file_path())?;
            if let Some(output) = output {
                std::fs::write(output, expanded)?;
            } else {
                let mut ctx: clipboard::ClipboardContext = clipboard::ClipboardProvider::new()?;
                ctx.set_contents(expanded)?;
            }
        }
        KomodoCliCommands::Code { question } => Question::from_str(&question).unwrap().run()?,
        KomodoCliCommands::Stress { question } => {
            let question = Question::from_str(&question).unwrap();
            run_stress(question);
        }
    }
    Ok(())
}
