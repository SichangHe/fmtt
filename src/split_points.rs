use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct SplitPoints {
    pub sub_start: SplitPoint,
    pub end: SplitPoint,
    pub sub_end: SplitPoint,
    pub connection_word: SplitPoint,
}

impl SplitPoints {
    /// Register chosen a split point with `n_char_after` characters after it.
    fn register_n_char_after(&mut self, n_char_after: usize) {
        for part in self.parts_ordered_mut() {
            if part.n_char_after >= n_char_after {
                // This split point was before the chosen one,
                // so it is now invalid.
                part.index = 0;
            } else {
                // This split point was after the chosen one,
                // so it is unaffected.
            }
        }
    }

    /// Signal that `reduction` number of splits have been consumed.
    fn reduce_index(&mut self, reduction: usize) {
        for part in self.parts_ordered_mut() {
            part.index = part.index.saturating_sub(reduction);
        }
    }

    pub fn parts_ordered_mut(&mut self) -> [&mut SplitPoint; 4] {
        [
            &mut self.end,
            &mut self.sub_end,
            &mut self.sub_start,
            &mut self.connection_word,
        ]
    }

    pub fn register_split(&mut self, split: &str, split_len: usize, n_split: usize) {
        for part in self.parts_ordered_mut() {
            part.n_char_after += split_len;
        }
        match word_sentence_position(split) {
            SentencePosition::End => {
                self.end.index = n_split;
                self.end.n_char_after = 0;
            }
            SentencePosition::SubEnd => {
                self.sub_end.index = n_split;
                self.sub_end.n_char_after = 0;
            }
            SentencePosition::SubStart => {
                self.sub_start.index = n_split.saturating_sub(1);
                self.sub_start.n_char_after = split_len;
            }
            SentencePosition::ConnectionWord => {
                self.connection_word.index = n_split;
                self.connection_word.n_char_after = 0;
            }
            SentencePosition::Other => {}
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }
}

impl Iterator for SplitPoints {
    type Item = SplitPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_best_split_point = self
            .parts_ordered_mut()
            .into_iter()
            .filter(|split_point| split_point.index > 0)
            .map(|split_point| *split_point)
            .next();
        maybe_best_split_point.map(
            |split_point @ SplitPoint {
                 index,
                 n_char_after,
             }| {
                self.reduce_index(index);
                self.register_n_char_after(n_char_after);
                split_point
            },
        )
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SplitPoint {
    pub index: usize,
    pub n_char_after: usize,
}
