# Migration: capsule ELF loading and relocation out of ring 0

Findings: NKA-2026-020 (medium), NKA-2026-021 (low). Target design: [09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md).

## Current state

### The live path

One function loads every capsule: `load_elf_entry_into` (`src/elf/loader/global.rs:50-54`), called from exactly one place, `load_elf_into_pid` in the spawn gate's install step (`src/kernel_core/process_spawn/capsule_spawn/runner/install/load_elf_into_pid.rs:34`). Under the global `ELF_LOADER` spin lock it:

1. Parses and validates the ELF header and program-header bounds (`src/elf/loader/core/loader/load_entry_into.rs:25-28`).
2. Picks a base: a fixed one for `ET_EXEC`, a random one for a PIE, or refuses when no entropy answers (`src/elf/loader/core/loader/base_addr.rs:22-29`).
3. For each `PT_LOAD`, allocates a fresh frame per page, maps it into the target ASID and copies the file bytes in through the direct map (`src/elf/loader/core/load_segment/run.rs:28-49`, `src/elf/loader/core/load_segment/populate_page.rs:28-55`).
4. Applies `R_X86_64_RELATIVE` relocations from `DT_RELA`: for each entry it checks the eight bytes lie in a writable `PT_LOAD` range, translates one virtual address, and stores `base + r_addend` at `DIRECTMAP_BASE + pa` (`src/elf/loader/core/relocate.rs:49-110`).
5. Drops write on every page of `PT_GNU_RELRO` (`src/elf/loader/core/relro.rs:31-57`).
6. Returns `base + e_entry` for a PIE (`src/elf/loader/core/loader/base_addr.rs:31-37`).

The capsules are static PIEs: the user target passes `-pie` and `relocation-model = "pic"` (`userland/x86_64-nonos-user.json`), and no capsule start code relocates itself, so step 4 is what makes their GOTs non-null; `src/kernel_core/process_spawn/capsule_spawn/runner/install/load_elf_into_pid.rs:29-33` says so.

The input is trusted by signature but not by construction: every capsule's ELF has passed the certificate, manifest and attestation checks before it is loaded (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:35-87`). NKA-2026-020 and NKA-2026-021 both need an enrolled publisher's capsule, A4 in the audit's terms. They matter because they turn "a publisher can ship a bad capsule" into "a publisher can write kernel memory or halt the machine".

### What else is in `src/elf`, and who uses it

| Part | phys | Callers outside `src/elf` |
|---|---:|---|
| The live path above, with `types`, `errors`, `aslr` | about 3,160 | `src/kernel_core/process_spawn/capsule_spawn/runner/install/load_elf_into_pid.rs:34`; `init_elf_loader` at `src/kernel_core/init/entry/init_runtime.rs:45` |
| `load_executable_into`, `load_parts`, `runtime_load`, `build_image`, `section_queries`, `parse_dynamic`, `parse_sections`, `image/` | about 990 | None: reached only from `load_elf_executable` and `load_elf_executable_into` (`src/elf/loader/global.rs:44-60`), which nothing outside `src/elf` calls |
| `libmgr`, `got`, `symbol`, `hash`, `dynlink`, `reloc`, `embedded`, `cache`, `init`, `fini`, `interpreter`, `tls`, `auxv`, `stack`, `process`, `minimal` | 7,304 | None |

The 61 strings `"... load_elf_executable error:"` in the per-capsule `spawn.rs` files are log tags (`debug_tag`), not calls.

The dead dynamic-linking code is unsafe in a way the live code is not: it dereferences user virtual addresses from ring 0 with no translation, for example the stack builder (`src/elf/stack/setup/push.rs:31-36`) and the GOT writer (`src/elf/got/table/access.rs:39-47`), and `write_irelative` calls a function pointer read from the image, in ring 0 (`src/elf/reloc/apply/write.rs:80-86`). None of it runs today. All of it is deleted in step 1.

### The embedded boot capsules and the spawn plan

