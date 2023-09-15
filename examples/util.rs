use tgars::{TargaImage, Color};

pub fn draw_rectagnle(image: &mut TargaImage, width: u16, height: u16, start:(u16,u16), color: Color)  {
    let mut y: u32;
    let mut x :u32;
    for i in 0..image.get_header().width as u32 * image.get_header().height as u32 {
    y = i / image.get_header().width as u32;
    x = i % image.get_header().width as u32;
        if  (x > start.0 as u32 && x < (width+start.0) as u32 ) && (y > start.1 as u32 && y < (height+start.1) as u32) {
            image.set_pixel(x as u16, y as u16, &color);
        }
    }
}

pub fn draw_triangle(image: &mut TargaImage, center: (usize, usize), base_width: usize, height: usize, color: Color, upside: bool) {
    let mut y: usize;
    let mut x :usize;
    let mut g = if upside {0} else  {base_width / 2};
    for i in 0..image.get_header().width as usize * image.get_header().height as usize  {
        y= i / image.get_header().width as usize;
        x = i % image.get_header().width as usize;
        if y > center.1 && (y< center.1 + height) {
            if x == 0 {
                g= if upside {g+1} else {g-1};
            }
            if (x > (center.0 -(base_width/ 2)) + g) && (x < (center.0 +(base_width/ 2) - g)){
                image[i] = color.clone();
                
                if upside && g >= height {
                    break
                }else if !upside && g <= 0 {
                    break;
                }
            }
        }
    }
}

//  fn draw_line(point1: (usize, usize), point2: (usize, usize), image: &mut TargaImage) {
//     let slope = (point2.1 - point1.1) as f32 / (point2.0 - point1.0) as f32;
//     for x in point1.0..point2.0 {
//         let y = slope * x as f32 ;
//         print!("slope is {} {} {} {}\n",slope, x, point2.0, (y as f32 * image.header.width as f32 + x as f32) as usize);
//         image.data[(y as f32 + image.header.width as f32 * x as f32) as usize] = Color::rgb(255,255,255);
//     }
//  }

fn main() {
}
