// NONOS Operating System (AGPL-3.0-or-later)

//! A transmit ring: reset as `__mt76_dma_queue_reset`, two slots always
//! free (`q->queued + 1 >= q->ndesc - 1` refuses), a buffer no bigger than
//! its slot, slots given back only as far as the WFDMA's `dma_idx`, and
//! nothing trusted from a card that reads all ones.

use crate::dma::{Grant, Ring};
use crate::model::rig::{chip, regs};

const REG: u32 = 0xd4300;

pub fn grant(bytes: usize) -> Grant {
    Grant::map(1, 1, bytes).unwrap()
}

pub fn ring(n: u16, stride: usize) -> Ring {
    Ring::new(grant(n as usize * 16), grant(n as usize * stride), stride, n, REG).unwrap()
}

#[test]
fn reset_gives_the_wfdma_the_array_its_size_and_zero_indices() {
    let c = chip();
    let r = regs(&c);
    let mut q = ring(8, 64);
    c.borrow_mut().regs.insert(REG + 12, 5);
    q.reset(&r);
    let ch = c.borrow();
    assert_eq!((ch.reg(REG + 4), ch.reg(REG + 8), ch.reg(REG + 12)), (8, 0, 0));
    assert_ne!(ch.reg(REG), 0, "the descriptors' device address");
}

#[test]
fn a_ring_of_n_holds_n_minus_two_and_refuses_an_oversize_buffer() {
    let c = chip();
    let r = regs(&c);
    let mut q = ring(8, 64);
    q.reset(&r);
    assert!(q.push(&[0u8; 65]).is_none(), "longer than a slot");
    assert!(q.push(&[]).is_none(), "empty");
    for i in 0..6 {
        assert_eq!(q.push(&[1u8; 64]), Some(i));
    }
    assert!(q.push(&[1u8; 8]).is_none(), "full at n - 2");
    assert!(!q.room());
}

#[test]
fn slots_come_back_as_far_as_dma_idx_and_never_from_a_dead_card() {
    let c = chip();
    let r = regs(&c);
    let mut q = ring(8, 64);
    q.reset(&r);
    for _ in 0..6 {
        q.push(&[2u8; 16]);
    }
    c.borrow_mut().regs.insert(REG + 12, 3);
    q.reclaim(&r);
    assert_eq!(q.push(&[3u8; 4]), Some(6), "three came back");
    c.borrow_mut().regs.insert(REG + 12, 6);
    c.borrow_mut().faults.dead = true;
    q.reclaim(&r);
    c.borrow_mut().faults.dead = false;
    assert_eq!(q.push(&[4u8; 4]), Some(7));
    assert!(q.push(&[5u8; 4]).is_some(), "the dead read gave none back");
    assert!(q.push(&[6u8; 4]).is_none());
}
