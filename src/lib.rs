// implemented from this document: http://www.paulbourke.net/dataformats/tga/

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::{
    fmt,
    fs::File,
    io::Write,
    io::{BufReader, BufWriter, Read},
    ops::{Index, IndexMut},
    usize,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TgaHeader {
    pub id_length: u8,
    pub color_map_type: u8,
    pub data_type_code: u8,
    pub color_map_origin: i16,
    pub color_map_length: i16,
    pub color_map_depth: u8,
    pub x_origin: i16,
    pub y_origin: i16,
    pub width: u16,
    pub height: u16,
    pub bits_ppx: u8,
    pub image_descriptor: u8,
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
pub struct Color(u8, u8, u8);

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(b, g, r)
    }

    pub fn bgr(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }

    pub fn bgr_from_slice(color_bytes: &[u8]) -> Color {
        Color(color_bytes[0], color_bytes[1], color_bytes[2])
    }

    pub fn rgb_from_slice(color_bytes: &[u8]) -> Color {
        Color(color_bytes[2], color_bytes[1], color_bytes[0])
    }

    pub const GREEN: Color = Color { 0: 0, 1: 255, 2: 0 };
    pub const RED: Color = Color { 0: 0, 1: 255, 2: 0 };
    pub const BLUE: Color = Color { 0: 0, 1: 255, 2: 0 };
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        if rhs <= 1.0 {
            self.0 = (self.0 as f32 * rhs) as u8;
            self.1 = (self.1 as f32 * rhs) as u8;
            self.2 = (self.2 as f32 * rhs) as u8;
        }
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0).min(255);
        self.1 = (self.1 + rhs.1).min(255);
        self.2 = (self.2 + rhs.2).min(255);
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::rgb(
            (self.0 + rhs.0).min(255),
            (self.1 + rhs.1).min(255),
            (self.2 + rhs.2).min(255),
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        if rhs <= 1.0 {
            Color::rgb(
                (self.0 as f32 * rhs) as u8,
                (self.1 as f32 * rhs) as u8,
                (self.2 as f32 * rhs) as u8,
            )
        } else {
            self.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargaImage {
    header: TgaHeader,
    data: Vec<Color>,
}

impl TargaImage {
    pub fn new(width: u16, height: u16) -> TargaImage {
        TargaImage {
            header: TgaHeader::new(width, height),
            data: vec![Color::rgb(0, 0, 0); width as usize * height as usize],
        }
    }
    pub fn set_data_type(&mut self, code: u8) {
        self.header.data_type_code = code;
    }

    pub fn save_file(&self, path: &str) -> Result<usize, Box<dyn Error>> {
        let path = Path::new(path);
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut buf = BufWriter::new(&file);
        let header_bw = match bincode::serialize(&self.header) {
            Ok(bytes) => buf.write(&bytes)?,
            Err(e) => return Err(e),
        };
        let data_bw = match bincode::serialize(&self.data[..]) {
            Ok(data_bytes) => {
                buf.write_all(&data_bytes[8..])?;
                data_bytes.len() - 8
            }
            Err(e) => return Err(e),
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
        let mut header_buffer: [u8; 18] = [0u8; 18];
        buf.read_exact(&mut header_buffer).unwrap();
        image.header = bincode::deserialize(&header_buffer).unwrap();

        let d = image.header.width as usize * image.header.height as usize;
        let mut data_buf: Vec<u8> = Vec::with_capacity(d * 3);
        // data_buf.push(5);
        if let Ok(size) = buf.read_to_end(&mut data_buf) {
            println!("No problem reading the file this size {}", size);
        } else {
            println!("Err is");
        }
        println!(
            "header width is : {} and the first value is: {}",
            image.header.width,
            data_buf.len()
        );

        match bincode::deserialize(data_buf[0..&data_buf.len() - 2].as_ref()) {
            Ok(data) => {
                image.data = data;
            }
            Err(why) => {
                println!("error happend because {}", why)
            }
        };
        image
    }

    pub fn set_pixel(&mut self, row: u16, col: u16, color: &Color) {
        self.data[row as usize * self.header.width as usize + col as usize] = color.clone();
    }

    pub fn get_header(&mut self) -> &TgaHeader {
        &self.header
    }

    pub fn load(path: &str) -> Result<TargaImage, String> {
        let path = Path::new(path);
        let file = match File::open(&path) {
            Err(_why) => return Err(format!("Failed to open the file {}", _why)),
            Ok(file) => file,
        };

        let mut buf = BufReader::new(&file);
        let mut header_buffer: [u8; 18] = [0u8; 18];
        buf.read_exact(&mut header_buffer).unwrap();
        let _header: TgaHeader = bincode::deserialize(&header_buffer).unwrap();
        let mut image = TargaImage::new(_header.width, _header.height);
        let d = image.header.width as usize * image.header.height as usize;
        let mut data_buf: Vec<u8> = Vec::with_capacity(d * 3);

        if let Ok(size) = buf.read_to_end(&mut data_buf) {
            println!("No problem reading the file this size {}", size);
        } else {
            return Err("Problem reading the file".to_owned());
        }

        if _header.data_type_code == 10 {
            let mut curr_pos = 0;
            let mut n: usize = 0;
            while n < image.header.width as usize * image.header.width as usize {
                let mut size = data_buf[curr_pos];
                if size > 127 {
                    size -= 127;
                    let color = Color::bgr(
                        data_buf[curr_pos + 1],
                        data_buf[curr_pos + 2],
                        data_buf[curr_pos + 3],
                    );
                    for _ in 0..size {
                        image[n] = color.clone();
                        n += 1;
                    }
                    curr_pos += 4;
                } else {
                    size += 1;
                    for _ in 0..size {
                        image[n] = Color::bgr_from_slice(&data_buf[curr_pos + 1..curr_pos + 4]);
                        n += 1;
                        curr_pos += 3;
                    }
                    curr_pos += 1;
                }
            }
        } else if _header.data_type_code == 2 {
            let mut n = 0;
            while n < image.header.width as usize * image.header.width as usize * 3 {
                image[n / 3] = Color::bgr_from_slice(&data_buf[n..n + 3]);
                n += 3;
            }
        } else {
            return Err("data type code is not supported!".to_owned());
        }
        Ok(image)
    }

    pub fn fill(&mut self, color: &Color) {
        for i in self.data.iter_mut() {
            *i = color.clone();
        }
    }
}

impl Index<usize> for TargaImage {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for TargaImage {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl fmt::Display for TargaImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "width:{},\nheight{}\nbpp:{}\ndataType: {}",
            self.header.width, self.header.height, self.header.bits_ppx, self.header.data_type_code
        )
    }
}
