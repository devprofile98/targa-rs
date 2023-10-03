# targa-rs
library to work with ``TARGA`` ``[.tga]`` format in rust

### Create a .tga image
    let mut image  = TargaImage::new(500, 500);
    // or load the image
    if let Ok(image) = TargaImage::load("path_to_image.tga") {
        .....
    }
### Change pixel values
    image.set_pixel(200, 300, &color);
    image[6000] = color.clone();
### Write it
    match image.save_file("shapes.tga"){
        Ok(size) => println!("number of Bytes written succesfully are : {}", size),
        Err(e) => println!("Failed to save image {:?}", e)
    }
