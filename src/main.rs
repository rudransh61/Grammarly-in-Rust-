// use std::io;
// use std::env;
// use std::fs;

// fn main() {
//     println!("Rusty English !! (*Gr@mmarly in Rust) ");
//     // let mut contents = fs::read_to_string("../words.txt");

//     println!("Enter a english word:");
//     let mut guess = String::new();

//     // io::stdin().read_line(&mut contents).expect("failed to readline");
 
//     io::stdin().read_line(&mut guess).expect("failed to readline");
 
//     print!("You entered {}", guess);

//     // println!("With text:\n{contents}");

   
//     // --snip--
//     let file_path = "file:///../words.txt";
//     println!("In file {}", file_path);

//     let contents = fs::read_to_string(file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");

// }


// use std::{
//     fs::File,
//     io::{self, BufRead, BufReader},
//     path::Path,
// };

// fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
//     BufReader::new(File::open(filename)?).lines().collect()
// }

// // ---

// fn main() {
//     let lines = lines_from_file("/words.txt").expect("Could not load lines");
//     for line in lines {
//         println!("{:?}", line);
//     }
// }

// use std::{
//     fs::File,
//     io::{prelude::*, BufReader},
//     path::Path,
// };

// fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("no such file");
//     let buf = BufReader::new(file);
//     buf.lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect()
// }

// // ---

// fn main() {

//     println!("Rusty English !! (*Gr@mmarly in Rust) ");

    
    // println!("Enter a english word:");
    // let mut guess = String::new();

    // io::stdin().read_line(&mut guess).expect("failed to readline");

    
    // print!("You entered {}", guess);

    // let lines = lines_from_file("file:///../words.txt");
//     for line in lines {
//         if line==guess {
//             println!("Spelling is Correct !!");
//             break;
//         };
//     }
    // let index = lines.iter().position(|r| r == "barararar").unwrap();
    // println!("{}", index);
    // if guess in lines {
    //     println!("yes");
    // }
    // else{
    //     println!("No");
    // }




//     let mut input_string = String::new();

//     // Prompt the user for input
//     println!("Please enter a English word:");

//     // Read the input from the console and handle errors
//     match io::stdin().read_line(&mut input_string) {
//         Ok(_) => {
//             // Trim the newline character from the input
//             let input_string = input_string.trim();

//             // Check if the input string exists in the vector
//             if lines.contains(&input_string) {
//                 println!("The string '{}' exists in the vector.", input_string);
//             } else {
//                 println!("The string '{}' does not exist in the vector.", input_string);
//             }
//         }
//         Err(error) => {
//             // Handle the error
//             eprintln!("Error reading input: {}", error);
//         }
//     }



// }

use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    // Prompt the user for the file path
    println!("Rusty English !! (*Gr@mmarly in Rust) ");

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");

    // Trim the newline character from the file path
    let file_path = file_path.trim();

    // Attempt to open the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    // Create a mutable String to store the input word
    let mut input_word = String::new();

    // Prompt the user for the input word
    println!("Please enter the word to search:");

    io::stdin().read_line(&mut input_word).expect("Failed to read line");

    // Trim the newline character from the input word
    let input_word = input_word.trim();

    // Use BufReader to read the file line by line
    let reader = io::BufReader::new(file);

    // Check if the input word exists in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(input_word) {
                println!("The spelling of word '{}' is correct !!", input_word);
                return;
            }
        }
    }

    println!("The spelling of word  '{}' is NOT correct xxx", input_word);
}
