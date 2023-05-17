pub mod utils;

use std::env;
use utils::init;

fn main()-> () {
    match env::args().nth(1) {
        Some(command)=>{
            match command.as_str() {
                "help"=>{
                    println!("Elsa is a dotfile manager");
                },
                "init"=>{
                    init("thewisepigeon/dotfiles")
                },
                _=>{
                    println!("Unrecognized command");
                }
            }
        },
        None=>{
            println!("Elsa");
        }
    }
}
