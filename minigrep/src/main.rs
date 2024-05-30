use std::env;

use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);

    // run(config);
    match minigrep::run(config) {
        Ok(_) => {},
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
   
}

// fn run(config:Config)->Result<(),Box<dyn Error>>{
//     let contents = fs::read_to_string(config.filename)?;
//     print!("{}",contents);

//     Ok(())
// }



