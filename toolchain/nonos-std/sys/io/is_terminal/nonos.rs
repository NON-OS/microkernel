// NONOS std PAL: IsTerminal asks the kernel (MTTQ) whether this process's
// stdin, stdout or stderr reaches a terminal. The launcher that renders the
// process's output says so when it starts it, and says nothing for a stage
// whose output feeds a pipe or a file, so a program picks colour and columns
// for a person and plain bytes for a pipe, as it would on any other system.

use crate::os::fd::{AsFd, AsRawFd};

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_TTY_QUERY: i64 = tag4(b"MTTQ");

pub fn is_terminal(fd: &impl AsFd) -> bool {
    let fd = fd.as_fd().as_raw_fd();
    // Only the three standard streams can reach a terminal; files and
    // sockets sit in the descriptor table above them.
    if !(0..=2).contains(&fd) {
        return false;
    }
    let r: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_TTY_QUERY => r,
            in("rdi") fd as u64,
            out("rcx") _,
            out("r11") _,
        );
    }
    r >= 0
}
