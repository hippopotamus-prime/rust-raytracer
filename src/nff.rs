use std::fmt;
use std::str::FromStr;
use std::io;
use std::error::Error;

use crate::vector_math::Vector;
use crate::vector_math::Point;
use crate::vector_math::PointNormal;
use crate::vector_math;
use crate::polygon::Polygon;
use crate::render::Color;
use crate::render::Scene;
use crate::render::View;

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

fn parse_background(args: &[&str]) -> Result<Color, NFFError> {
    let r = match args[0].parse() {
        Ok(value) => value,
        Err(e) => {
            return Err(NFFError::new("b", "invalid red value"));
        }
    };

    let g = match args[1].parse() {
        Ok(value) => value,
        Err(e) => {
            return Err(NFFError::new("b", "invalid green value"));
        }
    };

    let b = match args[2].parse() {
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

    loop {
        let mut line = String::new();
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

fn parse_polygon_patch(args: &[&str], stream: &mut std::io::Stdin) ->
        Result<Polygon, Box<dyn Error>> {
    let vertex_count = args[0].parse::<u32>()?;
    if vertex_count < 3 {
        return Err(Box::new(NFFError::new("pp", "insufficient vertex count")));
    }

    let mut vertices = Vec::<PointNormal>::new();

    for _ in 0..vertex_count {
        let mut line = String::new();
        let byte_count = stream.read_line(&mut line)?;
        if byte_count == 0 {
            return Err(Box::new(NFFError::new("pp", "missing paramters")));
        }

        let values = parse_values(&line, 0, 6)?;
        let point = Point {x: values[0], y: values[1], z: values[2]};
        let mut normal = Vector {dx: values[3], dy: values[2], dz: values[3]};
        normal.normalize();

        vertices.push(PointNormal {point: point, normal: normal});
    }

    Ok(Polygon {
        vertices: vertices
    })
}

fn parse_polygon(args: &[&str], stream: &mut std::io::Stdin) ->
        Result<Polygon, Box<dyn Error>> {
    let vertex_count = args[0].parse::<u32>()?;
    if vertex_count < 3 {
        return Err(Box::new(NFFError::new("p", "insufficient vertex count")));
    }

    let mut points = Vec::<Point>::new();

    for _ in 0..vertex_count {
        let mut line = String::new();
        let byte_count = stream.read_line(&mut line)?;
        if byte_count == 0 {
            return Err(Box::new(NFFError::new("p", "missing parameters")));
        }

        let values = parse_values(&line, 0, 3)?;
        let point = Point {x: values[0], y: values[1], z: values[2]};
        points.push(point);
    }

    // Calculate one normal vector for the whole polygon, assuming the
    // points are defined counter-clockwise (right handed).
    let v1 = &points[1] - &points[0];
    let v2 = &points[2] - &points[0];
    let normal = vector_math::cross(&v1, &v2);

    let mut vertices = Vec::<PointNormal>::new();
    for point in points {
        vertices.push(PointNormal {
            point: point,
            normal: normal.clone()
        })
    }

    Ok(Polygon {
        vertices: vertices
    })
}

pub fn read() -> Result<(View, Scene), Box<dyn Error>> {
    let mut view: Option<View> = None;
    let mut scene = Scene::new();

    let mut stream = io::stdin();
    loop {
        let mut line = String::new();
        let byte_count = stream.read_line(&mut line)?;
        if byte_count == 0 {
            break;
        }

        if line.starts_with("#") {
            continue;
        }

        let tokens = line.split_whitespace().collect::<Vec<_>>();
        if tokens.is_empty() {
            continue;
        }

        let command = tokens[0];
        let args = &tokens[1..];
        if command == "v" && args.len() == 0 {
            view = Some(parse_view(&mut stream)?);
        }
        else if command == "b" && args.len() == 3 {
            scene.background = parse_background(args)?;
        }
        else if command == "pp" && args.len() == 1 {
            let poly = parse_polygon_patch(args, &mut stream)?;
            scene.primitives.push(Box::new(poly));
        }
        else if command == "p" && args.len() == 1 {
            let poly = parse_polygon(args, &mut stream)?;
            scene.primitives.push(Box::new(poly));
        }
        else {
            eprintln!("unrecognized command: {}", line);
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