# Capsule isolation: Address spaces

Part of [Capsule isolation](capsule-isolation.md).

## Address spaces

Every process gets its own page tables. `create_address_space` allocates a fresh
top-level table under a new address space ID and copies in only the kernel half;
the user half starts empty
(`src/memory/paging/manager/address_space/create.rs:37-48`). A
[capsule](../overview/glossary-boot-root-record.md#capsule) therefore has no
mapping of another capsule's memory. Pages are shared where the kernel maps them
on purpose, for example a display surface (`MkSurfaceShare`) or a DMA buffer
(`MkDmaMap`). A surface is mapped only into its owner's process or a process its
owner named, read-only unless named to write; a guessed handle maps nothing
(`src/kernel_core/surface_registry/share/permit.rs`).

When a capsule hands the kernel a pointer, the kernel checks it before copying.
`check_range` refuses a null pointer, a length over `MAX_COPY_SIZE` (64 MiB) and
any range that ends above `USER_SPACE_END`, `0x0000_7FFF_FFFF_FFFF`
(`src/usercopy/policy.rs:35-50`). Each page in the range is then walked:
`translate_read` requires a mapped page with the user bit set, and
`translate_write` also requires it to be writable
(`src/usercopy/walk/access.rs:29-46`).

On x86_64 the kernel turns on SMEP, SMAP and UMIP when CPUID reports them, and
records what CR4 reads back rather than what it asked for, since a hypervisor
may drop the write (`enable` in `src/memory/mmu/mmu/protect/cr4.rs:32-51`). SMEP
stops ring 0 from executing user pages, and SMAP stops it from touching user
memory by accident.
