# boot_splash_proofs

Host-runnable proofs for the boot splash, on the splash's own source
included through `#[path]`. What asks the kernel or the compositor is left
out; everything it hands on is proven here.

| Tests | What they hold |
|---|---|
| `canvas_tests` | the splash drawn over a firmware screen (a logo, a pattern) is the drawing it makes on a blank surface, at every mode and with or without the warning, so no firmware pixel shows; NONOS's own title is drawn; nothing past a row is written |
| `attest_lines_tests`, `attest_record_tests` | the attestation panel's lines come from the boot's record: secure boot, the kernel's signature, capsules zk attested or path only (development); ATTESTED only with the signature and the STARK; no record claims nothing; no line is the same in every state |
| `splash_layout_tests` | the status line repaints its band whole and nothing outside it, every frame and mode |
| `follow_tests`, `follow_limit_tests` | the splash asks the canvas size every 250 ms, at once if the clock went back; a new size (or stride) is a move, the same size or a failed answer is not; four redraws at most; its time on screen kept across a redraw |
| `warning_tests`, `warning_draw_tests` | the no-IOMMU warning's row is 14 px under the panel and at least 14 above the status line at every mode; drawn as the toolkit draws the owner's words in red, centred, inside its row only |

Run: `cargo test` in this directory.
