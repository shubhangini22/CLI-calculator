use std::{env, fs};
//for accepting arguements from the user and *fs* for handling files
fn main() {
    let args: Vec<String> = env::args().collect();
    //collect function here helps to collect the arguements in vector or any data structure
    //from the command line
    let key=&args[1];
    //calling the variable by reference
    let file_path=&args[2];
   println!("Searching for: {}",key);
   //we are passing 2 arguements and searching the first one in file path
   println!("File path is: {}",file_path);
   //access the arguements starting from index 1
   //we created a new file in root folder 
   let contents=parse(&file_path);
   //fs::read_to_string to read the contents in the file
   println!("We have:\n{contents}");
}

fn parse(str1 : &[String]) -> String{
    let contents=fs::read_to_string(str1).expect("Oops");
    contents
}
