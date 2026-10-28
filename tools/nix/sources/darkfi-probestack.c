/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * AGPL-3.0-or-later
 *
 * __rust_probestack for darkfid's wasmer 6.0.1, which calls it from the
 * code it compiles (wasmer_vm::libcalls). Rust 1.97's compiler_builtins no
 * longer exports it, since rustc now probes the stack inline. This is the
 * x86_64 routine compiler_builtins shipped before, as one top-level asm
 * block, so this host build needs no assembly source file: rax holds the frame's
 * size; touch each 4 KiB page below the stack pointer, change no register
 * but r11 and the flags, and return with rsp as it was.
 */
__asm__(
    "	.text\n"
    "	.globl __rust_probestack\n"
    "	.type __rust_probestack, @function\n"
    "	.p2align 4\n"
    "__rust_probestack:\n"
    "	pushq %rbp\n"
    "	movq %rsp, %rbp\n"
    "	movq %rax, %r11\n"
    "	cmpq $0x1000, %r11\n"
    "	jna 3f\n"
    "2:\n"
    "	subq $0x1000, %rsp\n"
    "	test %rsp, 8(%rsp)\n"
    "	subq $0x1000, %r11\n"
    "	cmpq $0x1000, %r11\n"
    "	ja 2b\n"
    "3:\n"
    "	subq %r11, %rsp\n"
    "	test %rsp, 8(%rsp)\n"
    "	addq %rax, %rsp\n"
    "	leave\n"
    "	ret\n"
    "	.size __rust_probestack, . - __rust_probestack\n"
    "	.section .note.GNU-stack, \"\", @progbits\n"
);
