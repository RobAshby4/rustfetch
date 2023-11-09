use std::env::{self, Vars};
use std::fmt::Debug;
use std::io::BufRead;
use std::process::*;
use std::fs;
// allows running os commands

#[derive(Debug)]
struct EnvInfo {
    user: String,
    host: String,
    os: String,
    desktop: String,
    session_type: String,
    term: String,
    shell: String,
    package_count: String
}

impl EnvInfo {
    fn new() -> Self {
        Self { user: String::from("None"),
            host: String::from("None"),
            os: String::from("Not Found"),
            desktop: String::from("None"),
            session_type: String::from("None"),
            term: String::from("None"),
            shell: String::from("None"),
            package_count: String::from("None")
        }
    }

    fn populate(&mut self, variables: &mut Vars) {
        // loop through env-info 
        loop {
            let current_var = match variables.next() {
                Some(x) => x,
                None    => break
            };
            // dbg, prints all env vars
            // println!("{}: {}", current_var.0.clone(), current_var.1.clone());
            match current_var.0.as_str() {
                "USER"              => {self.user = current_var.1}
                "DESKTOP_SESSION"   => {self.desktop = current_var.1}
                "TERM"              => {self.term = current_var.1}
                "SHELL"             => {self.shell = current_var.1}
                "XDG_SESSION_TYPE"  => {self.session_type = current_var.1}
                _ => {},
            }

            self.host = EnvInfo::get_host_name();

        }
        // TODO: package count
        // CURRENTLY SUPPORTED: dpkg
        self.package_count = EnvInfo::get_num_packages();
        self.os = EnvInfo::get_os_type();
    }

    fn get_host_name() -> String {
        let mut hostname = match fs::read_to_string("/etc/hostname") {
            Ok(x) => x,
            Err(_) => String::from("host"),
        };
        hostname.pop();
        return hostname
    }

    fn get_num_packages() -> String {
        let mut num_packages = 0;
        match Command::new("dpkg-query").arg("-f").arg(".\n").arg("-W").output() {
            Ok(dpkg) => {num_packages += dpkg.stdout.lines().count()},
            Err(_) => {}
        }
        match Command::new("rpm").arg("-qa").output() {
            Ok(rpm) => {num_packages += rpm.stdout.lines().count()},
            Err(_) => {}
        }
        match Command::new("nix-store").arg("--query").arg("--requisites").arg("/run/current-system").output() {
            Ok(nix) => {num_packages += nix.stdout.lines().count()},
            Err(_) => {}
        }
        
        String::from(num_packages.to_string())
    }

    fn get_os_type() -> String {
        let os_info = match Command::new("cat").arg("/etc/os-release").output() {
            Ok(os) => os,
            Err(_) => {panic!("unable to retrieve os info")}
        };
        let mut name = String::new();
        os_info.stdout.lines().for_each(|line| {
            match line {
                Ok(field)   => {
                    if field.starts_with("NAME") {
                        name = field.split("=").last().expect("No os name").replace("\"", "").to_string();
                    }
                }
                Err(_)  => {}
            }
        });
        return name;
    }

    fn as_vec(&self) -> Vec<String> {
        let mut vals: Vec<String> = Vec::new();
        vals.push(self.os.clone());
        vals.push(self.desktop.clone());
        vals.push(self.session_type.clone());
        vals.push(self.term.clone());
        vals.push(self.shell.clone());
        vals.push(self.package_count.clone());
        return vals;
    }

    fn render(&self) {
        let values = self.as_vec();
        let separator = ":";
        
        let mut ascii_art: Vec<String> = Vec::new();
        ascii_art.push(String::from(r"      /`·.¸         "));
        ascii_art.push(String::from(r"     /¸...¸`:·      "));
        ascii_art.push(String::from(r" ¸.·´  ¸   `·.¸.·´) "));
        ascii_art.push(String::from(r": © ):´;      ¸  {  "));
        ascii_art.push(String::from(r" `·.¸ `·  ¸.·´\`·¸) "));
        ascii_art.push(String::from(r"     `\\´´\¸.·´     "));
        let ascii_len = ascii_art[0].clone().len() - 2;

        let mut labels: Vec<String> = Vec::new(); 
        labels.push(String::from("OS"));
        labels.push(String::from("Desktop"));
        labels.push(String::from("Session"));
        labels.push(String::from("Term"));
        labels.push(String::from("Shell"));
        labels.push(String::from("pkg #"));
        
        let mut blank_str = String::new();
        for _ in 0..ascii_len {
            blank_str.push(' ');
        }

        let mut largest = 0;
        if largest < labels.len() {largest = labels.len()};
        if largest < ascii_art.len() {largest = ascii_art.len()};
        
        println!("");
        println!("{0}│ {1}@{2}", blank_str.clone(), self.user.clone(), self.host.clone());
        for i in 0..=largest {
            let art = match ascii_art.clone().into_iter().nth(i) {
                Some(x) => x,
                None => blank_str.clone(),
            };
            let label = match labels.clone().into_iter().nth(i) {
                Some(x) => x,
                None => String::new(),
            };
            let value = match values.clone().into_iter().nth(i) {
                Some(x) => x,
                None => String::new(),
            };
            if label.len() > 0 {
                println!("{0}│ {1}{2} {3}", art, label, separator, value);
            } else {
                println!("{0}│ {1} {2}", art, label, value);
            }
        }
        println!("");
    }
}

fn main() {
    let mut variables = env::vars();
    let mut env_info = EnvInfo::new();
    env_info.populate(&mut variables);
    env_info.render();
}
