use tgars::TargaImage;

fn main() {
    {
        match TargaImage::load("path_to_image.tga") {
            Ok(image) => match image.save_file("save.tga") {
                Ok(size) => println!("number of Bytes written succesfully are : {}", size),
                Err(e) => println!("Failed to save image {:?}", e),
            },
            Err(e) => {
                println!("Failed to load the image because of '{}'", e);
            }
        }
    }
}
