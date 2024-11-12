fn main() {
    hello_world();
    tell_height(182);
    human_id( "Joel", 55,  182.2);
    let x: i32 = {
        let price = 5;
        let qty = 10;
        price * qty 
    };
    println!("Result is: {}", x);
    let y = add(4, 6);
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}.", add(4,6));

    // Calling BMI 
    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, rust");
}

fn tell_height(height: u32) {
    println!("My heigh is {}", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

const _X: &str = "";
static _Y: String = String::new();

fn add(a: i32, b: i32) -> i32 {
    a + b // Expression, no "return" and no ";"
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}

/*
Expressions and Statements

Expression: Anything returns a value
All below are expressions
5
true & false
add(3,4)
if condition {value1} else {value2}
({code})

Statement: Anything does not return a value
Almost all statements in Rust end with;
let y = let x = 10;  // Error: `let x = 10` is a statement, not an expression.
1 Variable declarations: let x = 5;
2 Function definitions: fn foo() {}
3 Control flow statements: if condition { code } else {}, while condition {}, etc.
*/

