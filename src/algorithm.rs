use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::time::Instant;
use std::io::{BufWriter, Write, Error};
use crate::sequence_database::Database;
use crate::pseudo_sequence::PseudoSequence;

const BUFFER: usize = 1000;

pub struct Algo {
    database: Database,
    pub(crate) threshold: u32,
}

impl Algo {
    pub(crate) fn new(input_path: String, min_sup: f32) -> Algo {
        let data = Database::new(input_path);
        let abs_min = ((data.num_rows as f32) * min_sup) as u32;
        Algo { database: data, threshold: abs_min}
    }

    pub(crate) fn run(&mut self, output_path: &str) {
        self.prune_infrequent();
        //println!("{}", self);
        let mut out = BufWriter::new(File::create(output_path).expect("Unable to create file"));

        for (key, value) in &self.database.items_frequency {
            if value >= &self.threshold {
                let mut item: Vec<i32> = Vec::new();
                item.push(*key);

                Self::write_file(&item, *value, &mut out);

                let pseudo_database = self.build_projected_database(*key);
                self.recursive(pseudo_database, item, &mut out);


            }
        }

        out.flush().expect("Failed to flush buffer to file");
    }

    fn recursive(&self, projected_database: Vec<PseudoSequence>, pattern: Vec<i32>, output: &mut BufWriter<File>) {
        let item_frequency = self.find_frequent_pair(projected_database);
        for (key, value) in item_frequency {
            if value.len() >= self.threshold as usize {
                let mut new_pattern = pattern.clone();
                new_pattern.push(key);
                Self::write_file(&new_pattern, value.len() as u32, output);

                self.recursive(value, new_pattern, output);
            }
        }
    }

    fn find_frequent_pair(&self, database: Vec<PseudoSequence>) -> HashMap<i32, Vec<PseudoSequence>> {
        let mut res: HashMap<i32, Vec<PseudoSequence>> = HashMap::new();

        for sequence in &database {
            let id = sequence.seq_id;
            let start = sequence.seq_index + 1;

            let original_sequence = self.database.sequences.get(id).unwrap();

            let mut contained: HashSet<i32> = HashSet::new();
            let mut pos:usize = start;
            for ele in &original_sequence.0[start..original_sequence.1] {
                if !contained.contains(ele) {
                    let new_sequence = PseudoSequence::new(id, pos);
                    res.entry(*ele).or_default().push(new_sequence);
                    contained.insert(*ele);
                }

                pos += 1;
            }
        }

        res
    }

    fn build_projected_database(&self, item: i32) -> Vec<PseudoSequence> {
        let mut res: Vec<PseudoSequence> = Vec::new();
        for (id, info) in self.database.sequences.iter().enumerate() {
            let sequence = info.0;
            let index = sequence.iter().position(|&r| r == item);
            match index {
                None => { continue }
                Some(idx) => {
                    let pseudo_sequence = PseudoSequence::new(id, idx);
                    res.push(pseudo_sequence);
                }
            }
        }

        res
    }

    pub(crate) fn prune_infrequent(&mut self) {
        let mut res: Vec<([i32; BUFFER], usize)> = Vec::new();
        for info in &self.database.sequences {
            let mut array: [i32; BUFFER] = [0; BUFFER];
            let items_frequency: &HashMap<i32, u32> = self.get_frequent_items();

            let mut cur_pos = 0;

            for ele in &info.0[0..info.1] {
                if *ele > 0 {
                    let is_frequent: bool = match items_frequency.get(&ele) {
                        None => { false },
                        Some(frequency) => {
                            if frequency >= &self.threshold {
                                true
                            } else { false }
                        }
                    };

                    if is_frequent {
                        array[cur_pos] = *ele;
                        cur_pos += 1;
                    }
                }
            }

            res.push((array, cur_pos));
        }

        res.retain(|vec| vec.1 != 0);
        self.database.sequences = res;
    }

    pub fn get_frequent_items(&self) -> &HashMap<i32, u32> {
        &self.database.items_frequency
    }

    pub fn write_file(items: &Vec<i32>, sup: u32, output: &mut BufWriter<File>) -> Result<(), Error>  {
        let pat_str: Vec<String> = items.iter().map(|&id| id.to_string()).collect();
        writeln!(output, "{} -2 #SUP: {}", pat_str.join(" -1 "), sup)?;
        Ok(())
    }
}

impl fmt::Display for Algo {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result{
        println!("The threshold is ({})", self.threshold);
        println!("{}", self.database);
        Ok(())
    }
}