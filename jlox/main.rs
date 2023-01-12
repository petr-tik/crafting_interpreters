fn main_impl() -> () {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: jlox-rs [script]");
            std::process::exit(64)
        }
    }
}

fn run_file(nth: &String) -> ! {
    todo!()
}

fn run_prompt() -> ! {
    todo!()
}

fn main() {
    main_impl();
}
