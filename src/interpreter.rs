use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};


use crate::environment::Environment;
use crate::lox_callable::LoxCallable;
use crate::lox_class::LoxClass;
use crate::lox_function::LoxFunction;
use crate::resolvable::Resolvable;
use crate::token::Token;
use crate::expr::{AssignExpr, BinaryExpr, CallExpr, CommaExpr, Expr, GetExpr, GroupingExpr, LiteralExpr, LogicalExpr, SetExpr, SuperExpr, ThisExpr, UnaryExpr, VariableExpr};
use crate::err::LoxErr;
use crate::stmt::{ClassDeclaration, FunctionDeclaration, Stmt};
use crate::object::{NativeFunction, Object};
use crate::token_type::TokenType;



pub struct Interpreter{
    pub had_runtime_error: bool,
    environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        let env = Environment::new();
        env.borrow_mut().define("clock", Object::NativeFunction(NativeFunction{ name: "clock".to_string() }));
        Interpreter {
            had_runtime_error: false,
            environment: Rc::clone(&env),
            globals: env,
        }
    }

    fn get_env(&self) -> Ref<Environment> {
        self.environment.borrow()
    }

    fn get_env_mut(&self) -> RefMut<Environment> {
        self.environment.borrow_mut()
    }

    fn get_globals(&self) -> Ref<Environment> {
        self.globals.borrow()
    }

    fn get_globals_mut(&self) -> RefMut<Environment> {
        self.globals.borrow_mut()
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            if let Err(lox_err) = self.execute(statement) {
                eprintln!("{}", lox_err);
                self.had_runtime_error = true;
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxErr> {
        match expr {
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            Expr::Binary(binary_expr) => self.visit_binary_expr(binary_expr),
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            Expr::Comma(comma_expr) => self.visit_comma_expr(comma_expr),
            Expr::Get(get_expr) => self.visit_get_expr(get_expr),
            Expr::Grouping(grouping_expr) => self.visit_grouping_expr(grouping_expr),
            Expr::Literal(literal_expr) => self.visit_literal_expr(literal_expr),
            Expr::Logical(logical_expr) => self.visit_logical_expr(logical_expr),
            Expr::Set(set_expr) => self.visit_set_expr(set_expr),
            Expr::Super(super_expr) => self.visit_super_expr(super_expr),
            Expr::This(this_expr) => self.visit_this_expr(this_expr),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Variable(variable_expr) => self.visit_variable_expr(variable_expr),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxErr>{
        match stmt {
            Stmt::Block { statements: stmts } => self.visit_block_stmt(stmts)?,
            Stmt::ClassDeclaration { class_declaration } => self.visit_class_declaration_stmt(class_declaration)?,
            Stmt::Expression{ expression: expr} => self.visit_expression_stmt(expr)?,
            Stmt::If { condition, then_branch, else_branch } => self.visit_if_stmt(condition, then_branch, else_branch)?,
            Stmt::While { condition, body } => self.visit_while_stmt(condition, body)?,
            Stmt::Print{ expression: expr} => self.visit_print_stmt(expr)?,
            Stmt::Var { name, initializer } => self.visit_var_stmt(name, initializer)?,
            Stmt::FunctionDeclaration { function_declaration } => self.visit_function_declaration_stmt(function_declaration)?,
            Stmt::Return { keyword: _, value } => self.visit_return_stmt(value)?,
        };
        Ok(())
    }

    pub fn execute_block(&mut self, stmts: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), LoxErr> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        // let mut f = || {
        //     for stmt in stmts {
        //         self.execute(stmt)?;
        //     }
        //     Result::<(), LoxErr>::Ok(())
        // };
        // let ret = f();
        let ret = stmts.iter().try_for_each(|stmt| self.execute(stmt)); // 这一行代替上面那么多真是妙啊

        self.environment = previous;
        ret
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), LoxErr> {
        let block_env = Environment::new();
        block_env.borrow_mut().set_enclosing(Rc::clone(&self.environment));
        self.execute_block(stmts, block_env)
    }

    fn visit_class_declaration_stmt(&mut self, class_declaration: &ClassDeclaration) -> Result<(), LoxErr> {

        let mut superclass = None;
        let mut superclass_obj = Object::default();
        if let Some(exist_superclass) = &class_declaration.superclass {
            superclass_obj = self.visit_variable_expr(exist_superclass)?;
            let Object::Class(lox_class) = superclass_obj.clone() else {
                return Err(LoxErr::Runtime { line: exist_superclass.name.line, message: "Superclass must be a class.".to_string() });
            };
            superclass = Some(Box::new(lox_class));
        }

        self.get_env_mut().define(&class_declaration.name.lexeme, Object::None);

        if class_declaration.superclass.is_some() {
            let env = Environment::new();
            env.borrow_mut().set_enclosing(Rc::clone(&self.environment));
            self.environment = env;
            self.get_env_mut().define("super", superclass_obj);
        }

        let mut methods = HashMap::new();
        for method_decl in &class_declaration.methods {
            let function = LoxFunction::new(method_decl, Rc::clone(&self.environment), &method_decl.name.lexeme == "init");
            methods.insert(method_decl.name.lexeme.clone(), function);
        }
        let class = LoxClass::new(class_declaration.name.lexeme.clone(), superclass, methods);
        
        if class_declaration.superclass.is_some() {
            let o_env = &self.get_env_mut().enclosing.clone().unwrap();
            self.environment = Rc::clone(&o_env);
        }

        self.get_env_mut().assign(&class_declaration.name, Object::Class(class))?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<(), LoxErr> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_function_declaration_stmt(&mut self, function_declaration: &FunctionDeclaration) -> Result<(), LoxErr> {
        let function = LoxFunction::new(function_declaration, Rc::clone(&self.environment), false);
        self.get_env_mut().define(&function_declaration.name.lexeme, Object::Function(function));
        Ok(())
    }

    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> Result<(), LoxErr> {
        if Interpreter::is_truthy(&self.evaluate(condition)?) {
            self.execute(then_branch)?;
        } else if let Some(exist_else_branch) = else_branch {
            self.execute(exist_else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Box<Stmt>) -> Result<(), LoxErr> {
        while Interpreter::is_truthy(&self.evaluate(condition)?) {
            self.execute(body)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), LoxErr> {
        let tl: Object = self.evaluate(expr)?;
        println!("{}", tl);
        Ok(())
    }

    fn visit_return_stmt(&mut self, value: &Option<Expr>) -> Result<(), LoxErr> {
        let ret_value = if let Some(expr) = value {
            self.evaluate(expr)?
        } else {
            Object::None
        };
        Err(LoxErr::RuntimeReturn { ret_value })
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), LoxErr> {
        let value = if initializer.is_some() {
            self.evaluate(initializer.as_ref().unwrap())?
        } else {
            Object::None
        };
        self.get_env_mut().define(&name.lexeme, value);
        Ok(())
    }

    fn visit_assign_expr(&mut self, assign_expr: &AssignExpr) -> Result<Object, LoxErr> {
        let value = self.evaluate(&assign_expr.value)?;

        if let Some(distance) = assign_expr.get_distance() {
            self.get_env_mut().assign_at(distance, assign_expr.name(), value.clone());
        } else {
            self.get_globals_mut().assign(assign_expr.name(), value.clone())?;
        }

        Ok(value)   // 赋值表达式可以嵌套在其它表达式里，比如：print a = 2;
    }

    fn visit_literal_expr(&self, literal_expr: &LiteralExpr) -> Result<Object, LoxErr> {
        Ok(literal_expr.literal.clone())
    }

    fn visit_grouping_expr(&mut self, grouping_expr: &GroupingExpr) -> Result<Object, LoxErr> {
        self.evaluate(&grouping_expr.expression)
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Result<Object, LoxErr> {
        let right = self.evaluate(&unary_expr.right)?;
        match unary_expr.operator.token_type {
            TokenType::Bang => {
                return Ok(Object::Bool(!Interpreter::is_truthy(&right)));
            },
            TokenType::Minus => {
                if let Object::Number(v) = right {
                    Ok(Object::Number(-v))
                } else {
                    Interpreter::number_err(unary_expr.operator.line)
                }
            },
            _ => unreachable!("Impossible operator for unary expr."),
        }
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr) -> Result<Object, LoxErr> {
        let callee = self.evaluate(&*(*call_expr).callee)?;
        let mut arguments = Vec::new();
        for arg in &call_expr.arguments {
            arguments.push(self.evaluate(arg)?);
        }
        
        match callee {
            Object::Function(mut function) => {
                if arguments.len() != function.arity() {
                    return Err(LoxErr::Runtime { line: call_expr.paren.line, message: format!("Expected {} arguments but got {}.", function.arity(), arguments.len()) });
                }
                return function.call(self, arguments);
            }
            Object::NativeFunction(mut native_function) => {
                return native_function.call(self, arguments);
            }
            Object::Class(mut class) => {
                return class.call(self, arguments);
            }
            _ => {
                return Err(LoxErr::Runtime { line: call_expr.paren.line, message: "Can only call functions and classes.".to_string() });
            }
        }
    }

    fn visit_comma_expr(&mut self, comma_expr: &CommaExpr) -> Result<Object, LoxErr> {
        let mut res = Object::default();
        for expr in &comma_expr.exprs {
            res = self.evaluate(expr)?;
        }
        Ok(res)
    }

    fn visit_get_expr(&mut self, get_expr: &GetExpr) -> Result<Object, LoxErr> {
        let object = self.evaluate(&*(*get_expr).object)?;
        if let Object::Instance(instance) = object {
            return instance.borrow().get(&get_expr.name, &instance);
        }
        Err(LoxErr::Runtime { line: get_expr.name.line, message: "Only instances have properties.".to_string() })

    }


    // 逻辑运算符并不承诺会真正返回`true`或`false`，而只是保证它将返回一个具有适当真实性的值。
    fn visit_logical_expr(&mut self, logical_expr: &LogicalExpr) -> Result<Object, LoxErr> {
        let left = self.evaluate(&logical_expr.left)?;
        if logical_expr.operator.token_type == TokenType::Or {
            if Interpreter::is_truthy(&left) {
                return Ok(left);
            } // else right
        } else {
            // And
            if !Interpreter::is_truthy(&left) {
                return Ok(left);
            } // else right
        }
        self.evaluate(&logical_expr.right)
    }

    fn visit_set_expr(&mut self, set_expr: &SetExpr) -> Result<Object, LoxErr> {
        let object = self.evaluate(&set_expr.object)?;
        match object {
            Object::Instance(instance) => {
                let value = self.evaluate(&set_expr.value)?;
                instance.borrow_mut().set(&set_expr.name, value.clone());
                Ok(value)
            }
            _ => Err(LoxErr::Runtime { line: set_expr.name.line, message: "Only instances have fields.".to_string() }),
        }
    }

    fn visit_super_expr(&mut self, super_expr: &SuperExpr) -> Result<Object, LoxErr> {
        let distance = super_expr.get_distance().unwrap();
        let superclass = self.get_env().get_at(distance, "super");

        let object = self.get_env_mut().get_at(distance - 1, "this");   // 从某 instance . get 到 method 时，会创建一个绑定 this 的 closure
        if let Object::Class(lox_class) = superclass {
            let method = lox_class.find_method(&super_expr.method.lexeme);
            if method.is_none() {
                return Err(LoxErr::Runtime { line: super_expr.method.line, message: format!("Undefined property '{}'.", super_expr.method.lexeme) });
            }
            if let Object::Instance(instance) = object {
                return Ok(Object::Function(method.unwrap().bind(instance)));
            } else {
                unreachable!("this is not instance, WTF?")
            }
        } else {
            unreachable!("superclass is not class, visit_class_declaration_stmt bug?");
        }
    }

    fn visit_this_expr(&mut self, this_expr: &ThisExpr) -> Result<Object, LoxErr> {
        self.look_up_variable(this_expr)    // 也就是说，这个 this 最终会变成 Instance 本身 
    }


    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr) -> Result<Object, LoxErr> {
        let left = self.evaluate(&binary_expr.left)?;
        let right = self.evaluate(&binary_expr.right)?;
        match binary_expr.operator.token_type {
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::Greater => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number > right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number >= right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Less => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number < right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::LessEqual => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Bool(left_number <= right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Minus => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Number(left_number - right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Slash => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    if right_number != 0.0 {
                        Ok(Object::Number(left_number / right_number))
                    } else {
                        Err(LoxErr::Runtime { line: binary_expr.operator.line, message: format!("Attempt to divide `{}` by zero.", left_number) })
                    }
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Star => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (left, right) {
                    Ok(Object::Number(left_number * right_number))
                } else {
                    Interpreter::number_err(binary_expr.operator.line)
                }
            }
            TokenType::Plus => {
                if let (Object::Number(left_number), Object::Number(right_number)) = (&left, &right) {
                    return Ok(Object::Number(left_number + right_number));
                }
                if let (Object::String(left_string), Object::String(right_string)) = (&left, &right) {
                    return Ok(Object::String(format!("{}{}", left_string, right_string)));
                }
                Err(LoxErr::Runtime { line: binary_expr.operator.line, message: "Operands must be two numbers or two strings.".to_string() })
            }
            

            _ => unreachable!("Impossible operator for binary expr."),
        }
        
    }

    fn visit_variable_expr(&self, variable_expr: &VariableExpr) -> Result<Object, LoxErr> {
        self.look_up_variable(variable_expr)
    }

    fn look_up_variable(&self, val: &impl Resolvable) -> Result<Object, LoxErr> {
        if let Some(distance) = val.get_distance() {
            Ok(self.get_env().get_at(distance, &val.name().lexeme))
        } else {
            self.get_globals().get(val.name())
        }
    }

    fn is_truthy(literal: &Object) -> bool {
        match literal {
            Object::None => false,
            Object::Bool(v) => *v,
            _ => true,
        }
    }

    fn number_err(line: usize) -> Result<Object, LoxErr> {
        Err(LoxErr::Runtime { line: line, message: "Operand must be a number.".to_string() })
    }
}



#[cfg(test)]
mod tests {
    use crate::lox::Lox;

    #[test]
    fn test_block() {
        let mut lox = Lox::new();
        let code = r#"var a = "global a";
var b = "global b";
var c = "global c";
{
var a = "outer a";
var b = "outer b";
{
    var a = "inner a";
    print a;
    print b;
    print c;
}
print a;
print b;
print c;
}
print a;
print b;
print c;
        "#;
        // let code = r#"print "code";"#;

        lox.test_code(code);
    }
}