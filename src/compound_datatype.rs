// Compound Data Types
// arrays, tuples, slices, and string (slice string)

fn main(){
    // Arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", mix_tuple);
    
    // Slices
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);
    
    let animal_slices: &[&str] = &["Lion", "Elephant", "Cat", "Dog"];
    println!("Animal Slice: {:?}", animal_slices);
    
    let book_slices: &[&str] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings Vs String Slices 
    // (&str, reference, good for memory efficiency, don't have to copy same variable, work with string without taking ownership, have specific size, no number of bytes to the stack, so stack much more faster than heap which is expandable , grow bigger but slower )
    // Stack can't have mutable data types while Heap can have dynamic mutable data types
    // Strings [growable, mutable, owned string type]
    // stone_cold is stored in both Heap and stack in the memory

    let mut stone_cold: String = String::from("Hell, "); 
    println!("Stone Cold Says: {:?}", stone_cold); 
    stone_cold.push_str("Yeah!");

    // B- &str (String Slice)
    let string: String = String::from("Hello, World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}