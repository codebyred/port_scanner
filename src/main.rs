use std::{
    env, process
};

use port_scanner::config::Config;
use port_scanner::run::run;

fn main() {
    
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        println!("{err}");
        process::exit(0);
    });
    
    run(config);
}
