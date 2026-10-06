# linux_guests

Linux programs that test the Linux personality (`userland/capsule_linux`) from the inside. Each
is a guest built for x86_64 Linux, then signed and enrolled through the capsule template exactly
as a shipped Linux program is, so the personality proves it before it runs and no check is
weakened for a test. Most check one area against what Linux does and exit non-zero when the
personality answers otherwise. The handbook pages are
[`docs/handbook/linux/personality.md`](../../docs/handbook/linux/personality.md) and
[`docs/handbook/linux/userland-tools.md`](../../docs/handbook/linux/userland-tools.md).

## When it is built

Only for test images. `mk/20-build.mk` includes `Guests.mk` when `NONOS_LINUX_GUESTS=1`, and
`Guests.mk` stops with an error unless `NONOS_DEV=1`, because it mints a scratch publisher
pair for each guest on first use (`guest_<name>_publisher` under `.keys/`, the public halves
under `nonos-data/trust/keys/`). No CI workflow and no flake derivation builds this directory.
The guest image drops the demo and media store entries, and `Userland.mk` empties the bundled
userland's store lists, so the guests fit the store the vfs loads.

`LINUX_GUEST` in `Guests.mk` declares one guest: a name, a service and reply port, the ELF and
the path under `/linux` (default `/bin/<name>`). Every guest holds no capabilities
(`CAPSULE_REQUIRED_CAPS := 0x0`); its endpoints are declared and never registered. The store
the vfs loads holds at most 16 MiB, which the whole set outgrows, so `LINUX_GUEST_SET` or
`LINUX_GUEST_ONLY` (`GuestOnly.mk`) names the guests an image carries. `LINUX_GUEST_BOOT_ARGS`
names a file with the program to boot and its arguments (`GuestFiles.mk`).

## What is here

| Where | What |
| --- | --- |
| `src/`, `Cargo.toml` | Rust guests built static and non-PIE for `x86_64-unknown-linux-musl`: `suite` runs every single-process probe in one boot; `holder` and `reader` are the memory pair; `window` is a Wayland client; `signal` proves a handler runs and returns |
| `c/`, `CGuests.mk`, `LifeGuests.mk`, `GuestProofs.mk` | C guests against musl: threads, futex and epoll waits, sockets (`csock`, `cudp`, `cunix`, `cpolicy`), the memory proofs in one `memproof` binary, process lifecycle and signals, and the file and `/proc` proofs (`cfiles`, `cproc`) |
| `Guests.mk` (`dyn`, `libprobe`, `ldmusl`) | a dynamically linked program, its library and musl's loader, each proved like a program |
| `go/`, `GoSuite.mk` | static Go guests (`gohello`, `goconc`, `gopoll`, `gopreempt`, `gohttp`, `goos`, `goexec`); Go's own standard-library tests as guests when `NONOS_LINUX_GO_SUITE=1` |
| `cpp/`, `QwenGuest.mk`, `QwenLlama.mk`, `QwenChat.mk` | `qwencheck`, which runs the pinned Qwen model and exits 0 only when its token ids equal the host's, and `qwenchat`; opt in with `NONOS_QWEN=1`. The shipped `qwenchat` is built from the same `cpp/` sources by `tools/nonos-linux-userland-build` |
| `Outside.mk` | programs, libraries and data built outside the tree, put into the image's Linux tree (`LINUX_GUEST_PROGRAMS`, `LINUX_GUEST_LIBS`, `LINUX_GUEST_FILES`) |
| `sh/` | `oracle.sh`, which runs a proof guest on the build host as the personality runs it, for the reference answers; the BusyBox suite scripts |
| `etc/` | `passwd`, `group` and the Qwen prompt the image places |

The embedded BusyBox is also enrolled as a guest from the store, from
`userland/capsule_linux/guests/busybox.elf`.

## Not done yet

- The comment above the `busybox` guest in `Guests.mk` calls it Alpine's static busybox. The
  binary is the one `tools/nonos-busybox-build` makes from upstream source with the committed
  config.
- The C guests need `musl-gcc` and the musl shared libc at `/usr/lib/x86_64-linux-musl`, and
  the Go guests a Go toolchain at `/usr/local/go/bin/go`, from the host.
