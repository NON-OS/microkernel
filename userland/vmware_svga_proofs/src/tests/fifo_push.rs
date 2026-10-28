// NONOS Operating System (AGPL-3.0-or-later)
//! NEXT_CMD lives in memory the host writes: push checks it on every word,
//! and a value out of the ring, below it or not word aligned, is an error,
//! never a write past the FIFO or a panic.

use crate::constants::fifo::FIFO_NEXT_CMD;
use crate::fifo::Fifo;

const WORDS: usize = 1024;
const MIN: u32 = 64;
const MAX: u32 = 4096;

fn ring(next: u32) -> (Vec<u32>, Fifo) {
    let mut mem = vec![0u32; WORDS + 8];
    mem[FIFO_NEXT_CMD] = next;
    let mut fifo = Fifo::new(mem.as_mut_ptr() as u64, (WORDS * 4) as u64);
    (fifo.min, fifo.max) = (MIN, MAX);
    (mem, fifo)
}

#[test]
fn a_command_goes_in_behind_next_cmd_and_moves_it() {
    let (mem, fifo) = ring(MAX - 4);
    assert_eq!(fifo.push(&[7, 8]), Ok(()));
    assert_eq!((mem[(MAX as usize - 4) / 4], mem[MIN as usize / 4]), (7, 8), "wrapped to MIN");
    assert_eq!(mem[FIFO_NEXT_CMD], MIN + 4);
}

#[test]
fn a_host_written_next_cmd_outside_the_ring_is_refused() {
    for next in [MAX, MAX + 4, MIN - 4, MIN + 1, u32::MAX - 3, u32::MAX] {
        let (mem, fifo) = ring(next);
        let before = mem.clone();
        assert!(fifo.push(&[0xDEAD]).is_err(), "NEXT_CMD {next:#x} taken");
        assert_eq!(mem, before, "nothing written for {next:#x}");
    }
}
