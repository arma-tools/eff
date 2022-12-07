use std::{fs::File, io::BufReader, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use eff::edds;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "eff-clli")]
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
struct Edds {
    #[command(subcommand)]
    command: EddsCommands,
}

#[derive(Debug, Subcommand)]
enum EddsCommands {
    //Compress(StashPush),
    Decompress {
        /// Output file (*.png)
        #[clap(short = 'o', long = "output")]
        outfile: Option<PathBuf>,

        /// Input file (*.edds)
        #[clap(name = "INFILE")]
        infile: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Edds(edds) => match edds.command {
            EddsCommands::Decompress { outfile, infile } => decompress_edds(infile, outfile),
        },
    }
}

fn decompress_edds(input: PathBuf, output: Option<PathBuf>) {
    if !input.exists() {
        println!("Error: {} doesn't exit!", input.display());
    }

    let file = File::open(input.clone()).unwrap();
    let mut reader = BufReader::new(file);
    let edds = edds::Edds::from(&mut reader).unwrap();
    let mm = edds.mipmaps.last().unwrap();

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

    output.set_extension("png");

    image::save_buffer(
        output,
        &mm.data,
        mm.width as u32,
        mm.height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
