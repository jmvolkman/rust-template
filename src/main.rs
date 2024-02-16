

fn main() {

    println!("Hello, JMV!");

    println!("Please enter number of elements to sum:");
    let count:i32 = my_library::input_number();
    println!("Item Count: {}", count);

    println!("Local Mac version?!");

    arg_fn();
    //file_main();
    //error_main();
    //name_main();
    //shape_main();
    filesize_main();
    //wine_main();
    //vector_main();
    //string_main();
    //user_main();
    //person_main();
    //file_create_read();
    //read_file_error();
    //borrow_main();
    arg_fn();
    //number_test();
    //condition();
    // array_test();
    // junk_test();
    // health_info();
    // loop_test();
    // match_try();
    // main_car();

    println!("Goodbye, JMV!");

//end of main
}
//sub functions to call

#[allow(dead_code)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FromFileSize {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String,
}
impl FromFileSize {
        // fn format_byte_size(byte_type:FileSize, size: u64) -> String {
        //     match byte_type {
        //         FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        //         FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1024.0),
        //         FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1024.0),
        //         FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1024.0),
        //         FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64 / 1024.0),
        //         _ => format!("{} bytes", bytes),
        //     }
        //  }

    fn new(size: String, byte_type:String) -> Self {
            //let my_byte_size: u64 = byte_size;
   // let str="-10.20".to_string();
    //let integer=str.split('.').next();

    let my_size: u64= match size.split(' ').next() {
        Some(num)=>match num.parse::<u64>() {
            Ok(num)=>num,
            Err(_)=>panic!("Err: Cannot Parse The Integer."),
        },
        None=>panic!("Err: Cannot Parse The String."),
    };

        let new_byte_type = byte_type.to_lowercase().chars().next().unwrap();
        let power = match new_byte_type {
            'b' => 0,
            'k' => 1,
            'm' => 2,
            'g' => 3,
            't' => 4,
            _ => 0,
        };
        let base_byte_size: u64 = 1024;
        let byte_size = my_size * base_byte_size.pow(power);
        Self {
            bytes: format_to_size(&byte_size, "bytes"),
            kilobytes: format_to_size(&byte_size, "kilobytes"),
            megabytes: format_to_size(&byte_size, "megabytes"),
            gigabytes: format_to_size(&byte_size, "gigabytes"),
            terabytes: format_to_size(&byte_size, "terabytes"),
        }
    }
}

fn format_to_size(byte_size: &u64, byte_type : &str) -> String {
    let new_byte_type = byte_type.to_lowercase().chars().next().unwrap();
       
    let power = match new_byte_type {
        'b' => 0.0,
        'k' => 1.0,
        'm' => 2.0,
        'g' => 3.0,
        't' => 4.0,
        _ => 0.0,
    };
    let my_byte_size: f64 = *byte_size as f64;
    let base_byte_size: f64 = 1024.0;
    format!("{:.4} {}", (my_byte_size / base_byte_size.powf(power)), byte_type)

}

#[allow(dead_code)]
fn format_byte(byte_type:FileSize) -> String {
    match byte_type {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1024.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1024.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1024.0),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64 / 1024.0),
    }
}
#[allow(dead_code)]
fn filesize_main() {

        println!("Size is: ");
    let input : String = input_string();
    let args = input.split(' ').collect::<Vec<&str>>();

    println!("Vector: {:?}",args);
   
    let input_file = FromFileSize::new(args[0].to_string(), args[1].to_string());
           
    println!("Sizes {:?}!", input_file);

    if args.len() > 2 {panic!("Too manay parameters! max")}

    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("Size is: {} {}.", args[0], args[1]);

    // let result = format_size(6880088373000099);
    // println!("{}", result);
    // println!("{}", format_byte(FileSize::Kilobytes(2500)));
}

#[derive(Debug)]
struct DivisionByZeroError;

#[allow(dead_code)]
fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

