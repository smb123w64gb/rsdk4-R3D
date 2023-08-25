use std::path::Path;
use std::path::PathBuf;
use std::fs;
use crate::r3d;

pub fn r3d_2_obj<P: AsRef<Path>>(input:P,output:P){
    let mdl = r3d::R3DHdr::open(&input);
                //println!("{}",&input.file_name().unwrap().to_str().unwrap());
                let mut newpath = PathBuf::new();
                newpath.push(input);
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
}

pub fn obj_2_r3d<P: AsRef<Path>>(input:Vec<obj::Obj>,output:P){
for obj in input{
    let basevt = obj.data.position;
    let basevn = obj.data.normal;
    let baseuv = obj.data.texture;
    let polys = obj.data.objects.clone().pop().unwrap().groups.pop().unwrap().polys;
    for pol in polys{
        println!("{}",pol.0[0]);
    }
}

}