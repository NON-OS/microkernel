# demo_screen_proofs

The desktop's screens, drawn on the host through the shipping code before any
boot. It mounts these whole by `#[path]`:
- About (`capsule_about/src/about`);
- the terminal (`capsule_terminal/src`);
- the desktop shell (`capsule_desktop_shell/src`): its menubar, dock and desk
  painted by its own `paint_chrome` over a context its own `build_context`
  makes;
- the wallpaper capsule's decode and cover-scale, over the catalog's first
  wallpaper;
- the window manager's placement (`cascade`, `clamp_to_display`).

`libc_shim` answers their kernel calls from one fixed machine: 9 processes,
a measured boot, 24 October 2026 10:00, no battery, the display mode and
driver a test picks, and nothing else running. The libc files that make no
call (the procstat header, the worker handoff, the deadline) are mounted
unchanged. `policy_shim` stands in for an absent policy store. These frames
prove layout and rendering only, never the values shown.

`tests/screens` runs on each mode the recording may use (1024x768, 1280x720,
1280x800, 1920x1080), under virtio-gpu and bga:
- it opens each app at the size app_skeleton's `fit_rule` gives it, frames
  it as the runner does, and checks that the app paints nothing outside its
  content rectangle;
- it paints every About section, and the terminal as it opens and after
  `help`;
- it composes whole screens as the compositor stacks them (wallpaper, desk,
  window, chrome), checking the menubar and dock are drawn and that no
  chrome pixel lands on the window;
- it reads About's Display ring off the frame.

The lib's own unit-test build is off: under `cfg(test)` the terminal's files
compile their host-test variants instead of what ships.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
DEMO_SHOTS=/some/dir cargo test --release     # also writes every frame as PNG
cargo clippy --release --lib --tests -- -D warnings
```

What a picture cannot show is the device: how the frame reaches the screen
through bga or virtio-gpu is proved in bga_proofs, virtio_gpu_proofs and
gfx_pool_proofs.
