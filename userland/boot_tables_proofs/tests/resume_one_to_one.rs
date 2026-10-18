// NONOS Operating System (AGPL-3.0-or-later)
//! The resume check over the boot tables the kernel builds: the image's
//! text runs one to one, and each way a range can fail is told apart:
//! read-only data is not executable, the high alias is not one to one, and
//! an address below RAM is not mapped.

use boot_tables_proofs::probe::{self, executable_one_to_one, KERNEL_SPACE_START, TEXT};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

fn board(ram: (u64, u64), uart: u64, dist: u64) -> BootInfo {
    BootInfo {
        regions: vec![MemoryRegion { base: ram.0, size: ram.1, region_type: MemoryType::Available }],
        uart: Window { base: uart, size: 0x1000 },
        gic_dist_base: dist,
        ..BootInfo::default()
    }
}

fn check(info: &BootInfo) {
    probe::build(info);
    let (text, end) = TEXT;
    assert_eq!(executable_one_to_one(text, end), Ok(()));
    for at in (text..end).step_by(0x1_1000) {
        assert_eq!(executable_one_to_one(at, at + 0x180), Ok(()), "{at:#x}");
    }
    assert_eq!(executable_one_to_one(text, text), Ok(()), "an empty range");
    let not_x: &[u8] = b"mapped but not executable at EL1";
    assert_eq!(executable_one_to_one(end, end + 0x10), Err(not_x));
    assert_eq!(executable_one_to_one(end - 0x10, end + 0x10), Err(not_x));
    let high = KERNEL_SPACE_START + text;
    assert_eq!(executable_one_to_one(high, high + 0x10), Err(&b"mapped, but not one to one"[..]));
    assert_eq!(executable_one_to_one(0x1000, 0x1010), Err(&b"not mapped"[..]));
}

#[test]
fn qemu_virt_text_runs_one_to_one() {
    check(&board((0x4000_0000, 0x4000_0000), 0x0900_0000, 0x0800_0000));
}
