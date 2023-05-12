use std::env::{self, Vars};
use std::io::BufRead;
use std::process::*;
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
        let mut num_packages = 0;
        match Command::new("dpkg-query").arg("-f").arg(".").arg("-W").output() {
            Ok(dpkg) => {num_packages += dpkg.stdout.lines().count()},
            Err(_) => {}
        }
        match Command::new("rpm").arg("-qa").output() {
            Ok(rpm) => {num_packages += rpm.stdout.lines().count()},
            Err(_) => {}
        }
        
        self.package_count = String::from(num_packages.to_string());
        dbg!(self.package_count.clone());
    }
}


fn main() {
    let mut variables = env::vars();
    let mut env_info = EnvInfo::new();
    env_info.populate(&mut variables);
    dbg!(env_info);
    
}
