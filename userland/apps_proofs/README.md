# apps_proofs

Host test crate for what the desktop apps decide and say when a service is
missing, a list comes back empty or an input is too large. Each app keeps that
decision in a small file away from its drawing and IPC code; this crate
compiles those files through `#[path]`, unchanged, so the tests hold the
source the capsule builds. Its one dependency is `libm`. The apps are
described in [docs/handbook/apps/system-apps.md](../../docs/handbook/apps/system-apps.md)
and [docs/handbook/audio-and-media.md](../../docs/handbook/audio-and-media.md).

## What is under test

63 `#[test]` functions:

| App | Mounted files | Tests |
|---|---|---|
| calculator | `calc/` fixed point, operators, scientific and unary functions, number and error formatting | `calc_tests.rs` (7) |
| clock | calendar arithmetic, stopwatch, timer, what it says when the clock cannot be read or set | `clock_tests.rs` (4), `clock_span_tests.rs` (5) |
| image viewer | the largest file and image it takes, what it says with nothing to show | `image_tests.rs` (11) |
| markdown viewer | whether it lays out what it read from `/readme.txt` | `mdview_tests.rs` (3) |
| video player | why the library is empty | `video_tests.rs` (5) |
| process manager | the table and status-strip notes | `pm_tests.rs` (8) |
| audio player | the WAV decoder, resampler, waveform peaks, track limit, the play, pause and feed state machine, and its trouble messages | `audio_tests.rs` (13), `audio_load_tests.rs` (7) |

The file manager, the text editor and settings have proof crates of their
own and are not mounted here.

## Running

```sh
cd userland/apps_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`), with overflow
checks on and clippy over all targets.
