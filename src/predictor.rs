use daachorse::DoubleArrayAhoCorasick;

use crate::dictionary::{Dictionary, WordEntry};
use crate::lattice::{Edge, Lattice};
use crate::matrix::ConnectionMatrix;

pub struct Predictor {
    pma: DoubleArrayAhoCorasick,
    entries: Vec<WordEntry>,
    matrix: ConnectionMatrix,
}

impl Predictor {
    pub fn new(dict: Dictionary, matrix: ConnectionMatrix) -> Self {
        Self {
            pma: DoubleArrayAhoCorasick::with_values(dict.patterns).unwrap(),
            entries: dict.entries,
            matrix,
        }
    }

    pub fn predict(&self, lattice: Lattice) -> Lattice {
        let Lattice {
            text,
            str_to_char_pos,
            char_to_str_pos,
            mut edges,
        } = lattice;
        for m in self.pma.find_overlapping_iter(&text) {
            for (i, e) in self.entries[m.value()..].iter().enumerate() {
                let start = str_to_char_pos[m.start()];
                let end = str_to_char_pos[m.end()];
                if start == 0 {
                    edges[end - 1].push(Edge {
                        start,
                        entry_id: m.value() + i,
                        score: e.score as i32,
                        best_prev: 0,
                        right_id: e.right,
                    });
                } else {
                    let mut min_score = i32::MAX;
                    let mut best_prev = 0;
                    for (i, edge) in edges[start - 1].iter().enumerate() {
                        let score = edge.score + self.matrix.score(e.left, edge.right_id) as i32;
                        if score < min_score {
                            min_score = score;
                            best_prev = i;
                        }
                    }
                    if min_score == i32::MAX {
                        min_score = 0;
                    }
                    edges[end - 1].push(Edge {
                        start,
                        entry_id: m.value() + i,
                        score: e.score as i32 + min_score,
                        best_prev,
                        right_id: e.right,
                    });
                }
                if e.is_end {
                    break;
                }
            }
        }
        Lattice {
            text,
            str_to_char_pos,
            char_to_str_pos,
            edges,
        }
    }
}
