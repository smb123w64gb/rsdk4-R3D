
use binrw::{BinRead, io::BufReader, BinResult, BinReaderExt};
use std::{path::Path, io::Read};
#[derive(BinRead)]
#[brw(little,import(vert_count: u16))]
pub struct Frame{
    #[br(count = vert_count)]
    pub model:Vec<(f32,f32,f32,f32,f32,f32)>,
}

#[derive(BinRead)]
#[brw(little, magic = b"R3D\x00")]
pub struct R3DHdr {
    pub vert_count:u16,
    #[br(count = vert_count)]
    pub uv:Vec<(f32,f32)>,
    pub index_count:u16,
    #[br(count = index_count)]
    pub indices:Vec<(u16,u16,u16)>,
    pub frame_count:u16,
    #[br(args{count: frame_count as usize,inner:(vert_count,)})]
    pub frames:Vec<Frame>,

}

impl R3DHdr{
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        BufReader::new(std::fs::File::open(path)?).read_le()
    }
}