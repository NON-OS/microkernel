# Design: OpenGL in a Linux box, drawn on the CPU

Status: approved by the owner on 28 October, in code (S1). 0.9.3 if it is done
properly, else 0.9.4. Host-read at main 680c92e3fc; bare paths are under
`userland/capsule_linux/src/linux/`. `mesa:` is Alpine v3.20's Mesa 24.0.9,
read at tag mesa-24.0.9; `darkfi:` is DarkFi at master on 2026-10-28 (v0.5.0,
S3's build, where said); `mq:` is narodnik/miniquad d50fcec, the commit
DarkFi's master locks (v0.5.0's a48953a has the same lines). Sizes are v3.20
x86_64 APKINDEX: S fetched, I installed.

## The promise

DarkFi's GUI, and later mpv's GL output, opens a window in its box and draws
with OpenGL ES 2. Mesa renders on the CPU into the wl_shm buffers the
personality already composes. The box needs no GPU, no DRM node, no JIT
grant and no platform bound raised, and it sees nothing it did not see
before: the pixels are its own.

## What is there

- **The display is wl_shm only.** ARGB8888 and XRGB8888 (`wayland/shm.rs:31-35`);
  a pool must lie over a descriptor the client has already mapped
  (`shm.rs:47-50`). A commit presents, releases the buffer and answers every
  frame callback at once (`wayland/commit.rs:34-57`). No wl_drm, no dmabuf,
  no `/dev/dri`; `env.rs:39-49` draws GTK and Qt without GL, "which no guest has".
- **Mesa falls back by itself.** `eglInitialize` tries the hardware path, then
  Zink unless `GALLIUM_DRIVER` is set, then software (`mesa: src/egl/main/eglapi.c:695-710`);
  `LIBGL_ALWAYS_SOFTWARE` goes straight to software (`:678-679`). On Wayland
  the hardware path stops when the compositor offers neither dmabuf feedback
  nor wl_drm (`src/egl/drivers/dri2/platform_wayland.c:2136-2141`). The
  software path binds wl_shm alone, needs one format it knows, loads the
  driver named `swrast` (`:2756-2771`, `:2815-2821`), and draws into a memfd
  it maps before making the pool (`:2324-2337`), which is what `shm.rs` asks.
  Each swap copies the frame in and damages the whole surface (`:2459-2504`).
- **How the driver is found.** `LIBGL_DRIVERS_PATH` replaces the built-in
  directory (`src/loader/loader.c:783-791`); the file is `<dir>/swrast_dri.so`
  (`:807`). `MESA_LOADER_DRIVER_OVERRIDE` is read only on the DRM path
  (`loader.c:678-681`) and, in `eglInitialize`, only as `zink`
  (`eglapi.c:684-685`): it cannot pick softpipe. Alpine builds without glvnd
  (`meson_options.txt:362`, default false; not set in aports
  `main/mesa/APKBUILD`, 3.20-stable), so no EGL vendor JSON is read. The
  platform comes from `EGL_PLATFORM`, then the display pointer, then the
  build default (`src/egl/main/egldisplay.c:161-174`): x11 for Alpine's
  `-Dplatforms=x11,wayland`.
- **llvmpipe.** `swrast` is softpipe, plus llvmpipe only with LLVM
  (`src/gallium/meson.build:62-66`). llvmpipe JITs through MCJIT and LLVM's
  SectionMemoryManager (`gallivm/lp_bld_misc.cpp:558,599`), writing then
  flipping to executable: W^X holds (`call/mem/prot_args.rs:34-36,65-67`) but
  the JIT grant is needed (`call/mem/map_anon.rs:29`, `call/mem/prot.rs:54`).
  It sizes its threads from the CPU count (`llvmpipe/lp_screen.c:1126-1130`);
  a box is told one CPU (`file/system/machine.rs:43-47`).
