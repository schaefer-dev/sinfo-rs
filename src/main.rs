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

struct SystemProperty {
    prefix: String,
    value: String,
}

struct System {
    username: SystemProperty,
    os: SystemProperty,
    hostname: SystemProperty,
    graphics: SystemProperty,
    cpu: SystemProperty,
    ram: SystemProperty,
    disk_percentage: SystemProperty,
    terminal: SystemProperty,
    shell: SystemProperty,
    editor: SystemProperty,
    is_mac: bool,
}

impl System {

    fn read_data() -> System {

        let username: SystemProperty = SystemProperty {
            value: whoami::username(),
            prefix: "Username:".cyan().to_string(),
        };

        let os: SystemProperty = SystemProperty {
            value: System::get_os(),
            prefix: "OS:".cyan().to_string(),
        };

        let hostname: SystemProperty = SystemProperty {
            value: whoami::hostname(),
            prefix: "Hostname:".cyan().to_string(),
        };

        let graphics: SystemProperty = SystemProperty {
            value: "unknown".to_string(),
            prefix: "GPU:".cyan().to_string(),
        };

        let cpu: SystemProperty = SystemProperty {
            value: "unknown".to_string(),
            prefix: "CPU:".cyan().to_string(),
        };

        let ram: SystemProperty = SystemProperty {
            value: "unknown".to_string(),
            prefix: "RAM:".cyan().to_string(),
        };

        let disk_percentage: SystemProperty = SystemProperty {
            value: "unknown".to_string(),
            prefix: "Disk Usage:".cyan().to_string(),
        };

        let terminal: SystemProperty = SystemProperty {
            value: System::get_shell_output("$TERM"),
            prefix: "Terminal:".cyan().to_string(),
        };

        let shell: SystemProperty = SystemProperty {
            value: System::get_shell_output("$SHELL"),
            prefix: "Shell:".cyan().to_string(),
        };

        let editor: SystemProperty = SystemProperty {
            value: System::get_shell_output("$EDITOR"),
            prefix: "Editor:".cyan().to_string(),
        };


        let mut system: System = System {
            username,
            os,
            hostname,
            graphics,
            cpu,
            ram,
            disk_percentage,
            terminal,
            editor,
            shell,
            is_mac: false,
        };
        system.get_hardware();
        system
    }

    // Perform post-processing to make Strings look more appealing
    fn post_processing(&mut self) -> () {
        // Remove all occurances of '.local' in hostname
        remove_local_substrings(&mut self.hostname.value);

        // TODO: Think of a safe approach to truncate 'Intel(R) Core(TM)' without hardcoding 18 chars
        self.cpu.value = self.cpu.value[18..].to_string();
    }

    // Returns the running Operating System
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

    // Depending on the OS read SystemProperties of hardware
    fn get_hardware(&mut self) {
        if self.os.value[0..3] == "OSX".to_string() {
            self.is_mac = true;
            self.get_osx_hardware();
        } else {
            self.get_linux_hardware();
        }
    }

    // Function reads OS dependant System Properties for macOS
    fn get_osx_hardware(&mut self) {
        self.graphics.value = System::get_shell_output("$(system_profiler SPDisplaysDataType | awk '/Model/{for (i=1; i<=NF-2; i++) $i = $(i+2); NF-=2; print}' | paste -sd '/' -)");

        self.cpu.value = System::get_shell_output("$(sysctl -n machdep.cpu.brand_string)");

        let ram_in_bytes_str: String = System::get_shell_output("$(sysctl -n hw.memsize)");
        self.ram.value = (ram_in_bytes_str.parse::<u64>().unwrap() / 1073741274).to_string() + " GB";

        self.disk_percentage.value = System::get_shell_output("$(df -Hl | head -2 | tail -1) | awk '{print $5}'");
    }

    // Function reads OS dependant System Properties for Linux Distributions
    fn get_linux_hardware(&mut self) {
        // TODO: implement in the future
    }

    // Return echo result of the execution of the passed shell command
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

        // Vector which defines the order in which the SystemProperties are printed
        let output_data_vector = vec![&self.username, &self.hostname, &self.os, &self.cpu, &self.ram, &self.graphics, &self.terminal, &self.shell, &self.editor, &self.disk_percentage];

        for element in &output_data_vector {
            let write_result = write!(f, "{} {}\n", element.prefix, element.value);
            match write_result {
                Ok(_v) => (),
                Err(_e) => return write_result,
            }
        }
        Ok(())
    }
}