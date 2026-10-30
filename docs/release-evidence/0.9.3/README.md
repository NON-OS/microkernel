# Release evidence for NONOS 0.9.3

One line for each of the 56 directories here: what it proves, taken from that
directory's own README. Three directories have no README; their line says what
the files are and which commit added them.

Unless a line says otherwise, a boot here is a development image (throwaway
keys, path-only attestation) under QEMU TCG, not a sealed release image and not
real hardware. Where a README names a serial log that is not committed beside
it, the line says so: `*.log` is ignored by `.gitignore`, and only some logs
were added by name.

## Boots of main

- [boot-ad2a58a01](boot-ad2a58a01/README.md): main at ad2a58a01, standard-dev:
  the default machine reaches setup with a DHCP lease and `[HW] 11 driven`;
  the e1000e, igb, e1000 and rtl8139 cards each get a lease; the five Wi-Fi drivers
  say `no controller present, not spawned`. The serial logs it names are not
  committed.
- [gate-a7af21a70](gate-a7af21a70/README.md): the first 0.9.3 gate boot,
  qemu-dev, 4 CPUs, Intel IOMMU: `enforcing=1, unconfined grants=0`, all 13
  setup steps to the desktop, 55 `[ZK-ATTEST] ok`, no FATAL or `[LOCK]`.
- [gate-23d57ac60](gate-23d57ac60/README.md): the second gate, standard-dev:
  IOMMU enforcing, `[SMP-PROOF] cpu_count=4 PASS`, 67 `[ZK-ATTEST] ok`, the 13
  setup steps to the desktop; one console line split by another CPU.
- [gate-6f0bbe716](gate-6f0bbe716/README.md): the third gate, standard-dev:
  IOMMU enforcing, 4 CPUs, 67 `[ZK-ATTEST] ok`, no `unanswered`, trap, panic or
  FATAL line, and no kernel line starting mid-line.
  of 26 October, from their serial excerpt: lock waits on every CPU, mappings
  kept after exit and a kernel heap OOM to the panic path. Marked NO-GO, with
- [regression-30a4b58c4-paging-lock](regression-30a4b58c4-paging-lock/README.md):
  main at 30a4b58c4 printed 12 and 80 `[LOCK]` lines in two boots, every
  waiter in the paging manager; a boot with a UHCI controller on the same image starts
  `driver.uhci0`.

## Kernel and isolation

- [s1-paging-lock-c3f522575](s1-paging-lock-c3f522575/README.md): the
  paging-lock fix (adb53a20d, abae61bc9) with MkGetPid put back: 0 `[LOCK]`
  lines where the same build printed 12 and 80 before.
- [s1-lock](s1-lock/README.md): six boots of the lock fix at 4 and 8 CPUs on
  a macOS host, and a Monero install and run: 0 `[LOCK]` and 0 `unanswered`
  in every boot.
- [s1-isolation-61ec29ea2](s1-isolation-61ec29ea2/README.md): a hostile Linux
  program's probes (ptrace, process_vm_readv, module load, kexec, mount, a
  W^X mapping, raw sockets, a fork bomb) each refused, on an image of a
  branch with MkGetPid on top, which this release does not carry.
- [s1-shared-maps-0d0cff02d4](s1-shared-maps-0d0cff02d4/README.md): a stored
  file mapped `MAP_SHARED`, as LMDB maps one: read and write maps served, one
  writer per range, an executable shared map refused, and the writes outlive
  the process.
- [ksec-6e1472210b-console-lines](ksec-6e1472210b-console-lines/README.md):
  every `[USER-FIRST]` line whole and at column 0 across 4 CPUs, 64 of 64,
  where the boot before had 5 of 64 behind another writer.
- [ksec-c8a952663-iommu-intel](ksec-c8a952663-iommu-intel/README.md): the
  load and tool-run bounds booted: tools, an installed proof capsule and a
  Launchpad app spawn with their own masks, and no `[CAP-DENY]` line.
- [ksec-d2a9d0bfa-iommu-intel](ksec-d2a9d0bfa-iommu-intel/README.md): the
  selftest's `attack suite: PASS`, with the open-with deputy escape refused;
  tools and an app spawn on qemu-dev with no `[CAP-DENY]`.

