use binrw::BinReaderExt;
use clap::{Parser, Subcommand};
use kindle_tool::UpdateBundle;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// display the metadata of a firmware file
  #[command(visible_alias = "info")]
  Inspect {
    /// kindle firmware (.bin) file to inspect
    input_file: PathBuf,
  },

  /// extract the deobfuscated tar.gz payload from a firmware file
  #[command(visible_alias = "convert")]
  Dump {
    /// kindle firmware (.bin) file to process
    input_file: PathBuf,

    /// output file for the .tar.gz payload [default: stdout]
    output_file: Option<PathBuf>,
  },
}

fn get_output(path: Option<&PathBuf>) -> Result<Box<dyn Write>, Box<dyn std::error::Error>> {
  let writer: Box<dyn Write> = if let Some(p) = path {
    Box::new(File::create(p)?)
  } else {
    Box::new(io::stdout())
  };
  Ok(writer)
}

fn main() {
  let cli = Cli::parse();

  let result = match cli.command {
    Commands::Inspect {
      input_file
    } => run_inspect(&input_file),
    Commands::Dump {
      input_file,
      output_file,
    } => run_dump(&input_file, output_file.as_ref()),
  };

  if let Err(e) = result {
    eprintln!("error: {}", e);
    process::exit(1);
  }
}

fn run_inspect(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  let mut file = File::open(path)?;
  let bundle: UpdateBundle = file.read_le()?;
  println!("{}", bundle);
  Ok(())
}

fn run_dump(in_path: &PathBuf, out_path: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
  let mut in_file = File::open(in_path)?;
  let writer = get_output(out_path)?;
  let mut buf_writer = BufWriter::new(writer);

  if let Some(path) = out_path {
    eprintln!("extracting payload from '{}' to '{}'...", in_path.display(), path.display());
  } else {
    eprintln!("extracting payload from '{}' to stdout...", in_path.display());
  };

  kindle_tool::dump_payload(&mut in_file, &mut buf_writer)?;
  Ok(())
}