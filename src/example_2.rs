// example_2.rs
// Traits

pub trait Describable {
    fn describe(&self);
}

// Struct implementing Describable
pub struct Dog {
    pub name: String,
}

impl Describable for Dog {
    fn describe(&self) {
        println!("This is a dog named {}", self.name);
    }
}

// Enum representing different shapes
pub enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

// Function to calculate area based on shape
pub fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Triangle(base, height) => 0.5 * base * height,
    }
}

// Generic function with constraints
pub fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Option type function
pub fn divide(num: f64, denom: f64) -> Option<f64> {
    if denom != 0.0 {
        Some(num / denom)
    } else {
        None
    }
}

// Using lifetimes in structs
pub struct Book<'a> {
    pub title: &'a str,
}

// Function using closures
pub fn filter_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&x| x % 2 == 0).collect()
}

// Additional function for the task
pub fn run_example() {
    let dog = Dog {
        name: String::from("Rusty"),
    };
    dog.describe();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_larger() {
        assert_eq!(super::larger(42, 17), 42);
    }

    #[test]
    fn test_divide() {
        assert_eq!(super::divide(10.0, 2.0), Some(5.0));
        assert_eq!(super::divide(10.0, 0.0), None);
    }
}
