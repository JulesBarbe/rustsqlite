use std::io::{self, Write};

/*
    TODO: statements should use algebraic data types. i.e.
    Insert {id: u32, name: String}, 
    ...
*/

#[derive(Debug)]
enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
struct Statement {
    statement_type: StatementType
}

fn print_prompt() {
    print!("> ");
}

fn prepare_statement(input: &str) -> Result<Statement, String> {
    // INSERT
    if input.to_lowercase().starts_with("insert") {
        Ok(Statement {
            statement_type: StatementType::Insert,
        })
    }
    // SELECT
    else if input.to_lowercase().starts_with("select") {
        Ok(Statement {
            statement_type: StatementType::Select,
        })
    }
    else {
        Err(format!("unrecognized keyword in '{}'", input))
    }
}

fn execute_statement(statement: Statement) {
    match statement.statement_type {
        StatementType::Insert => {
            println!("INSERT HERE");
        }
        StatementType::Select => {
            println!("SELECT HERE");
        }
    }
}


fn process_meta_command(input: &str) {
    match input {
        ".exit" => {
                println!("Ciao.");
                std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command: {}", input);
        }
    }
}

fn main() {
    loop {
        print_prompt();
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.starts_with('.') {
            process_meta_command(input);
        }

        match prepare_statement(input) {
            Ok(statement) => {
                execute_statement(statement);
                println!("executed statement.")
            }
            Err(err) => {
                println!("{}", err);
            }
        }


    }
}
