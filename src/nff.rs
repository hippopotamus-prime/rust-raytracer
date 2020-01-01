use std::fmt;
use std::str::FromStr;
use std::io;
use std::error::Error;
use std::rc::Rc;

use crate::vector_math::Vector;
use crate::vector_math::Point;
use crate::vector_math::PointNormal;
use crate::vector_math;
use crate::polygon::Polygon;
use crate::sphere::Sphere;
use crate::cone::Cone;
use crate::color::Color;
use crate::render::View;
use crate::render::Surface;
use crate::phong::Phong;
use crate::blinn_phong::BlinnPhong;
use crate::scene::Scene;
use crate::scene::Light;


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
    let mut at: Option<Point> = None;
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
            at = Some(Point {x: values[0], y: values[1], z: values[2]});
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
        let mut normal = Vector {dx: values[3], dy: values[4], dz: values[5]};
        normal.normalize();

        vertices.push(PointNormal {point: point, normal: normal});
    }

    Ok(Polygon {
        vertices: vertices
    })
}

fn parse_cone(stream: &mut std::io::Stdin) ->
        Result<Cone, Box<dyn Error>> {
    let mut base_line = String::new();
    stream.read_line(&mut base_line)?;
    let base_values = parse_values(&base_line, 0, 4)?;

    let mut apex_line = String::new();
    stream.read_line(&mut apex_line)?;
    let apex_values = parse_values(&apex_line, 0, 4)?;

    Ok(Cone {
        base: Point {x: base_values[0], y: base_values[1], z: base_values[2]},
        apex: Point {x: apex_values[0], y: apex_values[1], z: apex_values[2]},
        base_radius: base_values[3],
        apex_radius: apex_values[3]
    })
}

fn parse_cone_one_line(args: &[&str]) ->
        Result<Cone, Box<dyn Error>> {
    let bx = args[0].parse()?;
    let by = args[1].parse()?;
    let bz = args[2].parse()?;
    let br = args[3].parse()?;

    let ax = args[4].parse()?;
    let ay = args[5].parse()?;
    let az = args[6].parse()?;
    let ar = args[7].parse()?;

    Ok(Cone {
        base: Point {x: bx, y: by, z: bz},
        apex: Point {x: ax, y: ay, z: az},
        base_radius: br,
        apex_radius: ar
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
    let mut normal = vector_math::cross(&v1, &v2);
    normal.normalize();

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

fn parse_fill(use_phong: bool, args: &[&str]) ->
        Result<Rc<dyn Surface>, Box<dyn Error>> {
    let r = args[0].parse()?;
    let g = args[1].parse()?;
    let b = args[2].parse()?;
    let kd = args[3].parse()?;
    let ks = args[4].parse()?;
    let shine = args[5].parse()?;
    let transmittance = args[6].parse()?;
    let refraction_index = args[7].parse()?;

    // TO DO: The surface types may need a rethink.  The C++ code treated
    // Phong and Blinn-Phong as different surfaces that could coexist in the
    // same NFF file using command extensions, but in practice the extensions
    // were never used.  Here, they're treated as different ways of shading
    // the same kind of surface, so they're not logically part of the scene.
    // It might be more appropriate to have a single "SurfaceProperties"
    // type and feed the shading choice into render::render()?

    if use_phong {
        Ok(Rc::new(Phong {
            color: Color {r, g, b},
            diffuse_component: kd,
            specular_component: ks,
            shine: shine,
            reflectance: ks,
            transmittance: transmittance,
            refraction_index: refraction_index
        }))
    } else {
        Ok(Rc::new(BlinnPhong {
            color: Color {r, g, b},
            diffuse_component: kd,
            specular_component: ks,
            shine: shine,
            reflectance: ks,
            transmittance: transmittance,
            refraction_index: refraction_index
        }))
    }
}

fn parse_white_light(args: &[&str]) -> Result<Light, Box<dyn Error>> {
    let x = args[0].parse()?;
    let y = args[1].parse()?;
    let z = args[2].parse()?;

    Ok(Light {
        position: Point {x, y, z},
        color: Color {r: 1.0, g: 1.0, b: 1.0}
    })
}

fn parse_colored_light(args: &[&str]) -> Result<Light, Box<dyn Error>> {
    let x = args[0].parse()?;
    let y = args[1].parse()?;
    let z = args[2].parse()?;

    let r = args[3].parse()?;
    let g = args[4].parse()?;
    let b = args[5].parse()?;

    Ok(Light {
        position: Point {x, y, z},
        color: Color {r, g, b}
    })
}

fn parse_sphere(args: &[&str]) -> Result<Sphere, Box<dyn Error>> {
    let x = args[0].parse()?;
    let y = args[1].parse()?;
    let z = args[2].parse()?;

    let radius = args[3].parse()?;

    Ok(Sphere {
        center: Point {x, y, z},
        radius: radius
    })
}

pub fn read(use_phong: bool) -> Result<(View, Scene), Box<dyn Error>> {
    let mut view: Option<View> = None;
    let mut scene = Scene::new();

    let mut surface: Rc<dyn Surface> = Rc::new(Phong {
        color: Color {r: 1.0, g: 1.0, b: 1.0},
        diffuse_component: 1.0,
        specular_component: 0.0,
        shine: 1.0,
        reflectance: 0.0,
        transmittance: 0.0,
        refraction_index: 1.0
    });

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
        } else if command == "b" && args.len() == 3 {
            scene.background = parse_background(args)?;
        } else if command == "pp" && args.len() == 1 {
            let poly = parse_polygon_patch(args, &mut stream)?;
            scene.add_primitive(Box::new(poly), surface.clone());
        } else if command == "p" && args.len() == 1 {
            let poly = parse_polygon(args, &mut stream)?;
            scene.add_primitive(Box::new(poly), surface.clone());
        } else if command == "f" && args.len() == 8 {
            surface = parse_fill(use_phong, args)?;
        } else if command == "l" && args.len() == 3 {
            let light = parse_white_light(args)?;
            scene.add_light(light);
        } else if command == "l" && args.len() == 6 {
            let light = parse_colored_light(args)?;
            scene.add_light(light);
        } else if command == "s" && args.len() == 4 {
            let sphere = parse_sphere(args)?;
            scene.add_primitive(Box::new(sphere), surface.clone());
        } else if command == "c" && args.len() == 0 {
            let cone = parse_cone(&mut stream)?;
            scene.add_primitive(Box::new(cone), surface.clone());
        } else if command == "c" && args.len() == 8 {
            let cone = parse_cone_one_line(args)?;
            scene.add_primitive(Box::new(cone), surface.clone());
        } else {
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
