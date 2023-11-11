use std::{
    env, fs,
    process::{self, Command},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let destination = fs::read_to_string("bubbleConfig.txt").unwrap_or("C:/".to_owned());

    if args.len() < 3 {
        println!("Required arguments were not provided to bubble.");
        process::exit(1);
    }

    match args[1].as_str() {
        "pop" => {
            let name = &args[2];
            Command::new("cargo")
                .current_dir(destination)
                .args(["new", name])
                .spawn()
                .expect("There seems to be an error with cargo, kindly check if there are any problems with cargo.");
        }
        "destination" => {
            let dest = &args[2];
            fs::write("bubbleConfig.txt", &dest)
                .expect("There was an error with changing the destination.");
            println!("Destination changed to {}", dest);
        }
        _ => {
            println!("Invalid argument");
        }
    }
}
