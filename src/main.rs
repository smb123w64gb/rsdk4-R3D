use clap::{Parser, Arg,CommandFactory};
use std::path::PathBuf;
use std::io;
mod r3d;
use std::fs::{self, DirEntry};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Converts folder to model
    #[arg(short,long)]
    c: bool,

    /// Takes in folder or .bin to convert
    #[arg(value_name = "IN_FILE")]
    input_file: Option<PathBuf>,
    /// Takes in folder or .bin to convert to
    #[arg(value_name = "OUT_FILE")]
    output_file: Option<PathBuf>,

}


fn main() {
    let args = Args::parse();
    let mut cmd = Args::command();
    if args.c{
        match args.input_file {
            Some( input) => {
            println!("File {} inputed",input.display());
            let mut entries = fs::read_dir(input).unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>().unwrap();
                
                for a in entries{
                    match a.extension() {
                        Some(b) => {
                            match b.to_str() {
                                Some("obj") => {println!("{}",a.extension().unwrap().to_str().unwrap())},
                                _ => {},
                                
                            }
                        },
                        None => {},
                    }
                    
                }
        },
            None => println!("Can not progress\n No file found for input"),
        }
    }else{
        match args.input_file {
            Some( input) => {
            println!("File {} inputed",input.display());
        },
            None => cmd.print_help().unwrap(),
        }
    }
    
}