#[allow(dead_code)]
fn error_main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
struct MyPerson {
    first: String,
    middle: Option<String>,
    last: String,
}
#[allow(dead_code)]
fn build_full_name(person: &MyPerson) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

   // if let 
   let mut middle_name = "".to_string();
    match &person.middle {
        Some(middle) => middle_name = format!("{} ", middle),
        None => {},
    }

    full_name.push_str(&middle_name);

    // TODO: Implement the part of this function that handles the person's middle name.

    full_name.push_str(&person.last);
    full_name
}
#[allow(dead_code)]
fn name_main() {
    let john = MyPerson {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = MyPerson {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = MyPerson {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64,f64),
}
#[allow(dead_code)]
fn shape_main(){
    let shapes = vec![Shape::Circle(5.0), Shape::Triangle(1.0, 1.0), Shape::Square(3.0)];

    let total_area: f64 = shapes
        .iter()
        //.map(|shape| shape_area(shape))
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base,height ) => base * height * 0.5, 
        })
        .sum();

    println!("Total area: {} sq. units", total_area);

   // let new_shapes = vec![Shape::Circle(5.0), Shape::Triangle(1.0, 1.0), Shape::Square(3.0)];

    println!("Biggest Shape {:#?}", biggest_shape(&shapes));

    println!("Total area: {} sq. units", total_area);
}
fn biggest_shape(vector: &Vec<Shape>) -> &Shape{

    //let total_area: f64 = vector.iter().map(|my_shape| shape_area(my_shape)).sum(); 
 
     let mut big_shape : &Shape =&Shape::Circle(-1.0);
     let mut area :f64 = 0.0;
     for my_shape in vector.iter() {
         let new_area = shape_area(my_shape);
         if new_area > area {
             area = new_area;
             big_shape = my_shape  ;
            }
           
        }
       
        big_shape 
}
fn shape_area(item: &Shape) -> f64 {
    match item {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length, 
        Shape::Triangle(base,height ) => base * height * 0.5, 
    }
    }
#[allow(unused)]
#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Portugal,
}
struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}
fn region_popular(w: WineRegions) {
    match w {
        WineRegions::Bordeaux|WineRegions::Champagne|WineRegions::Tuscany => println!("{:?} is highly popular!", w),
        WineRegions::Burgundy|WineRegions::NapaValley => println!("{:?} is highly popular!", w),
        _ => println!("{:?} is not popular!", w),
    }
}

#[allow(dead_code)]
fn wine_main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::NapaValley,
    };

    let wine3 = Wine {
        name: String::from("Some Wine"),
        region: WineRegions::Portugal,
    };

     println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
     println!("Wine 2: {} from {:?}", wine2.name, wine2.region);

     println!("Wine 3: {} from {:?}", wine3.name, wine3.region);

    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
    region_popular(WineRegions::Bordeaux);
    region_popular(WineRegions::NapaValley);
    region_popular(WineRegions::Portugal);

}

#[allow(dead_code)]
fn vector_main(){

    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers: Vec<i32> = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers: Vec<i32> = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    v = vec_insert_first_last(v, 9999);
    println!("{:?}", v);
    #[allow(unused)]
    let mut other_numbers = vec![7, 8];
    v = vec_append(v, other_numbers);
    println!("{:?}", v);



    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);
    
    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    //let vec: Vec<i32> = vec![];
    // Retrieve the first value using pattern matching
     match vec.first() {
         Some(first_value) => println!("The first value in the vector is: {}", first_value),
         None => println!("The vector is empty!"),
     }
    println!("The sum of the vector is: {}", sum_vec(&vec)); 

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(&"coconut") => println!("Coconuts are awesome!!!"),
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}

let a_number: Option<u8> = Some(7);
match a_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
}

let a_number: Option<u8> = Some(7);
if let Some(7) = a_number {
    println!("That's my lucky number!");
}

let a = Some("value");
assert_eq!(a.expect("fruits are healthy"), "value");

let b: Option<&str> = None;
b.expect("fruits are healthy"); // panics with `fruits are healthy`
assert_eq!(Some("dog").unwrap_or("cat"), "dog");
assert_eq!(None.unwrap_or("cat"), "cat");


}

}
fn vec_insert_first_last(mut vector: Vec<i32>, value : i32) -> Vec<i32> {
    vector.insert(0,value);
    vector.insert(vector.len(),value);
    vector
}
fn vec_append(mut vector: Vec<i32>, mut vector_to_append: Vec<i32>) -> Vec<i32> {
    vector.append(&mut vector_to_append);
    vector
}
fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}
fn sum_vec(vector: &Vec<i32>) -> i32{
    let sum:i32= vector.iter().sum();
    sum
}

