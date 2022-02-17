use std::io::{self, Read, Write};

use crate::errors::Result;
use crate::utils;

pub struct ConnectionMatrix {
    n_left: u16,
    n_right: u16,
    data: Vec<i16>,
}

impl ConnectionMatrix {
    pub fn load_from_def<R>(rdr: R) -> Result<Self>
    where
        R: Read,
    {
        let mut matrix_rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .delimiter(b' ')
            .from_reader(rdr);
        let header = matrix_rdr.headers()?;
        let n_left = header[0].parse()?;
        let n_right = header[1].parse()?;
        let mut data = vec![0; usize::from(n_left) * usize::from(n_right)];
        for row in matrix_rdr.records() {
            let row = row?;
            let left_id: usize = row[0].parse()?;
            let right_id: usize = row[1].parse()?;
            let score = row[2].parse()?;
            data[usize::from(left_id) + usize::from(right_id) * usize::from(n_left)] = score;
        }
        Ok(Self {
            n_left,
            n_right,
            data,
        })
    }

    pub fn score(&self, left_id: u16, right_id: u16) -> i16 {
        self.data[usize::from(left_id) + usize::from(right_id) * usize::from(self.n_left)]
    }

    pub fn serialize<W>(&self, mut wtr: W) -> io::Result<()>
    where
        W: Write,
    {
        utils::write_u16(&mut wtr, self.n_left)?;
        utils::write_u16(&mut wtr, self.n_right)?;
        for &elem in &self.data {
            utils::write_i16(&mut wtr, elem)?
        }
        Ok(())
    }

    pub fn deserialize<R>(mut rdr: R) -> io::Result<Self>
    where
        R: Read,
    {
        let n_left = utils::read_u16(&mut rdr)?;
        let n_right = utils::read_u16(&mut rdr)?;
        let size = usize::from(n_left) * usize::from(n_right);
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(utils::read_i16(&mut rdr)?);
        }
        Ok(Self {
            n_left,
            n_right,
            data,
        })
    }
}
