use clap::{Parser,CommandFactory};
use std::path::{Path,PathBuf};

use std::io;
mod r3d;
use std::fs;
use regex::Regex;
use obj::{self, Obj};
mod translate;

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
    let re = Regex::new(r"\d+$").unwrap();
    let mut cmd = Args::command();
    if args.c{
        match args.input_file {
            Some( input) => {
            println!("File {} inputed",input.display());
            let entries = fs::read_dir(input).unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>().unwrap();
                let mut reorg  = Vec::new();
                for a in entries{
                    match a.extension() {
                        Some(b) => {
                            match b.to_str() {
                                Some("obj") => {
                                    let Some(caps) = re.captures(a.file_stem().unwrap().to_str().unwrap()) else {
                                        println!("no match!");
                                        return;
                                    };
                                    reorg.push((a.clone(),caps[0].parse::<i32>().unwrap() as i32));
                                },
                                _ => {},
                            }
                        },
                        None => {},
                    }
                }
                reorg.sort_by(|a, b| b.1.cmp(&a.1));
                reorg.reverse();
                let mut objs_in = Vec::new();
                let mut base:PathBuf = PathBuf::new();
            for c in reorg{
                objs_in.push(obj::Obj::load(c.0.clone()).unwrap());
                base = c.0.clone();
            }
            base.pop();
            base.set_extension("bin");
            translate::obj_2_r3d(objs_in.clone(), base);


        },
            None => println!("Can not progress\n No file found for input"),
        }
    }else{
        match args.input_file {
            Some( input) => {
                translate::r3d_2_obj(input.clone(),input.clone());
        },
            None => cmd.print_help().unwrap(),
        }
    }
    
}
