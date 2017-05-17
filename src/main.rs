extern crate csv;
#[macro_use]
extern crate clap;
extern crate contest_allocator;

use std::collections::HashMap;

use clap::{App, Arg};
use contest_allocator::participant::Participant;
use contest_allocator::role::Role;

fn main() {
    // Prepare the data structures to perform allocation to
    let mut participants: Vec<Participant> = Vec::new();
    
    // Parse command line args
    let args = App::new("Contest Allocator")
        .version(crate_version!())
        .author("Brennon York <brennon.york@gmail.com>")
        .about("Given a CSV input of contestants for an aerobatic contest, will slot
       them into the best respective volunteer roles")
        .arg(Arg::with_name("INPUT")
             .help("Sets the CSV file to use")
             .required(true)
             .index(1))
        .get_matches();

    // Determine CSV file passed in from the CLI
    let csv_file = args.value_of("INPUT").unwrap();

    // Read in data from the CSV file
    let rdr = csv::Reader::from_file(csv_file).unwrap();

    // Print each record of the CSV file
    for record in rdr.has_headers(false).decode() {
        // Decode each raw tuple into a Participant struct
        let p: Participant = Participant::from_tuple(record.unwrap());
        println!("({}, {}): {}", p.is_judge, p.category, p.full_name);
        // Add each participant into the Vector v
        participants.push(p);
    }
}
