mod sequence_database;
mod algorithm;
mod pseudo_sequence;

use std::time::Instant;
use crate::algorithm::Algo;
use crate::sequence_database::Database;

fn main() {
    //println!("Hello, world!");

    //let database: Database = Database::new("weighted.txt".to_owned());
    //println!("{}", database);

    //for (key, value) in database.items_frequency {
        //println!("{} : {}", key, value);
    //}

    let now = Instant::now();
    let mut algo: Algo = Algo::new("webview.txt".to_owned(), 0.01);
    println!("{}", algo.threshold);

    algo.run("output.txt");
    let late = now.elapsed();
    println!("Runtime is: {:.2?}", late);



}
