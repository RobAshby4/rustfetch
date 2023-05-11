use std::env::{self, Vars};
use std::process::Command;
// allows running os commands

#[derive(Debug)]
struct EnvInfo {
    user: String,
    host: String,
    desktop: String,
    session_type: String,
    term: String,
    shell: String,
    package_count: String
}

impl EnvInfo {
    fn new() -> Self {
        Self { user: String::new(),
            host: String::new(),
            desktop: String::new(),
            session_type: String::new(),
            term: String::new(),
            shell: String::new(),
            package_count: String::new()
        }
    }

    fn populate(&mut self, variables: &mut Vars) {
        // loop through env-info 
        loop {
            let current_var = match variables.next() {
                Some(x) => x,
                None    => break
            };
            println!("{}: {}", current_var.0.clone(), current_var.1.clone());
            match current_var.0.as_str() {
                "USER"              => {self.user = current_var.1}
                "HOSTNAME"          => {self.host = current_var.1}
                "NAME"              => {self.host = current_var.1}
                "DESKTOP_SESSION"   => {self.desktop = current_var.1}
                "TERM"              => {self.term = current_var.1}
                "SHELL"             => {self.shell = current_var.1}
                "XDG_SESSION_TYPE"  => {self.session_type = current_var.1}
                _ => {},
            }
        }
        // TODO: package count
        // CURRENTLY SUPPORTED: dpkg
        let package_count = Command::new("dpkg-query").arg("-f").arg(".").arg("-W").output().expect("Coudn't run command");
        let num_packages = package_count.stdout.len();
        self.package_count = String::from(num_packages.to_string());
        dbg!(num_packages);
    }
}


fn main() {
    let mut variables = env::vars();
    let mut env_info = EnvInfo::new();
    env_info.populate(&mut variables);
    dbg!(env_info);
    
}