## Installing and updating

  with a software TPM: image A installed and prepared, Update NØNOS (keep my
  data) from image B with a changed bootloader, then the data volume opens
  under B through the handover and, on the next boot, under B's own key.

## Linux programs, installs and the Monero wallet

- [s1-g-kvm](s1-g-kvm/README.md): on a KVM host, the Monero wallet
  syncs from the listing's node over Anyone and over Nym; why a real-mirror
  install failed (one stream per caller), and the spawn fix that gives an
  install its 320 MiB heap.
- [s3-boot1](s3-boot1/README.md): standard-dev with 28 catalogue apps: age
  installs from the relay and its three runs exit 0; the passphrase showed as
  typed (fixed later); the store's proof timed out; later installs starved.
- [s3-boot2](s3-boot2/README.md): age with the passphrase hidden, flac,
  minisign, git, vim, yara and curl run in their boxes; gpg, lame and radare2
  fail; the machine slowed after about 4 h 20 min.
- [s3-boot3](s3-boot3/README.md): gpg past "disable core dumps" with the
  prlimit fix; the store's proof still timed out at 30 s; the Monero install
  took over an hour beside a prove job.

## Linux programs in windows

- [gui-swayimg](gui-swayimg/README.md): swayimg, the first Wayland program
  drawn on NONOS: window, image, zoom and close, after four faults were fixed
  and booted again.
- [gui-frame](gui-frame/README.md): a Wayland window inside the NONOS frame;
  its close button ends swayimg with status 0, and maximise keeps the title
  bar. Drag and minimise are not shown.
- [gui-keepassxc](gui-keepassxc/README.md): KeePassXC 2.7.8 (Qt 5) draws its
  welcome screen in the frame, in a box of WAYLAND and FILES_HOME, once
  FUTEX_WAKE_OP was served.
- [gui-telegram](gui-telegram/README.md): Telegram Desktop 5.0.1 (Qt 6) draws
  its welcome screen in the frame, in a box of WAYLAND, NET_ANON and
  FILES_HOME, once `/proc` answered a thread's own number (b57b5d210e); its
  connections reach the box's SOCKS5 port and are refused, as that machine
  reached only the package mirror. A sign-in is not shown.
- [gui-foot](gui-foot/README.md): foot draws its window and runs a shell over
  `/dev/pts`, on a development image with foot's test recipe; foot is listed
  since afa860e173.
- [gui-large-installs](gui-large-installs/README.md): 304 packages verified in
  one boot (swayimg, KeePassXC, gnome-authenticator) with no stall, `[LOCK]`,
  OOM or panic; no window opened in that boot.
- [gui-store-tty-open](gui-store-tty-open/README.md): the Marketplace's Open
  of a terminal program runs it in the Terminal; micro then stops on
  `open /dev/tty`.
- gui-install-stall (no README): `serial.log`, `stuck.txt` (QEMU's registers
  for every CPU) and `zathura-xkb-enosys.png`, added by 7a229a3ebb: zathura's
  82-package install stopped at its 81st with every CPU in the kernel, one in
  `HoleList::allocate_first_fit` holding the heap lock.

## The desktop and display

- [gpu-469dddb58-no-iommu](gpu-469dddb58-no-iommu/README.md): with no IOMMU,
  `posture=dma-unconfined` on serial and `device DMA not confined by
  hardware` in red on the splash. The serial log it quotes is not committed.
- [gpu-469dddb58-iommu-intel](gpu-469dddb58-iommu-intel/README.md): the same
  image with VT-d enforcing shows no red marking. The serial log it quotes is
  not committed.
- [gpu-508ca4d90-iommu-intel](gpu-508ca4d90-iommu-intel/README.md): no
  `ipc.call unanswered` line, and the splash panel says `path only
  (development)` and `UNVERIFIED`, with no firmware logo. The serial log it
  quotes is not committed.
- [gpu-508ca4d90-no-iommu](gpu-508ca4d90-no-iommu/README.md): the same image
  without an IOMMU: the red marking, the honest panel, no unanswered line. The
  serial log it quotes is not committed.
