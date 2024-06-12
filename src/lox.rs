use std::fs;
use crate::err::LoxErr;
use std::io::Write;

struct Lox {

}


impl Lox {
    fn run_with_args(&mut self) -> Result<(), LoxErr>{
        let args: Vec<String> = std::env::args().collect();

        if args.len() > 2 {
            return Err(LoxErr::ScriptUsage);
        } else if args.len() == 2 {
            self.run_file(&args[1])?;
        } else {
            self.run_prompt()?;
        }
        Ok(())
    }

    fn run_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), LoxErr>{
        let code = fs::read_to_string(path)?;
        self.run(&code)?;
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<(), LoxErr> {
        let mut input_line = String::new();

        print!("\n> ");
        std::io::stdout().flush()?;
    
        loop {
            match std::io::stdin().read_line(&mut input_line) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }

                    self.run(&input_line.trim())?;
                    input_line.clear();


                }
                Err(error) => {
                    return Err(LoxErr::Io(error));
                }
            }
            print!("\n> ");
            std::io::stdout().flush()?;
    
        }
        Ok(())
    
    }

    fn run(&mut self, code: &str) -> Result<(), LoxErr> {
        todo!();

        Ok(())

    }

    
}