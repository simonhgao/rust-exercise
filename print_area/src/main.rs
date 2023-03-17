trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
    name: String,
}

struct Square {
    side: f64,
    name: String,
}

struct Triangle {
    base: f64,
    height: f64,
    name: String,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn name(&self) -> &str {
        &self.name
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("The area of {} is {}", shape.name(), shape.area());
}

fn main() {
    let circle = Circle {
        radius: 3.0,
        name: "circle".to_string(),
    };
    let triangle = Triangle {
        base: 4.0,
        height: 5.0,
        name: "triangle".to_string(),
    };
    let square = Square {
        side: 6.0,
        name: "square".to_string(),
    };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
