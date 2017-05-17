extern crate csv;

mod util;
mod role;
mod participant;

use participant::Participant;

fn main() {
    let mut rdr = csv::Reader::from_file("./2017_coalinga_data.csv").unwrap();
    for record in rdr.decode() {
        let p: Participant = Participant::from_tuple(record.unwrap());
        println!("({}, {}): {}", p.is_judge, p.category, p.full_name);
    }
}
