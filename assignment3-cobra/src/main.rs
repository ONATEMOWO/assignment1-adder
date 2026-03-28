use im::HashMap;
use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

const TRUE_VAL: i64 = 3;
const FALSE_VAL: i64 = 1;

#[derive(Debug)]
enum UnOp {
    Add1,
    Sub1,
    Negate,
    IsNum,
    IsBool,
}

#[derive(Debug)]
enum BinOp {
    Plus,
    Minus,
    Times,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Bool(bool),
    Input,
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
}

fn is_reserved(word: &str) -> bool {
    matches!(
        word,
        "let"
            | "add1"
            | "sub1"
            | "negate"
            | "input"
            | "true"
            | "false"
            | "if"
            | "block"
            | "loop"
            | "break"
            | "set!"
            | "isnum"
            | "isbool"
    )
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
        Sexp::Atom(F(_)) => panic!("Invalid"),
        Sexp::Atom(S(name)) if name == "true" => Expr::Bool(true),
        Sexp::Atom(S(name)) if name == "false" => Expr::Bool(false),
        Sexp::Atom(S(name)) if name == "input" => Expr::Input,
        Sexp::Atom(S(name)) => {
            if is_reserved(name) {
                panic!("Invalid");
            }
            Expr::Id(name.to_string())
        }

        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => {
                Expr::UnOp(UnOp::Add1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "sub1" => {
                Expr::UnOp(UnOp::Sub1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "negate" => {
                Expr::UnOp(UnOp::Negate, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "isnum" => {
                Expr::UnOp(UnOp::IsNum, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "isbool" => {
                Expr::UnOp(UnOp::IsBool, Box::new(parse_expr(e)))
            }

            [Sexp::Atom(S(op)), e1, e2] if op == "+" => {
                Expr::BinOp(BinOp::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => {
                Expr::BinOp(BinOp::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => {
                Expr::BinOp(BinOp::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => {
                Expr::BinOp(BinOp::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => {
                Expr::BinOp(BinOp::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => {
                Expr::BinOp(BinOp::LessEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => {
                Expr::BinOp(BinOp::GreaterEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => {
                Expr::BinOp(BinOp::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }

            [Sexp::Atom(S(op)), Sexp::List(binds), body] if op == "let" => {
                if binds.is_empty() {
                    panic!("Invalid");
                }
                let parsed_binds: Vec<(String, Expr)> = binds.iter().map(parse_bind).collect();
                Expr::Let(parsed_binds, Box::new(parse_expr(body)))
            }

            [Sexp::Atom(S(op)), cond, thn, els] if op == "if" => Expr::If(
                Box::new(parse_expr(cond)),
                Box::new(parse_expr(thn)),
                Box::new(parse_expr(els)),
            ),

            [Sexp::Atom(S(op)), expr] if op == "loop" => {
                Expr::Loop(Box::new(parse_expr(expr)))
            }

            [Sexp::Atom(S(op)), expr] if op == "break" => {
                Expr::Break(Box::new(parse_expr(expr)))
            }

            [Sexp::Atom(S(op)), Sexp::Atom(S(name)), expr] if op == "set!" => {
                if is_reserved(name) {
                    panic!("Invalid");
                }
                Expr::Set(name.to_string(), Box::new(parse_expr(expr)))
            }

            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => {
                if exprs.is_empty() {
                    panic!("Invalid");
                }
                Expr::Block(exprs.iter().map(parse_expr).collect())
            }

            _ => panic!("Invalid"),
        },
    }
}

fn new_label(counter: &mut i32, prefix: &str) -> String {
    let label = format!("{prefix}_{counter}");
    *counter += 1;
    label
}

fn stack_offset(si: i32) -> i32 {
    si * 8
}

fn compile_bool_result_from_jump(jump: &str, counter: &mut i32) -> String {
    let true_label = new_label(counter, "bool_true");
    let done_label = new_label(counter, "bool_done");
    format!(
        "{jump} {true_label}\nmov rax, {FALSE_VAL}\njmp {done_label}\n{true_label}:\nmov rax, {TRUE_VAL}\n{done_label}:"
    )
}

fn compile_check_num_rax(counter: &mut i32) -> String {
    let ok = new_label(counter, "num_ok");
    format!(
        "test rax, 1\njz {ok}\nmov rdi, 1\ncall _snek_error\n{ok}:"
    )
}

fn compile_check_num_rbx(counter: &mut i32) -> String {
    let ok = new_label(counter, "num_ok");
    format!(
        "test rbx, 1\njz {ok}\nmov rdi, 1\ncall _snek_error\n{ok}:"
    )
}

fn compile_overflow_guard(counter: &mut i32) -> String {
    let ok = new_label(counter, "overflow_ok");
    format!(
        "jno {ok}\nmov rdi, 2\ncall _snek_error\n{ok}:"
    )
}

fn compile_expr(
    e: &Expr,
    si: i32,
    env: &HashMap<String, i32>,
    counter: &mut i32,
    break_target: Option<&str>,
) -> String {
    match e {
        Expr::Number(n) => format!("mov rax, {}", (*n as i64) << 1),

        Expr::Bool(true) => format!("mov rax, {TRUE_VAL}"),
        Expr::Bool(false) => format!("mov rax, {FALSE_VAL}"),

        Expr::Input => "mov rax, rdi".to_string(),

        Expr::Id(name) => match env.get(name) {
            Some(offset) => format!("mov rax, [rbp - {}]", offset),
            None => panic!("Unbound variable identifier {}", name),
        },

        Expr::UnOp(UnOp::Add1, expr) => {
            let sub = compile_expr(expr, si, env, counter, break_target);
            let check = compile_check_num_rax(counter);
            let ovf = compile_overflow_guard(counter);
            format!("{sub}\n{check}\nadd rax, 2\n{ovf}")
        }

        Expr::UnOp(UnOp::Sub1, expr) => {
            let sub = compile_expr(expr, si, env, counter, break_target);
            let check = compile_check_num_rax(counter);
            let ovf = compile_overflow_guard(counter);
            format!("{sub}\n{check}\nsub rax, 2\n{ovf}")
        }

        Expr::UnOp(UnOp::Negate, expr) => {
            let sub = compile_expr(expr, si, env, counter, break_target);
            let check = compile_check_num_rax(counter);
            let ovf = compile_overflow_guard(counter);
            format!("{sub}\n{check}\nneg rax\n{ovf}")
        }

        Expr::UnOp(UnOp::IsNum, expr) => {
            let sub = compile_expr(expr, si, env, counter, break_target);
            let true_label = new_label(counter, "isnum_true");
            let done_label = new_label(counter, "isnum_done");
            format!(
                "{sub}\ntest rax, 1\njz {true_label}\nmov rax, {FALSE_VAL}\njmp {done_label}\n{true_label}:\nmov rax, {TRUE_VAL}\n{done_label}:"
            )
        }

        Expr::UnOp(UnOp::IsBool, expr) => {
            let sub = compile_expr(expr, si, env, counter, break_target);
            let true_label = new_label(counter, "isbool_true");
            let done_label = new_label(counter, "isbool_done");
            format!(
                "{sub}\ntest rax, 1\njnz {true_label}\nmov rax, {FALSE_VAL}\njmp {done_label}\n{true_label}:\nmov rax, {TRUE_VAL}\n{done_label}:"
            )
        }

        Expr::BinOp(BinOp::Plus, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let ovf = compile_overflow_guard(counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nmov rbx, [rbp - {offset}]\n{check_left}\nadd rax, rbx\n{ovf}"
            )
        }

        Expr::BinOp(BinOp::Minus, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let ovf = compile_overflow_guard(counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nmov rbx, [rbp - {offset}]\n{check_left}\nsub rbx, rax\nmov rax, rbx\n{ovf}"
            )
        }

        Expr::BinOp(BinOp::Times, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let ovf = compile_overflow_guard(counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nsar rax, 1\nmov rbx, [rbp - {offset}]\n{check_left}\nimul rax, rbx\n{ovf}"
            )
        }

        Expr::BinOp(BinOp::Less, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let bool_code = compile_bool_result_from_jump("jl", counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nmov rbx, [rbp - {offset}]\n{check_left}\ncmp rbx, rax\n{bool_code}"
            )
        }

        Expr::BinOp(BinOp::Greater, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let bool_code = compile_bool_result_from_jump("jg", counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nmov rbx, [rbp - {offset}]\n{check_left}\ncmp rbx, rax\n{bool_code}"
            )
        }

        Expr::BinOp(BinOp::LessEqual, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let bool_code = compile_bool_result_from_jump("jle", counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nmov rbx, [rbp - {offset}]\n{check_left}\ncmp rbx, rax\n{bool_code}"
            )
        }

        Expr::BinOp(BinOp::GreaterEqual, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let check_right = compile_check_num_rax(counter);
            let check_left = compile_check_num_rbx(counter);
            let bool_code = compile_bool_result_from_jump("jge", counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\n{check_right}\nmov rbx, [rbp - {offset}]\n{check_left}\ncmp rbx, rax\n{bool_code}"
            )
        }

        Expr::BinOp(BinOp::Equal, left, right) => {
            let left_code = compile_expr(left, si, env, counter, break_target);
            let right_code = compile_expr(right, si + 1, env, counter, break_target);
            let offset = stack_offset(si);
            let type_ok = new_label(counter, "eq_type_ok");
            let bool_code = compile_bool_result_from_jump("je", counter);
            format!(
                "{left_code}\nmov [rbp - {offset}], rax\n{right_code}\nmov rbx, [rbp - {offset}]\nmov rcx, rbx\nxor rcx, rax\ntest rcx, 1\njz {type_ok}\nmov rdi, 1\ncall _snek_error\n{type_ok}:\ncmp rbx, rax\n{bool_code}"
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

                let expr_code = compile_expr(expr, cur_si, &new_env, counter, break_target);
                let offset = stack_offset(cur_si);

                if !instrs.is_empty() {
                    instrs.push('\n');
                }
                instrs.push_str(&expr_code);
                instrs.push_str(&format!("\nmov [rbp - {}], rax", offset));

                new_env.insert(name.clone(), offset);
                cur_si += 1;
            }

            let body_code = compile_expr(body, cur_si, &new_env, counter, break_target);

            if instrs.is_empty() {
                body_code
            } else {
                format!("{instrs}\n{body_code}")
            }
        }

        Expr::If(cond, thn, els) => {
            let else_label = new_label(counter, "if_else");
            let done_label = new_label(counter, "if_done");
            let cond_code = compile_expr(cond, si, env, counter, break_target);
            let thn_code = compile_expr(thn, si, env, counter, break_target);
            let els_code = compile_expr(els, si, env, counter, break_target);
            format!(
                "{cond_code}\ncmp rax, {FALSE_VAL}\nje {else_label}\n{thn_code}\njmp {done_label}\n{else_label}:\n{els_code}\n{done_label}:"
            )
        }

        Expr::Block(exprs) => {
            let mut parts: Vec<String> = Vec::new();
            for expr in exprs {
                parts.push(compile_expr(expr, si, env, counter, break_target));
            }
            parts.join("\n")
        }

        Expr::Loop(body) => {
            let start_label = new_label(counter, "loop_start");
            let end_label = new_label(counter, "loop_end");
            let body_code = compile_expr(body, si, env, counter, Some(&end_label));
            format!(
                "{start_label}:\n{body_code}\njmp {start_label}\n{end_label}:"
            )
        }

        Expr::Break(expr) => match break_target {
            Some(target) => {
                let expr_code = compile_expr(expr, si, env, counter, break_target);
                format!("{expr_code}\njmp {target}")
            }
            None => panic!("break outside of loop"),
        },

        Expr::Set(name, expr) => match env.get(name) {
            Some(offset) => {
                let expr_code = compile_expr(expr, si, env, counter, break_target);
                format!("{expr_code}\nmov [rbp - {}], rax", offset)
            }
            None => panic!("Unbound variable identifier {}", name),
        },
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

    let mut counter = 0;
    let result = compile_expr(&expr, 2, &HashMap::new(), &mut counter, None);

    let asm_program = format!(
        "section .text
extern _snek_error
global our_code_starts_here
our_code_starts_here:
  push rbp
  mov rbp, rsp
  sub rsp, 800
  {}
  mov rsp, rbp
  pop rbp
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
