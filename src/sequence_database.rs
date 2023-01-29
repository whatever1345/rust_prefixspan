use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::SplitWhitespace;

const BUFFER: usize = 1000;

pub struct Database {
    pub(crate) sequences: Vec<([i32; BUFFER], usize)>,
    pub(crate) num_rows: usize,

    pub(crate) items_frequency: HashMap<i32, u32>,
}

pub(crate) fn read_file(input_path: String) -> (Vec<([i32; BUFFER], usize)>, HashMap<i32, u32>){
    let mut map: HashMap<i32, u32> = HashMap::new();
    let mut res: Vec<([i32; BUFFER], usize)> = Vec::new();

    let file = File::open(input_path);
    let greeting_file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    let reader = BufReader::new(greeting_file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let tokens: SplitWhitespace = line.split_whitespace();
                let mut array: [i32; BUFFER] = [0; BUFFER];
                let mut len: usize = 0;
                let mut set: HashSet<i32> = HashSet::new();
                for token in tokens {
                    let _number = match token.parse::<i32>() {
                        Ok(number) => {
                            if number != -1 && number != -2 {
                                array[len] = number;
                                if !set.contains(&number){
                                    *map.entry(number).or_default() += 1;
                                    set.insert(number);
                                }
                            }
                        },
                        Err(e) => println!("Error: {}", e),
                    };
                    len += 1;
                }

                res.push((array, len));
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    (res, map)
}

impl Database {
    pub(crate) fn new(input_path: String) -> Database {
        let info = read_file(input_path);
        let num = info.0.len();
        let map = info.1;
        Database { sequences: info.0, num_rows: num, items_frequency: map}
    }
}

impl fmt::Display for Database {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        println!("Number of rows ({})", self.num_rows);
        println!();
        for (arr, len) in &self.sequences {
            for i in &arr[0..*len] {
                print!("{} ", i);
            }
            println!();
        }
        Ok(())
    }
}