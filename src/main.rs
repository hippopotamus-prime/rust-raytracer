use std::io;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::ops;

#[derive(Debug, Clone)]
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

impl ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Point {
        Point {
            x: self.x + rhs.dx, 
            y: self.y + rhs.dy,
            z: self.z + rhs.dz
        }
    }
}

impl ops::AddAssign<&Vector> for Point {
    fn add_assign(&mut self, other: &Vector) {
        self.x += other.dx;
        self.y += other.dy;
        self.z += other.dz;
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Vector {
        Vector {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
            dz: self.z - rhs.z
        }
    }
}

impl ops::SubAssign<&Vector> for Point {
    fn sub_assign(&mut self, other: &Vector) {
        self.x -= other.dx;
        self.y -= other.dy;
        self.z -= other.dz;
    }
}

impl ops::Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, scale: f32) -> Vector {
        Vector {
            dx: self.dx * scale,
            dy: self.dy * scale,
            dz: self.dz * scale
        }
    }
}

impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, scale: f32) {
        self.dx *= scale;
        self.dy *= scale;
        self.dz *= scale;
    }
}

impl ops::Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            dx: -self.dx,
            dy: -self.dy,
            dz: -self.dz
        }
    }
}

fn cross(v1: &Vector, v2: &Vector) -> Vector {
    Vector {
        dx: v1.dy * v2.dz - v1.dz * v2.dy,
        dy: v1.dz * v2.dx - v1.dx * v2.dz,
        dz: v1.dx * v2.dy - v1.dy * v2.dx
    }
}

fn dot(v1: &Vector, v2: &Vector) -> f32 {
    v1.dx * v2.dx + v1.dy * v2.dy + v1.dz * v2.dz
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

struct IntersectResult {
    normal: Vector,
    dist: f32
}

trait Intersect {
    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult>;
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

struct PointNormal {
    point: Point,
    normal: Vector
}

struct Polygon {
    vertices: Vec<PointNormal>
}

impl Intersect for Polygon {
    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult> {
        None
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

fn parse_polygon(line: &str, stream: &mut std::io::Stdin) ->
        Result<Polygon, Box<dyn Error>> {
    let vertex_count = parse_values(line, 1, 1)?[0];
    if vertex_count < 3 {
        return Err(Box::new(NFFError::new("p", "insufficient vertex count")));
    }

    let mut vertices = Vec::<PointNormal>::new();

    let mut line = String::new();
    for _ in 0..vertex_count {
        let byte_count = stream.read_line(&mut line)?;
        if byte_count == 0 {
            return Err(Box::new(NFFError::new("p", "missing parameters")));
        }

        let values = parse_values(&line, 0, 3)?;
        let point = Point {x: values[0], y: values[1], z: values[2]};
        let normal = Vector {dx: 0.0, dy: 0.0, dz: 0.0};
        vertices.push(PointNormal {point: point, normal: normal});
    }

    // Calculate one normal vector for the whole polygon, assuming the
    // points are defined counter-clockwise (right handed).
    let v1 = &vertices[1].point - &vertices[0].point;
    let v2 = &vertices[2].point - &vertices[0].point;
    let normal = cross(&v1, &v2);

    for vertex in &mut vertices {
        vertex.normal = normal.clone();
    }

    Ok(Polygon {
        vertices: vertices
    })
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
        else if line.starts_with("p") {
            let poly = parse_polygon(&line, &mut stream)?;
            scene.primitives.push(Box::new(poly));
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
