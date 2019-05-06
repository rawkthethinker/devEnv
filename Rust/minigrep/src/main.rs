use std::env;
use std::process;


use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

   let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let (query, filename) = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


   if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the fiie");

    // println!("With text:\n{}", contents);

}

// all moved to lib file 

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);

//     Ok(())
// }



// fn parse_config(args: &[String]) -> (&str, &str){
//     let query = &args[1];
//     let flename = &args[2];

//     Config {query, filename}
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

// struct Config {
//     query: String,
//     filename: String,
// }