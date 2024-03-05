use magnus::{define_module, exception, function, prelude::*, Error};
use image::io::Reader as ImageReader;

fn png_to_jpg(input: String) -> Result<String, Error> {
    let output = format!("{input}.jpg");
    let reader = ImageReader::open(input)
        .map_err(|_| Error::new(exception::runtime_error(), "Could not open image"))?;
    let img = reader.decode()
        .map_err(|_| Error::new(exception::runtime_error(), "Could not decode image".to_string()))?;

    img.save_with_format(&output, image::ImageFormat::Jpeg)
        .map_err(|_| Error::new(exception::runtime_error(), "Could not save image".to_string()))?;

    Ok(output)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Rimage")?;
    module.define_singleton_method("png_to_jpg", function!(png_to_jpg, 1))?;
    Ok(())
}
