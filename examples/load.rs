use tgars::TargaImage;

fn main() {
    {
        if let Ok(image) = TargaImage::load("path_to_image.tga") {
            match image.save_file("save.tga") {
                Ok(size) => println!("number of Bytes written succesfully are : {}", size),
                Err(e) => println!("Failed to save image {:?}", e),
            }
        } else {
            println!("Failed to load the image");
        }
    }
}
