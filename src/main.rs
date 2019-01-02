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
    hostname: String,
    graphics: String,
    cpu: String,
    ram: String,
    disk_percentage: String,
    terminal: String,
    editor: String,
    is_mac: bool,
}

impl System {

    fn read_data() -> System {
        // TODO: call the get functions here directly instead of setting to unknown and afterwards
        let mut system: System = System {
            username: whoami::username(),
            os: System::get_os(),
            hostname: whoami::hostname(),
            graphics: "unknown".to_string(),
            cpu: "unknown".to_string(),
            ram: "unknown".to_string(),
            disk_percentage: "unknown".to_string(),
            terminal: System::get_shell_output("$TERM"),
            editor: System::get_shell_output("$EDITOR"),
            is_mac: false,
        };
        system.get_hardware();
        system
    }

    // Perform post-processing to make Strings look more appealing
    fn post_processing(&mut self) -> () {
        // Remove all occurances of '.local' in hostname
        remove_local_substrings(&mut self.hostname);

        // TODO: Think of a safe approach to truncate 'Intel(R) Core(TM)' without hardcoding 18 chars
        self.cpu = self.cpu[18..].to_string();
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

    fn get_hardware(&mut self) {
        if self.os[0..3] == "OSX".to_string() {
            self.is_mac = true;
            self.get_osx_hardware();
        } else {
            self.get_linux_hardware();
        }
    }

    // Function writes CPU, GPU and RAM values
    fn get_osx_hardware(&mut self) {
        self.graphics = System::get_shell_output("$(system_profiler SPDisplaysDataType | awk '/Model/{for (i=1; i<=NF-2; i++) $i = $(i+2); NF-=2; print}' | paste -sd '/' -)");

        self.cpu = System::get_shell_output("$(sysctl -n machdep.cpu.brand_string)");

        let ram_in_bytes_str: String = System::get_shell_output("$(sysctl -n hw.memsize)");
        self.ram = (ram_in_bytes_str.parse::<u64>().unwrap() / 1073741274).to_string() + " GB";

        self.disk_percentage = System::get_shell_output("$(df -Hl | head -2 | tail -1) | awk '{print $5}'");
    }

    // Function writes CPU, GPU and RAM values
    fn get_linux_hardware(&mut self) {
        // TODO: implement in the future
    }

    fn get_shell_output(shell_input_command: &str) -> String {
        let bash_command = "echo ".to_string() + shell_input_command;
        let output = Command::new("sh")
            .arg("-c")
            .arg(bash_command)
            .output()
            .expect("failed to execute process");

        match std::str::from_utf8(&output.stdout) {
            Ok(v) => {
                let mut env_value = v.to_string();
                // Remove trailing newline at the end of output
                env_value.pop();
                env_value
            },
            Err(_e) => "ERROR".to_string(),
        }
    }
}

impl fmt::Display for System {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let username_prefix = "Username:".cyan();
        let hostname_prefix = "Hostname:".cyan();
        let os_prefix = "Distro:".cyan();
        let cpu_prefix = "CPU:".cyan();
        let graphics_prefix = "GPU:".cyan();
        let terminal_prefix = "Term:".cyan();
        let editor_prefix = "Editor:".cyan();
        let ram_prefix = "Memory:".cyan();
        let disk_percentage_prefix = "Disk Usage:".cyan();

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

        let write_result = write!(f, "{} {}\n", cpu_prefix, self.cpu);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", ram_prefix, self.ram);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", graphics_prefix, self.graphics);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", terminal_prefix, self.terminal);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", editor_prefix, self.editor);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }

        let write_result = write!(f, "{} {}\n", disk_percentage_prefix, self.disk_percentage);
        match write_result {
            Ok(_v) => (),
            Err(_e) => return write_result,
        }


        Ok(())
    }
}