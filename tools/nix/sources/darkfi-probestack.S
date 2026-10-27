/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * AGPL-3.0-or-later
 *
 * __rust_probestack for darkfid's wasmer 6.0.1, which calls it from the
 * code it compiles (wasmer_vm::libcalls). Rust 1.97's compiler_builtins no
 * longer exports it, since rustc now probes the stack inline. This is the
 * x86_64 routine compiler_builtins shipped before: rax holds the frame's
 * size; touch each 4 KiB page below the stack pointer, change no register
 * but r11 and the flags, and return with rsp as it was.
 */
	.text
	.globl __rust_probestack
	.type __rust_probestack, @function
	.p2align 4
__rust_probestack:
	pushq %rbp
	movq %rsp, %rbp
	movq %rax, %r11
	cmpq $0x1000, %r11
	jna 3f
2:
	subq $0x1000, %rsp
	test %rsp, 8(%rsp)
	subq $0x1000, %r11
	cmpq $0x1000, %r11
	ja 2b
3:
	subq %r11, %rsp
	test %rsp, 8(%rsp)
	addq %rax, %rsp
	leave
	ret
	.size __rust_probestack, . - __rust_probestack
	.section .note.GNU-stack, "", @progbits
