use std::fs::File;
use std::io::Write;

use crate::utils::error::NesError;

pub fn path_to_name(path: &str) -> &str {
    let dir_vec: Vec<&str> = path.split("/").collect();
    let name = dir_vec[dir_vec.len() - 1];
    let name_vec: Vec<&str> = name.split(".").collect();

    name_vec[0]
}

pub fn create_and_write_file(path: &str, data: &[u8]) {
    match File::create(path) {
        Ok(mut file) => file.write_all(data)
            .expect("Unable to write"),
        Err(_) => panic!("{}", NesError::FileInvalid)
    };
}

pub fn join_bytes(vec: &[u8], sep: &str) -> String {
    let ret: Vec<String> = vec
        .iter()
        .map(|value| format!("{:02x?}", value))
        .collect();

    ret.join(sep)
}

pub fn unwrap_str<'a>(
    v: &'a Option<String>,
    prefix: &'a str,
    suffix: &'a str
) -> String {
    match v {
        Some(string) => format!(
            "{}{}{}",
            prefix,
            string,
            suffix
        ),
        None => String::from("")
    }
}

pub fn u16_from_mem(upper: u8, lower: u8) -> u16 {

    (lower as u16) << 8 | upper as u16
}

pub fn vec_bytes_to_string(vec: &Vec<u8>) -> String {
    let mut ret = Vec::<String>::new();

    for byte in vec {
        ret.push(format!("{:02x?}", byte));
    }

    ret.join(" ")
}
