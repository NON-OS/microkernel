// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::budget::{
    cost, sender_of, Held, INBOX_BYTES_MAX, MESSAGE_OVERHEAD, SHARE_BYTES_MAX, TOTAL_BYTES_MAX,
};

/// The kernel's largest payload (`ipc::channel::MAX_MESSAGE_SIZE`).
const MAX_MESSAGE: usize = 1 << 20;
/// The kernel heap (`KHEAP_SIZE`).
const KERNEL_HEAP: usize = 256 << 20;
const KERNEL: u32 = 0;

fn big() -> usize {
    cost(MAX_MESSAGE + 32)
}

/// Admit `cost` from `sender` until refused; how many got in.
fn fill(held: &mut Held, total: &mut usize, sender: u32, cost: usize) -> usize {
    let mut n = 0;
    while held.admit(total, sender, cost) {
        n += 1;
        assert!(n <= 1 << 20, "admission never stops");
    }
    n
}

#[test]
fn the_caps_leave_the_heap_most_of_itself_and_take_the_largest_message() {
    const { assert!(TOTAL_BYTES_MAX <= KERNEL_HEAP / 2) };
    const { assert!(INBOX_BYTES_MAX <= TOTAL_BYTES_MAX) };
    assert_eq!(SHARE_BYTES_MAX * 2, INBOX_BYTES_MAX);
    assert!(big() * 4 <= SHARE_BYTES_MAX, "a sender may queue several full messages");
    assert_eq!(cost(0), MESSAGE_OVERHEAD);
    assert_eq!(cost(usize::MAX), usize::MAX);
}

#[test]
fn one_sender_holds_at_most_half_an_inbox() {
    let (mut held, mut total) = (Held::new(), 0);
    let n = fill(&mut held, &mut total, 7, big());
    assert_eq!(n, SHARE_BYTES_MAX / big());
    assert!(held.held_by(7) <= SHARE_BYTES_MAX);
    assert!(held.held_by(7) + big() > SHARE_BYTES_MAX);
    assert_eq!(held.bytes(), total);
    // The other half is left to everyone else.
    assert!(held.admit(&mut total, 8, big()));
}

#[test]
fn an_inbox_holds_at_most_its_cap_whoever_sends() {
    let (mut held, mut total) = (Held::new(), 0);
    for sender in 1..=8 {
        fill(&mut held, &mut total, sender, big());
    }
    assert!(held.bytes() <= INBOX_BYTES_MAX);
    assert!(held.bytes() + big() > INBOX_BYTES_MAX);
    assert!(!held.admit(&mut total, 9, big()));
    assert_eq!(held.bytes(), total);
}

#[test]
fn every_inbox_together_holds_at_most_the_total() {
    let mut total = 0;
    let mut inboxes: Vec<Held> = (0..32).map(|_| Held::new()).collect();
    let mut sender = 1;
    for held in inboxes.iter_mut() {
        for _ in 0..4 {
            fill(held, &mut total, sender, big());
            sender += 1;
        }
    }
    assert!(total <= TOTAL_BYTES_MAX);
    assert!(total + big() > TOTAL_BYTES_MAX);
    assert_eq!(total, inboxes.iter().map(Held::bytes).sum::<usize>());
    // Only the total refuses now: a fresh inbox and a fresh sender still cannot get in.
    let mut fresh = Held::new();
    assert!(!fresh.admit(&mut total, 999, big()));
    assert_eq!(fresh.bytes(), 0);
}

#[test]
fn the_kernel_is_held_to_the_inbox_and_the_total_but_not_to_a_share() {
    let (mut held, mut total) = (Held::new(), 0);
    let n = fill(&mut held, &mut total, KERNEL, big());
    assert_eq!(n, INBOX_BYTES_MAX / big());
    assert!(held.bytes() > SHARE_BYTES_MAX);
    assert!(held.bytes() <= INBOX_BYTES_MAX);
    assert_eq!(held.held_by(KERNEL), 0);
}

#[test]
fn a_refusal_changes_nothing() {
    let (mut held, mut total) = (Held::new(), 0);
    fill(&mut held, &mut total, 3, big());
    let (bytes, share, before) = (held.bytes(), held.held_by(3), total);
    assert!(!held.admit(&mut total, 3, big()));
    assert!(!held.admit(&mut total, 4, usize::MAX));
    assert!(!held.admit(&mut total, KERNEL, usize::MAX));
    assert_eq!((held.bytes(), held.held_by(3), held.held_by(4), total), (bytes, share, 0, before));
}

#[test]
fn a_sum_that_would_overflow_is_refused() {
    let (mut held, mut total) = (Held::new(), TOTAL_BYTES_MAX - 1);
    assert!(!held.admit(&mut total, 1, usize::MAX - 1));
    let (mut held, mut total) = (Held::new(), usize::MAX);
    assert!(!held.admit(&mut total, KERNEL, 1));
    assert_eq!(held.bytes(), 0);
}

