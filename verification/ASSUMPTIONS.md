# What NØNOS trusts without proof

Everything the security claims rest on that no theorem, test or check in this
tree establishes. `tools/nonos-assumptions` rebuilds the found rows from the
tree and fails when one is missing here or listed here but gone. `stated` rows
have no detector. This file is one list by design, so it is longer than the
75-line rule allows.

| id | kind | what is trusted |
|---|---|---|
| `stated:cpu-isa` | stated | The CPU implements x86-64 paging, rings, SYSCALL/SYSRET, SWAPGS and the TSS as documented. |
| `stated:firmware` | stated | UEFI firmware is honest until ExitBootServices, and the Secure Boot keys are the owner's. |
| `stated:dma-no-iommu` | stated | With no IOMMU, every DMA-capable device and its driver capsule can reach all of physical memory. |
| `stated:physical` | stated | No attacker with bus, JTAG or cold-boot access to the machine. |
| `stated:hash-collision` | stated | BLAKE3, SHA-2, SHA-3 and the width-8 Poseidon are collision resistant. |
| `stated:fri-soundness` | stated | The FRI proximity bound the STARK soundness figures use holds at these parameters. |
| `stated:lean-kernel` | stated | Lean's kernel and its three standard axioms (propext, Classical.choice, Quot.sound) are sound. |
| `stated:extraction` | stated | Charon and Aeneas lower MIR faithfully, so an extracted definition is the kernel function. |
| `stated:debian-musl` | stated | The musl loader and libc the Tier 2 test image carries are Debian's musl 1.2.4 build, trusted as packaged. |
| `stated:alpine-busybox` | stated | The busybox guest test images carry is Alpine's static build, trusted as built by Alpine. |
| `crate:bitflags` | crate | Third-party code linked into ring 0. |
| `crate:bitvec` | crate | Third-party code linked into ring 0. |
| `crate:blake3` | crate | Third-party code linked into ring 0; the capsule and kernel measurement. |
| `crate:curve25519-dalek` | crate | Third-party code linked into ring 0. |
| `crate:ed25519-dalek` | crate | Third-party code linked into ring 0; classical signature verification. |
| `crate:heapless` | crate | Third-party code linked into ring 0. |
| `crate:lazy_static` | crate | Third-party code linked into ring 0. |
| `crate:linked_list_allocator` | crate | Third-party code linked into ring 0; the kernel heap. |
| `crate:sha2` | crate | Third-party code linked into ring 0. |
| `crate:sha3` | crate | Third-party code linked into ring 0. |
| `crate:smallvec` | crate | Third-party code linked into ring 0. |
| `crate:smoltcp` | crate | Third-party code linked into ring 0 where a profile enables it. |
| `crate:spin` | crate | Third-party code linked into ring 0; every kernel lock. |
| `crate:volatile` | crate | Third-party code linked into ring 0. |
| `crate:x25519-dalek` | crate | Third-party code linked into ring 0. |
| `crate:x86_64` | crate | Third-party code linked into ring 0; page tables and descriptor tables. |
| `boot-crate:bitflags` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:blake3` | boot-crate | Third-party code in the bootloader; the kernel measurement. |
| `boot-crate:bootloader_api` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:curve25519-dalek` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:ed25519-dalek` | boot-crate | Third-party code in the bootloader; the kernel signature. |
| `boot-crate:goblin` | boot-crate | Third-party code in the bootloader; ELF parsing of the kernel. |
| `boot-crate:heapless` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:log` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:noto-sans-mono-bitmap` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:r-efi` | boot-crate | Third-party code in the bootloader; UEFI bindings. |
| `boot-crate:sha2` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:spin` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:uefi` | boot-crate | Third-party code in the bootloader; UEFI bindings. |
| `boot-crate:uefi-services` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:uuid` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:xmas-elf` | boot-crate | Third-party code in the bootloader; ELF parsing of the kernel. |
| `boot-crate:zerocopy` | boot-crate | Third-party code in the bootloader. |
| `boot-crate:zeroize` | boot-crate | Third-party code in the bootloader; key material wiping. |
| `prim:crypto/asymmetric/alg_id` | prim | In-tree algorithm identifiers, unproven. |
| `prim:crypto/asymmetric/curve25519` | prim | In-tree Curve25519, unproven. |
| `prim:crypto/asymmetric/ed25519` | prim | In-tree Ed25519, unproven. |
| `prim:crypto/asymmetric/p256` | prim | In-tree P-256, unproven. |
| `prim:crypto/asymmetric/p384` | prim | In-tree P-384, unproven. |
| `prim:crypto/asymmetric/rsa` | prim | In-tree RSA, unproven. |
| `prim:crypto/hash/blake3` | prim | In-tree BLAKE3 glue, unproven. |
| `prim:crypto/hash/sha3` | prim | In-tree SHA-3, unproven. |
| `prim:crypto/hash/sha384` | prim | In-tree SHA-384, unproven. |
| `prim:crypto/hash/sha512` | prim | In-tree SHA-512, unproven. |
| `prim:crypto/hash/unified` | prim | In-tree hash dispatch, unproven. |
| `prim:crypto/pqc/kyber` | prim | In-tree ML-KEM, unproven. |
| `prim:crypto/pqc/mceliece` | prim | In-tree Classic McEliece, unproven. |
| `prim:crypto/pqc/ml_dsa_65` | prim | In-tree ML-DSA-65, the post-quantum half of every signature check, unproven. |
| `prim:crypto/pqc/ntru` | prim | In-tree NTRU, unproven. |
| `prim:crypto/pqc/quantum` | prim | In-tree post-quantum dispatch, unproven. |
| `prim:crypto/pqc/sphincs` | prim | In-tree SPHINCS+, unproven. |
| `prim:crypto/symmetric/aes` | prim | In-tree AES, unproven. |
| `prim:crypto/symmetric/aes_gcm` | prim | In-tree AES-GCM, unproven. |
| `prim:crypto/symmetric/chacha20poly1305` | prim | In-tree ChaCha20-Poly1305, unproven. |
| `prim:crypto/zk` | prim | In-tree zero-knowledge helpers, unproven. |
| `prim:crypto/zk_kernel` | prim | In-tree Pedersen and Sigma proofs for local signing, unproven. |
| `prim:stark-core` | prim | The STARK prover and verifier behind every attestation gate; tested, not proven. |
| `hw:rdrand` | hw | RDRAND and RDSEED return unpredictable values. |
| `hw:smep-smap` | hw | SMEP and SMAP stop ring 0 executing or reading user pages. |
| `hw:nx` | hw | The NX bit stops execution from data pages. |
| `hw:iommu` | hw | VT-d translates and faults device DMA as its tables say. |
| `hw:tpm` | hw | The TPM keeps its counters monotonic and its keys inside. |
| `tool:rustc-nightly-2026-01-16` | tool | The compiler, a nightly, generates what the source says. |
| `tool:lean-v4.15.0` | tool | The Lean toolchain checks proofs soundly. |
| `tool:aeneas` | tool | The extractor from Rust to Lean. |
| `tool:charon` | tool | The MIR front end the extractor reads. |
| `tool:kani` | tool | The bounded model checker behind the Kani harnesses. |
| `tool:verus` | tool | The verifier behind the Verus proofs. |
| `lean-axiom:core.option.Option.ok_or` | lean-axiom | Aeneas's opaque model of `Option::ok_or`, in the closure of the extracted IRQ and policy theorems. |
