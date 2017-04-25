extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;
use greprs::Config;


fn main() {
    let mut stderr = std::io::stderr();
    

    let mut args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(&mut stderr,"problem parsing arguments: {}",err).expect("could not write to stderr");
        process::exit(1);
    });

    println!("search = {:?}", config.search);
    println!("file = {:?}", config.file);

    if let Err(e) = greprs::run(config){
        writeln!(&mut stderr, " Application error = {:?}",e).expect("could not write to stderr");
        process::exit(1);
    }


}




