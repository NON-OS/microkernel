# vl-2b, first boot: no image reached the model

A dev image of models/vision 027b6afb3, booted under QEMU without KVM
(TCG), `--fresh --lane --mem=12G --model=vl-2b`, driven by
tools/nonos-drive on 2026-10-26 from 03:04 to 04:13 (host clock). This boot
**proves nothing about vision**; it is kept because it is what happened.

## What went wrong

The drive's `type:` step could only type letters, digits, space, `-`, `.`
and `/` (userland/capsule_install/e2e/qmp.py). Every other character was sent
to QEMU as a key name it does not know, and QEMU dropped it without a word.
So every line lost its `<`, `>`, `?`, quotes and parentheses:

- `make-png.png`: the Python line that writes red.png arrived without its
  quotes and failed (`SyntaxError`); no red.png was made.
- `refused-fake.png`: `echo hello > fake.png` became `echo hello  fake.png`,
  and `qwen vl-2b What is this? < fake.png` became a plain question with no
  image, so the refusal was never tried.
- The lines typed after it went to the running chat's stdin, not the shell.

The drive is fixed in the commit that adds this folder: every printable character is typed
with its key (shifted where it needs shift), and a character with no key
stops the run before anything is sent.

## What it does show

- `vl-10.png`, `vl-30.png`: on its first run the tier imports both of its
  pinned files from the disk into the session's encrypted volume and checks
  each against its signed SHA-256 pin: `Qwen3VL-2B-Instruct-Q4_K_M.gguf`
  (1.1 GB) `verified` by 03:22, `mmproj-Qwen3VL-2B-Instruct-Q8_0.gguf`
  (446 MB) `verified` by 03:42.
- `vl-40.png` to `vl-exit.png`: no answer to the plain question in the 31
  minutes after that, under TCG, before the run ended.

## Serial log

This boot's serial.log and qemu.log were overwritten by the next boot, which
wrote into the same folder: my mistake. Its `[LINUX]` lines, read before
that, were:

```
[LINUX] personality up
[LINUX] store settled after 19 ms
[LINUX] personality ready, nothing asked to run
[LINUX] process 2 exited, status 1
[LINUX] last calls: 4 2 1 1 1 1 13 11(0xffe53000)=0x0 11(0xffeaf000)=0x0 14 14 11(0x10011f000)=0x0 14 14 14 14
```

(process 2 is the failed Python line.)

drive.steps is the plan it was driven by.
