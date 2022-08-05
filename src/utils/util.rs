use std::fs::File;
use std::io::{
    Write,
    Read
};

use crate::utils::error::NesError;

pub fn path_to_name(path: &str) -> &str {
    let dir_vec: Vec<&str> = path.split("/").collect();
    let name = dir_vec[dir_vec.len() - 1];
    let name_vec: Vec<&str> = name.split(".").collect();

    name_vec[0]
}

pub fn write_file(path: &str, data: &[u8]) {
    match File::create(path) {
        Ok(mut file) => file.write_all(data)
            .expect("Unable to write"),
        Err(_) => panic!("{}", NesError::FileInvalid)
    };
}

pub fn read_file(path: &String) -> Vec<u8> {
    let f = File::open(path);
        
    match f {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();

            match file.read_to_end(&mut data) {
                Ok(_) => data,
                Err(_) => panic!("{}", NesError::FileInvalid)
            }
        },
        Err(_) => panic!("{}", NesError::FileNotFound)
    }
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
