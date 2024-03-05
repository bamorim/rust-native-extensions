use neon::prelude::*;
use image::io::Reader as ImageReader;

fn png_to_jpg(mut cx: FunctionContext) -> JsResult<JsString> {
    let input_arg: Handle<JsString> = cx.argument::<JsString>(0)?;
    let input = input_arg.value(&mut cx);
    let output = format!("{input}.jpg");
    let reader = ImageReader::open(input)
      .or_else(|_| cx.throw_error("Could not open image"))?;
    let img = reader.decode()
      .or_else(|_| cx.throw_error("Could not decode image"))?;

    img.save_with_format(&output, image::ImageFormat::Jpeg)
      .or_else(|_| cx.throw_error("Could not save image"))?;

    Ok(cx.string(output))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("png_to_jpg", png_to_jpg)?;
    Ok(())
}