#[allow(dead_code)]
pub struct SearchLetter {
    letter: char,
    count: i32,
}
#[allow(dead_code)]
impl SearchLetter {
       fn new(letter: char) -> SearchLetter {
        SearchLetter {
            letter,
            count: 0,
        }
    }
    fn add(&mut self) {
        self.count += 1;
    }

}
#[allow(dead_code)]
fn string_main(){

    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    #[allow(unused)]
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence

    #[allow(unused)]
    let mut find_letter = vec![SearchLetter{letter: 'a', count: 0 },
    SearchLetter{letter: 'e', count: 0 },
    SearchLetter{letter: 'i', count: 0 },
    SearchLetter{letter: 'o', count: 0 },
    SearchLetter{letter: 'u', count: 0 }];

    let char :Vec<char> = sentence.chars().collect();
    char_count(&char, 'a');
    char_count(&char, 'e');
    char_count(&char, 'i');
    char_count(&char, 'o');
    char_count(&char, 'u');

        for c in sentence.chars() {
          match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }
    

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    let mut longest_word : String = "".to_string();
    for word in &words {
        if word.len() > longest_word.len() {longest_word = word.to_string(); }
    }
    println!("Longest word: {}", longest_word);

    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
fn char_count(char_vec :&Vec<char>, search :char){
    println!("char {} Count: {}", search,  char_vec.iter().filter(|&n| *n == search).count());
}
//impl construct to define struct methods
pub struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}
impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(&self) -> String{
        //format!("{}",self.email)
        // Use the find() method to search for the character 'o'
        match &self.email.find('@') {
        Some(index) => self.email[0..*index].to_string(),
        None => format!("{}",self.email),
        }
    }
    fn update_uri(&mut self, new_uri: String) {
        self.uri = new_uri;
    } 
}
#[allow(dead_code)]
fn user_main(){
    
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
    println!("User {}!", new_user.from_email());
    new_user.update_uri(String::from("Check_this_out!"));
    println!("Current URI: {}!", new_user.uri);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone: String
}
impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn new(first_name: String, last_name: String, age: u8, email: String, phone: String) -> Person {
        Person {
            first_name,
            last_name,
            age,
            email,
            phone
        }
    }
}

#[allow(dead_code)]
fn person_main(){
    
    let new_person = Person::new("John".to_string(), "Doe".to_string(), 
    25, "john.doe@example.com".to_string(), "+1-555-555-1234".to_string());

    println!("{:?} Full Name: {}", new_person, new_person.full_name());

    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "john.doe@example.com".to_string(),
        phone: "+1-555-555-1234".to_string(),
    });
}



#[allow(dead_code)]
fn file_create_read(){

    let msg_write_all = b"Hello from write_all()!";
    let msg_write: &[u8; 19] = b"Hello from write()!";
    let file_to_read: &str = "./example.txt";
    // Create file using the write_all() method
    create_file_write_all(file_to_read, msg_write_all);
    // Create file using the write()
    create_file_write(file_to_read, msg_write);
    // Reading a file to a string
    let file_string_contents =  read_file_string(file_to_read).unwrap();
    println!("read_file_string(): {:?}", file_string_contents);
    // Reading a file to a vector
    let file_vec_contents = read_file_vec(file_to_read).unwrap();
    println!("read_file_string_vec(): {:?}", file_vec_contents);
    // Reading a file to a string using a Buffered Reader
    // let bufreader_str = mods::file_mod::reading_files::read_with_bufreader_str(file_to_read).unwrap();
    let bufreader_str = read_with_bufreader_str(file_to_read).unwrap();
    println!("read_with_bufreader_str(): {:?}", bufreader_str);
    // Reading a file to a vector using a Buffered Reader
    let bufreader_vec = read_with_bufreader_vec(file_to_read).unwrap();
    println!("read_with_bufreader_vec(): {:?}", bufreader_vec);


}

