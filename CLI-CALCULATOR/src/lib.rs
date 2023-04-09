use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool,
    //seperated by commas
}
//associating new functions to config

impl Config {
    pub fn func(str1 : &[String]) -> Result<Config,&'static str>{
    if str1.len()<3
    {
        return Err("Please enter correctly");
    }
    let query= str1[1].clone();
    //clone() function returns copy of the value
    let file_path= str1[2].clone();

    let ignore_case=env::var("Ignore_Case").is_ok();
    Ok(Config { 
        query, 
        file_path , 
        ignore_case,
    })
 }
}
pub fn read_Config(config : Config)-> Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.file_path)?;
    //rather than panic/expect on error ? will return the r=error message
    let results = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }


    
    Ok(())
    //the brackets inside ok indicates that we are calling run fuction only for its sideffects
}
//search function
pub fn search<'a>(query : & str, contents: &'a str) -> Vec<& 'a str>{
    let mut results=Vec::new();
    /*Iterate through each line of the contents.
      Check whether the line contains our query string.
If it does, add it to the list of values we’re returning.
If it doesn’t, do nothing.
Return the list of results that match. */
    for line in contents.lines(){
        if line.contains(query){
           results.push(line);
        }
        
        //returning result
       //we are using lines() method here
    }
    results
    // 'a is a lifetime parameter : lifetimes ensure that references are valid as long as we need them to be.
}
pub fn search_insensitive<'a>(query : & str, contents: &'a str) -> Vec<& 'a str>{
    let mut results=Vec::new();
    /*Iterate through each line of the contents.
      Check whether the line contains our query string.
If it does, add it to the list of values we’re returning.
If it doesn’t, do nothing.
Return the list of results that match. */
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
           results.push(line);
        }
        //to_lowercase()
        //returning result
       //we are using lines() method here
    }
    results
    // 'a is a lifetime parameter : lifetimes ensure that references are valid as long as we need them to be.
}
//adding a new module test 
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn result(){
        let query="subzero";
        let contents="\
        Rust:
        safe,fast,productive.
        Pick three";
        assert_eq!(vec!["safe,fast,productive"],search(query,contents));
        //the following function is case sensitive
    }
    fn case_insensitive(){
        let query="sUbzero";
        let contents="\
        Rust:
        safe,fast,productive.
        Pick three
        Duct tape";
        assert_eq!(vec!["safe,fast,productive"],
        search_insensitive(query,contents));
        //the following function is case insensitive
    }
}
 //error handling of read config file

 //we are successfully able to print the lines containing the required query