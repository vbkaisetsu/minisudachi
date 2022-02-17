mod dictionary;
mod errors;
mod lattice;
mod matrix;
mod predictor;
mod utils;

use std::fs::File;
use std::io::BufReader;

use crate::dictionary::Dictionary;
use crate::lattice::Lattice;
use crate::matrix::ConnectionMatrix;
use crate::predictor::Predictor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load matrix
    /*
    let f = File::open("matrix.def")?;
    let matrix = ConnectionMatrix::load_from_def(f);

    let f = File::create("matrix.bin")?;
    let mut bufwriter = BufWriter::new(f);

    matrix.serialize(&mut bufwriter)?;
    */

    let f = File::open("matrix.bin")?;
    let bufreader = BufReader::new(f);
    let matrix = ConnectionMatrix::deserialize(bufreader).unwrap();

    let f = File::open("small_lex.csv")?;
    let bufreader = BufReader::new(f);
    let dict = Dictionary::load_from_csv(bufreader)?;

    let predictor = Predictor::new(dict, matrix);

    let lattice = Lattice::new("すもももももももものうち");
    let lattice = predictor.predict(lattice);

    println!("{:?}", lattice.best_path());

    Ok(())
}
