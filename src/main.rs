use std::env::{self, Vars};
use std::io::BufRead;
use std::process::*;
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
        self.package_count = EnvInfo::get_num_packages();
        self.os = EnvInfo::get_os_type();
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

    fn render(&self) {
        let mut lines: Vec<String> = Vec::new(); 
        lines.push(self.user.clone());
        lines.push(String::from("@"));
        lines.push(self.host.clone());
        lines.push(String::from("\nOS: "));
        lines.push(self.os.clone());
        lines.push(String::from("\nDesktop: "));
        lines.push(self.desktop.clone());
        lines.push(String::from("\nSession: "));
        lines.push(self.session_type.clone());
        lines.push(String::from("\nTerm: "));
        lines.push(self.term.clone());
        lines.push(String::from("\nShell: "));
        lines.push(self.shell.clone());
        lines.push(String::from("\npkg #: "));
        lines.push(self.package_count.clone());

        for line in lines {
            print!("{}", line);
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
