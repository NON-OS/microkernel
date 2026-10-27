// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's rings after a boot: each free entry a page-aligned
//! buffer's address with its id in the low 12 bits, and the command ring of
//! 32 entries under an 8-bit write pointer, hundreds of commands on.

use super::rig::{polls, rig, run, QU_B, QU_B0};
use crate::gen3::plan::RX_FREE;
use crate::gen3::region::Region;
use crate::gen3::regs::HBUS_TARG_WRPTR;
use crate::gen3_model::{reply, Seen};

#[test]
fn every_free_entry_is_a_page_aligned_buffer_and_its_id() {
    let r = rig(QU_B, QU_B0, false);
    run(&r, &mut polls(), |_, _, _| ()).0.expect("alive");
    for i in 0..64u64 {
        let mut e = [0u8; 8];
        r.model.ctrl.read(RX_FREE + i as usize * 8, &mut e);
        let e = u64::from_le_bytes(e);
        assert_eq!(e & !0xFFF, r.model.rbs.dev + ((e & 0xFFF) - 1) * 4096, "entry {i}");
        assert!((1..=64).contains(&(e & 0xFFF)));
    }
}

#[test]
fn the_command_ring_is_32_entries_and_its_pointer_8_bits() {
    let r = rig(QU_B, QU_B0, false);
    r.model.s.borrow_mut().responder = Some(Box::new(|s: &Seen| vec![reply(s, s.payload.clone())]));
    let (alive, n) = run(&r, &mut polls(), |dev, c, _| {
        (0..300u32)
            .filter(|i| {
                let mut got = None;
                let ok = dev.command(
                    c,
                    (2, 0x30),
                    &i.to_le_bytes(),
                    &mut |p| got = Some(p.to_vec()),
                    &mut |_| {},
                );
                ok.is_ok() && got.as_deref() == Some(&i.to_le_bytes()[..])
            })
            .count()
    });
    alive.expect("alive");
    assert_eq!(n, Some(300), "every command answered with its own payload");
    let s = r.model.s.borrow();
    let ptrs: Vec<u32> =
        s.writes.iter().filter(|w| w.0 == HBUS_TARG_WRPTR && w.1 >> 16 == 0).map(|w| w.1).collect();
    assert_eq!(ptrs.len(), 300);
    assert!(ptrs.iter().enumerate().all(|(i, &p)| p == (i as u32 + 1) & 0xFF), "8 bits");
}
