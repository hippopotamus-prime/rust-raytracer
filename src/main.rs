use std::error::Error;

mod vector_math;
mod intersect;
mod polygon;
mod nff;
mod render;
mod ppm;


fn main() -> Result<(), Box<dyn Error>> {
    let (view, scene) = nff::read()?;

    let mut target = render::RenderTarget::new(
        view.width as usize, view.height as usize);

    render::render(&view, &scene, &mut target);

    ppm::write(&target, "trace.ppm")?;

    Ok(())
}
