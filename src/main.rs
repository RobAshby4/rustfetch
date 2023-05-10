use core::panic;
use std::env::{self, Vars};

struct EnvInfo {
    user: String,
    host: String,
    desktop: String,
    session_type: String,
    shell: String,
    package_count: String
}

impl EnvInfo {
    fn new() -> Self {
        Self { user: String::new(),
            host: String::new(),
            desktop: String::new(),
            session_type: String::new(),
            shell: String::new(),
            package_count: String::new()
        }
    }

    fn populate(&self, variables: &mut Vars) {
        for n in 0..variables.count() {
            let current_var = match variables.nth(n) {
                Some(T) => T,
                None => panic!("Couldn't read environment")
            };
            match current_var.0.as_str() {
                "" => println!("{}", current_var.1),
                _ => println!("{}", current_var.1)
            }
        }
    }
}


fn main() {
    let mut variables = env::vars();
    let env_info = EnvInfo::new().populate(&mut variables);
    
}