- [gpu-68dc19026-iommu-intel](gpu-68dc19026-iommu-intel/README.md): MkBootLogo
  retired, its number answering EINVAL, and the gate boot still reaches the
  desktop. The serial log it quotes is not committed.
- [gpu-bdd4278c2-iommu-intel](gpu-bdd4278c2-iommu-intel/README.md): the
  Settings card notes wrap to the card's width (74f56105e). The serial log it
  quotes is not committed.
- [gpu-e3b94f50d-iommu-intel](gpu-e3b94f50d-iommu-intel/README.md): the menu
  bar follows focus, one Terminal process serves its window, and no late
  virtio-gpu FLUSH. The serial log it quotes is not committed.
- [gpu-f04a17623-iommu-intel](gpu-f04a17623-iommu-intel/README.md): the
  indicator band's glyphs, and Settings' Network page with no empty address
  row on QEMU. The serial log it quotes is not committed.
- [gpu-fde2defc9-smp1-iommu-intel](gpu-fde2defc9-smp1-iommu-intel/README.md):
  on one CPU a slow compositor keeps its frame and the driver never clears the
  screen. The serial logs it quotes are not committed.
- [ksec-665d9ca13d-windows](ksec-665d9ca13d-windows/README.md): demo app
  windows hold their content at 1080p and 1024x768, and refused installs say
  why.
- [ksec-74779a915-terminal-gui](ksec-74779a915-terminal-gui/README.md): a GUI
  app installed from the Terminal draws its own window, where before the
  Terminal marked it failed after 5 s.
- [ksec-1d5ef69e84-browser-palette](ksec-1d5ef69e84-browser-palette/README.md):
  2048's tiles readable, and the browser refuses a `file://` address with a
  reason.

## The Terminal

- terminal-00-baseline (no README): six screenshots of the Terminal at main
  2c7f9d08 before the changes below, which the terminal-01 to 04 READMEs name
  as their "before".
- [terminal-01-search](terminal-01-search/README.md): Esc leaves a history
  search with the line put back, and Enter on no match runs nothing.
- [terminal-02-hints](terminal-02-hints/README.md): the hint bar names the
  keys of the state the reader is in.
- [terminal-03-geometry](terminal-03-geometry/README.md): opens at 1240x720
  centred on a 1080p screen, with the prompt under the last output.
- [terminal-04-ctrl-alt-t](terminal-04-ctrl-alt-t/README.md): Ctrl+Alt+T opens
  or raises the Terminal and no focused window sees the keys.
- [terminal-05-prompt](terminal-05-prompt/README.md): the prompt shows the git
  branch and the background jobs.
- [terminal-06-contrast](terminal-06-contrast/README.md): every theme's text
  at WCAG AA, 4.5:1.
- [terminal-07-tab-list](terminal-07-tab-list/README.md): Tab's candidates as
  a list beside the prompt that the next key clears.
- [terminal-08-emblem](terminal-08-emblem/README.md): the slashed-zero emblem
  in 3D on the splash and in neofetch.
- [terminal-09-market](terminal-09-market/README.md): `market list` prints as
  pacman prints a search.
- [terminal-10-theme-picker](terminal-10-theme-picker/README.md): `theme`
  alone opens a picker that tries each theme on the window.
- [terminal-11-first-run](terminal-11-first-run/README.md): the first screen
  names where to start.
- [terminal-12-latency](terminal-12-latency/README.md): `latency` times key to
  screen: 68 ms middle, 86 ms at 95% under TCG with whole repaints.
- [terminal-13-split](terminal-13-split/README.md): two tabs side by side in
  one window.
- [terminal-14-gate](terminal-14-gate/README.md): the F12 drop-down, row-only
  repaint (5 ms middle under TCG), tab pills and the dimmed pane, on a gate
  boot with VT-d.

## Drivers

- [drivers](drivers/): one folder a card. e1000, e1000e, igb, pcnet, rtl8139,
  tulip and vmxnet3 each get a DHCP lease through the card on QEMU with only
  its own driver spawned, on standard-dev at 4b1eeb014 (the serial logs the
  READMEs name are not committed). rtl8821ce holds the release report that
  Wi-Fi works on their HP laptop, on their seal of main 680c92e3fc, with no
  log or screenshot.
