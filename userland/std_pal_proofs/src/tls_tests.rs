// NONOS Operating System (AGPL-3.0-or-later)
//! The thread pointer read finds the word the thread pointer names.

#[test]
fn the_thread_pointer_read_finds_the_word_it_names() {
    // Linux points the thread pointer at the C library's thread block; the
    // read must give back exactly the first word there, as on NONOS it gives
    // back the TLS table's self-pointer.
    let word0: usize;
    // SAFETY: each arm only reads the thread pointer register.
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("mov {}, qword ptr fs:[0]", out(reg) word0);
        #[cfg(target_arch = "aarch64")]
        {
            let tp: usize;
            core::arch::asm!("mrs {}, tpidr_el0", out(reg) tp);
            word0 = *(tp as *const usize);
        }
        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("ld {}, 0(tp)", out(reg) word0);
    }
    assert_eq!(crate::thread_regs::tls_table() as usize, word0);
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
#[test]
fn the_read_gives_back_a_planted_tables_self_pointer() {
    // A table laid out as init_tls lays it: slot 0 its own address, the next
    // slot something else, so a read one word off is told apart.
    let mut table = [0usize, 0x7a11_7a11];
    table[0] = table.as_ptr() as usize;
    let (saved, got): (usize, usize);
    // SAFETY: the thread pointer names the planted table only between these
    // two blocks, where nothing reads a thread-local; it is put back after.
    unsafe {
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!("mrs {s}, tpidr_el0", "msr tpidr_el0, {t}", s = out(reg) saved,
            t = in(reg) table.as_ptr());
        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("mv {s}, tp", "mv tp, {t}", s = out(reg) saved,
            t = in(reg) table.as_ptr());
        got = crate::thread_regs::tls_table() as usize;
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!("msr tpidr_el0, {}", in(reg) saved);
        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("mv tp, {}", in(reg) saved);
    }
    assert_eq!(got, table.as_ptr() as usize);
}

#[cfg(target_arch = "riscv64")]
#[test]
fn set_thread_pointer_installs_a_table_the_read_then_finds() {
    let mut table = [0usize, 0x7a11_7a11];
    table[0] = table.as_ptr() as usize;
    let (saved, got, answer): (usize, usize, i64);
    // SAFETY: as above: tp names the planted table only between the save
    // and the restore, where nothing reads a thread-local.
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) saved);
        answer = crate::thread_regs::set_thread_pointer(table.as_ptr() as u64);
        got = crate::thread_regs::tls_table() as usize;
        core::arch::asm!("mv tp, {}", in(reg) saved);
    }
    assert_eq!((answer, got), (0, table.as_ptr() as usize));
}
