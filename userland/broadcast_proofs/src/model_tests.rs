// NONOS Operating System (AGPL-3.0-or-later)
//! Generated sleep, wake and firing orders: the frame always holds the
//! earliest sleeper's deadline, every CPU is woken at or before its own
//! deadline (or at once, when it slept past it), and none before it is due.

use crate::model::{World, CPUS};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) % bound
    }
}

#[test]
fn no_cpu_is_woken_late_over_generated_orders() {
    let mut r = Lcg(38);
    let mut wakes = 0;
    for run in 0..20_000 {
        let mut world = World::at([0, 1 << 32, u64::MAX - 100_000][run % 3]);
        for _ in 0..200 {
            let cpu = r.next(CPUS as u64) as usize;
            let result = match r.next(8) {
                0..=2 => {
                    // A deadline ahead, at once, already gone, or none.
                    let due = match r.next(6) {
                        0 => None,
                        1 => Some(world.now.saturating_sub(r.next(50))),
                        _ => Some(world.now.saturating_add(r.next(500))),
                    };
                    // A compare already reached asserts the level at once.
                    world.sleep(cpu, due);
                    world.run_to(world.now, 0)
                }
                3 => {
                    world.wake(cpu);
                    world.check()
                }
                _ => {
                    let until = world.now.saturating_add(r.next(300));
                    world.run_to(until, if r.next(4) == 0 { r.next(40) } else { 0 })
                }
            };
            result.unwrap_or_else(|e| panic!("run {run}: {e}"));
        }
        wakes += world.wakes;
    }
    assert!(wakes > 100_000, "the orders hardly ever reach a deadline: {wakes}");
}
