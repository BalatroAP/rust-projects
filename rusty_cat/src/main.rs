use std::{env, fs, io, path::Path};


fn main() {
    let args: Vec<String> = env::args().collect();
    let flags: Vec<&String> = args.iter().filter(|arg| arg.contains("--")).collect();
    let mut files: Vec<&String> = args.iter().filter(|arg| !arg.contains("--")).collect();
    let curr_exec_path= match env::current_dir() {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };
    let mut file_buffer = String::new();

    if flags.len() == 0 && files.len() == 2 {
        let working_path = Path::new(files[1]);
        match working_path.has_root() && working_path.is_file() {
            true => {
                match fs::read_to_string(working_path) {
                    Ok(file_data) => println!("{file_data}"),
                    Err(_) => panic!("Failed to load file")
                }
            }
            false => {
                match fs::read_to_string(curr_exec_path.join(working_path)) {
                    Ok(file_data) => println!("{file_data}"),
                    Err(_) => panic!("Failed to load file")
                }
            }
        }
    }

    if flags.contains(&&String::from("--write")) {
        if files.len() == 2 {
            let working_path = Path::new(files[1]);
            println!("Enter in text for file:");
            match io::stdin().read_line(&mut file_buffer) {
                Ok(_) => (),
                Err(_) => panic!("Error taking in input")
            }

            match working_path.has_root() && working_path.is_file() {
                true => {
                    match fs::write(working_path, file_buffer) {
                        Ok(_) => (),
                        Err(_) => panic!("")
                    };
                },
                false => {
                    match fs::write(curr_exec_path.join(working_path), file_buffer) {
                        Ok(_) => (),
                        Err(_) => panic!("")
                    }
                }
            }

        }
    }

}
