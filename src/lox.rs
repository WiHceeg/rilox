/*
后缀`*`允许前一个符号或组重复零次或多次
后缀`+`与此类似，但要求前面的生成式至少出现一次
后缀`?`表示可选生成式，它之前的生成式可以出现零次或一次，但不能出现多次

expression     -> assignment ( "," assignment )*    // 支持了逗号表达式
assignment     → ( call "." )? IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;     // term 项，项之间通常通过加法或减法连接
factor         → unary ( ( "/" | "*" ) unary )* ;       // factor 因子，因子之间通常通过乘法或除法连接
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;     // . 也是一种 call
arguments      → assignment ( "," assignment )* ;   // 这里之前是 expression，但是现在 expression 里可能有逗号，就改成没有逗号的 assignment
primary        → "true" | "false" | "nil" | "this"
               | NUMBER | STRING | IDENTIFIER | "(" expression ")"
               | "super" "." IDENTIFIER ;
*/

/*
program        → declaration* EOF ;

declaration    → classDecl      // 类的方法没有前导的`fun`关键字
               | funDecl
               | varDecl
               | statement ;

classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )?
                 "{" function* "}" ;
funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

returnStmt     → "return" expression? ";" ;
forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;
whileStmt      → "while" "(" expression ")" statement ;
ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;
block          → "{" declaration* "}" ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
*/

use std::fs;
use std::io::Write;

use crate::err::LoxErr;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::Scanner;

pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
        }
    }
    pub fn start(&mut self) {
        if let Err(lox_err) = self.run_with_args() {
            self.report_error(lox_err);
        }
    }

    pub fn test_code(&mut self, code: &str) {
        if let Err(lox_err) = self.run(code) {
            self.report_error(lox_err);
        }
    }

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

        loop {
            print!("> ");
            std::io::stdout().flush()?;
    
            match std::io::stdin().read_line(&mut input_line) {
                Ok(n) => {
                    if n == 0 {
                        // Windows 系统 Ctrl + Z
                        break;
                    }

                    self.run(&input_line.trim())?;
                    input_line.clear();


                }
                Err(error) => {
                    return Err(LoxErr::Io(error));
                }
            }
        }
        Ok(())
    
    }

    fn run(&mut self, code: &str) -> Result<(), LoxErr> {
        
        // 扫描遇到错误的话，在这里打印出来，并继续处理 token
        let mut scanner = Scanner::new(code);
        if let Err(scan_err) = scanner.scan_tokens() {
            self.report_error(scan_err);
        }

        // 解析（语法分析）遇到错误的话，内部会处理
        let mut parser = Parser::new(&scanner.tokens);
        let mut statements = parser.parse();
        
        // 语义分析遇到错误的话，内部会处理，并停止
        let mut resolver = Resolver::new();
        resolver.resolve(&mut statements);
        if resolver.had_resolve_error {
            return Ok(())
        }

        // 可以看下 statements 长啥样
        // dbg!(&statements);

        // 解释执行遇到错误的话，内部会处理
        self.interpreter.interpret(&statements);

        Ok(())

    }

    fn report_error(&self, lox_err: LoxErr) {
        eprintln!("{}", lox_err)
    }

    
}