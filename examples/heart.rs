
mod util;
use util::{draw_rectagnle, draw_triangle};

use tgars::{TargaImage, Color};

fn main () {
        
    let mut image  = TargaImage::new(500, 500);
    draw_rectagnle(&mut image,500, 500, (0,0), Color::rgb(245, 224, 142));

    draw_triangle(&mut image,(250, 100), 300, 300, Color::rgb(100,0,190), false);
    draw_triangle(&mut image,(319, 300), 160, 145, Color::rgb(100,0,190), true);
    draw_triangle(&mut image,(181, 300), 160, 145, Color::rgb(100,0,190), true);
    draw_rectagnle(&mut image,52, 300, (249, 100), Color::rgb(100,0,190));

    match image.save_file("heart.tga"){
        Ok(size) => println!("number of Bytes written succesfully are : {}", size),
        Err(e) => println!("Failed to save image {:?}", e)
    }
}