Each boot capsule's ELF is an `include_bytes!` of its userland build output, and its certificate, manifest and trailer come through `baked_trust!` (for example `src/userspace/capsule_linux/embed.rs:22`, `src/security/nonos_trust_anchor/baked_trust.rs:23-27`); there are 147 `include_bytes!` and 422 `baked_trust!` sites in `src/`. Init's spawn plan, in ring 0, calls each capsule's spawn function through `spawn_plan::boot::capsule` (`src/userspace/init/spawn_plan/boot.rs:24`) and `capsule_boot::run` (`src/userspace/init/capsule_boot/run.rs:27`); that function builds the spec and calls `spawn_verified`, as the nvme driver's does (`src/hardware/nvme_capsule/spawn.rs:41-61`).

### Linux guests

The personality already loads guests from userland: it parses the ELF, maps segments with `mk_peer_map` and writes them with `mk_peer_copy`, and leaves dynamic relocation to the guest's own `ld.so` (`userland/capsule_linux/src/linux/image/load.rs`, `userland/capsule_linux/src/linux/image/segment.rs`). The kernel's peer copy translates per page and splits at 4 KiB (`src/process/foreign/peer_copy.rs`, `src/process/foreign/peer_chunk.rs`). That is the model this migration follows.

## The boundary failure

