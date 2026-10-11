# nonos_linux_policy

`nonos_linux_policy` is the least-privilege box of one Linux program on NONOS,
as 256 canonical bytes (`LinuxAppPolicy`, magic `NLXPOL01`). It names:
- the program, by the BLAKE3 of its ELF;
- the services capsule_linux may serve it (`mask`);
- the market listing and release it was published under;
- the system calls it may complete (`syscall_set`, one bit per bucket of the
  kernel's seal).

The layout and the reasons for it are in the plan of the marketplace plan.

`decode` accepts exactly the bytes `encode` writes. It refuses:
- a wrong length, magic or version;
- a non-zero reserved byte;
- a mask bit with no meaning;
- a syscall bit past bucket 512.

A policy therefore has one encoding, so it has one `policy_hash`. That hash
is what a release signs. `app_id` binds that hash to the program; it is what
capsule_linux notes in the kernel's seal, and what an isolation proof is
checked against.

`unlisted` is the box for a program that arrived with no policy: a terminal,
its own home, and exec. That default is the owner's decision (O1). Changing it
is changing `UNLISTED_MASK`.

It is a `no_std` library with no capsule, service or capability word.

## Tests

19 host tests in `src/tests/`:
- every field's offset;
- every refusal;
- a single-bit flip anywhere in the 256 bytes either is refused or decodes to
  a policy that encodes back to the flipped bytes;
- the hashes are domain-separated;
- two pinned vectors.

```sh
cargo test --locked
cargo build --locked -Zbuild-std=core,alloc --target ../../x86_64-nonos.json
```
