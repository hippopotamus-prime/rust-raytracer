use std::error::Error;

extern crate clap;
use clap::{App, Arg, ArgGroup};

mod vector_math;
mod color;
mod shape;
mod polygon;
mod sphere;
mod cone;
mod nff;
mod render;
mod ppm;
mod phong;
mod blinn_phong;
mod scene;
mod space_partition;


fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Ray Tracer")
        .about("Generates PPM images using NFF commands read from stdin")
        .arg(Arg::with_name("phong")
            .long("phong")
            .help("Use Phong shading (default)"))
        .arg(Arg::with_name("blinn-phong")
            .long("blinn-phong")
            .help("Use Blinn-Phong shading"))
        .group(ArgGroup::with_name("shading")
            .args(&["phong", "blinn-phong"]))
        .get_matches();

    let use_phong = !matches.is_present("blinn-phong");

    let (view, scene) = nff::read(use_phong)?;

    let mut target = render::RenderTarget::new(
        view.width as usize, view.height as usize);

    let partition = scene.build_space_partition();
    render::render(&view, &scene, &mut target);

    ppm::write(&target, "trace.ppm")?;

    Ok(())
}
