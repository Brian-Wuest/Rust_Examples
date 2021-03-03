use std::io;

fn main() {
    println!("Enter your weight (kg): ");

    // Creates an empty string.
    // The string is a pointer since it is not known how big the string can be.
    // String is a type of Smart-Pointer
    let mut input = String::new();

    // The & here says that the variable is being passed "by reference" instead of "by value".
    // This way the function doesn't take ownership of the variable and deallocates it when the function
        // falls out of scope.
    // Need to pass "&mut" when the function declares the referance as mutable (writable).
    // some_fn(&mut input);

    // The "unwrap", function will kill the app if the function fails.
    // The "unwrap", function will yield the result if the function suceeds.
    io::stdin().read_line(&mut input).unwrap();

    // Remove whitespace from string.
    let weight: f32 = input.trim().parse().unwrap();

    dbg!(weight);

    //borrow_string(&input);
    //own_string(input);

    //println!("Input: {}", input);

    // Type is inferred from the result of the function.
    // All variables are immutable by default.
    // Need to mark variable as mutable by using "mut" before variable name.
    let mars_weight = calculate_weight_on_mars(weight);
    
    // println! is a macro; macros are special where they generate more rust code.
    println!("Weight on Mars: {} kg", mars_weight);

    // Ownership rules
    // 1. Each value in Rust is owned by a variable.
    // 2. When the owner goes out of scope, the value will be deallocated.
    // 3. There can only be ONE owner at a time.
}

/*
    Calculates the inputed weight as what the weight would be on mars.
    It expects that the value supplied is in Kilograms not Pounds.
 */
fn calculate_weight_on_mars(weight: f32) -> f32 {
    // There is an implicit return here since there is no semi-colon.
   (weight / 9.81) * 3.711 
}

/*
    By putting an & before the type it is expecting a referece to a variable instead of the actual variable value.
    This is important since the function would otherwise take "ownership" of the variable value 
        and de-allocate it when the function falls out of scope.
    The function "borrows" the string. 
    References are immutable (read-only) by default
    Use "&mut" to make the referance mutable (writable).
    Note: Calling functions need to be updated to pass &mut as well
*/
/* fn some_fn(s: &mut String) {
    s.push_str("a");
}

fn borrow_string(s: &String) {
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
} */