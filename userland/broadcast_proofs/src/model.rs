// NONOS Operating System (AGPL-3.0-or-later)
//! The book driven as the kernel drives it, beside a timer frame modelled as
//! the hardware is: a level interrupt asserted whenever the compare it holds
//! is at or before the count. Each CPU remembers when it slept and its
//! deadline, so a wake can be judged late against them.

use crate::book::Book;

pub const CPUS: usize = 6;

#[derive(Clone, Copy, Default)]
pub(crate) struct Cpu {
    /// When it slept and the count it must be awake by, while asleep.
    pub(crate) asleep: Option<(u64, Option<u64>)>,
}

pub struct World {
    pub now: u64,
    pub(crate) cpus: [Cpu; CPUS],
    /// What the frame holds: exactly what the book last returned.
    pub(crate) compare: Option<u64>,
    pub(crate) book: Book<CPUS>,
    pub wakes: u64,
}

impl World {
    /// `hand_off`: the CPU's deadline to the book, the book's answer to the
    /// frame. A CPU already asleep is not handed off again.
    pub fn sleep(&mut self, cpu: usize, due: Option<u64>) {
        if self.cpus[cpu].asleep.is_none() {
            self.compare = self.book.sleep(cpu, due).expect("in range");
            self.cpus[cpu].asleep = Some((self.now, due));
        }
    }

    /// `take_back`, after a wake from anything at all.
    pub fn wake(&mut self, cpu: usize) {
        self.cpus[cpu].asleep = None;
        self.compare = self.book.wake(cpu);
    }

    /// Time passes to `until`, the frame firing on the way whenever its
    /// compare is reached. The first firing reads a count `lag` early, as a
    /// carry between the halves can make it.
    pub fn run_to(&mut self, until: u64, mut lag: u64) -> Result<(), String> {
        // Each firing wakes someone, or follows a late read and is followed
        // by one that does; more than that means the frame fires for ever.
        let mut firings = 0;
        while let Some(at) = self.compare.filter(|at| *at <= until) {
            firings += 1;
            if firings > 2 * CPUS + 2 {
                return Err(format!("the frame keeps firing at {at} and wakes no one"));
            }
            self.now = self.now.max(at);
            let seen = self.now.saturating_sub(std::mem::take(&mut lag));
            let mut woken = Vec::new();
            self.compare = self.book.expire(seen, |cpu| woken.push(cpu));
            for cpu in woken {
                let (slept, due) = self.cpus[cpu].asleep.ok_or(format!("{cpu} woken awake"))?;
                let due = due.ok_or(format!("{cpu} woken with no deadline"))?;
                if due > seen {
                    return Err(format!("{cpu} woken at {seen}, not due till {due}"));
                }
                if self.now > due.max(slept) {
                    return Err(format!("{cpu} woken at {}, due {due}, slept {slept}", self.now));
                }
                self.wakes += 1;
                self.wake(cpu);
            }
        }
        self.now = self.now.max(until);
        self.check()
    }
}
