#[derive(Clone, Debug)]
pub struct Edge {
    pub start: usize,
    pub entry_id: usize,
    pub score: i32,
    pub best_prev: usize,
    pub right_id: u16,
}

pub struct Lattice {
    pub(crate) text: String,
    pub(crate) str_to_char_pos: Vec<usize>,
    pub(crate) char_to_str_pos: Vec<usize>,
    pub(crate) edges: Vec<Vec<Edge>>,
}

impl Lattice {
    pub fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        let text = text.into();
        let mut str_to_char_pos = vec![0; text.len() + 1];
        let mut char_to_str_pos = vec![0];
        let mut pos = 0;
        for (i, c) in text.chars().enumerate() {
            pos += c.len_utf8();
            str_to_char_pos[pos] = i + 1;
            char_to_str_pos.push(pos);
        }
        let edges = vec![vec![]; *str_to_char_pos.last().unwrap()];
        Self {
            text,
            str_to_char_pos,
            char_to_str_pos,
            edges,
        }
    }

    pub fn best_path(&self) -> Vec<&str> {
        let mut score = i32::MAX;
        let mut best_prev_idx = 0;
        let mut best_start = 0;
        let mut ranges = vec![];
        for edge in self.edges.last().unwrap() {
            if edge.score < score {
                score = edge.score;
                best_prev_idx = edge.best_prev;
                best_start = edge.start;
            }
        }
        ranges.push(self.edges.len());
        loop {
            ranges.push(best_start);
            let edge = &self.edges[best_start - 1][best_prev_idx];
            best_prev_idx = edge.best_prev;
            best_start = edge.start;
            if best_start == 0 {
                break;
            }
        }
        let mut start = 0;
        let mut results = vec![];
        for i in ranges.iter().rev() {
            let end = self.char_to_str_pos[*i];
            results.push(&self.text[start..end]);
            start = end;
        }
        results
    }
}
