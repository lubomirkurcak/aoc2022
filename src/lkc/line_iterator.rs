use super::v2::{V2i32, V2};

pub struct LineIterator {
    step_options: Vec<V2i32>,
    at: V2i32,
    end: V2i32,
}

impl LineIterator {
    pub fn new(start: V2i32, end: V2i32) -> Self {
        LineIterator {
            at: start,
            end,
            step_options: vec![],
        }
    }
}

impl Iterator for LineIterator {
    type Item = V2i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_options.is_empty() {
            let dir = self.end - self.at;
            match dir.x.cmp(&0) {
                std::cmp::Ordering::Less => {
                    self.step_options.push(V2::new(-1, 0));
                    match dir.y.cmp(&0) {
                        std::cmp::Ordering::Less => {
                            self.step_options.push(V2::new(0, -1));
                            self.step_options.push(V2::new(-1, -1));
                        }
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => {
                            self.step_options.push(V2::new(0, 1));
                            self.step_options.push(V2::new(-1, 1));
                        }
                    };
                }
                std::cmp::Ordering::Equal => {
                    match dir.y.cmp(&0) {
                        std::cmp::Ordering::Less => self.step_options.push(V2::new(0, -1)),
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => self.step_options.push(V2::new(0, 1)),
                    };
                }
                std::cmp::Ordering::Greater => {
                    self.step_options.push(V2::new(1, 0));
                    match dir.y.cmp(&0) {
                        std::cmp::Ordering::Less => {
                            self.step_options.push(V2::new(0, -1));
                            self.step_options.push(V2::new(1, -1));
                        }
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => {
                            self.step_options.push(V2::new(0, 1));
                            self.step_options.push(V2::new(1, 1));
                        }
                    };
                }
            }

            Some(self.at)
        } else if self.at != self.end {
            let delta = self.end - self.at;
            let best_step = *self
                .step_options
                .iter()
                .max_by_key(|step| delta.inner(**step))
                .unwrap();
            self.at += best_step;

            Some(self.at)
        } else {
            None
        }
    }
}
