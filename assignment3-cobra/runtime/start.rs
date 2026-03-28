use std::env;

#[link(name = "our_code")]
extern "C" {
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64) -> i64;
}

#[no_mangle]
extern "C" fn snek_error(errcode: i64) {
    if errcode == 1 {
        eprintln!("invalid argument");
    } else if errcode == 2 {
        eprintln!("overflow");
    } else {
        eprintln!("an error occurred {errcode}");
    }
    std::process::exit(1);
}

fn parse_input(input: &str) -> i64 {
    match input {
        "true" => 3,
        "false" => 1,
        _ => {
            let n: i64 = input.parse().expect("invalid input");
            n << 1
        }
    }
}

fn print_value(v: i64) {
    if v == 3 {
        println!("true");
    } else if v == 1 {
        println!("false");
    } else if v & 1 == 0 {
        println!("{}", v >> 1);
    } else {
        println!("unknown value: {v}");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        parse_input(&args[1])
    } else {
        1
    };

    let i: i64 = unsafe { our_code_starts_here(input) };
    print_value(i);
}
