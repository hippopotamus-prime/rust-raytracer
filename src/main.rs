use std::io;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

struct Color {
    r: f32,
    g: f32,
    b: f32
}

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Debug, Clone)]
struct Vector {
    dx: f32,
    dy: f32,
    dz: f32
}

struct View {
    from: Point,
    up: Vector,
    at: Vector,
    angle: f32,
    hither: f32,
    width: u32,
    height: u32
}

trait Intersect {
    fn intersect(&self, ray: &Vector) -> Option<Point>;
}

struct Scene {
    background: Color,
    primitives: Vec<Box<dyn Intersect>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            background: Color {r: 1.0, g: 1.0, b: 1.0},
            primitives: vec! {}
        }
    }
}

struct RenderTarget {

}

#[derive(Debug, Clone)]
struct NFFError {
    command: String,
    message: String
}

impl NFFError {
    fn new(command: &str, message: &str) -> NFFError {
        NFFError {
            command: command.to_owned(),
            message: message.to_owned()
        }
    }
}

impl fmt::Display for NFFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing command {}: {}",
            self.command, self.message)
    }
}

impl Error for NFFError {
}


fn parse_values<T>(line: &str, start_word: usize, count: usize) -> 
    Result<Vec<T>, Box<dyn Error>> where
        T: FromStr,
        <T as FromStr>::Err: Error,
        <T as FromStr>::Err: 'static {
        
    let words = line.split_whitespace().collect::<Vec<_>>();
    if words.len() != start_word + count {
        // TO DO: Define an error type for this
        panic!("wrong value count");
    }

    let mut values = Vec::<T>::new();

    for word in &words[start_word..] {
        match word.parse::<T>() {
            Ok(value) => {
                values.push(value);
            },
            Err(e) => {
                // TO DO: The returned error should include the word position
                return Err(Box::new(e));
            }
        }
    }

    Ok(values)
}

fn parse_background(line: &str) -> Result<Color, NFFError> {
    let colors = line.split_whitespace().collect::<Vec<_>>();
    if colors.len() != 3 {
        return Err(NFFError::new("b", "insufficient arguments"));
    }

    let r = match colors[0].parse() {
        Ok(value) => value,
        Err(e) => {
            return Err(NFFError::new("b", "invalid red value"));
        }
    };

    let g = match colors[1].parse() {
        Ok(value) => value,
        Err(e) => {
            return Err(NFFError::new("b", "invalid green value"));
        }
    };

    let b = match colors[2].parse() {
        Ok(value) => value,
        Err(e) => {
            return Err(NFFError::new("b", "invalid blue value"));
        }
    };

    Ok(Color {r, g, b})
}

fn parse_view(stream: &mut std::io::Stdin) -> Result<View, Box<dyn Error>> {
    let mut from: Option<Point> = None;
    let mut at: Option<Vector> = None;
    let mut up: Option<Vector> = None;
    let mut angle: Option<f32> = None;
    let mut hither: Option<f32> = None;
    let mut res: Option<(u32, u32)> = None;

    let mut line = String::new();
    loop {
        let byte_count = stream.read_line(&mut line)?;
        if byte_count == 0 {
            // TO DO: Add detail about which parameter is missing
            return Err(Box::new(NFFError::new("v", "missing parameters")));
        }

        if line.starts_with("from") {
            let values = parse_values(&line, 1, 3)?;
            from = Some(Point {x: values[0], y: values[1], z: values[2]});
        }
        else if line.starts_with("at") {
            let values = parse_values(&line, 1, 3)?;
            at = Some(Vector {dx: values[0], dy: values[1], dz: values[2]});
        }
        else if line.starts_with("up") {
            let values = parse_values(&line, 1, 3)?;
            up = Some(Vector {dx: values[0], dy: values[1], dz: values[2]});
        }
        else if line.starts_with("angle") {
            let values = parse_values(&line, 1, 1)?;
            angle = Some(values[0]);
        }
        else if line.starts_with("hither") {
            let values = parse_values(&line, 1, 1)?;
            hither = Some(values[0]);
        }
        else if line.starts_with("resolution") {
            let values = parse_values(&line, 1, 2)?;
            res = Some((values[0], values[1]));
        }

        if let (Some(from), Some(at), Some(up),
                Some(angle), Some(hither), Some(res)) =
                    (&from, &at, &up, angle, hither, res) {
            return Ok(View {
                from: from.clone(),
                at: at.clone(),
                up: up.clone(),
                angle: angle,
                hither: hither,
                width: res.0,
                height: res.1
            });
        }
    }
}

fn read_nff() -> Result<(View, Scene), Box<dyn Error>> {
    let mut view: Option<View> = None;
    let mut scene = Scene::new();

    let mut stream = io::stdin();
    let mut line = String::new();
    loop {
        let byte_count = stream.read_line(&mut line)?;
        if byte_count == 0 {
            break;
        }

        if line.starts_with("#") {
            continue;
        }
        else if line.starts_with("v") {
            view = Some(parse_view(&mut stream)?);
        }
        else if line.starts_with("b") {
            scene.background = parse_background(&line)?;
        }
    }

    match view {
        Some(view) => {
            return Ok((view, scene));
        },
        None => {
            return Err(Box::new(NFFError::new("v", "missing view")));
        }
    }
}

fn render(view: &View, scene: &Scene, target: &mut RenderTarget) {

}

fn main() -> Result<(), Box<dyn Error>> {
    // TO DO: Parse the simplest NFF commands
    // - p: polygon with no normals
    // - v: view, with from/at/up/angle/hither/res
    // - b: background color

    // Polygons go in a data structure. Start with a flat list.

    // Create an array to hold the output pixels

    // Loop over each pixel and trace rays
    // - start by drawing the background color
    // - do some math to intersect polygons

    // Write the output file

    let (view, scene) = read_nff()?;

    let mut target = RenderTarget {};

    render(&view, &scene, &mut target);

    Ok(())
}
