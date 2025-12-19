fn main() {
    let _x: i32 = -32; // Signed int
    let _y: u32 = 90; // Unsigned int
    let _z: f64 = -10000.8984; // Float

    let _is_male: bool = true; // Boolean
    let _s: &str = "Rup"; // String
    let _is_age: i32 = 18;

    println!("Hello, world!");
    println!("x : {}, y : {}, z : {}", _x, _y, _z);
    println!("Name : {}", _s);

    // If-Else Condition
    // Turnary operators
    if _is_male && _is_age < 18 {
        println!("You are a male")
    } else if _is_male && _is_age >= 18 {
        println!("You are an adult male");
    } else {
        println!("You are a female");
    }

    let sentence = String::from("Your name is Rup");
    println!("First Word : {}", get_first_word(sentence));
    // println!("{}", sentence) // The ownership of this sentence string is removed as it passes through the function

    // Borrowers & References
    let word1 = String::from("I Love Radha Krishna");
    let word2  = get_first_word_new(&word1);
    println!("{}", word2);
    println!("{}", word1); // This can be done because word1 is passed through reference or the word2 borrow the value of word1 so the ownership of word1 is not changed ...
    // We can also do word1.clone() but that operation is expensive as it makes an additional pointer and points at a newly made word1 string in heap but in this case word2 makes an additional pointer but points as the same as word1 address in heap. 
}


// Functions
fn get_first_word(sentence: String) -> String {
    let mut ans = String::new(); // mut => mutable(the value can be changed)

    // For Loop
    for char in sentence.chars() {
        ans.push(char);
        if char == ' ' {
            break;
        }
    }
    return ans;
}
// Functions
fn get_first_word_new(sentence: &String) -> String {
    let mut ans = String::new(); // mut => mutable(the value can be changed)

    // For Loop
    for char in sentence.chars() {
        ans.push(char);
        if char == ' ' {
            break;
        }
    }
    return ans;
}
