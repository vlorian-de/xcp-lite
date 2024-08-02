//TODO: Remove
#![allow(warnings)]

//TODO Cleanup imports
use xcp_idl_generator_derive::*;
use xcp::{IdlGenerator, Field, FieldList, Struct, translate_idl_struct}; //TODO: Refactor translate_idl_struct
use xcp_idl_generator::IDL::CDR;
use std::{fs::File,io::Write};

#[derive(IdlGenerator)]
struct Measurement {
    id: u32,
}

fn write_string_to_file(filename: &str, content: &str) {
    let mut file = File::create(filename).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn main() {
    let my_measurement = Measurement { id: 1 };
    let description = Measurement::description();
    // let cdr_str = Measurement::format(IdlType::CDR);
    let idl_str = translate_idl_struct(CDR, &description);
    println!("{}", idl_str);
    write_string_to_file("./gen.txt", &idl_str);
}