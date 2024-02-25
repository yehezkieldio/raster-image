extern crate raster; 

fn main() {
    let image = raster::open("tests/in/Avatar.jpg");

    if image.is_err() {
        println!("Error: {:?}", image.err().unwrap());
        return;
    }

    let mut image_real = image.unwrap();
    raster::filter::grayscale(&mut image_real).unwrap();

    raster::save(&image_real, "tests/out/Avatar_out.jpg").unwrap();
    print!("Test image successful.");
}
