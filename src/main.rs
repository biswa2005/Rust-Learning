fn main() {
    let _x: i32 = -32; // signed int
    let _y: u32 = 90; // unsigned int
    let _z: f64 = -10000.8984; // float

    let _is_male: bool = true; // boolean
    let _s: &str = "Rup"; // String
    let _is_age: i32 = 18;

    print!("Hello, world!");
    print!("\nx : {}, y : {}, z : {}", _x, _y, _z);
    print!("\nName : {}", _s);

    // If-Else Condition 
    // Turnary operators

    if _is_male && _is_age < 18 {
        print!("\nYou are a male\n")
    }
    else if _is_male && _is_age >= 18 {
        print!("\nYou are an adult male\n");
    }
    else{
        print!("\nYou are a female\n");
    }
}