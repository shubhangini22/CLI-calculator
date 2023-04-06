use std::fs;
use std::error::Error;


pub struct Config {
    pub query : String,
    pub file_path : String,
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
    Ok(Config { query, file_path })
 }
}
pub fn read_Config(config : Config)-> Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.file_path)?;
    //rather than panic/expect on error ? will return the r=error message
    Ok(())
    //the brackets inside ok indicates that we are calling run fuction only for its sideffects
}
//search function
pub fn search<'a>(query : &str, contents: &'a str) -> Vec<& 'a String>{
    vec![]
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

    }
}
 //error handling of read config file