use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, Read, Write};

pub fn input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in reading.");
    input.trim().to_string()
}
    
pub fn input_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in reading.");
    //input.trim().parse().expect("parse error")
    let value = match input.trim().parse::<i32>() { 
        Ok(n) => n,
        Err(error) => {
            match error.kind() {
                // Error::ParseInt(e) => e,

                std::num::IntErrorKind::Empty => {panic!("Input not a number: {} - Empty Error: {}", input, error)}
                std::num::IntErrorKind::InvalidDigit => {panic!("Input not a number: {} - Invalid Error: {}", input, error)}
                std::num::IntErrorKind::PosOverflow => {panic!("Input not a number: {} - Large Error: {}", input, error)}
                std::num::IntErrorKind::NegOverflow => {panic!("Input not a number: {} - Small Error: {}", input, error)}
                std::num::IntErrorKind::Zero => {panic!("Input not a number: {} - Zero Error: {}", input, error)} 
                _ => {panic!("Input not a number: {} - Error: {}", input, error)}
            }
        }
    };
    value
}
pub fn create_file_write_all(file_path: &str, content: &[u8]){

    let mut file = match fs::File::create(file_path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", file_path, why),
    
    };
    //file.write_all(content).unwrap();
    match file.write_all(content) {
        Err(why) => panic!("couldn't write to {}: {}", file_path, why),
        Ok(_) => println!("successfully wrote to {}", file_path),
    }
}
pub fn create_file_write(file_path: &str, msg: &[u8]){ 
    match fs::write(file_path, msg) {
        Err(why) => panic!("couldn't write to {}: {}", file_path, why),
        Ok(_) => println!("successfully wrote to {}", file_path),
    }
    //fs::write(file_path, msg).unwrap(); 
}
// READ ENTIRE FILE INTO A STRING
pub fn read_file_string(read_path: &str) -> Result<String,  std::io::Error> {
    //let mut file = File::open(read_path).unwrap();
    let mut file = match File::open(read_path)  {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    Ok(file_contents)
}
// READ ENTIRE FILE INTO A BYTES VECTOR
pub fn read_file_vec(read_path: &str) -> Result<Vec<u8>,  std::io::Error>{
    //let mut file = File::open(read_path)?;

    let mut file = match File::open(read_path)  {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();
    Ok(data)
}
// READ USING BufReader INTO STRING
pub fn read_with_bufreader_str(read_path: &str) -> Result<String,  std::io::Error> {
    //let file = File::open(read_path).unwrap();

    let file = match File::open(read_path)  {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut file_contents = String::new();
    buf_reader.read_to_string(&mut file_contents).unwrap();
    Ok(file_contents)
}
// READ USING BufReader INTO VECTOR
pub fn read_with_bufreader_vec(read_path: &str) -> Result<Vec<u8>,  std::io::Error> {
    //let file = File::open(read_path).unwrap();

    let file = match File::open(read_path)  {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut data = vec![];
    buf_reader.read_to_end(&mut data).unwrap();
    Ok(data)
}

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Access a file at a specified path
    // ---------------------------------
    // TODO #1:
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) =>  return Err(io_error),
    };

    // Read file contents into `String` variable with `read_to_string`
    // ---------------------------------
    // Success path is already filled in
    // TODO #2: Return from function early if there's an error
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // TODO #3: Return `string` variable as expected by function signature
    Ok(string)

}
fn read_file_error(){
    let file: Result<File, io::Error> = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        println!("Permission denied: {}", error) 
                    }
                    _ => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        }
    }
}



// Rust code for creating a library
pub fn public_function() {
    println!("Inside public function");
}

fn pvt_func() {
    println!("Calling pvt function");
}

pub fn indirect_fn_access() {
    print!("Accessing indirect functions ");

    pvt_func();
}

//pre-built examples
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

