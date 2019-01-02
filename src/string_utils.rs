use std::collections::VecDeque as VecDeque;
use colored::*;

// Remove '.local' from Hostname
pub fn remove_local_substrings(input_string: &mut String) {
        // Build Vector out of string split around . character
        let hostname_raw = input_string.clone();
        let hostname_split = hostname_raw.split(".");
        let mut hostname_vec: VecDeque<&str> = hostname_split.collect();

        let mut remove_indexes: VecDeque<usize> = VecDeque::new();

        let mut iter_index: usize = 0;
        for element in &hostname_vec {
            if *element == "local" {
                remove_indexes.push_back(iter_index);
            }
            iter_index += 1;
        }

        // Remove all the indexes that contains strings that are removed
        let mut remove_counter: usize = 0;
        while remove_indexes.is_empty() == false {
            let remove_index = remove_indexes.pop_front().unwrap();
            hostname_vec.remove(remove_index - remove_counter);
            remove_counter += 1;
        }

        // if no elements remain, write Error
        if hostname_vec.is_empty() {
            *input_string = "ERROR".to_string();
            return;
        }

        // Rebuilding Hostname String from remaining vector elements
        *input_string = hostname_vec.pop_front().unwrap().to_string();

        for host_part in &hostname_vec {
            input_string.push_str(".");
            input_string.push_str(host_part);
        }
}

pub fn get_os_logo(is_mac: bool) -> Vec<String>{

    if is_mac {
        let mut apple_logo_lines: Vec<String> = Vec::new();

        apple_logo_lines.push("                 ###                  ".yellow().to_string());
        apple_logo_lines.push("               ####                   ".yellow().to_string());
        apple_logo_lines.push("               ###                    ".yellow().to_string());
        apple_logo_lines.push("       #######    #######             ".yellow().to_string());
        apple_logo_lines.push("     ######################           ".yellow().to_string());
        apple_logo_lines.push("    #####################             ".yellow().to_string());
        apple_logo_lines.push("    ####################              ".yellow().to_string());
        apple_logo_lines.push("    ####################              ".yellow().to_string());
        apple_logo_lines.push("    #####################             ".yellow().to_string());
        apple_logo_lines.push("     ######################           ".yellow().to_string());
        apple_logo_lines.push("      ####################            ".yellow().to_string());
        apple_logo_lines.push("        ################              ".yellow().to_string());
        apple_logo_lines.push("         ####     #####               ".yellow().to_string());

        apple_logo_lines

    } else {
        let mut linux_logo_lines: Vec<String> = Vec::new();

        linux_logo_lines.push(" ####                                 ".yellow().to_string());
        linux_logo_lines.push(" ####                                 ".yellow().to_string());
        linux_logo_lines.push(" ####                                 ".yellow().to_string());
        linux_logo_lines.push(" ####                     ###         ".yellow().to_string());
        linux_logo_lines.push(" ####                     ###         ".yellow().to_string());
        linux_logo_lines.push(" ####                                 ".yellow().to_string());
        linux_logo_lines.push(" ####                     ###         ".yellow().to_string());
        linux_logo_lines.push(" ####                     ###         ".yellow().to_string());
        linux_logo_lines.push(" ####                     ###         ".yellow().to_string());
        linux_logo_lines.push(" ####                     ###         ".yellow().to_string());
        linux_logo_lines.push(" #####                    ###         ".yellow().to_string());
        linux_logo_lines.push(" #####################    ###         ".yellow().to_string());
        linux_logo_lines.push(" ######################   ###         ".yellow().to_string());

        linux_logo_lines
    }
}

#[test]
pub fn test_remove_local_substrings() {

    let mut test_input_1: String = "hackintosh.local.test.local".to_string();
    let test_output_1: String = "hackintosh.test".to_string();
    remove_local_substrings(&mut test_input_1);
    assert_eq!(test_input_1, test_output_1);

    let mut test_input_2: String = "local.hackintosh.local.test.local".to_string();
    let test_output_2: String = "hackintosh.test".to_string();
    remove_local_substrings(&mut test_input_2);
    assert_eq!(test_input_2, test_output_2);

    let mut test_input_3: String = "hackintosh.local.local".to_string();
    let test_output_3: String = "hackintosh".to_string();
    remove_local_substrings(&mut test_input_3);
    assert_eq!(test_input_3, test_output_3);

    let mut test_input_4: String = "hackintosh".to_string();
    let test_output_4: String = "hackintosh".to_string();
    remove_local_substrings(&mut test_input_4);
    assert_eq!(test_input_4, test_output_4);

    let mut test_input_5: String = "local".to_string();
    let test_output_5: String = "ERROR".to_string();
    remove_local_substrings(&mut test_input_5);
    assert_eq!(test_input_5, test_output_5);
}

#[test]
pub fn test_remove_local_substrings_none() {

    let mut test_input_4: String = "hackintosh".to_string();
    let test_output_4: String = "hackintosh".to_string();
    remove_local_substrings(&mut test_input_4);
    assert_eq!(test_input_4, test_output_4);
}

#[test]
pub fn test_remove_local_substrings_error() {

    let mut test_input_5: String = "local".to_string();
    let test_output_5: String = "ERROR".to_string();
    remove_local_substrings(&mut test_input_5);
    assert_eq!(test_input_5, test_output_5);
}