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
| `crate:nox_verify` | crate | NON-OS/STARKs's verifier at the pinned 1b4b5a3, linked into ring 0; it checks the STARK proof in every v4 trailer the capsule gate reads. |
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
| `boot-crate:nox_verify` | boot-crate | NON-OS/STARKs's verifier at the pinned 1b4b5a3, in the bootloader; it checks the STARK proof in the kernel's v4 trailer. |
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
| `lean-axiom:Usize.Insts.CoreConvertTryFromU64TryFromIntError.try_from` | lean-axiom | Aeneas's opaque model of `usize::try_from(u64)`, an axiom of the extracted modules Elf, ElfStackLayoutLayoutInfo. |
| `lean-axiom:alloc.collections.btree.map.BTreeMap` | lean-axiom | Aeneas's opaque model of the `BTreeMap` type, an axiom of the extracted modules ArchX8664PciStatsTypes, DriversPciStatsPciStats. |
| `lean-axiom:alloc.collections.btree.map.BTreeMapKVGlobal.Insts.CoreDefaultDefault.default` | lean-axiom | Aeneas's opaque model of `BTreeMap::default`, an axiom of the extracted module DriversPciStatsPciStats. |
| `lean-axiom:alloc.collections.btree.map.BTreeMapKVGlobal.new` | lean-axiom | Aeneas's opaque model of `BTreeMap::new`, an axiom of the extracted modules ArchX8664PciStatsTypes, DriversPciStatsPciStats. |
| `lean-axiom:alloc.string.String.new` | lean-axiom | Aeneas's opaque model of `String::new`, an axiom of the extracted module ProcfsTypes. |
| `lean-axiom:alloc.vec.Vec.is_empty` | lean-axiom | Aeneas's opaque model of `Vec::is_empty`, an axiom of the extracted modules CapabilitiesChainChain, ElfLoaderImageDynamic, X8664UefiSignatureTypes and 1 more. |
| `lean-axiom:core.core_arch.x86.cpuid.__cpuid` | lean-axiom | Aeneas's opaque model of `__cpuid`, an axiom of the extracted module SpectreMitigationsCpuid. |
| `lean-axiom:core.core_arch.x86.cpuid.__cpuid_count` | lean-axiom | Aeneas's opaque model of `__cpuid_count`, an axiom of the extracted module SpectreMitigationsCpuid. |
| `lean-axiom:core.mem.size_of` | lean-axiom | Aeneas's opaque model of `size_of`, an axiom of the extracted modules AcpiTablesMadtHeader, Elf, ElfLoaderImageDynamic and 2 more. |
| `lean-axiom:core.num.U64.count_ones` | lean-axiom | Aeneas's opaque model of `u64::count_ones`, an axiom of the extracted module ProcessSignalSetBits. |
| `lean-axiom:core.num.U64.wrapping_neg` | lean-axiom | Aeneas's opaque model of `u64::wrapping_neg`, an axiom of the extracted module Ct. |
| `lean-axiom:core.num.U8.wrapping_neg` | lean-axiom | Aeneas's opaque model of `u8::wrapping_neg`, an axiom of the extracted module Ct. |
| `lean-axiom:core.num.Usize.div_ceil` | lean-axiom | Aeneas's opaque model of `usize::div_ceil`, an axiom of the extracted module MemoryPagingConstantsAlignFuncs. |
| `lean-axiom:core.num.Usize.saturating_mul` | lean-axiom | Aeneas's opaque model of `usize::saturating_mul`, an axiom of the extracted module TypesZoneStats. |
| `lean-axiom:core.ops.range.Range.Insts.CoreIterTraitsIteratorIterator.collect` | lean-axiom | Aeneas's opaque model of `Range::collect`, an axiom of the extracted module SchedulerTaskTypesAffinity. |
| `lean-axiom:core.option.Option.map` | lean-axiom | Aeneas's opaque model of `Option::map`, an axiom of the extracted module AcpiTablesSlitNeighbors. |
| `lean-axiom:core.result.Result.is_err` | lean-axiom | Aeneas's opaque model of `Result::is_err`, an axiom of the extracted module X8664VgaOpsLock. |
| `lean-axiom:core.result.Result.map_err` | lean-axiom | Aeneas's opaque model of `Result::map_err`, an axiom of the extracted module Elf. |
| `lean-axiom:core.slice.Slice.first` | lean-axiom | Aeneas's opaque model of `<[T]>::first`, an axiom of the extracted module X8664UefiSignatureTypes. |
| `lean-axiom:core.sync.atomic.Atomic` | lean-axiom | Aeneas's opaque model of the generic `Atomic` type, an axiom of the extracted modules ArchX8664SerialWriter, CacheTypes, CpuMsrStats and 33 more. |
| `lean-axiom:core.sync.atomic.AtomicBoolAlign1U8.compare_exchange_weak` | lean-axiom | Aeneas's opaque model of `AtomicBool::compare_exchange_weak`, an axiom of the extracted module X8664VgaOpsLock. |
| `lean-axiom:core.sync.atomic.AtomicBoolAlign1U8.load` | lean-axiom | Aeneas's opaque model of `AtomicBool::load`, an axiom of the extracted modules InterruptApicIdleTimerHaltSafe, ObservabilityPolicy, X8664VgaOpsLock. |
| `lean-axiom:core.sync.atomic.AtomicBoolAlign1U8.new` | lean-axiom | Aeneas's opaque model of `AtomicBool::new`, an axiom of the extracted modules InterruptApicIdleTimerHaltSafe, ObservabilityPolicy, X8664VgaOpsLock. |
| `lean-axiom:core.sync.atomic.AtomicBoolAlign1U8.store` | lean-axiom | Aeneas's opaque model of `AtomicBool::store`, an axiom of the extracted modules ObservabilityPolicy, X8664VgaOpsLock. |
| `lean-axiom:core.sync.atomic.AtomicU16Align2U16.load` | lean-axiom | Aeneas's opaque model of `AtomicU16::load`, an axiom of the extracted modules Riscv64CpuCapsQuery, Riscv64CpuExtensionsQuery. |
| `lean-axiom:core.sync.atomic.AtomicU16Align2U16.new` | lean-axiom | Aeneas's opaque model of `AtomicU16::new`, an axiom of the extracted modules Riscv64CpuCapsQuery, Riscv64CpuExtensionsQuery. |
| `lean-axiom:core.sync.atomic.AtomicU32Align4U32.fetch_add` | lean-axiom | Aeneas's opaque model of `AtomicU32::fetch_add`, an axiom of the extracted module ServicesLifecycleStateRespawn. |
| `lean-axiom:core.sync.atomic.AtomicU32Align4U32.load` | lean-axiom | Aeneas's opaque model of `AtomicU32::load`, an axiom of the extracted modules MulticoreState, ServicesLifecycleStateRespawn. |
| `lean-axiom:core.sync.atomic.AtomicU32Align4U32.new` | lean-axiom | Aeneas's opaque model of `AtomicU32::new`, an axiom of the extracted module MulticoreState. |
| `lean-axiom:core.sync.atomic.AtomicU32Align4U32.store` | lean-axiom | Aeneas's opaque model of `AtomicU32::store`, an axiom of the extracted module ServicesLifecycleStateRespawn. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.compare_exchange_weak` | lean-axiom | Aeneas's opaque model of `AtomicU64::compare_exchange_weak`, an axiom of the extracted module MemoryBuddyAllocStatsRecord. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.fetch_add` | lean-axiom | Aeneas's opaque model of `AtomicU64::fetch_add`, an axiom of the extracted modules CpuMsrStats, DriversPciStatsRecord, InterruptsHandlersIrqSyscall and 8 more. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.fetch_or` | lean-axiom | Aeneas's opaque model of `AtomicU64::fetch_or`, an axiom of the extracted module IrqReserved. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.fetch_sub` | lean-axiom | Aeneas's opaque model of `AtomicU64::fetch_sub`, an axiom of the extracted modules MemoryBuddyAllocStatsRecord, MemoryDmaStatsRecord, MemoryMmioStatsRecord and 2 more. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.load` | lean-axiom | Aeneas's opaque model of `AtomicU64::load`, an axiom of the extracted modules CpuMsrStats, DriversPciStatsGetters, DriversPciStatsPciStats and 14 more. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.new` | lean-axiom | Aeneas's opaque model of `AtomicU64::new`, an axiom of the extracted modules CacheTypes, CpuMsrStats, DriversPciStatsGetters and 12 more. |
| `lean-axiom:core.sync.atomic.AtomicU64Align8U64.store` | lean-axiom | Aeneas's opaque model of `AtomicU64::store`, an axiom of the extracted modules CacheTypes, DriversPciStatsRecord, GicState and 5 more. |
| `lean-axiom:core.sync.atomic.AtomicU8Align1U8.load` | lean-axiom | Aeneas's opaque model of `AtomicU8::load`, an axiom of the extracted module ObservabilityPolicy. |
| `lean-axiom:core.sync.atomic.AtomicU8Align1U8.new` | lean-axiom | Aeneas's opaque model of `AtomicU8::new`, an axiom of the extracted module ObservabilityPolicy. |
| `lean-axiom:core.sync.atomic.AtomicU8Align1U8.store` | lean-axiom | Aeneas's opaque model of `AtomicU8::store`, an axiom of the extracted module ObservabilityPolicy. |
| `lean-axiom:core.sync.atomic.AtomicUsizeAlign8Usize.compare_exchange_weak` | lean-axiom | Aeneas's opaque model of `AtomicUsize::compare_exchange_weak`, an axiom of the extracted modules NonosInboxStats, SysSyncSemaphoreRelease. |
| `lean-axiom:core.sync.atomic.AtomicUsizeAlign8Usize.fetch_add` | lean-axiom | Aeneas's opaque model of `AtomicUsize::fetch_add`, an axiom of the extracted modules MemoryBuddyAllocStatsRecord, MemoryDmaStatsRecord, MemoryMmioStatsRecord and 2 more. |
| `lean-axiom:core.sync.atomic.AtomicUsizeAlign8Usize.fetch_sub` | lean-axiom | Aeneas's opaque model of `AtomicUsize::fetch_sub`, an axiom of the extracted modules MemoryDmaStatsRecord, MemoryMmioStatsRecord, MemoryRegionStatsRecord. |
| `lean-axiom:core.sync.atomic.AtomicUsizeAlign8Usize.load` | lean-axiom | Aeneas's opaque model of `AtomicUsize::load`, an axiom of the extracted modules ArchX8664SerialWriter, MemoryBuddyAllocStatsQuery, MemoryDmaStatsQuery and 6 more. |
| `lean-axiom:core.sync.atomic.AtomicUsizeAlign8Usize.new` | lean-axiom | Aeneas's opaque model of `AtomicUsize::new`, an axiom of the extracted modules ArchX8664SerialWriter, NonosInboxStats, PipeBuffer. |
| `lean-axiom:core.sync.atomic.AtomicUsizeAlign8Usize.store` | lean-axiom | Aeneas's opaque model of `AtomicUsize::store`, an axiom of the extracted module PipeBuffer. |
| `lean-axiom:core.sync.atomic.compiler_fence` | lean-axiom | Aeneas's opaque model of `compiler_fence`, an axiom of the extracted modules ConstantTimeAes, Ct, EdField. |
| `lean-axiom:core.sync.atomic.private.Align1` | lean-axiom | Aeneas's opaque model of the 1-byte alignment marker under the atomic types, an axiom of the extracted modules InterruptApicIdleTimerHaltSafe, ObservabilityPolicy, X8664VgaOpsLock. |
| `lean-axiom:core.sync.atomic.private.Align2` | lean-axiom | Aeneas's opaque model of the 2-byte alignment marker under the atomic types, an axiom of the extracted modules Riscv64CpuCapsQuery, Riscv64CpuExtensionsQuery. |
| `lean-axiom:core.sync.atomic.private.Align4` | lean-axiom | Aeneas's opaque model of the 4-byte alignment marker under the atomic types, an axiom of the extracted modules MulticoreState, ServicesLifecycleStateRespawn. |
| `lean-axiom:core.sync.atomic.private.Align8` | lean-axiom | Aeneas's opaque model of the 8-byte alignment marker under the atomic types, an axiom of the extracted modules ArchX8664SerialWriter, CacheTypes, CpuMsrStats and 27 more. |
| `lean-axiom:memory.addr.phys.PhysAddr.Insts.CoreCmpPartialOrdPhysAddr.ge` | lean-axiom | Aeneas's opaque model of the derived `PartialOrd::ge` of the kernel's own `PhysAddr`, an axiom of the extracted module MemoryFrameAllocTypesRange. |
