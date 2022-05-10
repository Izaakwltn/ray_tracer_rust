//------------------------------------------------------------------------
//Point struct
//------------------------------------------------------------------------

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn build_point(x: f32, y: f32, z: f32) -> Point {
    Point { x, y, z }
}

fn default_point() -> Point {
    build_point(0.0, 1.0, 0.0)
}

impl Point {
    fn copy(&self) -> Point {
        build_point(self.x, self.y, self.z)
    }

    fn point_to_vector(&self) -> Vector {
        build_vector(self.x, self.y, self.z)
    }
}

//------------------------------------------------------------------------
//Vector struct
//------------------------------------------------------------------------

struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

fn build_vector(x: f32, y: f32, z: f32) -> Vector {
    Vector { x, y, z }
}

fn default_vector() {
    build_vector(0.0, 1.0, 0.0);
}

impl Vector {
    fn copy(&self) -> Vector {
        build_vector(self.x, self.y, self.z)
    }

    fn vector_to_point(&self) -> Point {
        build_point(self.x, self.y, self.z)
    }

    fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    fn scalar_mult(&self, n: f32) -> Vector {
        build_vector(self.x * n, self.y * n, self.z * n)
    }
}

//------------------------------------------------------------------------
//Other Vector Calculations
//------------------------------------------------------------------------

fn vect_add(a: Vector, b: Vector) -> Vector {
    build_vector(a.x + b.x, a.y + b.y, a.z + b.z)
}

fn vect_subtract(a: Vector, b: Vector) -> Vector {
    build_vector(a.x - b.x, a.y - b.y, a.z - b.z)
}

fn dot_product(a: Vector, b: Vector) -> f32 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

fn cross_product(a: Vector, b: Vector) -> Vector {
    Vector {
        x: (a.y * b.z) - (a.z * b.y),
        y: (a.z * b.x) - (a.x * b.z),
        z: (a.x * b.y) - (a.y * b.x),
    }
}
//------------------------------------------------------------------------
//Rays
//------------------------------------------------------------------------

struct Ray {
    origin: Point,
    direction: Vector,
    t_max: f32,
}

fn build_ray(origin: Point, direction: Vector, t_max: f32) -> Ray {
    Ray {
        origin,
        direction,
        t_max,
    }
}

fn default_ray() {}
//direction is normalized- x^2 + y^2 + z^2 = 1 (direction)
//------------------------------------------------------------------------
//Shapes
//------------------------------------------------------------------------

fn is_intersection

struct Sphere {
    c: Point,
}

fn is_intersection(ray: Ray, shape: Shape) -> bool {}
fn main() {
    let test_point = default_point();
    println!(
        "Hello, world! {} {} {}",
        test_point.x, test_point.y, test_point.z
    );
}