- **The bounds.** PROGRAM_MAX 128 MiB (`program_max.rs:24`) holds every
  whole-file reader, among them `call/mem/map_exec.rs:30-40`, which reads a
  file whole into the run heap and proves it for each executable mapping
  (`call/mem/map_file.rs:35`). Kernel MAX_ELF 128 MiB, hashed through a bounce
  buffer (`src/syscall/microkernel/local_image/digest.rs:23`, `local_image.rs:29-40`);
  vfs MAX_FILE_BYTES 128 MiB (`userland/capsule_vfs/src/store/fdtable/types.rs:26`);
  a pack 64 MiB (`userland/nonos_linux_pack/src/format.rs:28`, said at
  `install/place/land_said.rs:48`); an inflated apk 128 MiB (`install/unpacked.rs:29`,
  `install/auth/package.rs:35`); INSTALL_HEAP 320 MiB; RUN_HEAP 64 MiB, or
  PROGRAM_MAX + 32 MiB with 2 GiB free (`heap.rs:27,32-38`). An ELF whose
  trailer cannot be minted lands without one (`install/place/packed.rs:57`).
- **RAM.** Packages stay in RAM (`docs/design/linux-package-pack.md`, section 6);
  the vfs budget is a quarter of RAM, never under 160 MiB
  (`capsule_vfs/src/store/fdtable/budget.rs:40`, `budget_size.rs:37`). A run
  loads only its program's closure's packs (`file/store/pack/running.rs:43-46`).
- **Publishing.** In-tree programs are static musl executables built by zig
  (`userland/linux_userland/README.md`). One the image does not carry is
  `linux.nonos-<tool>`, pinned by the BLAKE3 of a content tar served at
  `/linux/<hex>.tar` by `NONOS_PACKAGE_MIRROR` (`install/tools.rs:43-50`,
  `install/tools_install.rs:37-58`); `nonos.toml:47` sets none. DarkFi's CLI
  tools take that path as source recipes; content is regular files only, and
  only Debian content may pin what it needs (`docs/userland/marketplace-catalogue.md:114-127,181`).
- **DarkFi's GUI.** No `darkfi` in v3.20 main or community. bin/app uses
  narodnik's miniquad (`darkfi: bin/app/Cargo.toml:12`), Wayland first
  (`bin/app/src/main.rs:984-994`; v0.5.0 `gfx/mod.rs:1079`). It dlopens
  `libEGL.so.1` (`mq: src/native/egl.rs:51-52`), libwayland-client,
  libwayland-egl and libxkbcommon; asks RGB888 with 16-bit depth and an ES 2
  context (`egl.rs:188-197,240`), swap interval 1 and every GL function
  through eglGetProcAddress (`linux_wayland.rs:1171-1177`), which Mesa answers
  from libglapi (`eglapi.c:2831`): no libGLESv2. Shaders are GLSL ES 1.00, a
  texture fetch times a colour (`gfx/shader/gl_fragment_rgb.frag`). It blocks
  for events only on Android (`gfx/mod.rs:1374-1375`); else miniquad draws
  and swaps every turn (`linux_wayland.rs:1256,1322-1326`). 1024x768 by
  default, high_dpi (`gfx/mod.rs:1358-1370`). It must be dynamic: static musl
  cannot dlopen.

| Package (24.0.9-r1 unless said) | S | I | What |
|---|---:|---:|---|
| mesa-egl | 118 KiB | 288 KiB | libEGL.so.1; needs libgbm, libdrm, libglapi, wayland, X11 and xcb; no driver |
| mesa-gles | 21 KiB | 96 KiB | libGLESv1_CM, libGLESv2 |
| mesa-glapi / mesa-gbm | 47 / 26 KiB | 240 / 68 KiB | libglapi.so.0 / libgbm.so.1 |
| libdrm 2.4.120-r0 | 191 KiB | 532 KiB | needs libpciaccess, so hwdata-pci (1.4 MiB) |
| mesa-dri-gallium | 11.1 MiB | 32.5 MiB | 13 driver names over one megadriver; needs libLLVM-17.so |
| llvm17-libs 17.0.6-r2 | 49.4 MiB | 143.1 MiB | one ELF under four names (pkgs.alpinelinux.org contents) |
| mesa-osmesa | 4.0 MiB | 12.9 MiB | GL frontend and the software drivers, LLVM shared: the size proxy for A |

