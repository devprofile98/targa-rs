// implemented from this document: http://www.paulbourke.net/dataformats/tga/

use std::error::Error;
use std::os::unix::prelude::FileExt;
use std::{
        io::{BufWriter, BufReader, Read},
        fs::File,
        io::Write,
        usize,
        ops::{Index, IndexMut},
};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TgaHeader {
    pub id_length: u8,
    pub color_map_type: u8 ,
    pub data_type_code:u8,
    pub color_map_origin: i16,
    pub color_map_length:i16,
    pub color_map_depth:u8,
    pub x_origin:i16,
    pub y_origin:i16,
    pub width:u16,
    pub height:u16,
    pub bits_ppx:u8,
    pub image_descriptor:u8
 }

impl TgaHeader {
    pub fn new(width: u16, height: u16) -> TgaHeader {
        TgaHeader {
            id_length: 0,
            data_type_code: 2,
            bits_ppx: 24,
            width: width,
            height: height,
            ..TgaHeader::default()
        }
    }
 }

 #[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color(u8,u8,u8);

 impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(b, g, r)
    }

    pub fn bgr(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }

    const GREEN:Color = Color{0:0, 1:255, 2:0};
    const RED:Color = Color{0:0, 1:255, 2:0};
    const BLUE:Color = Color{0:0, 1:255, 2:0};
 }
 
 #[derive(Serialize, Deserialize, Debug)]
 pub struct TargaImage {
    header: TgaHeader,
    data: Vec<Color>
 }

 impl TargaImage {
    pub fn new(width: u16, height: u16) -> TargaImage{
        TargaImage {
            header: TgaHeader::new(width, height),
            data: vec![Color::rgb(0,0,0); width as usize * height as usize]
        }
    }

    pub fn save_file(&self, path: &str) -> Result<usize, Box<dyn Error>>{
        let path = Path::new(path);
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut buf = BufWriter::new(&file);
        let header_bw = match bincode::serialize(&self.header){
            Ok(bytes) => buf.write(&bytes)?,
            Err(e) => return Err(e)
        };
        let data_bw = match bincode::serialize(&self.data[..]){
            
            Ok(data_bytes) => {
                buf.write_all(&data_bytes[8..])?;
                data_bytes.len() -8
            },
            Err(e) => return Err(e)
        };
        Ok(header_bw + data_bw)
    }

 pub fn open_image(path: &str) -> TargaImage {
    let path = Path::new(path);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut image = TargaImage::new(1024, 1024);

    let mut buf = BufReader::new(&file);
    let mut header_buffer: [u8; 18] = [0u8;18];
    buf.read_exact(& mut header_buffer).unwrap();
    image.header =  bincode::deserialize(&header_buffer).unwrap();
    
    let d = image.header.width as usize * image.header.height as usize;
    let mut data_buf:Vec<u8>  = Vec::with_capacity(d*3+10 );
    data_buf.push(0);
    if let Ok(size) = file.read_at(data_buf.as_mut_slice() , 18){
        println!("No problem reading the file this size {}", size);
    }
    else{
        println!("Err is");
    }
    println!("header width is : {} and the first value is: {:?}", image.header.width, data_buf);
    
    image.data = bincode::deserialize(data_buf.as_slice()).unwrap();
    image 
 }

 pub fn set_pixel(&mut self, row: u16, col: u16, color: &Color) {
    self.data[row as usize * self.header.width as usize  + col as usize] = color.clone();
 }

 pub fn get_header(&mut self) -> &TgaHeader {
    &self.header
 }

 //  fn load(path: &str) -> TargaImage {
//     let path = Path::new(path);
//     let mut file = match File::open(&path) {
//         Err(why) => panic!("couldn't open {}: {}", path.display(), why),
//         Ok(file) => file,
//     };

//     let mut image = TargaImage::new(1024, 1024);

//     let mut buf = BufReader::new(&file);
//     let mut header_buffer: [u8; 18] = [0u8;18];
//     buf.read_exact(& mut header_buffer).unwrap();
//     image.header =  bincode::deserialize(&header_buffer).unwrap();
    
//     let d = image.header.width as usize * image.header.height as usize;
//     let mut data_buf:Vec<u8>  = Vec::with_capacity(d*3+10 );
//     data_buf.push(0);
//     if let Ok(size) = file.read_at(data_buf.as_mut_slice() , 18){
//         println!("No problem reading the file this size {}", size);
//     }
//     else{
//         println!("Err is");
//     }
//     println!("header width is : {} and the first value is: {:?}", image.header.width, data_buf);
    
//     image.data = bincode::deserialize(data_buf.as_slice()).unwrap();
//     image 
//  }

    pub fn fill(&mut self, color: &Color) {
        for i in self.data.iter_mut() {
            *i = color.clone();
        }
    }

}

impl Index<usize> for TargaImage{
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]   
    }
}

impl IndexMut<usize> for TargaImage{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}