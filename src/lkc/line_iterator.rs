use super::vector::Vector;

pub struct LineIterator<const B: bool, const C: usize> {
    step_options: Vec<Vector<C, i32>>,
    at: Vector<C, i32>,
    end: Vector<C, i32>,
}

impl<const B: bool, const C: usize> LineIterator<B, C> {
    pub fn new(start: Vector<C, i32>, end: Vector<C, i32>) -> Self {
        LineIterator {
            at: start,
            end,
            step_options: vec![],
        }
    }
}
impl<const B: bool, const C: usize> Iterator for LineIterator<B, C> {
    type Item = Vector<C, i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_options.is_empty() {
            if self.at == self.end {
                self.step_options.push(Vector::all(0));
                if B {
                    return Some(self.at);
                } else {
                    return None;
                }
            }

            let mut dir = self.end - self.at;
            for i in 0..C {
                if dir.values[i] > 0 {
                    dir.values[i] = 1;
                }
                if dir.values[i] < 0 {
                    dir.values[i] = -1;
                }
            }

            for i in 0..C {
                if dir.values[i] != 0 {
                    #[allow(clippy::unnecessary_to_owned)]
                    for mut opt in self.step_options.to_vec() {
                        debug_assert_eq!(opt.values[i], 0);
                        opt.values[i] = dir.values[i];
                        self.step_options.push(opt);
                    }

                    let mut values = [0; C];
                    values[i] = dir.values[i];
                    self.step_options.push(Vector::new(values));
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

            if !B && self.at == self.end {
                return None;
            }

            Some(self.at)
        } else {
            None
        }
    }
}

// pub type LineIterator2 = LineIterator<2>;
// pub type LineIterator3 = LineIterator<3>;

// impl<const C: usize> Iterator for LineIterator<C> {
//     type Item = Vector<C, i32>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.step_options.is_empty() {
//             let dir = self.end - self.at;
//             match dir.x().cmp(&0) {
//                 std::cmp::Ordering::Less => {
//                     self.step_options.push(V2::from_xy(-1, 0));
//                     match dir.y().cmp(&0) {
//                         std::cmp::Ordering::Less => {
//                             self.step_options.push(V2::from_xy(0, -1));
//                             self.step_options.push(V2::from_xy(-1, -1));
//                         }
//                         std::cmp::Ordering::Equal => (),
//                         std::cmp::Ordering::Greater => {
//                             self.step_options.push(V2::from_xy(0, 1));
//                             self.step_options.push(V2::from_xy(-1, 1));
//                         }
//                     };
//                 }
//                 std::cmp::Ordering::Equal => {
//                     match dir.y().cmp(&0) {
//                         std::cmp::Ordering::Less => self.step_options.push(V2::from_xy(0, -1)),
//                         std::cmp::Ordering::Equal => (),
//                         std::cmp::Ordering::Greater => self.step_options.push(V2::from_xy(0, 1)),
//                     };
//                 }
//                 std::cmp::Ordering::Greater => {
//                     self.step_options.push(V2::from_xy(1, 0));
//                     match dir.y().cmp(&0) {
//                         std::cmp::Ordering::Less => {
//                             self.step_options.push(V2::from_xy(0, -1));
//                             self.step_options.push(V2::from_xy(1, -1));
//                         }
//                         std::cmp::Ordering::Equal => (),
//                         std::cmp::Ordering::Greater => {
//                             self.step_options.push(V2::from_xy(0, 1));
//                             self.step_options.push(V2::from_xy(1, 1));
//                         }
//                     };
//                 }
//             }
//
//             Some(self.at)
//         } else if self.at != self.end {
//             let delta = self.end - self.at;
//             let best_step = *self
//                 .step_options
//                 .iter()
//                 .max_by_key(|step| delta.inner(**step))
//                 .unwrap();
//             self.at += best_step;
//
//             Some(self.at)
//         } else {
//             None
//         }
//     }
// }
