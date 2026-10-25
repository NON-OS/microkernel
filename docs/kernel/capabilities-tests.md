# Capabilities: Tests

Part of [Capabilities](capabilities.md).

## Tests

The [proof crate](../overview/glossary-peer-list.md#proof-crate) `userland/kernel_proofs`
compiles the cap table (`userland/kernel_proofs/src/syscall/contract/mod.rs`),
the predicates (`userland/kernel_proofs/src/syscall/caps/mod.rs`), the
capability list and the bit helpers
(`userland/kernel_proofs/src/capabilities/mod.rs`) from the kernel sources. It
passed, 388 tests, in the flake check run on this commit.
`scripts/check_syscall_caps.py` compares the gate each call publishes in
`abi/syscalls.toml` with the cap table; it prints nothing and exits 0 when they
agree, as it does here:

```
$ python3 scripts/check_syscall_caps.py
```

[Checking the security claims
yourself](../security/checking-the-claims-every-capability-and.md#every-capability-and-control-is-enforced)
covers three more tools that hold the capability list to the code, and what each
printed at this commit.

## See also

- [System calls](syscalls.md): the entry path that runs this check.
- [Capability ABI](../abi/capabilities.md): the published bits and groups for
  capsule authors.
- [Manifests and capabilities](../userland/manifests-and-capabilities.md): how a
  capsule asks for bits.
- [IPC](ipc.md): endpoint requirements, held endpoints and the peer list.
- [Processes and capsule spawn](processes-and-spawn.md): where a capsule's token
  is installed.
- [Boot modes](../install/boot-modes.md): the profiles that remove `Network`.
