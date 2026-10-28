# NONOS 0.9.3 (pre-release)

0.9.3 follows the 0.9.2 pre-release. Linux programs can now open windows on the desktop through
a Wayland bridge, the Monero wallet syncs over Nym and Anyone, Linux programs get
pseudo-terminals, and the standard image carries Wi-Fi drivers for more chips. A disk installed
by 0.9.3 can be updated in place to a later release and keep its encrypted data volume. Each
claim below names the file, commit or evidence directory it rests on. The evidence for this
release is indexed in `docs/release-evidence/0.9.3/README.md`.

It is a pre-release. What it does not do yet is listed under [Known limitations](#known-limitations),
and it should be read before you install it on a machine you care about.

## What you download

| File | Made by | What it is |
|---|---|---|
| `nonos.img`, `nonos.iso` | the release seal, on the signing machine | The bootable image, its kernel and loader signed (Ed25519 and ML-DSA-65). |
| `nonos-release-bundle.tar.gz` | CI, from a reproducible build | The kernel ELF, the UEFI loader, the capsule catalogue, the SBOM, every trust root and the STARK trailer of every enrolled capsule. |
| `BUNDLE-SHA256SUMS`, `BUNDLE-BLAKE3SUMS` | CI | The bundle's own sums, over the files inside the tarball. |
| `verifier.wasm` | CI | The STARK verifier the kernel's gate links (`nox_verify`), for checking proofs in a browser. |
| `proof-verify-web.tar.gz` | CI | The proof verifier's page (`nonos-proof-verify` in wasm): checks the release's committed attestation proofs the same way the boot gates do, in a browser tab. |
| `nonos.cdx.json`, `nonos-build.json`, `provenance.json` | CI | Bill of materials, build record with every input hash, and the bundle's provenance. |
| `SHA256SUMS`, `BLAKE3SUMS` | CI | Sums over exactly the CI files above. |
| `IMAGE-SHA256SUMS`, `IMAGE-BLAKE3SUMS` | the release seal | Sums over `nonos.img` and `nonos.iso`. |

The CI files are the ones `.github/workflows/ci-release-artifacts.yml` uploads, and the release
job publishes all of them. CI holds no signing key and signs nothing. The image is the same build
CI makes, plus signatures.

## Verify it

```
sha256sum -c SHA256SUMS && sha256sum -c IMAGE-SHA256SUMS
b3sum -c BLAKE3SUMS && b3sum -c IMAGE-BLAKE3SUMS
gh attestation verify nonos-release-bundle.tar.gz --repo NON-OS/microkernel
```

Rebuild it yourself and compare: `nix build .#standard` at this tag gives the kernel, loader and
capsules in the bundle, byte for byte, on x86_64 Linux and on Apple silicon macOS. The
`reproducible` CI lane builds on both and compares every file.

## What is new since 0.9.2

### Linux programs in their own windows

The Linux personality carries a Wayland bridge. A program whose box grants WAYLAND opens its own
window on the NONOS desktop, inside the NONOS frame: title bar, close, minimise and maximise
(53d8f03bf4, 272b635d32). DejaVu fonts are stored for window programs (ca7275f90).

- **swayimg** draws its window, zooms and closes (`gui-swayimg`). The frame's close button ends
  it with status 0, and maximise keeps the title bar (`gui-frame`).
- **KeePassXC** 2.7.8 (Qt 5) draws its welcome screen in the frame, in a box with no network,
  once the personality served FUTEX_WAKE_OP, which Qt's semaphore needs (ceb0f0ee6a,
  `gui-keepassxc`).
- **Telegram Desktop** 5.0.1 (Qt 6) draws its welcome screen in the frame on a development
  image, in a box of WAYLAND, FILES_HOME and NET_ANON, once `/proc` answered a thread's own
  number (71e34654ac, `gui-telegram`). A sign-in is not shown: that test machine reached only the
  package mirror.
- **foot** 1.17.2 is listed in a box of WAYLAND, TTY, FILES_HOME and EXEC (fbbd8c5f80). It drew
  its window and ran a shell over `/dev/pts` on a development image (`gui-foot`).
- **mpv** 0.38.0, drawing to `wl_shm` with no GPU (`--vo=wlshm`), and **Mousepad** 0.6.2, a GTK
  text editor, are listed (fbbd8c5f80). **zathura** 0.5.6 is listed with its mupdf PDF plugin,
  which the listing adds to the install (c7bc7960c4, 1a48cc9bef). For these three, no boot in the
  evidence shows a window.
