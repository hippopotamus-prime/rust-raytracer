use std::ops;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32
}

impl Vector {
    pub fn magnitude(&self) -> f32 {
        let m2 = self.dx * self.dx + self.dy * self.dy + self.dz * self.dz;
        m2.sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        *self /= mag;
    }

    pub fn normalized(&self) -> Vector {
        let mag = self.magnitude();
        self / mag
    }
}

#[derive(Debug, Clone)]
pub struct PointNormal {
    pub point: Point,
    pub normal: Vector
}

impl ops::Add<&Vector> for Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Point {
        Point {
            x: self.x + rhs.dx, 
            y: self.y + rhs.dy,
            z: self.z + rhs.dz
        }
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
            z: self.z + rhs.dz
        }
    }
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

impl ops::Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
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

impl ops::Sub<&Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Vector {
        Vector {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
            dz: self.z - rhs.z
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
            dz: self.z - rhs.z
        }
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

impl ops::Sub<Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
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

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scale: f32) -> Vector {
        Vector {
            dx: self.dx * scale,
            dy: self.dy * scale,
            dz: self.dz * scale
        }
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

impl ops::Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: &Vector) -> Vector {
        Vector {
            dx: vector.dx * self,
            dy: vector.dy * self,
            dz: vector.dz * self
        }
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            dx: vector.dx * self,
            dy: vector.dy * self,
            dz: vector.dz * self
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

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scale: f32) -> Vector {
        Vector {
            dx: self.dx / scale,
            dy: self.dy / scale, 
            dz: self.dz / scale
        }
    }
}

impl ops::Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, scale: f32) -> Vector {
        Vector {
            dx: self.dx / scale,
            dy: self.dy / scale, 
            dz: self.dz / scale
        }
    }
}

impl ops::DivAssign<f32> for Vector {
    fn div_assign(&mut self, scale: f32) {
        self.dx /= scale;
        self.dy /= scale;
        self.dz /= scale;
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            dx: -self.dx,
            dy: -self.dy,
            dz: -self.dz
        }
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

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
            dz: self.dz + rhs.dz
        }
    }
}

impl ops::Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
            dz: self.dz + rhs.dz
        }
    }
}

impl ops::Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
            dz: self.dz + rhs.dz
        }
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
            dz: self.dz + rhs.dz
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
            dz: self.dz - rhs.dz
        }
    }
}

impl ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        Vector {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
            dz: self.dz - rhs.dz
        }
    }
}

impl ops::Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
            dz: self.dz - rhs.dz
        }
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        Vector {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
            dz: self.dz - rhs.dz
        }
    }
}

pub fn cross(v1: &Vector, v2: &Vector) -> Vector {
    Vector {
        dx: v1.dy * v2.dz - v1.dz * v2.dy,
        dy: v1.dz * v2.dx - v1.dx * v2.dz,
        dz: v1.dx * v2.dy - v1.dy * v2.dx
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
    v1.dx * v2.dx + v1.dy * v2.dy + v1.dz * v2.dz
}

pub fn interpolate(v1: &Vector, v2: &Vector, scale: f32) -> Vector {
    let mut result = Vector {
        dx: v1.dx * scale + v2.dx * (1.0 - scale),
        dy: v1.dy * scale + v2.dy * (1.0 - scale),
        dz: v1.dz * scale + v2.dz * (1.0 - scale)
    };
    result.normalize();
    result
}
