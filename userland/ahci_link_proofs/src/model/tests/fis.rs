// NONOS Operating System (AGPL-3.0-or-later)
//! Slot 0 as the HBA fetches it, byte by byte, written out from the specs
//! rather than the driver's structs: the command header (AHCI 1.3.1,
//! 4.2.2), the Register H2D FIS (SATA 3.x, 10.5.5) carrying READ DMA EXT,
//! WRITE DMA EXT and FLUSH CACHE EXT (ACS-3: 25h, 35h, EAh) with LBA48, and
//! the one PRD entry (AHCI 1.3.1, 4.2.3).

use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::model::Rig;

/// Every byte of the 48-bit LBA differs, so a byte out of place shows.
const LBA: u64 = 0x8a9b_acbd_cedf;

fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |a, &x| a << 8 | u64::from(x))
}

#[test]
fn read_and_write_dma_ext_are_laid_out_as_the_specs_say() {
    for (write, op) in [(false, 0x25u8), (true, 0x35)] {
        for sectors in [1u32, 7, 64] {
            let mut rig = Rig::new(Vec::new(), true);
            rig.port.capacity_sectors = 1 << 48;
            let regs = rig.regs();
            assert_eq!(transfer(&mut rig.port, regs, LBA, sectors, write), Ok(()));
            let (t, h, (table, data)) = (rig.table(), rig.header(), rig.table_and_data_addr());
            let (count, at) =
                ([sectors as u8, (sectors >> 8) as u8], format!("{op:#x} x{sectors}"));
            let lba = [0xdf, 0xce, 0xbd, 0x40, 0xac, 0x9b, 0x8a, 0];
            let fis = [&[0x27, 0x80, op, 0][..], &lba, &count, &[0; 6]].concat();
            assert_eq!(t[..20], fis[..], "{at}: FIS");
            assert!(t[20..0x80].iter().all(|&b| b == 0), "{at}: table past the FIS");
            // CFL 5 dwords, W on a write; one PRD entry; the table's address.
            assert_eq!(h[0], 5 | if write { 0x40 } else { 0 }, "{at}: CFL and W");
            assert_eq!((h[1], le(&h[2..4])), (0, 1), "{at}: PMP and PRDTL");
            assert_eq!(le(&h[8..16]), table, "{at}: CTBA");
            // DBA, a reserved dword, then DBC: bytes less one, I clear.
            let bytes = u64::from(sectors) * 512;
            assert_eq!(le(&t[0x80..0x88]), data, "{at}: DBA");
            assert_eq!((le(&t[0x88..0x8c]), le(&t[0x8c..0x90])), (0, bytes - 1), "{at}: DBC");
            assert_eq!(le(&h[4..8]), bytes, "{at}: PRDBC after the command");
        }
    }
}

#[test]
fn flush_cache_ext_carries_no_data() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    assert_eq!(flush(&mut rig.port, regs), Ok(()));
    let (t, h) = (rig.table(), rig.header());
    let fis = [0x27, 0x80, 0xea, 0, 0, 0, 0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(t[..20], fis[..]);
    assert_eq!((h[0], h[1], le(&h[2..4])), (5, 0, 0), "CFL 5, no W, no PRD");
    assert!(t[20..0x90].iter().all(|&b| b == 0), "a PRD entry left in the table");
}
