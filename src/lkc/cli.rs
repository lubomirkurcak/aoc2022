use std::fmt::Display;

pub struct Progress<T> {
    calls: usize,
    max: T,
    start_time: std::time::Instant,
    last_print_time: std::time::Instant,
}

pub struct ProgressBar {
    pub progress: f32,
}

impl Display for ProgressBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_width = 20;
        let current = if self.progress >= 1.0 {
            total_width
        } else if self.progress <= 0.0 {
            0
        } else {
            (total_width as f32 * self.progress).round() as usize
        };
        let full = "#";
        let empty = " ";
        write!(
            f,
            "[{}{}]",
            full.repeat(current),
            empty.repeat(total_width - current)
        )
    }
}

impl ProgressBar {
    pub fn new(progress: f32) -> Self {
        Self { progress }
    }
}

pub struct Duration {
    pub seconds: f64,
}

impl Duration {
    pub fn new(seconds: f64) -> Self {
        Self { seconds }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.seconds;
        if s > 60.0 {
            s /= 60.0;
            if s > 60.0 {
                s /= 60.0;
                if s > 24.0 {
                    s /= 24.0;
                    if s > 365.0 {
                        s /= 365.242196;
                        write!(f, "{} years", s)
                    } else if s > 50.0 {
                        s /= 30.4368496667;
                        write!(f, "{} months", s)
                    } else if s > 14.0 {
                        s /= 7.0;
                        write!(f, "{} weeks", s)
                    } else {
                        write!(f, "{} days", s)
                    }
                } else {
                    write!(f, "{} hours", s)
                }
            } else {
                write!(f, "{} minutes", s)
            }
        } else {
            write!(f, "{} seconds", s)
        }
    }
}

impl Progress<usize> {
    pub fn new(max: usize) -> Self {
        Progress {
            calls: 0,
            max,
            start_time: std::time::Instant::now(),
            last_print_time: std::time::Instant::now(),
        }
    }

    pub fn progress(&mut self, current: usize) {
        self.calls += 1;
        if self.last_print_time.elapsed() >= std::time::Duration::new(10, 0) {
            self.last_print_time = std::time::Instant::now();

            let elapsed = self.start_time.elapsed().as_secs_f64();
            let progress = current as f64 / self.max as f64;
            let total = elapsed / progress;
            let time_left = total - elapsed;

            println!(
                "{} Calls since last print: {}  Elapsed: {}  Progress: {}%  ETA: {}",
                ProgressBar::new(progress as f32),
                self.calls,
                Duration::new(elapsed),
                100.0 * progress,
                Duration::new(time_left)
            );

            self.calls = 0;
        }
    }
}
