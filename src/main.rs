use std::os::unix::prelude::FileExt;
use std::usize;
use std::{fs::File, io::Write};
use std::path::Path;
use::std::mem;
use std::io::{BufWriter, BufReader, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct TgaHeader {
    id_length: u8,
    color_map_type: u8 ,
    data_type_code:u8,
    color_map_origin: i16,
    color_map_length:i16,
    color_map_depth:u8,
    x_origin:i16,
    y_origin:i16,
    width:u16,
    height:u16,
    bits_ppx:u8,
    image_descriptor:u8
 }

 impl TgaHeader {
    fn new(width: u16, height: u16) -> TgaHeader {

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
 struct RGB(u8,u8,u8);
 
 #[derive(Serialize, Deserialize, Debug)]
 struct TargaImage {
    header: TgaHeader,
    data: Vec<RGB>
 }

 impl TargaImage {
    fn new(width: u16, height: u16) -> TargaImage{
        // let mut temp_vec = Vec::new();
        // temp_vec.reserve(width as usize * height as usize);
        TargaImage {
            header: TgaHeader::new(width, height),
            data: vec![RGB(255,255,255); width as usize * height as usize]
        }
    }

    fn draw_rectagnle(& mut self, width: usize, height: usize, start:(usize,usize), color: RGB)  {
        let mut y: usize;
        let mut x :usize;
        for i in 0..self.header.width as usize * self.header.height as usize {
        y= i / self.header.width as usize;
        x = i % self.header.width as usize;
            if  (x > start.0 && x<width+start.0) && (y > start.1 && y<height+start.1) {
                self.data[i] = color.clone();
            }
        }
    }

    fn draw_triangle(& mut self,center: (usize, usize), base_width: usize, height: usize, color: RGB, upside: bool) {
        let mut y: usize = 0;
        let mut x :usize = 0;
        let mut g = if upside {0} else  {base_width / 2};
        for (i,pixel ) in self.data.iter_mut().enumerate() {
            y= i / self.header.width as usize;
            x = i % self.header.width as usize;
            if y > center.1 && (y< center.1 + height) {
                if x == 0 {
                    g= if upside {g+1} else {g-1};
                }
                if (x > (center.0 -(base_width/ 2)) + g) && (x < (center.0 +(base_width/ 2) - g)){
                    *pixel = color.clone();
                    if upside && g >= height {
                        break
                    }else if !upside && g <= 0 {
                        break;
                    }
                    ;
                }
            }

        }
    }
 }

 fn open_image(path: &str) -> TargaImage {
    let path = Path::new(path);
    let mut file = match File::open(&path) {
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

 fn write_image(image: &TargaImage, path: &str) {

 }

fn main() {
    {
        let mut image  = TargaImage::new(500, 500);
        println!("the size for image is : {}", mem::size_of::<TgaHeader>());
        image.draw_rectagnle(500, 500, (0,0), RGB(142, 224, 245));

        image.draw_triangle((250, 100), 300, 300, RGB(100,0,190), false);
        image.draw_triangle((319, 300), 160, 145, RGB(100,0,190), true);
        image.draw_triangle((181, 300), 160, 145, RGB(100,0,190), true);
        image.draw_rectagnle(300, 52, (100, 249), RGB(100,0,190));

        // image.draw_rectagnle(100, 200, RGB(0, 0, 100));
        let path = Path::new("hello.tga");
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        // image.data.push(RGB(1, 2, 3));
        let mut buf = BufWriter::new(&file);
        let bytes = bincode::serialize(&image.header).unwrap();
        let data_bytes = bincode::serialize(&image.data[..]).unwrap();
        println!("The length of data_bytes is :{}", data_bytes.len());
        buf.write(&bytes);
        buf.write_all(&data_bytes[8..]);
    }
    // open_image("hello.tga");
}