#[allow(dead_code)]
fn borrow_main(){

    // Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
    // or another part of your program without actually transferring ownership of the variable. 
    // When you borrow a variable, you're essentially saying 
    // "I want to use this variable for a little while, but I promise I won't modify it."

    let my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");
    let mut my_string_mut = String::from("Hello, world!");

    borrow_string_mut(&mut my_string_mut);
    println!("Mutable string after borrow: {}", my_string_mut);

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    borrow_string_nonmut(&my_string);
    println!("Non-Mutable string after borrow: {}", my_string);

    own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    //println!("{:?}", my_string); // this will not compile!

    borrow_vec(&my_vec);

    own_vec(my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    //println!("{:?}", my_vec); // this will not compile!

    
}
// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.
#[allow(dead_code)]
fn borrow_vec(vector: &Vec<i32>) {
    //vector.push(10);
    println!("{:?}", vector);
}
#[allow(dead_code)]
fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}
#[allow(dead_code)]
fn own_integer(x: i32) {
    let _ = x + 1;
}
#[allow(dead_code)]
fn borrow_string_nonmut(s: &String) {
    let s = String::from(format!("{} {}", s," appended"));
    println!("{}", s);
}
#[allow(dead_code)]
fn borrow_string_mut(s: &mut String) {
    s.push_str(&" appended");
    println!("{}", s);
}
#[allow(dead_code)]
fn own_string(s: String) {
    println!("{}", s);
}


#[allow(dead_code)]
fn arg_fn(){
    println!("Please enter number of elements to sum:");
    let count:i32 = my_library::input_number();
    println!("Item Count: {}", count);
    
    let mut total = 0;
    let mut items: Vec<i32> = Vec::new();
    if count > 0 {
        for int_loop in 1..=count {
            println!("Entry {}: ", int_loop);
            let entry:i32 = input_number();
            items.push(entry);
            //io::stdin().read_line(&mut input).expect("Failed to read input");
            println!("Entered: {}", entry);
            total += entry;
        }
    }
     
    println!("The total is {}", total);
    println!("The vector total is {}", sum(&items));
    println!("The vector average is {}", average(&items));

    // There are no variadic arguments in Rust
    let numbers = [1,2,3,4];
    let result = sum(&numbers);
    println!("The sum is {}", result);

}
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}
#[allow(dead_code)]
fn average(numbers: &[i32]) -> f64 {
    let sum:i32= numbers.iter().sum();
    f64::from(sum) / (numbers.len() as f64)
}

