// NONOS Operating System (AGPL-3.0-or-later)
//! A model to start from, and what must hold of it after every step.

use crate::book::Book;
use crate::model::{Cpu, World, CPUS};

impl World {
    pub fn at(now: u64) -> Self {
        let cpus = [Cpu::default(); CPUS];
        Self { now, cpus, compare: None, book: Book::new(), wakes: 0 }
    }

    /// The frame holds the earliest sleeper's deadline, and no sleeper's
    /// deadline has gone by.
    pub fn check(&self) -> Result<(), String> {
        let dues = self.cpus.iter().filter_map(|c| c.asleep.and_then(|(_, due)| due));
        if self.compare != dues.clone().min() {
            return Err(format!("frame holds {:?}, earliest is {:?}", self.compare, dues.min()));
        }
        match dues.filter(|due| *due <= self.now).min() {
            Some(late) => Err(format!("a CPU due at {late} still asleep at {}", self.now)),
            None => Ok(()),
        }
    }
}