Closures, resolved from both indexes: Alpine's GL as is (mesa-egl, mesa-gles,
mesa-dri-gallium, wayland-libs-egl) is 29 packages, 64.4 MiB fetched,
187.9 MiB installed. What DarkFi's window needs besides a driver (mesa-egl,
wayland-libs-egl, libxkbcommon, and a driver's libstdc++, zlib, expat) is 26
packages, 4.2 MiB, 15.6 MiB; xkeyboard-config (3.9 MiB) and libx11 (3.0 MiB)
are the largest. mpv 0.38.0-r0's closure is 163 packages, 164.1 MiB, mesa-egl
already in it; with mesa-dri-gallium, 166 and 339.9 MiB.

## The two options measured

**A) A NØNOS softpipe driver, no LLVM.** One file,
`usr/lib/nonos-mesa/swrast_dri.so`, built from the mesa-24.0.9.tar.xz Alpine
builds (its APKBUILD sha512 de2ee6c9...), with Alpine's `musl-stacksize.patch`
and `no-unlink-megadrivers.patch`, and `-Dllvm=disabled -Dgallium-drivers=swrast
-Dvulkan-drivers= -Dplatforms=wayland -Dglx=disabled -Dgles1=disabled
-Dgles2=enabled -Dshared-glapi=enabled -Dshader-cache=disabled
-Ddri-drivers-path=/usr/lib/nonos-mesa`, video and compute front ends off.
libEGL, libglapi, libgbm and libdrm stay Alpine's, pinned, so a closure never
holds two libEGL.so.1 (mpv's has one). Size: not built; by mesa-osmesa, 10 to
14 MiB, one pack far under 64 MiB, read whole into a run heap of 64 MiB at
dlopen. env.rs adds `LIBGL_ALWAYS_SOFTWARE=1` (no DRM try, no Zink try),
`LIBGL_DRIVERS_PATH=/usr/lib/nonos-mesa` and `EGL_PLATFORM=wayland`. Only
apps whose install brought the driver see it; the rest fail as today. No
JIT, no bound, no TCB change.

Speed is the cost. Mesa's own figure: one GL 3.0 test ran at about 36 fps on
llvmpipe and 3.09 on softpipe, Core i5-3317U (Mesa commit 791d06b2, 2015):
a tenth. In a box llvmpipe has one thread, so the gap there is smaller,
unmeasured. DarkFi is the easy case: 2D quads, one texture fetch, a blend.
**Estimate, for the lane to replace:** at 10 to 30 Mpixel/s on one KVM core,
every pixel drawn twice, 1024x768 runs 6 to 19 fps and 1920x1080 2 to 7; a
display the personality scales by 2 (`wayland/output.rs:72`) gives high_dpi
four times the pixels. The two copies a frame (Mesa into the buffer, the
personality into its own) are 8 MiB each at 1080p, small beside drawing.
Nothing paces DarkFi, so it keeps one core busy while open.

**B) Raise the bounds; Alpine's llvmpipe as is.**

