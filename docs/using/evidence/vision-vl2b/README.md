# vl-2b answers an image on a NONOS boot

A dev image of models/vision 027b6afb3 (`make dev-image`, throwaway keys;
nonos.img sha256 `1b0dc89b31733050…`), booted under QEMU **without KVM
(TCG)**, `--fresh --lane --lane iommu=intel --mem=12G --model=vl-2b`, and
driven by tools/nonos-drive with the typing fix of 31084728c
(`drive.steps`), on 2026-10-26 from about 04:33 to 07:15 (host clock). Two
attempts before it did not reach the question: see vision-vl2b-attempt1
and vision-vl2b-attempt2.

## What it shows

1. `make-png.png`: `linux python3 -c "…" > red.png` writes a 64x64 red PNG
   in the guest (`wrote to /home/nonos/red.png`; serial line 1833,
   `[LINUX] process 2 exited, status 0`).
2. `refused-fake.png`: `echo hello > fake.png`, then
   `qwen vl-2b What is this? < fake.png`. The chat refuses the frame before
   it loads a model: "That file is not a PNG or a JPEG, the two kinds this
   model is shown.", `process 2 exited, status 3`, 2.3 s.
3. `refused-absent.png`: `qwen vl-2b What is this? < absent.png`. The
   Terminal refuses before it starts anything:
   `qwen: /home/nonos/absent.png: vfs open failed`, status 1.
4. `vl-10.png`, `vl-60.png`: `qwen vl-2b What colour is this picture?
   Answer in one word. < red.png`. Neither file is on the data volume yet,
   so each is sealed into the session's encrypted volume and checked
   against its signed SHA-256 pin, once: both `verified`. The serial log
   has the imports with their digests (`[DATA] imported 1107409952 bytes,
   sha256 089d75c5…` and `445053216 bytes, sha256 f9a68fab…`), the pins in
   pinned_vl.rs.
5. `vl-90.png`: the model's answer, **`red`**.
6. `vl-exit.png`: `/exit`: "The conversation was wiped from memory.",
   `process 2 exited, status 0`, `calls served=124880 unserved=0`.

## Measured, on TCG

From the chat's metrics lines (`vl-90.png`, `vl-exit.png`). The chat's
own lines, these among them, are kept off the serial log.

| | |
|---|---|
| load_ms | 4845367 (about 81 minutes, the two imports included) |
| ttft_ms | 236651 (the image encoded and the question read) |
| tokens | 1 (`red`) |
| peak_rss_kb | 2365208 (2,421,972,992 bytes) |

These are QEMU's software CPU: they say nothing about the speed on real
hardware, and no speed is claimed. The peak resident memory is the one
figure that carries over: it is above what need.rs's rule gave vl-2b
(2,324,215,104 bytes with a 384 MiB vision margin), so the margin is
raised to 512 MiB (2,458,432,832 bytes), and
userland/model_fetch_proofs/tests/vision_peak.rs holds the rule at or
above this peak.

serial.log is this boot's; drive.steps is the plan it was driven by.
