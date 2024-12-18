#[allow(unused_variables)]
#[allow(dead_code)]
//Using `const` for compile-time constants 
const MAX_USERS: u32 = 1000; 
// Using `static` for global constants 
static APP_NAME: &str = "MyApp";
//=============================== async, await
use tokio::time::{sleep, Duration}; 
// An asynchronous function that simulates a delay 
async fn perform_task() { 
    println!("Task started"); 
    sleep(Duration::from_secs(5)).await; //== 5초간 프로그램 진행 정지
    println!("Task completed"); 
} 
//===============================
async fn divide(a: f64, b: f64) -> Result<f64, &'static str> { 
    if b == 0.0 { 
        Err("Cannot divide by zero") 
    } else { 
        Ok(a / b) 
    } 
}
enum Shape { 
    Circle(f64), // Circle with a radius 
    Rectangle(f64, f64), // Rectangle with width and height 
    Triangle(f64, f64, f64), // Triangle with three sides 
} 
// Function to calculate the area of a shape 
fn area(shape: &Shape) -> f64 { 
    match shape { 
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius, 
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(a, b, c) => { 
            let s = (a + b + c) / 2.0; 
            (s * (s - a) * (s - b) * (s - c)).sqrt() 
        } 
    } 
}

// 트래이트 선언의 유용성 ---------------------------
trait Describable { 
    fn describe(&self) -> String; 
} 
struct Dog { 
    name: String, 
    age: u8, 
} 
impl Describable for Dog { //트레이트 사용에 의한 제약(표준화) 발생 
    fn describe(&self) -> String { 
        format!("Dog named {} is {} years old.", self.name, self.age) 
    } 
} 
struct Car { 
    model: String, 
    year: u16, 
} 
impl Describable for Car { //트레이트 사용에 의한 제약(표준화) 발생  
    fn describe(&self) -> String { 
        format!("Car model {} from year {}.", self.model, self.year) 
    } 
} 
// 한 개 타입의 메소드는 impl 블럭에 구현함 ---------------
struct Rectangle { 
    width: u32, 
    height: u32, 
} 
impl Rectangle { 
    fn new(width: u32, height: u32) -> Self { 
        Self { width, height } 
    } 
    fn area(&self) -> u32 { 
        self.width * self.height 
    }  
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height 
    } 
} 
//-----------------------------------------------
use std::borrow::Cow; //Cow = (Clone on Write)
fn process_data(input: &str) -> Cow<str> { 
    if input.contains("special") { 
        let mut owned_data = input.to_string(); 
        owned_data.push_str(" processed"); 
        Cow::Owned(owned_data) 
    } else { 
        Cow::Borrowed(input) 
    } 
} 
//---------------------------------
fn find_first_even(numbers: &[i32]) -> Option<i32> { 
    for &num in numbers { 
        if num % 2 == 0 { 
            return Some(num); // Found the first even number, break the loop 
        } 
    } 
    None // No even number found 
} 
fn skip_negatives_and_sum(numbers: &[i32]) -> i32 { 
    let mut sum = 0; 
    for &num in numbers { 
        if num < 0 { 
            continue; // Skip negative numbers 
        } 
        sum += num; 
    } 
    sum 
}
//---------------------------------
fn process_data_2(data: Option<&str>) -> Result<(), &'static str> { 
    // Check for error condition early 
    let data = match data { 
        Some(d) => d, 
        None => return Err("No data provided"), 
    }; 
    // Proceed with processing 
    if data.is_empty() { 
        return Err("Data is empty"); 
    } 
    println!("Processing data: {}", data); 
    Ok(()) 
}
//---------------------------------
fn double_number(input: &str) -> Result<i32, std::num::ParseIntError> { 
    input.parse::<i32>().map(|n| n * 2) 
}
//---------------------------------
// Function to get the length of a string or return a default value 
fn get_length_or_default(input: Option<&str>) -> usize { 
    input.map(|s| s.len()).unwrap_or(0) 
} 
// Function to parse a string to an integer or return a default value 
fn parse_or_default(input: &str) -> i32 { 
    input.parse::<i32>().unwrap_or_else(|_| 0) 
}
//---------------------------------
fn get_length(opt: Option<&str>) -> Option<usize> {
    opt.map(|s| s.len()) 
} 
fn get_first_char(opt: Option<&str>) -> Option<char> { 
    opt.and_then(|s| s.chars().next()) 
}
//---------------------------------
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> { 
    s.parse::<i32>() 
} 
fn get_first_element(vec: &Vec<i32>) -> Option<i32> { 
    vec.get(0).cloned() 
}
fn get_third_element(vec: &Vec<i32>) -> Option<i32> { 
    vec.get(2).cloned() 
}
//---------------------------------
fn process_data_3(input: &str) -> Cow<str> { 
    if input.contains("special") { 
        Cow::Owned(input.replace("special", "특수")) 
    } else { 
        Cow::Borrowed(input) 
    } 
}
//---------------------------------
//Rc (Reference Counted) and Arc (Atomic Reference Counted)
//---------------------------------
// Good example using Box for heap allocation and dynamic dispatch 
trait Animal { 
    fn speak(&self); 
} 
struct Dog_2; 
struct Cat_2; 
impl Animal for Dog_2 { 
    fn speak(&self) { 
        println!("개: 멍 멍!"); 
    } 
} 
impl Animal for Cat_2 { 
    fn speak(&self) { 
        println!("고양이: 야옹 야옹!"); 
    } 
} 
//---------------------------------
#[allow(unused_imports)]
use std::rc::Rc; 
use std::sync::Arc; 
use std::thread; 
//---------------------------------
// module for geometry-related functions 
mod geometry { 
    pub fn calculate_area(width: f64, height: f64) -> f64 { 
        width * height 
    } 
} 
// module for printing-related functions 
mod printer { 
    pub fn print_area(area: f64) {
        println!("The area of the rectangle is: {}", area); 
    } 
}
//================================= async, await
#[tokio::main] 
async fn main() { 
    println!("Starting main function"); 
    perform_task().await; //=== 프로그램 진행 정지
    match divide(4.0, 2.0).await { 
        Ok(result) => println!("Result: {}", result), 
        Err(e) => println!("Error: {}", e), 
    } 
    println!("Main function completed"); 
    println!("------------------------------------");
//fn main() {
    match divide(4.0, 2.0).await { 
        Ok(result) => println!("Result: {}", result), 
        Err(e) => println!("Error: {}", e), 
    } 
    println!("------------------------------------");
    match divide(4.0, 0.0).await { 
        Ok(result) => println!("Result: {}", result), 
        Err(e) => println!("Error: {}", e), 
    } 
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let result: Vec<i32> = numbers.iter() 
        .filter(|&&x| x % 2 == 0) 
        .map(|&x| x * x) 
        .collect(); 
    println!("{:?}", numbers); 
    println!("{:?}", result); 
    println!("{:?}", numbers); 
    println!("{:?}", result); 
    println!("------------------------------------");
    let circle = Shape::Circle(5.0); 
    let rectangle = Shape::Rectangle(4.0, 6.1); 
    let triangle = Shape::Triangle(3.0, 4.0, 6.0); 
    println!("Circle area: {:08.4}", area(&circle)); 
    println!("Rectangle area: {:08.4}", area(&rectangle));
    println!("Triangle area: {:08.4}", area(&triangle)); 
    println!("------------------------------------");
    let my_dog = Dog { name: String::from("누렁이"), age: 3 }; 
    let my_car = Car { model: String::from("쵸코"), year: 2020 }; 
    println!("{}", my_dog.describe()); 
    println!("{}", my_car.describe()); 
    println!("------------------------------------");
    let borrowed_data = "This is a test"; 
    let owned_data = "This is a special test"; 
    let result1 = process_data(borrowed_data); 
    let result2 = process_data(owned_data); 
    println!("Result 1: {}", result1); 
    println!("Result 2: {}", result2); 
    println!("Result 1_2: {}", result1); 
    println!("Result 2_2: {}", result2); 
    println!("------------------------------------");
    let data = Rc::new(vec![1, 2, 3]); 
    let data_clone = Rc::clone(&data); 
    println!("Data: {:?}", data_clone); 
    println!("------------------------------------");
    let numbers = vec![1, 2, 3, 4, 5]; 
    // Using `for` loop to iterate over the vector 
    for number in numbers.iter() { 
        println!("The number is: {}", number); 
    }
    println!("------------------------------------");
    let numbers = [1, 3, 5, 6, 7, -2, 8]; 
    if let Some(even) = find_first_even(&numbers) { 
        println!("First even number: {}", even); 
    } else { 
        println!("No even number found."); 
    } 
    let sum = skip_negatives_and_sum(&numbers); 
    println!("Sum of non-negative numbers: {}", sum);
    println!("------------------------------------");
    println!("{:?}", process_data_2(Some("Hello")));
    println!("{:?}", process_data_2(Some("")));
    println!("{:?}", process_data_2(None));
    println!("------------------------------------");
    match double_number("10") { 
        Ok(n) => println!("Doubled number: {}", n), 
        Err(e) => println!("Error: {}", e), 
    }
    println!("------------------------------------");
    let length = get_length_or_default(Some("안녕하세요")); 
    println!("len(안녕하세요): {}", length);
    let length = get_length_or_default(None); 
    println!("len(None): {}", length);
    let number = parse_or_default("42"); 
    println!("Number: {}", number);
    let number = parse_or_default("not a number"); 
    println!("not a number: {}", number);
    println!("------------------------------------");
    let some_string = Some("안녕하세요. 러스트!"); 
    let none_string: Option<&str> = None; 
    let length = get_length(some_string); 
    let Some(length_1) = length else {return ();};
    println!("len(안녕하세요. 러스트!): {:?}", length);  
    println!("len(안녕하세요. 러스트!): {:?}", length_1);  
    let first_char = get_first_char(some_string); 
    println!("1st(안녕하세요. 러스트!): {:?}", first_char); 
    let no_length = get_length(none_string); 
    println!("len(none_string): {:?}", no_length);  
    let no_first_char = get_first_char(none_string); 
    println!("1st(none_string): {:?}", no_first_char);
    println!("------------------------------------");
    let valid_number = "42"; 
    let invalid_number = "abc"; 
    match parse_number(valid_number) { 
        Ok(n) => println!("Parsed number: {}", n), 
        Err(e) => println!("Failed to parse number: {}", e), 
    } 
    match parse_number(invalid_number) { 
        Ok(n) => println!("Parsed number: {}", n), 
        Err(e) => println!("Failed to parse number: {}", e), 
    } 
    let numbers = vec![1, 2]; 
    let empty_vec: Vec<i32> = vec![]; 
    match get_first_element(&numbers) { 
        Some(n) => println!("First element: {}", n), 
        None => println!("No elements in the vector"), 
    }
    match get_third_element(&numbers) { 
        Some(n) => println!("Third element: {}", n),
        None => println!("No third element in the vector"), 
    } 
    match get_first_element(&empty_vec) { 
        Some(n) => println!("First element: {}", n),
        None => println!("No elements in the vector"), 
    }
    println!("------------------------------------");    
    let data = "This is a special string."; 
    let result = process_data_3(data); 
    println!("{}", result); // Output: This is a ordinary string.
    println!("------------------------------------");    
    // Single-threaded example with Rc 
    let data = Rc::new("Hello, Rc!".to_string()); 
    let data_clone = Rc::clone(&data); 
    println!("{}", data_clone); // Output: Hello, Rc! 
    // Multi-threaded example with Arc 
    let data = Arc::new("Hello, Arc!".to_string()); 
    let data_clone = Arc::clone(&data); 
    let handle = thread::spawn(move || { 
        println!("{}", data_clone); // Output: Hello, Arc! 
    }); 
    handle.join().unwrap();
    println!("------------------------------------");    
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog_2), Box::new(Cat_2)]; 
    for animal in animals { 
        animal.speak(); 
    }
    println!("------------------------------------");    
    let width = 5.0; 
    let height = 10.0; 
    let area = geometry::calculate_area(width, height); 
    printer::print_area(area);    
}
