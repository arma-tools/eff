use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use eff::edds;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "eff-cli")]
#[command(about = "eff CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Edds(Edds),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "edds texture file commands", long_about = None)]
struct Edds {
    #[command(subcommand)]
    command: EddsCommands,
}

#[derive(Debug, Subcommand)]
enum EddsCommands {
    //Compress(),
    #[command(about = "Decode an edds texture file", long_about = None)]
    Decode {
        /// Output file (*.png/*.jpg etc.)
        #[clap(short = 'o', long = "output")]
        outfile: Option<PathBuf>,

        /// Input file (*.edds)
        #[clap(name = "INFILE")]
        infile: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Edds(edds) => match edds.command {
            EddsCommands::Decode { outfile, infile } => decompress_edds(infile, outfile),
        },
    }
}

fn decompress_edds(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    if !input.exists() {
        return Err(anyhow!("{} doesn't exit.", &input.display()));
    }

    let file =
        File::open(&input).with_context(|| format!("Failed to open '{}'.", input.display()))?;
    let mut reader = BufReader::new(file);
    let edds = edds::Edds::from(&mut reader).with_context(|| "Failed to read the edds.")?;
    let mm = edds.mipmaps.last().with_context(|| "No Mipmaps found.")?;

    let mut output = if let Some(out) = output {
        out
    } else {
        PathBuf::from(
            input
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        )
    };

    if output.extension().is_none() {
        output.set_extension("png");
    }

    image::save_buffer(
        &output,
        &mm.data,
        mm.width as u32,
        mm.height as u32,
        image::ColorType::Rgba8,
    )
    .with_context(|| format!("Couldn't save the image to '{}'.", output.display()))?;

    println!("Decoding successful.");
    Ok(())
}
