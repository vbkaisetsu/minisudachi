use std::collections::HashMap;
use std::io::Read;

use crate::errors::Result;

pub struct WordEntry {
    pub left: u16,
    pub right: u16,
    pub score: i16,
    pub is_end: bool,
}

pub struct Dictionary {
    pub patterns: Vec<(String, u32)>,
    pub entries: Vec<WordEntry>,
}

impl Dictionary {
    pub fn load_from_csv<R>(rdr: R) -> Result<Self>
    where
        R: Read,
    {
        let mut csv_rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(rdr);
        let mut dict = HashMap::new();
        for result in csv_rdr.records() {
            let record = result?;
            let left: i16 = record[1].parse()?;
            let right: i16 = record[2].parse()?;
            if left < 0 || right < 0 {
                continue;
            }
            let score = record[3].parse()?;
            dict.entry(record[0].to_string())
                .or_insert_with(|| vec![])
                .push(WordEntry {
                    left: left as u16,
                    right: right as u16,
                    score,
                    is_end: false,
                });
        }
        let mut patterns = vec![];
        let mut entries = vec![];
        for (word, mut es) in dict {
            es.last_mut().unwrap().is_end = true;
            patterns.push((word, entries.len() as u32));
            entries.append(&mut es);
        }
        Ok(Self { patterns, entries })
    }
}
