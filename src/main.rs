use clap::{Parser,CommandFactory};
use std::path::{Path,PathBuf};

use std::io;
mod r3d;
use std::fs;
use regex::Regex;
use obj::{self, Obj};


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
                let first = reorg.pop();
                //let mut basePoly = Vec::new();
                reorg.reverse();
                match first {
                    Some(ittm) => {
                        let mut fobj = Obj::load(ittm.0).unwrap();
                        let objone = fobj.data.objects.pop().unwrap().groups.pop().unwrap().polys;
                        for polypoly in objone{
                            for poly in polypoly.0{
                            println!("{}",poly);
                            }
                        }
                        
                        
                        //println!("{}",ittm.0.display())
                    },
                    None => {}
                }

            for c in reorg{
                println!("{}",c.0.display());
            }
        },
            None => println!("Can not progress\n No file found for input"),
        }
    }else{
        match args.input_file {
            Some( input) => {
                let mdl = r3d::R3DHdr::open(&input);
                //println!("{}",&input.file_name().unwrap().to_str().unwrap());
                let mut newpath = input.clone();
                newpath.set_extension("");
                fs::create_dir_all(&newpath).unwrap();
                //println!("VertCount:{}",&mdl.unwrap().vert_count);
                
                let mut simp = Vec::new();
                for i in (mdl.as_ref().unwrap().indices).clone(){
                    let mut poly = Vec::new();
                    poly.push(obj::IndexTuple(i.0 as usize,Some(i.0 as usize),Some(i.0 as usize)));
                    poly.push(obj::IndexTuple(i.1 as usize,Some(i.1 as usize),Some(i.1 as usize)));
                    poly.push(obj::IndexTuple(i.2 as usize,Some(i.2 as usize),Some(i.2 as usize)));
                    simp.push(obj::SimplePolygon(poly));
                }
                let mut Group = obj::Group::new(("Base").to_string());
                Group.polys = simp;
                let object = obj::Object{name:("obj").to_string(),groups:vec![Group]};
                for (i,frame) in mdl.as_ref().unwrap().frames.iter().enumerate(){
                    let mut pp = newpath.clone();
                    let mut ob = obj::ObjData::default();
                    ob.objects = vec![object.clone()];
                    let mut vt = Vec::new();
                    let mut nm = Vec::new();
                    let mut uv = Vec::new();
                    let uuvv = mdl.as_ref().unwrap().uv.clone();
                    for (ii,f) in frame.model.clone().iter().enumerate(){
                        vt.push([f.0*-1.0,f.1,f.2]);
                        uv.push([uuvv[ii].0,(uuvv[ii].1*-1.0)+1.0]);
                        nm.push([f.3,f.4,f.5]);
                    }
                    ob.position = vt;
                    ob.texture = uv;
                    ob.normal = nm;
                    pp.push(Path::new(format!("base{}.obj",i).as_str()));
                    println!("Frame {} out",i);
                    //let npp = 
                    //p.push(".obj");
                    let objOut = obj::Obj{data:ob,path:pp.clone()};
                    objOut.save(pp.clone()).unwrap();

                }
        },
            None => cmd.print_help().unwrap(),
        }
    }
    
}
