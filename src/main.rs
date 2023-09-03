use rustyline::{Config, Editor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().auto_add_history(true).build();
    let mut rl: Editor<(), rustyline::history::FileHistory> = Editor::with_config(config)?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == ".exit" {
                    break;
                }
                println!("Line: {:?}", line);
            }
            Err(e) => match e {
                rustyline::error::ReadlineError::Eof
                | rustyline::error::ReadlineError::Interrupted => break,
                _ => println!("Error: {:?}", e),
            },
        }
    }
    Ok(())
}
