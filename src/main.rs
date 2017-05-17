extern crate csv;
#[macro_use]
extern crate clap;
extern crate contest_allocator;

use clap::App;
use contest_allocator::participant::Participant;

fn main() {
    // Parse command line args from YAML config file
    let yml = load_yaml!("cli.yml");
    let args = App::from_yaml(yml).get_matches();

    // Determine CSV file passed in from the CLI
    let csv_file = args.value_of("INPUT").unwrap();
    
    // Read in data from the CSV file
    let rdr = csv::Reader::from_file(csv_file).unwrap();

    // Print each record of the CSV file
    for record in rdr.has_headers(false).decode() {
        // Decode each raw tuple into a Participant struct
        let p: Participant = Participant::from_tuple(record.unwrap());
        println!("({}, {}): {}", p.is_judge, p.category, p.full_name);
    }
}
