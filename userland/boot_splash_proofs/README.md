# boot_splash_proofs

Host-runnable proofs for the boot splash's firmware-logo path, on the
splash's own source included through `#[path]`
(`../capsule_boot_splash/src/logo/`). `fetch.rs`, the one file that calls
the kernel, is left out; everything it hands on is proven here.

| Tests | What they hold |
|---|---|
| `record_tests` | MkBootLogo's 28-byte record, every field where F-50 puts it; a short record and a fourth turn refused |
| `bmp_tests` | every pixel of 24- and 32-bit files, top-down and bottom-up, padded rows; every file the splash would misread refused |
| `turn_tests` | each quarter turn's inverse lookup equals turning the image forward; four turns are none |
| `place_tests` | at the firmware's mode the logo lands on its own pixels; a scaled or reshaped canvas keeps its centre and shape; a centred logo gives back its mode |
| `prepare_tests` | record and file to the logo shown: pixel for pixel, turned, halved, and placed from its margins when the mode is unknown |
| `refuse_tests` | not on screen, a record and a file that disagree, or a logo too large to hold: nothing is drawn |
| `draw_tests` | the prepared logo is copied to its spot, clipped at every edge, with nothing written outside |

`build.rs` writes the BMP files and records the tests hand the splash.

Run: `cargo test` in this directory.
