use std::env;

fn read_and_print_env_vars()
{
    let vars = env::vars();
    for (k, v) in vars {
        println!("{}:{}", k, v);
    }
}

fn main() {
    read_and_print_env_vars();
}
