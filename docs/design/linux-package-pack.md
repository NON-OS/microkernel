# A Linux package as one store object (the pack)

This note answers the Lead's ruling of 2026-10-26 on the store's name budget: a package lands as one
read-only store object, its "pack", with its own index. capsule_linux serves the guest's files out of the
pack. Two names per package, the pack and the trailer over its index, whatever its file count.

It was first a design note. Sections 3 to 6 now say what was built (userland/nonos_linux_pack, and
capsule_linux's file/store/pack/ and install/place/) and the Lead's rulings on it: no unlink while open, a pack
over 64 MiB refused when its listing is built, the per-file check on every read.

Each claim is marked (booted), (host-tested), (host-read: file:line), (measured), (built, not booted)
or (design). Paths under
capsule_vfs are relative to userland/capsule_vfs/src/.

## 1. What a vfs name costs today, and why 2048

The store is a `Vec<File>` (host-read: store/fdtable/types.rs:102). One `File` holds:
- its name as a `String`;
- its bytes as a `Vec<u8>`;
- `is_dir`, `mtime: u64`, `mode: u16`, `owner: u32`;
- `streamed: Option<Extent>`, which is two u64s.

(host-read: types.rs:49-73)

Memory per name, on x86_64 (design, from the layout):
- **Fixed part:** about 88 bytes. Two 24-byte headers, an 8-byte mtime, a 24-byte `Option<Extent>`, and the small fields with padding.
- **The name's heap allocation:** a Linux path such as `/linux/usr/share/vim/vim91/syntax/vim.vim` is 40 to 70 bytes, plus the allocator's header.
- **The file's bytes:** a separate allocation, rounded up by the doubling growth rule (store/fdtable/budget.rs:74-77).

So the bookkeeping is roughly 150 to 200 bytes a name. 2048 names come to about 0.4 MiB, which is negligible
against the 320 MiB heap (host-read: main.rs:34).

Memory is not why the count is 2048. The work is. Every lookup is a linear scan comparing whole
strings (host-read: store/fdtable/lookup.rs:20-22 `files.iter().position(|f| f.name == path)`). `dirstat`,
`list_paths` and `search` walk every name. So each open, stat, rename or quota check of a guest costs
O(names). At 2048 names with ~60-byte paths, one miss compares about 120 KiB of strings, and capsule_linux does
several lookups per guest syscall. The cap keeps that bounded. (host-read; design)

`NAMES_PER_OWNER` is 512, a quarter of the store (host-read: store/fdtable/budget.rs:42). It keeps one
client from starving the rest: the install role, the Terminal, Files and the wallet share the 2048.
(host-read: capsule_vfs/README.md:141)

The on-disk index per name. The kept store is a container with:
- 128-byte entries;
- 96-byte names;
- at most 512 entries;
- 96 MiB of loaded payload.

(host-read: userland/nonos_disk_map/src/container.rs:33-50) A Linux package never reaches it: installs stay in
RAM (host-read: capsule_linux/src/linux/install/place/keep.rs:50-53). So today the disk index costs a package
nothing. A pack would cost one 128-byte entry if a kept boot ever keeps it. (design)

## 2. The catalogue's packages, by names they need

These were measured on 2026-10-26. I took every package in the dependency closure of the 18 listings: 65 Alpine
v3.20 x86_64 apks, resolved from both APKINDEXes and fetched from dl-cdn.alpinelinux.org. I then counted the
store names each would take. A name here is a regular file, a symlink or hardlink, or a directory. Today a link
is a line in the link table, not a store name (host-read: install/place_links), so the store names a package
takes are its files plus its directories. (measured)

The 20 largest packages (names = files + links + dirs):

| names | files | links | dirs | unpacked | package |
|------:|------:|------:|-----:|---------:|---------|
| 1870 | 1803 | 0 | 67 | 26.2 MB | vim-common 9.1.0707-r0 |
| 214 | 195 | 2 | 17 | 2.9 MB | kakoune 2024.05.18-r0 |
| 192 | 41 | 143 | 8 | 6.5 MB | git 2.45.4-r0 |
| 161 | 147 | 2 | 12 | 9.4 MB | helix 24.03-r0 |
| 139 | 123 | 0 | 16 | 0.2 MB | ca-certificates 20261020-r0 |
| 53 | 39 | 1 | 13 | 0.1 MB | ncurses-terminfo-base 6.4_p20240420-r2 |
| 46 | 39 | 0 | 7 | 1.3 MB | bash 5.2.26-r0 |
| 36 | 23 | 6 | 7 | 5.3 MB | glib 2.80.5-r0 |
| 35 | 29 | 0 | 6 | 1.4 MB | profanity 0.14.0-r1 |
| 28 | 7 | 0 | 21 | 0.8 MB | busybox 1.36.1-r31 |
| 20 | 10 | 1 | 9 | 4.7 MB | libcrypto3 3.3.7-r0 |
| 18 | 11 | 3 | 4 | 0.2 MB | ncurses 6.4_p20240420-r2 |
| 11 | 3 | 0 | 8 | 13.2 MB | micro 2.0.13-r6 |
| 8 | 3 | 3 | 2 | 0.9 MB | brotli-libs 1.1.0-r2 |
| 7 | 1 | 4 | 2 | 2.8 MB | vim 9.1.0707-r0 |
| 6 | 2 | 2 | 2 | 0.7 MB | pcre2 10.43-r0 |
| 6 | 2 | 1 | 3 | 0.3 MB | readline 8.2.10-r0 |
| 6 | 2 | 1 | 3 | 0.2 MB | libgpg-error 1.49-r0 |
| 6 | 2 | 1 | 3 | 0.1 MB | libeconf 0.6.3-r0 |
| 5 | 3 | 0 | 2 | 0.2 MB | less 643-r2 |

By listing, the whole closure if nothing is installed yet: (measured)
- vim: 1940 names over 6 packages.
- tig: 500 over 21.
- git: 438 over 18.
- profanity: 426 over 33.
- kakoune: 224.
- helix: 197.
- lazygit: 163.
- bat: 121.
- Each of the other ten is under 75.

So vim alone fills the store. git and tig together pass 512. With packs, the largest closure, profanity, is
33 names. (measured; design)

The vim boot agrees (booted: HANDOFF/catalogue/evidence/boot-plan-2026-10-25/vim-fixed/):
- vim-common unpacked at a 51 MiB peak.
- 1348 of its files were not written once the owner reached 512 names.
- The 455 written were taken out.

## 3. The pack

### Format, version 1 (host-tested: userland/nonos_linux_pack)

All integers are little-endian. Every offset is from the start of the pack.

```
header (64 bytes)
  magic        8   "NONOSPK1"
  version      u16 1
  flags        u16 0 (any other value: refused)
  count        u32 entries in the index
  index_off    u64 = 64
  index_len    u64 = count * 48
  names_off    u64 = index_off + index_len
  names_len    u64
  data_off     u64 = names_off + names_len, rounded up to 4096
  reserved     u64 0
index (count * 48 bytes), sorted by name bytes, no two equal
  name_off     u32 into the names block
  name_len     u16 1..=255
  kind         u8  0 file, 1 dir, 2 symlink
  mode         u8  1 runs, 0 does not (a file only); nothing else
  data_off     u64 from the start of the pack (0 for a dir)
  data_len     u64 (link target length for a symlink)
  digest       [u8; 16] the first 16 bytes of BLAKE3 of the bytes; zero for a dir
  reserved     [u8; 8] 0
names block  the names, packed in sorted order, each a path under the package root, no NUL
data area    each file's bytes at the first 64-byte boundary after the last file's end; an empty
             file at data_off; every gap zero; nothing past the last file
```

Bounds rules. A pack is refused whole if any of these fail:
- magic, version or flags differ;
- the header's offsets are not exactly as above;
- the pack's length is not `data_off + the end of the last data`;
- count > 65536;
- a name is out of the names block, empty, longer than 255 bytes, absolute, or holds `..`, `.`, `//` or a NUL;
- the names are not strictly sorted;
- a file's data is out of the data area, or overlaps another file's data;
- a directory has data;
- a symlink target is empty, longer than 4095 bytes, or holds a NUL;
- any byte between the names and the end that is no file's is not zero, so one set of files has one pack;
- the pack is larger than `MAX_FILE_BYTES` (64 MiB) (host-read: store/fdtable/types.rs:25).

`build` writes that one form from the files, whatever order they come in, and refuses (never cuts) a package
past 64 MiB or 65536 entries. It allocates the pack fallibly: too little memory is a refusal the installer
says as UNPACK_MEMORY, not a panic. (host-tested)

The parser does no allocation proportional to `count` before it has checked `index_len` against the pack's
length. (design)

### Trust (booted: HANDOFF/catalogue/evidence/pack-2026-10-26)

The installer builds the pack from the apk it has already verified: the signed index, the pin and the
checksums (host-read: install/auth/package.rs). The machine mints a local-sign trailer (mk_local_sign, the
vouch the installer gives every ELF) over the pack's head: the header, the index and the names, which hold
every file's digest. It is kept beside the pack as `/linux-packs/<pkg>.pack.zk_trailer.bin`.

At run time, capsule_linux:
- reads the head of each pack under /linux-packs once, and loads it only if the trailer verifies over it
  (mk_local_verify) and `Index::parse` takes it (file/store/pack/load.rs);
- checks each file's length and 16-byte digest every time it serves it, a read or an exec alike
  (`Index::verify`, file/store/pack/serve.rs). A read that is not the digest is refused, never served cut.

A store tamper after install is therefore refused at load if it touches the head, and at the file if it
touches the file. Each ELF's own trailer is a file in the pack, so exec's gate reads it as it reads a
store file's. The market's verification of the release is unchanged. (the parser and verify
host-tested; the load and serve booted: HANDOFF/catalogue/evidence/pack-2026-10-26)

### How a guest reads a file (booted: HANDOFF/catalogue/evidence/pack-2026-10-26)

capsule_linux's tree (`file/root.rs`) gains a table of mounted packs, one per installed package:
- package name;
- its store object (`/linux-packs/<pkg>.pack`, one vfs name);
- its parsed index, kept in capsule_linux's own heap. The index is 48 bytes an entry plus names; vim-common's
  is about 90 KiB.

`resolve::key` looks a visible path up in the packs' indexes first (binary search, O(log n)), then falls back to
the store's tree for files that are not in a pack: the family's private roots, the home, and /etc written by an
install script.

- **Read:** copy, not mmap. The vfs has no shared-memory read. A guest read of `len` at `off` becomes
  `read_window(pack_data_off + off, min(len, 64 KiB))` (host-read: app_skeleton/src/clients/vfs/stream_read.rs:39,
  MAX_CHUNK = 65536 in stream.rs:21). A read is bounded by the file's `data_len` and by one MAX_CHUNK per call, as
  reads are now.
- **Exec:** capsule_linux reads a program into its own heap, as it reads a store file today (up to 64 MiB,
  start_guest.rs). The bytes come through `Index::verify` like any read.
- **Writes:** none. A pack is read-only. A guest that writes a pack path gets EROFS, as it does on the shared tree
  now (file/root_write.rs).
- **Stat:** the size from the index, read-only. The mtime is 0: a pack keeps no time.
- **readdir:** the index's sorted names under the directory, merged with the store's own names.

The installer's own reads of a package are unchanged. It writes one object, the pack, instead of each file,
except a market program held until its policy is bound: that stays a file of its own with PENDING beside it
(minted.rs), because binding later writes beside it and a pack does not change once in. Links stay lines in the
link table. (booted: HANDOFF/catalogue/evidence/pack-2026-10-26)

## 4. Update, uninstall, and the home

- **Install** (file/store/pack/swap.rs, the order host-tested against a store that stops dead after any step):
  1. Write the whole pack as `/linux-packs/<pkg>.pack.new`. If the store will not take it, the old pack serves on.
  2. Unlink the old pack. If a run holds it open the store answers EBUSY: the `.new` is removed, nothing of the
     package changes, and the install fails as IN_USE (code 38, "close the app to update or remove it").
  3. Rename `.new` to `.pack`. The store's rename never writes over a name, which is why step 2 comes first.
  4. Write the trailer minted over the new head.

  A crash leaves one of: the old pack under its trailer; no pack; the new pack under the old trailer, which does
  not verify and so serves nothing; the new pack under its own trailer. No state serves a file that is not the
  package's. A stray `.new` is no pack's name, nothing loads it, and the next install or uninstall removes it.
  (host-tested)
- **Update:** the same steps. There is no unlink while open (Lead ruling): the vfs refuses unlink, rename and
  truncate of a path under `/linux-packs/` that another process holds open (capsule_vfs
  store/fdtable/held.rs, host-tested in fs_proofs), and opens a new handle on a held pack read-only. Each run
  keeps a handle on every pack it loaded; that handle is its hold, and it goes when the run is reaped. The
  installer lets go of its own handles before it swaps a pack, and loads again after. (host-tested; an update
  while a run holds the pack is not booted)
- **Uninstall:** the pack goes first, then its trailer and any `.new`; EBUSY is IN_USE and nothing of the package
  is taken out. Then its files, its link lines and its record go as today; a packed file is gone once its pack is.
  A package installed before packs, with no pack, uninstalls as before. (built, not booted: these boots
  uninstall nothing)
- **A failed install** takes its pack out with the rest of what it wrote (install/place/undo.rs). (built, not booted)
- **The RAM home** (linux/files-home 4f5df02cd) sits apart:
  - `/linux-home/<key>`, writable, under the 16 MiB, 128-name and 64 MiB quota, keyed by listing;
  - the pack is read-only under /linux;
  - the two never share a name, so the home's quota never counts pack bytes;
  - uninstall erases both the pack and the home, as it erases the home now.

  (host-read: files-home; design)
- **Store use:** two names per package, its pack and its trailer. A market listing's held program adds itself,
  its PENDING and, once bound, its policy and their trailers. vim's install took the store from 231 names to
  255, where its files would be 1940 (booted: HANDOFF/catalogue/evidence/pack-2026-10-26). The 512-name owner
  budget then holds every listing in the catalogue at once: the largest closure, profanity's, is 33 packages.
  (measured; design)
- **Memory:** a pack is the package's bytes once, as today's files are. The install heap must still hold one
  unpacked apk at a time. vim-common peaked at 51 MiB of 320, packed too (booted).

## 5. The fuzz target and the mutants

`fuzz/pack_parse` in nonos_linux_pack, which has the format, the parser, the builder and verify, no I/O, and is
`no_std`; capsule_linux depends on the same crate. Two runs, 33.9M and 12.8M executions, ended with no failure
after the canonical-layout and directory-digest refusals were added; the first runs found both. The seed corpus
is written by `cargo run --example seeds` (small packs and their truncations), not committed. (host-tested)

- **Input:** arbitrary bytes, up to 1 MiB.
- **Calls:** `Pack::parse(bytes)`, then, if it parses, `lookup(name)` for each indexed name and for 64 mutated
  names, and `file(entry)` for each entry.
- **Properties asserted:**
  - parse never panics;
  - any accepted pack round-trips through the builder byte for byte;
  - each lookup finds exactly the entries whose names are equal;
  - every returned slice lies inside the input;
  - no two files' data overlap;
  - no name has `..` or a leading `/`.
- **Host mutants** (verification/host_mutants/linux-pack.json, `tools/nonos-host-mutants`; all 7 killed):
  - a held pack taken as gone;
  - the `.new` left behind when a swap is refused;
  - a pack the store would not take swapped anyway;
  - the trailer left when a pack is taken out;
  - a file's digest unchecked on serve;
  - a gap byte unchecked;
  - the IN_USE reason unsaid.
- **The catalogue's pack size** (tools/nonos_market_catalogue/pack_size.py): three mutants (the trailers
  uncounted, the 64-byte alignment dropped, the bound off by one) each fail test_pack_size.py. (host-tested)

## 6. What is not in this unit (design)

- Packs on the data volume. Packages stay RAM-only, as ruled.
- Sharing one pack's bytes between two installs of different releases.
- mmap. There is no shared-memory read, and copy matches today's per-read bound.

## 7. 64 MiB is enough (measured)

The format's bound is versioned in its header: a later version may raise it. Version 1's 64 MiB holds every
package measured, by far:
- python3 3.12.13: 779 names, 23.6 MB unpacked. Its whole closure is 916 names and 35.4 MB, over several packs.
- The largest single package is vim-common, 26.2 MB.

A package whose pack would pass 64 MiB, or 65536 entries, is refused when its listing is built: the catalogue
builder computes the pack's length exactly as `build` lays it out, trailers included, and leaves the listing out
with the reason said (pack_size.py; its 9288-byte vector was read off `build` for the same files). A package
named at the Terminal that would pass it is refused at install as PACKAGE, with the reason in the log; nothing of
it is written. (host-tested)

## Open questions

None open. The Lead ruled on both: 64 MiB stands (section 7), and there is no unlink while open (section 4).
Privacy: none. Nothing here reaches a disk.
