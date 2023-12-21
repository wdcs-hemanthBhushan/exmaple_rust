mod example_1;
mod example_2;

use example_1::{execute_task, modify_task_description, SimpleTask, Task};
use example_2::{
    calculate_area, divide, filter_even_numbers, larger, Book, Describable, Dog, Shape,
};

fn main() {
    //----------------example1-----------------------
    let mut task: SimpleTask = SimpleTask::new("description");

    println!("task.description() {}", task.description());

    task.modify_description("new_description");

    println!("task.description() {}", task.description());

    task.execute();

    execute_task(&task);
    modify_task_description(&mut task, "new_description");

    //----------------end of example one-------------

    // Traits and Structs
    let dog = Dog {
        name: String::from("Buddy"),
    };
    dog.describe();

    // Enums and Pattern Matching
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let triangle = Shape::Triangle(3.0, 6.0);

    println!("Area of Circle: {}", calculate_area(&circle));
    println!("Area of Square: {}", calculate_area(&square));
    println!("Area of Triangle: {}", calculate_area(&triangle));

    // Generic Function with Constraints
    let larger_result = larger(42, 17);
    println!("Larger of 42 and 17: {}", larger_result);

    // Option Type Function
    match divide(10.0, 2.0) {
        Some(result) => println!("Division result: {}", result),
        None => println!("Cannot divide by zero!"),
    }

    // Lifetimes in Structs
    let book_title = String::from("The Rust Programming Language");
    let rust_book = Book { title: &book_title };

    // Using Closures
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = filter_even_numbers(numbers);
    println!("Even Numbers: {:?}", even_numbers);

    // Example from task2.rs
    example_2::run_example();
}
