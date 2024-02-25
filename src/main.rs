use image::io::Reader as ImageReader;

extern crate raster; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://s4.anilist.co/file/anilistcdn/media/anime/banner/145064-esDtAY2He7sk.jpg").await?.bytes().await?;

    let img = ImageReader::new(std::io::Cursor::new(resp)).with_guessed_format()?.decode()?;
    img.save("tests/in/1.jpg")?;

    let raster_image = raster::open("tests/in/1.jpg");
    let mut raster_image_real = raster_image.unwrap();
    raster::filter::grayscale(&mut raster_image_real).unwrap();
    raster::save(&raster_image_real, "tests/out/1.jpg").unwrap();

    print!("Test image successful.");

    Ok(())
}
