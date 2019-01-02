extern crate whoami;
extern crate colored;


use std::fmt;
use colored::*;

fn main() {
    let system = System::read_data();

    println!("{}", system);

}

struct System {
    username: String,
    os: String,
    env: String,
    hostname: String,
}

impl System {

    fn read_data() -> System {
        System {
            username: whoami::username(),
            os: whoami::os(),
            env: whoami::env().to_string(),
            hostname: whoami::hostname(),
        }
    }
}

impl fmt::Display for System {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let username_prefix = "Username:".cyan();
        let hostname_prefix = "Hostname:".cyan();
        let os_prefix = "OS:".cyan();
        let env_prefix = "ENV:".cyan();

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

        Ok(())
    }
}