| Change | Where | Risk for every other app | Evidence |
|---|---|---|---|
| PROGRAM_MAX, MAX_ELF and MAX_FILE_BYTES to 192 MiB | `program_max.rs:24`, `digest.rs:23`, `types.rs:26` | LARGE_HEAP follows (`heap.rs:37`): every run with 2 GiB free backs 224 MiB up front, 64 MiB more per open app; one file may take most of a 160 or 256 MiB vfs budget | free memory with four apps open, before and after; each whole-file reader tested at cap and cap+1 |
| A pack over 64 MiB | `format.rs:28`: VERSION 2, or one package as several packs | a parser change under every package; v1 packs must still load | the fuzz target rerun; the pack's 7 and the catalogue's 3 mutants killed; every listing's pack BLAKE3 unchanged |
| MAX_INFLATED past 143 MiB | `unpacked.rs:29` | a larger bomb bound for indexes too | tests at bound and bound+1 |
| A streaming unpack | today the apk (49.4 MiB), its data (143, moved: `auth/package.rs:48-52`), a copy of each file (`install/tar_kind.rs:34`) and the pack (`place/land.rs:33`): about 480 MiB of 320. Entries borrowed, the pack written in windows, index first | the all-or-nothing install and `pack/swap.rs`'s crash order, for every package | the order host-tested against a store that stops after each window; the install heap's peak logged |
| Store and run RAM | packages in RAM; budget floor 160 MiB | 188 MiB of closure does not fit a machine under 640 MiB and takes 73% of a 1 GiB one's; each open GL app also holds libLLVM in its heap at each executable mapping and in its frames: about 290 MiB | free memory at install and with DarkFi open, on 1, 2 and 4 GiB guests |
| JIT in the listing | `map_anon.rs:29`, `prot.rs:54` | none for others; DarkFi's box, a wallet on the network, may run any code it writes | the box report names JIT |

Below 2 GiB free a run keeps 64 MiB of heap (`heap.rs:32,38`), so libLLVM can
never map there, whatever the bounds. One CPU per box would leave llvmpipe
one thread unless `LP_NUM_THREADS` is set.

## The choice: A

1. Nothing changes for any other app: no bound, no format, no install path,
   no TCB.
2. DarkFi's box gets no JIT, so map_exec's proofs still bound its code.
3. About 12 MiB plus 15.6 MiB of Alpine's, against 188 MiB.
4. The one driver also serves mpv, whose closure already has Alpine's libEGL.
5. Speed is the risk, measured on the lane before any listing ships (Q5).

Not chosen: the driver in every image through `Userland.mk` (12 MiB in every
store for one app; linux_userland ships static programs); our own libEGL (two
in mpv's closure); `GALLIUM_DRIVER=softpipe` for all (it would pin out
llvmpipe if B ever lands). B stays written here, not planned.

## Units of work, in landing order

