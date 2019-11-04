use std::error::Error;

mod vector_math;
mod intersect;
mod polygon;
mod nff;
mod render;


fn main() -> Result<(), Box<dyn Error>> {
    let (view, scene) = nff::read()?;

    // TO DO: Create an array to hold the output pixels
    let mut target = render::RenderTarget {};

    render::render(&view, &scene, &mut target);

    // TO DO: Write the output file

    Ok(())
}
