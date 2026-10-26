// NONOS Operating System (AGPL-3.0-or-later)
//! The loader's set/way walk, assembled from its own source between the
//! stand-ins and the harness, and a safe call into it.

core::arch::global_asm!(
    include_str!("arch/aarch64/asm/standin.S"),
    include_str!("../../../nonos-bootloader/src/arch/aarch64/asm/walk.S"),
    include_str!("arch/aarch64/asm/harness.S"),
);

extern "C" {
    fn walk_harness(fixture: *const u64, log: *mut u64, cap: u64, overflow: *mut u64) -> u64;
}

/// The system registers one CPU would show the walk.
pub struct Fixture {
    pub clidr: u64,
    pub mmfr2: u64,
    /// CCSIDR_EL1 for levels 1..=7, index 0 being level 1.
    pub ccsidr: [u64; 7],
}

/// Every DC CISW operand the walk issues, in order.
pub fn run(f: &Fixture) -> Vec<u64> {
    let mut regs = [0u64; 9];
    regs[0] = f.clidr;
    regs[1] = f.mmfr2;
    regs[2..].copy_from_slice(&f.ccsidr);
    let mut log = vec![0u64; 1 << 21];
    let mut overflow = 0u64;
    // SAFETY: the harness reads the 9 words of `regs`, writes at most `cap`
    // words of `log` and one of `overflow`, and keeps the ABI's registers.
    let n =
        unsafe { walk_harness(regs.as_ptr(), log.as_mut_ptr(), log.len() as u64, &mut overflow) };
    assert_eq!(overflow, 0, "the walk issued more operands than the log holds");
    log.truncate(n as usize);
    log
}
