extern crate whoami;
extern crate colored;


use std::fmt;
use colored::*;
use std::collections::VecDeque as VecDeque;

fn main() {
    let mut system = System::read_data();
    system.post_processing();

    println!("{}", system);

    let mut test_input: String = "hackintosh.local.test.local".to_string();
    remove_local_substrings(&mut test_input);
    println!("Output :'{}'", test_input);

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

    // Perform post-processing to make Strings look more appealing
    fn post_processing(&mut self) -> () {
        remove_local_substrings(&mut self.hostname);
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

// Remove '.local' from Hostname
fn remove_local_substrings(input_string: &mut String) {
        // Build Vector out of string split around . character
        let hostname_raw = input_string.clone();
        let hostname_split = hostname_raw.split(".");
        let mut hostname_vec: Vec<&str> = hostname_split.collect();

        let mut remove_indexes: VecDeque<usize> = VecDeque::new();

        let mut iter_index: usize = 0;
        for element in &hostname_vec {
            if *element == "local" {
                remove_indexes.push_back(iter_index);
                println!("DEBUG: Hostname Index {} has to be removed later", iter_index);
            }
            iter_index += 1;
        }

        // Remove all the indexes that contains strings that are removed
        let mut remove_counter: usize = 0;
        while remove_indexes.is_empty() == false {
            let remove_index = remove_indexes.pop_front().unwrap();
            println!("DEBUG: Removing Index {} now", remove_index);
            hostname_vec.remove(remove_index - remove_counter);
            remove_counter += 1;
        }

        // if no elements remain, write Error
        if hostname_vec.is_empty() {
            *input_string = "ERROR".to_string();
            return;
        }

        // Rebuilding Hostname String from remaining vector elements
        *input_string = hostname_vec.pop().unwrap().to_string();

        for host_part in &hostname_vec {
            input_string.push_str(".");
            input_string.push_str(host_part);
        }
}