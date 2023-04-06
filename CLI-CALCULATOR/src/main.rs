use std::{env,process};
use std::error::Error;

use CLI_CALCULATOR::Config;
//helps in fetching codes from other files
fn main(){
    //this will print
    let args: Vec<String> = env::args().collect();
    let config = Config::func(&args).unwrap_or_else(
    |err|
    {
        //closure is called , err
        println!("{err}");
        process::exit(1)
        //this will stop the program immediately
        //unwrap or else helps to return the required panic statement from the func
    }
    );
    // we are directly creating struct with function inside new 
    //calling the function from the impl
    println!("Query : {}",config.query);
    println!("File_path : {}",config.file_path);
    if let Err(e)=CLI_CALCULATOR::read_Config(config)
    {
        print!("Application Error {e}");
        process::exit(1);
        //e here declares the box<dyn error>
    }
   
}

//apart from main function we moved all the codes to lib.rs to handle all effectively