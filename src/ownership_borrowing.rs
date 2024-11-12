/*
Problems in Memory Control in other languages: 
1) Free more than once
2) Forgot to free that chunk of memory

Garbage collector: reserving / release the data in the memory in runtime in background
Problem: freezing the program during the cleanup process for a few seconds, program resume working after cleaning 
Safety: Prevents bugs like null pointer referencing, buffer overflow, data race etc

Rust
Ownership: 
Every value has a single owner [every variable has one value, and it is its sole owner]

Borrowing and references: 
Allows you to access data without taking ownership. 
	•	Immutable Borrowing (&T): Allows read-only access.
	•	Mutable Borrowing (&mut T): Allows read and write access, but only one mutable reference is allowed at a time.
*/


fn print_value(val: &String) {
    println!("Value: {}", val);
}

fn increment(val: &mut i32) {
    *val += 1;
}

fn modify_string(val: &mut String) {
    val.push_str(" is great!");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


fn main() {
    //1. Ownership
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership of the String is moved to s2

    // println!("{}", s1);  // This would cause a compile-time error because s1 no longer owns the data.
    println!("{}", s2);  // Valid

    //1. Immutable References

    let s = String::from("Rust");
    print_value(&s);  // Borrowing s immutably
    print_value(&s);  // Borrow again

    //2. Mutable Borrowing (&mut)
    
    let mut x = 10;
    increment(&mut x);  // Mutable borrow
    println!("Updated value: {}", x);  // Output: 11

    //3. Dangling reference
    let r;
    {
        let k = 5;
        r = &k;  // Error: `x` does not live long enough
    }
    //println!("{}", r);  // Would cause a dangling reference

    //4. Simultaneous Mutable and Immutable References
    let mut s = String::from("Rust");
    let r1 = &s;
    // let r2 = &mut s;  // Error: Cannot borrow `s` as mutable because it’s already borrowed as immutable
    println!("{}", r1);

    //5. Limiting the Scope of a Mutable Borrow , exit borrowing with {}
    let mut s = String::from("Rust");

    // First mutable borrow
    {
        modify_string(&mut s);  // Mutable borrow within a smaller scope
    } // Mutable borrow ends here

    // Now it's safe to create another borrow
    let r1 = &s;  // Immutable borrow
    println!("Immutable borrow: {}", r1);

    // Another mutable borrow is also possible after the previous scope ends
    {
        let r2 = &mut s;  // New mutable borrow
        r2.push_str("!");
    }

    println!("Final string: {}", s);  // "Rust is great!!"

    //6. Explicit Lifetimes
    //To manage references across different scopes, Rust uses lifetimes to ensure references remain valid.
    let string1 = String::from("Hello");
    let string2 = String::from("World");
    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}