#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use csv;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};
use zkcsv_core::Output;

risc0_zkvm::guest::entry!(main);

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct TypT {
    type1: i64,
    type2: i64,
}



fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: String = env::read();
    let input_hash = *Impl::hash_bytes(&input.as_bytes());

    // TODO: do something with the input
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());


    let headers = rdr.headers().expect("error parsing age");
    // let column_index = headers.iter().position(|header| header == column_name);
    for head in headers{
        println!("Header: {}", head);
    }

    let mut ty = TypT { 
        type1: 0,
        type2: 0,
    };

    for result in rdr.records() {
        let record = result.expect("expect result");
        ty.type1 += record[0].parse::<i64>().expect("error parsing age");
        ty.type2 += record[1].parse::<i64>().expect("error parsing age");
    }

    println!{"The first line is {}", ty.type1};
    println!{"The first line is {}", ty.type2};


    let out = Output{
        first_column_total: ty.type1,
        second_column_total: ty.type2,
        hash: input_hash
    };

    // write public output to the journal
    env::commit(&out);
}