- Large installs hold: 304 packages verified in one boot, KeePassXC (126) and
  gnome-authenticator (155) among them, with no stall, lock wait or OOM (`gui-large-installs`).
- A run's private directories (`/tmp`, `/run/user/0`) hold 1536 names, where they held 128.
  GTK's mime database is built whole there before the program starts and kept as its caches,
  fifteen names, so GTK and Qt programs no longer stop on ENOSPC (b7521ace2b).
- GTK's preparation tools run before the program's guest is made, so a GTK program's run is
  noted to the seal. Before, every GTK program's run went unnoted and unkept, with "the seal
  refused the app's note" (fd2a80e365). A KVM run on that commit drew KeePassXC 2.7.8's window
  with no ENOSPC and no seal refusal; no log of that run is committed.
- A program that asks EGL for OpenGL is pointed at Mesa's software driver, drawing through
  Wayland, where its box's package closure carries that driver (`LIBGL_ALWAYS_SOFTWARE`,
  `EGL_PLATFORM=wayland` in `linux/env.rs`; 1f56b890a6). A surface's frame callbacks are answered
  at most 30 times a second, and at once for an idle window (`wayland/pace_rule.rs`; 664185e8a8).

### Pseudo-terminals

A Linux program can open `/dev/ptmx` and `/dev/pts/N`, as a terminal emulator does before it
starts its shell: line discipline, signals, window size and hang-up (1c81ded90a). A family holds
at most 64 pairs (`pty/table.rs`, `MAX`). A master that writes with no reader stops at 4096
pending bytes or lines on the slave and 64 KiB of output, and keeps each signal it raises once
(ec07d31412, with two flood proofs).

### A Monero wallet over Nym and Anyone

`linux.monero` starts `monero-wallet-cli` against the listing's node,
`xmr-node.cakewallet.com:18081`, with `--untrusted-daemon`, and keeps its log to 1 MiB in one
file (`linux-catalogue.json`; 1addaf292b). On a KVM host the wallet synced from that node over
Anyone and over Nym, on development images (`s1-g-kvm`). The serial log shows the streams; the
progress in the window is quoted from the notes of those runs. These fixes made it work:

- net.anon picks an exit whose policy takes the stream's port, as the wallet's 18081 needs, and
  sends a refused open once more to another exit (b5876fcae6).
- A spawned process holds its arguments from its first instruction, so an install reads them in
  time and takes its 320 MiB heap, not a program's 160 MiB (b5bb69a407). Every tree in
  `s1-g-kvm` with the fix installs at `of 327680 KiB`.
- `get_robust_list` is answered, so LMDB can make its lock and the wallet's ring database opens
  (e16e16d6b4).
- A stored file mapped shared, as LMDB maps one, is served, one writer per range
  (373e35a4d5, `s1-shared-maps-0d0cff02d4`).

### Installing Linux apps

Every install goes over Anyone, and both proxies keep one stream per caller. The installer had
opened three at once, so each took the conversation from the one before and no package arrived.
It now fetches one package at a time over its kept stream (9fc7634b39, proven in
`install_window_tests.rs`). Before the fix, a lone stream brought the 24.7 MB Monero package
from the real Alpine mirror whole (`s1-g-kvm`, `diag-694ba69799-realmirror-anyone.log`).

A failed install says which package and which check stopped it, for example
`libzmq-4.3.5-r2 (its checksum did not match the signed index)`, in the Terminal's `market`
commands and the Marketplace's headline (e3bed0a8f0).

An install holds one package about twice its installed size, its tar and its pack, where it held
it three times; the tar walk borrows each file from the tar. Before a package is unpacked, the
installer checks that it fits the heap left, and a package that cannot fit ends the install with
the reason "Too little free memory to unpack one of its packages; close other apps, then retry"
and the package's name, never a fault. An install takes 640 MiB where 2 GiB is
free, then 320 MiB, then 160 MiB (f03a6abe69). On the owner's 8 GB HP laptop, Telegram Desktop's
install had stopped with code 134; in a KVM run with 4 and 8 GiB guests, Telegram's own package
peaked at 143 MiB.

A need that several packages provide now takes the provider an earlier install placed, where
there is one, instead of the index's first (5dcb276cc4). Telegram Desktop's install after
KeePassXC's had been refused: it took `icu-data-en`, which holds the same file as the
`icu-data-full` KeePassXC placed. In a KVM run, Telegram's install after KeePassXC placed
`icu-data-full` again, with no refused member.

One file a Linux app writes may now fill its home, 16 MiB, where it was held to 8 MiB. A write
past it fails with EFBIG, and the first refusal in a run prints `linux: this app wrote a file
past 16 MiB, the most one file may hold here; that write was refused.` in the app's window
(38f906c9c7).

