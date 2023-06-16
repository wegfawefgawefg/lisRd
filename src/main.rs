// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;
use std::fs;

#[derive(Debug, Clone)]
enum Expr {
    Float(f64),
    Symbol(String),
    List(Vec<Expr>),
}

enum ExprType {
    Float,
    Symbol,
    List,
}

fn tokenize(code: &str) -> Vec<String> {
    let mut tokens = vec![];
    let mut token = String::new();
    let mut chars = code.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' | ')' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            c if c.is_whitespace() => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
            }
            ';' => {
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            c => {
                token.push(c);
            }
        }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

fn parse(tokens: &[String], i: usize) -> (Expr, usize) {
    let mut expr = vec![];
    let mut i = i;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "(" => {
                let (e, new_i) = parse(tokens, i + 1);
                expr.push(e);
                i = new_i;
            }
            ")" => {
                return (Expr::List(expr), i + 1);
            }
            token => {
                i += 1;
                let expr_token = if let Ok(num) = token.parse::<f64>() {
                    Expr::Float(num)
                } else {
                    Expr::Symbol(token.to_string())
                };
                expr.push(expr_token);
            }
        }
    }
    (expr[0].clone(), i)
}

// #[derive(Clone)]
// struct Env {
//     data: Rc<RefCell<HashMap<String, Expr>>>,
//     parent: Option<Rc<Env>>,
// }

// impl Env {
//     fn new(parent: Option<Rc<Env>>) -> Env {
//         Env {
//             data: Rc::new(RefCell::new(HashMap::new())),
//             parent,
//         }
//     }

//     fn find(&self, key: &str) -> Option<Rc<Env>> {
//         if self.data.borrow().contains_key(key) {
//             Some(Rc::new(self.clone()))
//         } else {
//             match &self.parent {
//                 Some(parent) => parent.find(key),
//                 None => None,
//             }
//         }
//     }
// }

// fn eval(expr: Expr, env: Rc<Env>) -> Expr {
//     match expr {
//         Expr::Float(x) => Expr::Float(x),
//         Expr::Symbol(s) => {
//             let e = env.find(&s).expect("Undefined symbol");
//             e.data.borrow()[&s].clone()
//         }
//         Expr::List(list) => {
//             let first = &list[0];
//             match first {
//                 Expr::Symbol(s) if s == "def" => {
//                     let symbol = match &list[1] {
//                         Expr::Symbol(s) => s.clone(),
//                         _ => panic!("Expected a symbol"),
//                     };
//                     let value = eval(list[2].clone(), Rc::new(env.clone()));
//                     env.data.borrow_mut().insert(symbol, value);
//                     Expr::Symbol("ok".to_string())
//                 }
//                 // Implement the other forms here, similar to the `def` case above
//                 _ => panic!("Unknown form"),
//             }
//         }
//     }
// }

fn print_tokens(tokens: &[String]) {
    for token in tokens {
        print!("{} ", token);
    }
    println!();
}
fn print_ast(ast: &Expr, depth: usize) {
    match ast {
        Expr::Float(x) => print!("{}", x),
        Expr::Symbol(s) => print!("{}", s),
        Expr::List(list) => {
            for expr in list {
                for _ in 0..depth + 1 {
                    print!(" ");
                }
                print_ast(expr, depth + 1);
                println!();
            }
            println!();
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Unable to read file");
    let tokens = tokenize(&code);
    print_tokens(&tokens);
    let (ast, _) = parse(&tokens, 0);
    print_ast(&ast, 0);

    // let env = Rc::new(Env::new(None));
    // let res = eval(ast, env);
    // println!("{:?}", res);
}