#[test]
fn what_leaves_is_given_back() {
    let (mut held, mut total) = (Held::new(), 0);
    let n = fill(&mut held, &mut total, 7, big());
    held.release(&mut total, 7, big());
    assert!(held.admit(&mut total, 7, big()), "a freed share takes a message again");
    for _ in 0..n {
        held.release(&mut total, 7, big());
    }
    assert_eq!((held.bytes(), held.held_by(7), total), (0, 0, 0));
    // A release past what was held stops at zero and touches no other inbox's bytes.
    let mut other = Held::new();
    assert!(other.admit(&mut total, 2, big()));
    held.release(&mut total, 7, big());
    assert_eq!((held.bytes(), total), (0, big()));
}

#[test]
fn a_cleared_or_dropped_inbox_gives_all_of_its_bytes_back() {
    let mut total = 0;
    let (mut a, mut b) = (Held::new(), Held::new());
    fill(&mut a, &mut total, 1, big());
    fill(&mut a, &mut total, KERNEL, 4096);
    fill(&mut b, &mut total, 2, big());
    let kept = b.bytes();
    a.release_all(&mut total);
    assert_eq!((a.bytes(), a.held_by(1), total), (0, 0, kept));
    assert!(a.admit(&mut total, 1, big()));
}

#[test]
fn a_client_that_never_reads_cannot_cut_a_server_off_from_the_rest() {
    const SERVER: u32 = 5;
    let mut total = 0;
    // Eight clients ask and never read: the server's replies fill each one's share.
    let mut idle: Vec<Held> = (0..8).map(|_| Held::new()).collect();
    for held in idle.iter_mut() {
        assert!(fill(held, &mut total, SERVER, big()) > 0);
    }
    let waiting: usize = idle.iter().map(|h| h.held_by(SERVER)).sum();
    assert!(waiting > 4 * SHARE_BYTES_MAX, "no cap on all a sender has waiting");
    // The next client still gets its reply.
    let mut next_client = Held::new();
    assert!(next_client.admit(&mut total, SERVER, big()));
}

#[test]
fn sender_of_reads_only_the_kernel_stamp() {
    assert_eq!(sender_of("proc.42"), 42);
    assert_eq!(sender_of("proc.4294967295"), u32::MAX);
    for kernel in ["proc.", "proc.x", "proc.-1", "proc.4294967296", "kernel.vfs", "stdin.3", ""] {
        assert_eq!(sender_of(kernel), KERNEL, "{kernel}");
    }
}

/// xorshift64, so the run is the same every time.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

#[test]
fn every_sequence_of_sends_and_receives_keeps_the_books_and_the_caps() {
    const INBOXES: usize = 12;
    const SENDERS: u32 = 6;
    for seed in 1..=64u64 {
        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let mut total = 0;
        let mut inboxes: Vec<Held> = (0..INBOXES).map(|_| Held::new()).collect();
        let mut queued: Vec<Vec<(u32, usize)>> = vec![Vec::new(); INBOXES];
        for _ in 0..4000 {
            let i = rng.below(INBOXES);
            match rng.below(10) {
                0..=5 => {
                    let sender = rng.below(SENDERS as usize + 1) as u32;
                    let size = match rng.below(4) {
                        0 => cost(MAX_MESSAGE),
                        1 => cost(rng.below(MAX_MESSAGE)),
                        2 => cost(rng.below(4096)),
                        _ => usize::MAX - rng.below(4),
                    };
                    let before = (inboxes[i].bytes(), inboxes[i].held_by(sender), total);
                    if inboxes[i].admit(&mut total, sender, size) {
                        queued[i].push((sender, size));
                    } else {
                        assert_eq!((inboxes[i].bytes(), inboxes[i].held_by(sender), total), before);
                    }
                }
                6..=8 => {
                    if !queued[i].is_empty() {
                        let at = rng.below(queued[i].len());
                        let (sender, size) = queued[i].remove(at);
                        inboxes[i].release(&mut total, sender, size);
                    }
                }
                _ => {
                    inboxes[i].release_all(&mut total);
                    queued[i].clear();
                }
            }
            assert!(total <= TOTAL_BYTES_MAX);
            assert_eq!(total, inboxes.iter().map(Held::bytes).sum::<usize>());
            for (held, msgs) in inboxes.iter().zip(&queued) {
                assert!(held.bytes() <= INBOX_BYTES_MAX);
                assert_eq!(held.bytes(), msgs.iter().map(|m| m.1).sum::<usize>());
                for sender in 1..=SENDERS {
                    let share: usize = msgs.iter().filter(|m| m.0 == sender).map(|m| m.1).sum();
                    assert_eq!(held.held_by(sender), share);
                    assert!(share <= SHARE_BYTES_MAX);
                }
            }
        }
    }
}
