# GUI 01: swayimg, the first Linux window program drawn on NØNOS, booted 27 October 2026

swayimg 2.2 (Alpine v3.20, MIT), installed from the Marketplace as
`linux.swayimg`, opens its own window on the NØNOS desktop, draws the image
and its text, zooms on a key, and closes on `q`. No Wayland client had been
seen drawing on a boot before this.

## How it ran

- Image: `make dev-image PROFILE=standard` at 934291c72 on s2/gui
  (standard-dev, throwaway keys, never a release).
- `tools/nonos-mirror-relay --cache target/mirror-cache`, run inside
  `nix develop` (it needs b3sum), serving the lock-pinned swayimg .apk.
- QEMU TCG (no KVM), 4 CPUs, 8 GiB, `--tpm`, fresh disk, 1920x1080, driven
  by `tools/nonos-drive` with `drive.steps`: setup by keys, Ctrl+Alt+T,
  `market install linux.swayimg`, then
  `linux swayimg /usr/share/icons/hicolor/64x64/apps/swayimg.png`,
  `=` twice, Right, a click, `q`.

## What it showed

- `swayimg-window.png`: `[LINUX] policy bound: from=market ... mask=0x9
  WAYLAND FILES_HOME`, then `[WAYLAND] window 800x600 at 624,283, focused
  true`. The window holds the 64x64 PNG, its info text in DejaVu (file name,
  format, size) and `100%`.
- `swayimg-zoom-121.png`: two `=` presses, the image larger and `121%`.
- `swayimg-closed.png`: `q`; `[LINUX] process 2 exited, status 0`, the
  window gone, the command marked done.
- The window is see-through: swayimg 2.2's own default window colour is
  #00000000, and the compositor blends its ARGB as sent.
- No `[WAYLAND] unserved` line; no `[CAP-DENY]`. One refusal, said and
  harmless: `inotify_init1` (the store sends no change events).
- The 25-package closure installed with
  `[LINUX] provenance Verified: index signature and checksums match` for each.

## What it took: four faults, each fixed and booted again

1. `before-1-no-font.png`: SIGSEGV at swayimg 0x5ab2, `face->size` of a
   FreeType face never loaded. The guest had no font. ca7275f90 stores DejaVu
   (Sans, Sans Bold, Sans Mono, Serif) at /usr/share/fonts/dejavu.
2. `before-2-wl-output-v3.png`: swayimg binds wl_output at 3; 2 was offered.
   00b4375bf offers 4 (release, name, description).
3. `before-3-shm-enosys.png`: `Unable to map shared file: [38]`. Its pool is
   shm_open + ftruncate + mmap(MAP_SHARED). 835df51d3 makes such a /dev/shm
   object a memfd when mapped shared.
4. `before-4-cmsg.png`: after the first frame, a fault in libwayland's
   decode_cmsg loop at the stack top. recvmsg on the display left
   msg_controllen as the client set it when no descriptor was owed.
   934291c72 sets it (and msg_flags) on every call, as Linux does.

## Not yet shown here

- Resize by the window manager and a pointer drag (the click reached the
  window; swayimg does nothing visible on a click).
- Scale other than 1 (wl_output says 1; no fractional scale is offered).
- A frame or title bar around the window: swayimg draws none and asks for
  no decoration.