| # | Unit | Owner | Size (est.) |
|---|---|---|---|
| 1 | env.rs: the three variables, the comment at `:39-43` reworded; with no driver EGL fails as today | S1 | 10 lines, a host test |
| 2 | The build: `tools/linux-userland/mesa-swrast.sh`; the tarball and two patches in `tools/nix/sources.txt` (sha256, checked against the APKBUILD's sha512); built against v3.20's gcc 13.2.1, libstdc++, libdrm, wayland and expat from pinned apks, since zig's static shape makes no .so for Alpine's loader | S3 | 150 lines sh, 60 nix |
| 3 | `nonos-mesa-swrast`, content with no program: pinned in `linux-catalogue.sources.lock.json`, served at `/linux/<blake3>.tar`, never listed alone | S3 | 150 py, tests |
| 4 | Alpine pins in content: `usr/share/nonos-apks/<package>`, a line per need, `<name> <version> <C:>`, or `nonos-<name> <blake3>`; the generator holds every DT_NEEDED and each dlopen name a recipe declares to a placed file, as `elf_needed.py` does for Debian | S3 | 300 py, tests |
| 5 | The device installs them as `install/deb/needs.rs` does: each apk held to its C: (`auth/package.rs:34`), each content to its BLAKE3, each placed as its own package and named in the closure (`pack/closure.rs`) | S1 | 300 rs, proofs |
| 6 | DarkFi's GUI: bin/app at the chosen commit (Q3), x86_64 musl, dynamic, S3's transport patch; pins mesa-egl, wayland-libs-egl, libxkbcommon, libstdc++, zlib, expat, nonos-mesa-swrast; `linux.nonos-darkfi-app`, WAYLAND FILES_HOME NET_ANON, no JIT | S3 recipe, S2 app | 200 lines |
| 7 | If Q4 is yes: frame callbacks answered at most 60 times a second, not at commit (`commit.rs:50-57`), and a line of commits per 10 s per window | S1 | 80 rs |
| 8 | mpv: an Alpine recipe's needs name the driver; `--vo=gpu --gpu-context=wayland --profile=fast` against `--vo=wlshm --profile=sw-fast`, mpv's own fallback | S3, S2 | later |

## Proofs and the KVM lane

| # | Rule | Mutant that must fail a test |
|---|---|---|
| G.1 | every guest's environment carries the three variables | drop `LIBGL_ALWAYS_SOFTWARE` |
| G.2 | the pins parser refuses a bad line, a family mismatch, a need past the bounds (fuzz target) | skip the checksum field |
| G.3 | each pinned apk is held to its C:, each content to its BLAKE3 | accept by name |
| G.4 | a run serves the driver's pack only when its closure names it | closure without the pins |
| G.5 | the generator refuses content whose DT_NEEDED or dlopen names are not all placed | drop libstdc++ from the pins |
| G.6 | if unit 7 lands: no frame done before its period | answer at commit |

KVM lane, logs under `docs/release-evidence/0.9.4/`, CPU model named:
(1) **DarkFi's window drawn**: installed from the dev relay, started from the
store, screenshotted; stderr with `EGL_LOG_LEVEL=debug` (`mesa: egllog.c:109`)
shows "Native platform type: wayland" and
"MESA-LOADER: dlopen(/usr/lib/nonos-mesa/swrast_dri.so)", no EGL error.
(2) **Frame rate measured**: `GALLIUM_HUD=fps` in the screenshot, and unit 7's
count over 60 s idle and while scrolling, at 1024x768 and 1920x1080, with
CPU use. (3) **A box without the grant refused**: DarkFi's box has no JIT and
draws; in it a probe mapping anonymous memory executable is refused, "JIT"
in the box report (`map_anon.rs:29`); a box without WAYLAND gets no display
and says WAYLAND; a program outside the closure finds no driver. (4) **Nothing
else moves**: a GTK 4 app, a Qt app and foot draw as before
(`GSK_RENDERER=cairo` stays, `env.rs:47`).

## Open questions for the owner

1. The build sandbox for a .so on Alpine's ABI: a v3.20 root from pinned apks inside nix, or another way?
2. The driver beside Alpine's libEGL 24.0.9: Mesa keeps that interface internal, so the pairing rests on one release and the lane. Accept, or ship our own libEGL and libglapi?
3. Which DarkFi commit for the GUI: v0.5.0's bin/app is 0.1.0; master's is 0.4.0-alpha and on Linux pulls `x11-clipboard` (`bin/app/Cargo.toml:113-114`)?
4. Frame pacing in the personality (unit 7): yes, and at what rate?
5. Below what measured rate is softpipe not enough for DarkFi? Then B, or a NØNOS llvmpipe with LLVM linked statically for x86 alone (size not measured; JIT still needed)?
6. high_dpi on a display scaled by 2 draws four times the pixels: leave it, or ask DarkFi upstream for a flag?
7. mpv: is GL on softpipe wanted, if `--vo=wlshm` is faster?

## The owner's decisions (28 October)

Yes to all seven: Mesa built in nix on a pinned Alpine v3.20 root; our own
libEGL, libglapi and libgbm from the same build, beside the driver, rather
than Alpine's (their EGL-to-driver interface is Mesa's own, held to one
release; where Alpine's mesa-egl would hold the same file, the closure takes
ours instead, as two packages may not hold one file); DarkFi v0.5.0's GUI;
frame pacing at 30 a second (unit 7, at 33 ms); if the lane measures under
about 10 frames a second at DarkFi's default window, SwiftShader with ANGLE
(OpenGL ES on Vulkan in software, its JIT Subzero rather than LLVM) is the
next choice before any bound is raised; high_dpi off; mpv on `--vo=wlshm`.
