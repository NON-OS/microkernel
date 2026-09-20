/* Words the syscall entry stub leaves on the kernel stack before it calls the
 * handler. The stub counts its own pushes against this and refuses to
 * assemble on a mismatch; asm/syscall.rs reads the same line for Rust. */
.set SYSCALL_FRAME_WORDS, 17
