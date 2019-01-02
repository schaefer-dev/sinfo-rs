extern crate whoami;
extern crate colored;
extern crate os_type;

mod string_utils;
use crate::string_utils::*;

use std::fmt;
use colored::*;

use std::path::Path;
use std::process::Command;

fn main() {
    let mut system = System::read_data();
    system.post_processing();

    println!("{}", system);
}

struct System {
    username: String,
    os: String,
    env: String,
    hostname: String,
    graphics: String,
    is_mac: bool,
}

impl System {

    fn read_data() -> System {
        System {
            username: whoami::username(),
            os: whoami::os(),
            env: whoami::env().to_string(),
            hostname: whoami::hostname(),
            graphics: "unknown".to_string(),
            is_mac: false,
        }
    }

    // Perform post-processing to make Strings look more appealing
    fn post_processing(&mut self) -> () {
        // Remove all occurances of '.local' in hostname
        remove_local_substrings(&mut self.hostname);

        // if OS not detected, use my own approach
        // NOTE: whoami library outputs a typo in the case of OS being unknown
        if self.os == "uknown" || self.os == "unknown" {
            self.os = System::get_os();
            if self.os[0..3] == "OSX".to_string() {
                self.is_mac = true;
                self.get_osx_graphics();
            } else {
                self.get_linux_graphics();
            }
        }
    }

    fn get_os() -> String {
        // Test for arch
        let pacman_path = Path::new("/etc/pacman.conf");
        if pacman_path.exists() {
            return "Arch Linux".to_string();
        }

        // use os_type library to figure out OS type and version
        let os = os_type::current_platform();
        let os_string: String = format!("{:?} {}", os.os_type, os.version);
        return os_string;
    }

    fn get_osx_graphics(&mut self) {
        let output = Command::new("sh").arg("-c").arg("echo $(system_profiler SPDisplaysDataType | awk '/Model/{for (i=1; i<=NF-2; i++) $i = $(i+2); NF-=2; print}' | paste -sd '/' -)").output().expect("failed to execute Graphics command");
        match std::str::from_utf8(&output.stdout) {
            Ok(v) => self.graphics = v.to_string(),
            Err(_e) => self.graphics = "ERROR".to_string(),
        }
        println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    }

    fn get_linux_graphics(&mut self) {
        // TODO: implement in the future
    }



}

impl fmt::Display for System {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let username_prefix = "Username:".cyan();
        let hostname_prefix = "Hostname:".cyan();
        let os_prefix = "Distro:".cyan();
        let env_prefix = "ENV:".cyan();
        let graphics_prefix = "GPU:".cyan();

        let write_result = write!(f, "{} {}\n", username_prefix, self.username);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", hostname_prefix, self.hostname);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", os_prefix, self.os);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", env_prefix, self.env);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", graphics_prefix, self.graphics);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        Ok(())
    }
}