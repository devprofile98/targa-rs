mod util;
use util::{draw_rectagnle, draw_triangle};

use tgars::{TargaImage, Color};

fn main() {
        {
        let mut image  = TargaImage::new(500, 500);
        draw_rectagnle(&mut image,500, 500, (0,0), Color::rgb(142, 224, 245));
        draw_rectagnle(&mut image,100, 100, (200,200), Color::rgb(255, 0, 0));
        draw_triangle(&mut image, (181, 300), 160, 145, Color::rgb(100,0,190), true);

        match image.save_file("shapes.tga"){
            Ok(size) => println!("number of Bytes written succesfully are : {}", size),
            Err(e) => println!("Failed to save image {:?}", e)
        }
    }
}