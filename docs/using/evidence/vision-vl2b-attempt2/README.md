# vl-2b, second boot: shift stuck in the guest, stopped early

The same dev image as the first boot (models/vision 027b6afb3), under QEMU
without KVM (TCG), `--fresh --lane --mem=12G --model=vl-2b`, driven by
tools/nonos-drive with the first typing fix (5d3ee02fc), on 2026-10-26 from
04:17 (host clock). Stopped by hand at about 04:30, once `refused-absent.png`
showed this run could not reach the image question. It **proves nothing
about the model**; it is kept because it is what happened.

## What went wrong

Symbols now reached the guest, but each character was one QMP `send-key`
chord, 50 ms apart. After the capitals in `\x89PNG` the guest kept shift
held (`make-png.png`: `PNG\r\n` arrived as `|RN|!|NCB`, and the rest of the
line in capitals), so the Python line failed (`SyntaxError`) and no red.png
was made. Shift stayed held into the next lines: `echo hello > fake.png`
arrived as `O HELLO FAKE>PNG` (`wrote to /home/nonos/PNG`), and the fake
image line as `QWEN VL_@B What is this? < fake.png` (`pipe: unknown filter
QWEN`).

The drive now types each key as its own press and release event
(input-send-event), shift down before the key and up after it, with a gap
between every event.

## What it does show

- `refused-absent.png`: `qwen vl-2b What is this? < absent.png`, typed
  whole, is refused by the Terminal before any model starts:
  `qwen: /home/nonos/absent.png: vfs open failed`, status 1 (the red mark).

The not-a-PNG refusal and the image question were not reached.

serial.log is this boot's; drive.steps is the plan it was driven by.
