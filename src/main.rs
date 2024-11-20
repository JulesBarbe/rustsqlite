use std::io::{self, Write};

fn print_prompt() {
    print!("> ");
}

fn main() {
    loop {
        print_prompt();
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            ".exit" => {
                println!("Ciao.");
                break;
            }
            _ => {
                println!("Unrecognized command: {}", input);
            }
        }



    }
}
