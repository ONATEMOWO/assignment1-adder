use im::HashMap;
use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
}

#[derive(Debug)]
enum Op2 {
    Plus,
    Minus,
    Times,
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
}

fn is_reserved(word: &str) -> bool {
    matches!(word, "let" | "add1" | "sub1")
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(name)), expr] => {
                if is_reserved(name) {
                    panic!("Invalid");
                }
                (name.to_string(), parse_expr(expr))
            }
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Number(i32::try_from(*n).unwrap()),

        Sexp::Atom(S(name)) => {
            if is_reserved(name) {
                panic!("Invalid");
            }
            Expr::Id(name.to_string())
        }

        Sexp::Atom(F(_)) => panic!("Invalid"),

        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => {
                Expr::UnOp(Op1::Add1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "sub1" => {
                Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => {
                Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => {
                Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => {
                Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), Sexp::List(binds), body] if op == "let" => {
                if binds.is_empty() {
                    panic!("Invalid");
                }
                let parsed_binds: Vec<(String, Expr)> = binds.iter().map(parse_bind).collect();
                Expr::Let(parsed_binds, Box::new(parse_expr(body)))
            }
            _ => panic!("Invalid"),
        },

    }
}

fn stack_offset(si: i32) -> i32 {
    si * 8
}

fn compile_to_instrs(e: &Expr, si: i32, env: &HashMap<String, i32>) -> String {
    match e {
        Expr::Number(n) => format!("mov rax, {}", *n),

        Expr::Id(name) => match env.get(name) {
            Some(offset) => format!("mov rax, [rsp - {}]", offset),
            None => panic!("Unbound variable identifier {}", name),
        },

        Expr::UnOp(Op1::Add1, expr) => {
            let sub = compile_to_instrs(expr, si, env);
            format!("{sub}\nadd rax, 1")
        }

        Expr::UnOp(Op1::Sub1, expr) => {
            let sub = compile_to_instrs(expr, si, env);
            format!("{sub}\nsub rax, 1")
        }

        Expr::BinOp(Op2::Plus, left, right) => {
            let left_code = compile_to_instrs(left, si, env);
            let right_code = compile_to_instrs(right, si + 1, env);
            let offset = stack_offset(si);
            format!(
                "{left_code}\nmov [rsp - {offset}], rax\n{right_code}\nadd rax, [rsp - {offset}]"
            )
        }

        Expr::BinOp(Op2::Minus, left, right) => {
            let left_code = compile_to_instrs(left, si, env);
            let right_code = compile_to_instrs(right, si + 1, env);
            let offset = stack_offset(si);
            format!(
                "{left_code}\nmov [rsp - {offset}], rax\n{right_code}\nmov rbx, [rsp - {offset}]\nsub rbx, rax\nmov rax, rbx"
            )
        }

        Expr::BinOp(Op2::Times, left, right) => {
            let left_code = compile_to_instrs(left, si, env);
            let right_code = compile_to_instrs(right, si + 1, env);
            let offset = stack_offset(si);
            format!(
                "{left_code}\nmov [rsp - {offset}], rax\n{right_code}\nimul rax, [rsp - {offset}]"
            )
        }

        Expr::Let(bindings, body) => {
            let mut instrs = String::new();
            let mut new_env = env.clone();
            let mut cur_si = si;
            let mut names_in_this_let: Vec<String> = Vec::new();

            for (name, expr) in bindings {
                if names_in_this_let.contains(name) {
                    panic!("Duplicate binding");
                }
                names_in_this_let.push(name.clone());

                let expr_code = compile_to_instrs(expr, cur_si, &new_env);
                let offset = stack_offset(cur_si);

                if !instrs.is_empty() {
                    instrs.push('\n');
                }
                instrs.push_str(&expr_code);
                instrs.push_str(&format!("\nmov [rsp - {}], rax", offset));

                new_env.insert(name.clone(), offset);
                cur_si += 1;
            }

            let body_code = compile_to_instrs(body, cur_si, &new_env);

            if instrs.is_empty() {
                body_code
            } else {
                format!("{instrs}\n{body_code}")
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = parse_expr(&parse(&in_contents).unwrap());
    let result = compile_to_instrs(&expr, 2, &HashMap::new());

    let asm_program = format!(
        "section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