The file store, which holds installed Linux apps in memory, may give its clients' files a
quarter of memory, between 160 MiB and 4 GiB, and halves its ask when a heap cannot be mapped
(f8f2f81c74, `capsule_vfs/src/store/fdtable/budget_size.rs`). A program may be up to 128 MiB
(36310ba533; `linux/program_max.rs`, and `MAX_ELF` in
`src/syscall/microkernel/local_image/digest.rs`).

### The Marketplace

The catalogue (`userland/capsule_market/linux-catalogue.json`) holds 30 recipes. A release
image lists its 26 Alpine apps, each in its own box, among them KeePassXC, Telegram Desktop,
Mousepad, foot, mpv, zathura, swayimg, the Monero wallet, Profanity, age, git and vim. The
Marketplace's Open of a terminal program runs it in the Terminal (c55d58e10,
`gui-store-tty-open`).

The other four are SimpleX Chat's terminal client (`linux.kali.nonos-simplex-chat`, from
SimpleX's signed v7.0.3 binary, d8e427216b) and DarkFi's `drk`, `darkirc` and `darkfid`
(1a17cb72ad). They are served from a NONOS package mirror. A seal lists them only when
`nonos.toml` names a mirror and their built content (`linux_packages`, `linux_sources`;
2c9ecace25), and both are empty, so a release image built from this tree does not list them
(`docs/userland/marketplace-catalogue.md`, "No mirror, none listed").

### Update NØNOS (keep my data)

A disk installed by 0.9.3 or later can be updated in place to a later release and keep its
encrypted data volume. The installer rewrites the boot partition and the store and nothing else
(8587032e4b; `keep_queue` in `userland/nonos_disk/src/session/queue_keep.rs`). A new data
volume's key is random, wrapped under a key the TPM gives only a kernel the release approved, on
this machine's firmware, bootloader and Secure Boot state (85a8d41ac1). A new bootloader changes
PCR 4, so before an update the installed system prepares a one-boot handover: open Install NØNOS
and press P beside `Prepare this computer for an update` (7ff8129c52, 6335608d67). On QEMU with a
software TPM, a volume prepared under one image opened under a second image with a changed
bootloader through the handover, then under that bootloader's own key on the next boot
(`s1-k1-update`). An update keeps the data volume, not store apps, settings since setup, the
wallet's vault or saved Wi-Fi (`docs/install/update.md`).

A USB stick booted beside an installed disk comes up as the stick (9eb8a850bb). On the owner's
HP laptop it had come up as the installed system. The loader takes no other disk's store on a
USB boot and recognises its own disk by its GPT partition GUID. The kernel never keeps another
disk on a USB boot, and `log BLOCK` says why a boot kept the disk it did. In a KVM run, a new
stick beside an installed NVMe disk came up with the stick's store
(`[STORE] the loader's copy of the store: 88369152 bytes`) and setup at its first step, and the
NVMe disk alone came up as the installed desktop. No log of that run is committed.

### Drivers

- **Wired, on QEMU.** The e1000, e1000e, igb, RTL8139, PCnet, vmxnet3 and Tulip drivers each got
  a DHCP lease through their card, with net.core reading it in batches (`drivers/`). That is the
  0.9.2 receive fault of e1000 and RTL8139 shown fixed on a boot. The RTL8169 fix is proven on
  the host only.
- **Wi-Fi on hardware.** The owner reports RTL8821CE Wi-Fi working on their HP laptop, on their
  seal of main 8f2d183dda (`drivers/rtl8821ce`). It is their report only: no log or screenshot
  was sent. Two vendor
  drivers may now share an isolation group on a standard boot, which let that card be claimed
  beside the NVMe disk (82ee1a3110).
- **New Wi-Fi drivers, modelled only.** The standard image carries drivers for the RTL8822CE,
  RTL8852BE, MediaTek MT7921, MT7922 and MT7920, and Intel BE200 and BE201, beside RTL8821CE and
  the Intel SO cards (`docs/drivers/wifi/README.md`). None has run on its card.

### Setup and the desktop

- First-boot setup has 13 steps (`capsule_setup_wizard/src/render/layout.rs`, `STEPS`), and
  each gate boot of this tree walked all 13 to the desktop (`gate-a7af21a70`). Setup, About and
  the Terminal read the release from `VERSION`, which says 0.9.3 (f68bb8ac3a).
