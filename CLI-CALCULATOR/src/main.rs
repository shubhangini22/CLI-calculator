use std::{env, fs, process};


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
    read_Config(config);
}

struct Config {
    query : String,
    file_path : String,
}
//associating new functions to config

impl Config {
    fn func(str1 : &[String]) -> Result<Config,&'static str>{
    if str1.len()<3
    {
        return Err("Please enter correctly");
    }
    let query= str1[1].clone();
    //clone() function returns copy of the value
    let file_path= str1[2].clone();
    Ok(Config { query, file_path })
 }
}
fn read_Config(config : Config){
    let contents=fs::read_to_string(config.file_path).expect("Error");
    print!("Text in the file \n{contents}");
}

 