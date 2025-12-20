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

    // Structs
    struct User {
        username : String,
        email : String,
        age : u32,
        active : bool,
    }

    let user1 = User {
        username : String::from("Biswarup Biswas"),
        email : String::from("bbiswasrup2005@gmail.com"),
        age : 20,
        active : false,
    };

    println!(
        "{} is {} years old, email is : {}, {}Active",
        user1.username,
        user1.age,
        user1.email,
        if user1.active {""} else {"Not "},    
    );
    
    let rec1 = Rectangle{
        width : 40,
        height : 30,
    };

    println!("The area of rectangle is {} square pixels", rec1.area());

    // Enums & Pattern Matching
    let circle = Shape :: Circle(4.5);
    let square = Shape :: Square(2.5);
    let rectangle_new = Shape :: RectangleNew(1.5, 2.0);

    let area1 = calc_area(circle);
    println!("The area of circle is {}", area1);
    
    let area2 = calc_area(square);
    println!("The area of square is {}", area2);
    
    let area3 = calc_area(rectangle_new);
    println!("The area of rectangle is {}", area3);

}

    // Struct & Implementation
    struct Rectangle {
        width : u32,
        height : u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // Enums & pattern matching
    enum Shape {
        Circle(f64),
        Square(f64),
        RectangleNew(f64, f64),
    }

fn calc_area(shape : Shape) -> f64 {
    let ans = match shape{
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::RectangleNew(width, height ) => width * height,
    };
    return ans;
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