Ring 0 acts on numbers a capsule image chooses: where to write (`r_offset`), what to write (`base + r_addend`), where to start (`e_entry`), how far to walk (`PT_GNU_RELRO`'s size). Each check that bounds them is a check the kernel can get wrong, and when it does the result is a kernel write (NKA-2026-020, through a store at `src/elf/loader/core/relocate.rs:106` that can cross into the physically next frame) or a kernel halt (NKA-2026-021, through the overflow at `src/elf/loader/core/loader/base_addr.rs:33` with `panic = "abort"`, and the unbounded walk at `src/elf/loader/core/relro.rs:44-54` under the global lock of `src/elf/loader/global.rs:30`).

## Target design

- **Capsules relocate themselves.** One shared start crate, `nonos_start`, provides `_start` for every capsule. Before it touches any data that needs relocation, it reads its own `_DYNAMIC`, applies its `R_X86_64_RELATIVE` (and, if the linker emits them, `DT_RELR`) entries against its runtime base, asks the kernel to drop write on its own RELRO span, and calls the capsule's entry. A bad `r_offset` now faults the capsule.
- **The loader maps; it does not relocate.** Mapping runs in userland, in init's address space for boot and store-loaded capsules: parse the header and program headers, create a MemoryObject per writable segment and copy the file bytes in, map read-only segments straight from the sealed image object, zero the bss, set permissions. It never writes a pointer value.
- **The kernel measures what is mapped.** At `process_start` the kernel computes `D_load` over the image span and compares it with the verdict ([09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md)). Because pages hold unrelocated file bytes, `D_load` does not depend on the random base.
- **No ELF code in ring 0.** `src/elf` is deleted. The kernel starts init from a flat image the build produced and checked ([09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md)).

The alternative of a loader running inside the new address space before it holds any capability was considered. It needs the new process to hold, for a while, the authority to map into itself, which is exactly the authority a capsule must not have, and it puts the loader's own bytes into every capsule's measurement. Self-relocation needs no authority beyond writing the capsule's own writable pages, which it has anyway.

## Interface specification

### The self-relocation marker

A capsule built with `nonos_start` carries an ELF note so the kernel, and later the loader, can tell it relocates itself:

```
Note section .note.nonos.start, type PT_NOTE
  namesz = 6, name = "NONOS\0" (padded to 8)
  descsz = 8
  type   = 0x4e534c46            /* "FLSN": self-relocating start */
  desc   = u32le version (1), u32le flags (bit 0: handles DT_RELR)
```

### Interim system call: drop write on one's own pages

Needed in step 4, before the object model exists. Today's memory family has only `MMAP` and `MUMP` (`abi/syscalls.toml:352`, `abi/syscalls.toml:358`).

```toml
[desc.MPRS]          # MkProtectSelf
nr   = 0x5352504D
caps = ["Memory"]
args = [
  { name = "va",    type = "u64", dir = "in" },
  { name = "len",   type = "u64", dir = "in" },
  { name = "perms", type = "u32", dir = "in" },  # bit 0 read, bit 1 write, bit 2 exec
]
ret  = { type = "i64" }
```

Semantics: on pages of the caller's own address space only; permissions may only shrink (a call that would add write or execute returns `EPERM`); the range must be page-aligned and wholly mapped. In the target this is `as_protect` on the caller's own address-space capability ([08-target-syscall-surface.md](../08-target-syscall-surface.md)).

### Target calls used by the loader

`as_create`, `mem_create`, `mem_seal`, `as_map`, `as_protect`, `as_copy`, `process_create`, `process_start`, as specified in [08-target-syscall-surface.md](../08-target-syscall-surface.md). The loader's library interface:

```rust
pub struct LoadPlan {
    pub base: u64,               // chosen by the caller, page-aligned
    pub entry_offset: u64,       // e_entry, checked inside an executable PT_LOAD
    pub span_pages: u32,         // image span from base
    pub segments: Vec<Segment>,  // at most 16
}
pub struct Segment { pub offset: u64, pub file_len: u64, pub mem_len: u64, pub vaddr: u64, pub perms: u8 }

/// Parse and check; never panics on any input (every arithmetic step checked).
pub fn plan(image: &[u8]) -> Result<LoadPlan, LoadError>;
/// Map the plan into an unstarted address space the caller holds.
pub fn map(plan: &LoadPlan, image: CapHandle, aspace: CapHandle, quota: CapHandle) -> Result<(), LoadError>;
/// The load digest of 09-target-spawn-and-boot-trust.md, computed from the file alone.
pub fn load_digest(image: &[u8]) -> Result<[u8; 32], LoadError>;
```

`load_digest` is the same function the build uses to publish `D_load` beside `D_file`, and the verifier uses to fill the verdict.

## Migration steps

Each step ships and is tested on its own; each preserves the invariant named with it.

1. **Delete the dead ELF code.** With an evidence report and with the proof bindings that point at the deleted code (the `Bounds` model's binding to `src/elf/reloc`, `verification/lean/REFINEMENT.md:46`, and `ElfReloc`) rebound to the live relocator and re-extracted with Charon and Aeneas, none retired, as [15-decisions.md](../15-decisions.md) records; the same change widens the proof corpus root to the extraction and the other proof directories, and one root-change note for the next seal records both ([13-migration-plan.md](../13-migration-plan.md)). The figure in [04-classification.md](../04-classification.md) of 7,304 lines with no caller outside `src/elf`, plus the roughly 990 dead lines inside `src/elf/loader` (`load_elf_executable`, `load_elf_executable_into` and what only they reach), is the check on that report. Build every Cargo profile and every `nonos-mk-*` recipe.
   *Invariant*: the one load path, `load_elf_entry_into`, is byte-for-byte unchanged. *Test*: the build; a boot of the default image shows the same `[LAUNCH]` lines (`nonos-launch-trace`).
2. **Land the in-place fixes.** The page-safe store primitive for the relocation store, the alignment and page-crossing refusal, checked entry arithmetic with the entry inside an executable `PT_LOAD`, RELRO bounded to the image.
   *Invariant*: every image that loaded before loads after, unless it has an unaligned or page-crossing `RELATIVE` entry, an entry outside its executable segments or a RELRO span outside the image. *Test*: the audit's harnesses for NKA-2026-020 and NKA-2026-021 as host tests over the real files.
3. **Ship `nonos_start` and move every capsule onto it.** The 140 crates that define `_start` themselves, the `nonos_main!` and `sdk_main!` macros and `toolchain/nonos-rt` all call into one relocation prologue. The kernel still relocates; RELA relocation sets each word to `base + r_addend` regardless of its prior contents, so applying it twice gives the same result.
   *Invariant*: every capsule's memory after its prologue equals what it was after the kernel's relocation alone. *Test*: a debug build that hashes each capsule's writable segments after the prologue and compares with a hash taken right after the kernel's relocation; equal for all capsules in the image.
4. **Stop relocating in the kernel.** For an image with the `FLSN` note the kernel skips relocation and RELRO; the start code calls `MkProtectSelf` on its RELRO span. A PIE with a non-empty `DT_RELA` and no note is refused.
   *Invariant*: no ring-0 store's address or value comes from an image's relocation entries. *Test*: the NKA-2026-020 harness, run against the kernel, now writes nothing; the capsule with the straddling entry faults in ring 3 instead.
5. **Record `D_load`.** The build computes `D_load` for every capsule and publishes it beside the trailer. The kernel computes `D_load` over the mapped image span before the first instruction and records it with `D_file` in the attestation registry; a mismatch with the published value is logged, not yet refused.
   *Invariant*: admission is unchanged. *Test*: every boot capsule's recorded `D_load` equals the build's.
6. **Shrink the kernel loader to mapping only.** With relocation gone, what remains in ring 0 is header and program-header parsing and page population, about 800 lines.
   *Invariant*: as step 4. *Test*: as step 1.
7. **Move mapping to userland.** When the object model's address-space, MemoryObject and verdict calls exist ([13-migration-plan.md](../13-migration-plan.md), phase 3), init and the installer load through the `nonos_load` library; the kernel refuses a start whose `D_load` differs from the verdict's. `src/elf` is deleted.
   *Invariant*: the set of images that run, and the measurement recorded for each, are the same as after step 5. *Test*: boot every profile; compare the attestation registry before and after.
8. **Replace the embeds with the boot package.** One section of the kernel image holds every boot capsule's four artefacts and init's spawn plan; the 147 `include_bytes!` and 422 `baked_trust!` sites and the per-capsule `embed.rs` files are deleted.
   *Invariant*: the kernel image's signed bytes still cover every boot capsule. *Test*: the bootloader's signature check (`nonos-bootloader/src/kernel_verify/verify.rs:29`) over the new image; the boot set spawns.

## Do the in-place fixes stay necessary?

| In-place fix | After step 4 | After step 7 |
|---|---|---|
| Page-safe store primitive (`src/memory/asid_store/`) | Still used by `usercopy`, IPC delivery and the peer copies of `src/process/foreign` | Still used; it is the one primitive for every kernel write into another address space |
| Relocation alignment and page-crossing refusal | Dead for capsules with the note | Deleted with `src/elf` |
| Checked entry and entry inside an executable segment | Still needed while the kernel parses | Moves into `nonos_load::plan` as a correctness check |
| RELRO bounded to the image | Dead for capsules with the note | Deleted |

## Risks

- **Relocating before relocation is done.** The prologue must use no relocated data and no function pointers until it finishes. Write it in assembly or in a `#[naked]` function with position-independent references only, and test it on every capsule with the step-3 hash comparison.
- **Linker drift.** A newer `rust-lld` may emit `DT_RELR` (pack-relative-relocs). The kernel handles only RELA today; the prologue should handle both, and the build should fail if an image has relocation types other than RELATIVE.
- **Measurement order.** The kernel must measure after the loader's last mapping and before the first instruction, on an address space no one else can still map into. `process_start` is that point only if the loader's address-space capability is revoked before measuring; the kernel does that revocation itself.
- **Boot-time cost.** Self-relocation moves work from ring 0 to ring 3 without changing its amount; measuring `D_load` adds a BLAKE3 pass the size of the image, which today's preflight already makes once over the file.
- **The development loop.** A capsule built without `nonos_start` stops loading at step 4. The note makes the refusal explicit in the spawn log.
