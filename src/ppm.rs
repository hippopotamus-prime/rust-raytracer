use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::render::RenderTarget;


pub fn write(image: &RenderTarget, path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    let mut file = File::create(path)?;

    writeln!(&mut file, "P6")?;
    writeln!(&mut file, "{} {}", image.width, image.height)?;
    writeln!(&mut file, "{}", 255)?;

    let mut row = vec![0; image.width * 3];
    for j in 0..image.height {
        for i in 0..image.width {
            let color = image.get(i, j);
            row[i * 3 + 0] = (color.r * 255.9) as u8;
            row[i * 3 + 1] = (color.g * 255.9) as u8;
            row[i * 3 + 2] = (color.b * 255.9) as u8;
        }
        file.write_all(&row[..])?;
    }
    Ok(())
}
