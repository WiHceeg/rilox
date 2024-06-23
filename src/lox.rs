use std::fs;
use std::io::Write;

use crate::err::LoxErr;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Lox;

impl Lox {
    pub fn new() -> Lox {
        Lox
    }
    pub fn start(&self) {
        if let Err(lox_err) = self.run_with_args() {
            self.report_error(lox_err);
        }
    }

    fn run_with_args(&self) -> Result<(), LoxErr>{
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

    fn run_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), LoxErr>{
        let code = fs::read_to_string(path)?;
        self.run(&code)?;
        Ok(())
    }

    fn run_prompt(&self) -> Result<(), LoxErr> {
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

    fn run(&self, code: &str) -> Result<(), LoxErr> {
        
        // 扫描遇到错误的话，在这里打印出来，并继续处理 token
        let mut scanner = Scanner::new(code);
        if let Err(scan_err) = scanner.scan_tokens() {
            self.report_error(scan_err);
        }

        // 解析遇到错误的话，内部会处理
        let mut parser = Parser::new(&scanner.tokens);
        let statements = parser.parse();
        
        // 解释执行遇到错误的话，内部会处理
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements);

        Ok(())

    }

    fn report_error(&self, lox_err: LoxErr) {
        println!("{}", lox_err)
    }

    
}