- Power Off in the desktop's menu, and the Power key, ask "Power off and wipe memory?" and then
  power off through the wipe (`capsule_power`; `docs/using/desktop.md`). A development boot
  held a secret in RAM 8 times before Power Off and 0 times after
  (`docs/security/privacy-evidence/2026-10-25/zs-6d87d9aff`). The installers' restart goes
  through the same service and they hold no Admin
  (`docs/security/privacy-evidence/2026-10-26/s14-653523a51`).
- With no IOMMU in service, the splash says `device DMA not confined by hardware` in red
  (`gpu-469dddb58-no-iommu`).

### The Terminal

History search, a hint bar that follows the state, Ctrl+Alt+T, the git branch and jobs in the
prompt, WCAG AA contrast on every theme, a theme picker, split view, the F12 drop-down and a
`latency` command, each booted (`terminal-01` to `terminal-14`). A password prompt is not shown
as it is typed (717a125504).

### Boots of this tree

Three gate boots of main (a7af21a70, 23d57ac60, 6f0bbe716) reached the desktop on 4 CPUs with
VT-d enforcing and no unconfined grant, with no FATAL, panic or lock wait (`gate-*`). The
paging-lock waits seen on 30a4b58c4 are gone after the paging-lock fix (adb53a20d, abae61bc9):
0 `[LOCK]` lines in eight boots at 4 and 8 CPUs (`s1-paging-lock-c3f522575`, `s1-lock`).

## Breaking changes

- `MkBootLogo` (MBGL) is retired. Its number is kept and answers EINVAL to every caller
  (`src/syscall/microkernel/retired.rs`).
- A disk made before 0.9.3 cannot be updated in place. Those releases keyed the data volume to
  their own kernel, and the installer says `NØNOS keyed to its own kernel: its data cannot be
  kept` (`MadeBefore` in `userland/nonos_disk/src/keep/why.rs`). Install 0.9.3 over it; nothing
  carries over from the old disk.

## Known limitations

- **Linux apps keep nothing past power off.** A home is held in memory, up to 16 MiB and 128
  names, and is gone at power off. A home kept across a reboot is designed and not in this
  release (`docs/design/kept-homes-and-installs.md`). One file an app writes may fill its home
  and no more (`MAX_FILE` in `file/held/cache/table.rs`).
- **Monero.** Saving a large wallet cache, as the wallet does when it exits after a sync, can
  fail: the cache can pass the 16 MiB its home holds (`HOME` in `file/system/declared/home.rs`).
  On every wallet open the first refresh fails with "net.socks5 sent an answer this cannot read",
  and the next connects. An install from the real mirror with one package at a time is not yet
  shown (`s1-g-kvm`, "Not shown here"). Write the seed on paper; it is the only copy that
  survives power off.
- **Update NØNOS (keep my data)**: from the prepare until the next boot, an approved kernel opens
  the data volume on this machine whatever bootloader measured it, so prepare right before the
  update (`docs/security/device-secrets-and-keys.md`, "The handover").
- **Installs fetch one package at a time** (9fc7634b39).
- **Not in this release:** SimpleX's desktop window and DarkFi's GUI. SimpleX comes only as its
  terminal client, and only on an image sealed with a NONOS package mirror (above). No boot has
  shown it reach an SMP server (`docs/using/simplex.md`).
- **Linux windows**: Mousepad, mpv and zathura are listed, but no boot in the evidence shows
  their windows. Copy and paste between a Linux window and the desktop is tested on the host
  only.
- **Very large installs**: installing a very large app, such as Telegram, can fail partway on
  the sealed image, when the network service runs out of memory after a few hundred downloads.
  It restarts on its own, and retrying the install works. A fix is in the follow-up release
  (884cd0faae, 23edef7e60).
- **Wi-Fi**: only the RTL8821CE has run on a card. The RTL8822CE, RTL8852BE, MT7921 family and
  every Intel card have run only against a modelled device (`docs/drivers/wifi/README.md`).
- **Seen on development boots, not shown fixed:** lame ends on signal 11 (`s3-boot2`); micro
  opened from the Marketplace stops on `open /dev/tty` (`gui-store-tty-open`).
- **Hardening**: no KASLR, no compiler stack canaries, and the package store is not encrypted.
  A machine with no IOMMU in service boots Standard with device DMA unconfined, said on the
  splash and in About. See `docs/release/0.9.3-security-limits.md`.
- **aarch64** is a preview with no loader and no release image; **riscv64** is not supported,
  and under QEMU it stops at its unported VM layer (`docs/architectures/README.md`).
- **Not yet tested on a booted image**: Secure Boot enabled, and an update from one release to
  the next. The update was tested between two development images of one tree (`s1-k1-update`).
