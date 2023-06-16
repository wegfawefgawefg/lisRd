use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Expr {
    Float(f64),
    Symbol(String),
    List(Vec<Expr>),
}

#[derive(Clone)]
struct Env {
    data: Rc<RefCell<HashMap<String, Expr>>>,
    parent: Option<Rc<Env>>,
}

impl Env {
    fn new(parent: Option<Rc<Env>>) -> Env {
        Env {
            data: Rc::new(RefCell::new(HashMap::new())),
            parent,
        }
    }

    fn find(&self, key: &str) -> Option<Rc<Env>> {
        if self.data.borrow().contains_key(key) {
            Some(Rc::new(self.clone()))
        } else {
            match &self.parent {
                Some(parent) => parent.find(key),
                None => None,
            }
        }
    }
}

fn eval(expr: Expr, env: Rc<Env>) -> Expr {
    match expr {
        Expr::Float(x) => Expr::Float(x),
        Expr::Symbol(s) => {
            let e = env.find(&s).expect("Undefined symbol");
            e.data.borrow()[&s].clone()
        }
        Expr::List(list) => {
            let first = &list[0];
            match first {
                Expr::Symbol(s) if s == "def" => {
                    let symbol = match &list[1] {
                        Expr::Symbol(s) => s.clone(),
                        _ => panic!("Expected a symbol"),
                    };
                    let value = eval(list[2].clone(), Rc::new(env.clone()));
                    env.data.borrow_mut().insert(symbol, value);
                    Expr::Symbol("ok".to_string())
                }
                // Implement the other forms here, similar to the `def` case above
                _ => panic!("Unknown form"),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let program = fs::read_to_string(filename).expect("Cannot read file");

    // You'll have to write your own tokenize and parse functions

    let tokens = tokenize(&program);
    let ast = parse(&tokens);

    let env = Rc::new(Env::new(None));
    let result = eval(ast, env);

    println!("{:?}", result);
}