#[allow(dead_code)]
fn number_test(){
    let numbers = [1, 2, 3, -5];   // Include a negative number to trigger the panic
    print_sum(&numbers);          // Call the unit function with the slice as an argument
    process_numbers(&numbers);     // Call function with slice of integers as an argument
}
#[allow(dead_code)]
// a unit function that doesn't return anything
fn print_sum(numbers: &[i32]) {
    let sum:i32= numbers.iter().sum(); // Calculate the sum of elements in slice
    if sum % 2 == 0 {               // Check if sum is even
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}
#[allow(dead_code)]
fn process_numbers(slice: &[i32]) {
    for (index, number) in slice.iter().enumerate() {
        if *number < 0 {
            panic!("Negative number found at index {}", index); // Stop execution and show error message
        }
    }
}
#[allow(dead_code)]
fn array_test(){

    // Declare array, initialize all values, compiler infers length = 7
    let _days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    // Declare array, initialize all values to 0, length = 5
    let _bytes = [0; 5];

    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();
    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
// Add 5 to the value at index 1, which is 5 + 3 = 8
index_vec[1] = index_vec[1] + 5;
println!("Vector: {:?}", index_vec);

}
#[allow(dead_code)]
fn condition(){
    let formal = true;
    let greeting = if formal { // if used here as an expression
    "Good day to you."     // return a String
    } else {
    "Hey!"                 // return a String
    };
    println!("{}", greeting);   // prints "Good day to you."

    let num = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }

    if out_of_range { // if used here as an expression
        println!("out of range");    // return a String
        } else {
            println!("good input");                 // return a String
        };
    
}
#[allow(dead_code)]
fn match_try(){

    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let str_hello: &'static str ="hello";
    let str_good_bye: &'static str ="good bye";
    let str_how_are_you: &'static str ="how are you?";

    // use of match expression to pattern match against variable "name"
    match name.to_lowercase().trim() {
        //str_how_are_you => println!("I'm doing well!"),
        "How are you?"|"how are you?" => println!("I'm doing well!"), 
        "Good bye"|"Good Bye"|"good bye" => println!("Sorry to see you go."),
        "hello"|"Hello" => println!("Hi, nice to meet you!"),
        "good morning" => println!("Good day!"),
        "good evening" => println!("Good night!"),
        //str_good_bye => println!("Sorry to see you go."),
        //str_hello => println!("Hi, nice to meet you!"),        
        _ => println!("I can't find a greeting, good bye."),
    }

    println!("Input: {}:{}:{}:{}", name.trim(), str_good_bye, str_hello, str_how_are_you);
    

    println!("Please enter a greeting:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        "Good Bye" => println!("Good Bye"),
        "Hello" => println!("Hello"),
        "World" => println!("World"),
        _ => println!("Other: {}", input.to_lowercase().trim()),
    }
}
#[allow(dead_code)]
fn loop_test(){
    // use 1..10 or 1..=10
    for int_loop in 1..=10 {
        println!("int_loop = {}", int_loop);
    }

    for int_loop in (1..=10).rev() {
        println!("int_loop = {}", int_loop);
    }

    let  numbers = vec!{1,2,3,4,5};
    for number in numbers {
        println!("number = {}", number);
    }

    for int_loop in 1..=10 {
        if int_loop % 2 == 0 {
            //skip even numbers
            continue;
        }
        println!("int_loop = {}", int_loop);
        if int_loop == 7 {
            //exit loop when 7
            break;
        }
    }
}
#[allow(dead_code)]
fn junk_test(){
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    //let maybe_number = Some(42);
    let maybe_number: Option<Option<()>> = Some(None);
if let Some(number) = maybe_number {
    println!("The number is: {:?}", number);
} else  {
    println!("There is no number!");
    }


    let spaces = "   ";
    
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _z) = tup;
    
    println!("The value of x is: {}", x);
    println!("The value of **y** is: **{}**", y);

    let bln_proceed = true;
    if bln_proceed {
        println!("Proceeding...");
    } else {
        println!("Not Proceeding...");
    }

    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
    
    let sententence = "the quick brown fox jumps over the lazy dog.".to_string();
    //let words = sententence.split(' ').collect.<vec<&str>>;
    let words: Vec<&str> = sententence.split_whitespace().collect();

    println!("{:?}",words);
}
#[allow(dead_code)]
fn health_info(){

    let gender: &str = "Male";
    let age = 41;

    #[allow(unused)]
    let mut age_message: String = "".to_string();

    if age > 19 {
        age_message = format!("{}{}{}",&age.to_string()," year old ",adult(gender));
    } else if age > 12 {
        age_message = format!("{}{}{}",&age.to_string()," year old ",teen(gender));
    } else {
        age_message = format!("{}{}{}",&age.to_string()," year old ",child(gender));
    }

    println!("{}",age_message);

    let mut height = 179;
    height = height - 20;

   let height_message: &str = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {    
        "Short"
    };
    println!("Height Result: {}", height_message);

    let health = if height < 180 {"good"} else {"unknown"};
    println!("Health Result: {}", health);

    //let health = if height < 180 {true} else {false};

}


fn adult(gender: &str)-> &'static str {
    if gender.to_lowercase().starts_with("m") {
        "Man"
    } else if gender.to_lowercase().starts_with("f") {
       "Woman"
    } else {
        "Human"
    }
}
fn teen(gender: &str) -> &'static str{
    if gender.to_lowercase().starts_with("m") {
        "Teenage Boy"
    } else if gender.to_lowercase().starts_with("f")  {
        "Teenage Girl"
    } else {
        "Teenager"
    }
}
fn child(gender: &str) -> &'static str{

    if gender.to_lowercase().starts_with("m") {
        "Boy"
    } else if gender.to_lowercase().starts_with("f")  {
        "Girl"
    } else {
        "Child"
    }
}


#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {    color: String,    motor: Transmission,    roof: bool,    age: (Age, u32)}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }
#[derive(PartialEq, Debug)]
enum Age { New, Used }
#[allow(dead_code)]
fn main_car() {

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    use std::collections::HashMap;
    let mut orders:  HashMap<i32, Car> = HashMap::new();
    
    // Initialize counter variable
    let order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;
    // Start with zero miles
    let mut miles = 0;

    // Call car_factory to fulfill order
    // Add order <K, V> pair to "orders" hash map
    // Call println! to show order details from the hash map     

    let order_total = 11;
    for order in order..=order_total {

        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }

}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {        
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = (color % 4) + 1;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

        // Show details about car order
    // Corrected code: If order is for Used car, check roof type, print details
    // Corrected code: Else, order is for New car, check roof tye, print details
    // Call the `println!` macro to show the car order details
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

//EOF