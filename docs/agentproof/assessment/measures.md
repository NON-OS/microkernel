# What the assessment measures

Every number in the report comes from one of the sources below, and each source is code in the
NONOS tree that has been booted. Nothing is estimated.

## Per run: the box report

At the end of each run the Terminal shows `BOX REPORT v1` and writes one `[BOX] v=1` line to the
serial log ([Run a program in a box](../../using/linux-box.md)).

| Measure | What it is | Where it comes from |
|---|---|---|
| policy hash | the box, one hash whatever the program | `boxed/hash.rs` |
| allowed | what the program had: the box narrowed by its own policy | `boxed/allowed.rs` |
| refused | how many actions the box refused | `boxed/tally.rs` |
| by kind | a count for each of nine kinds, and the first three examples | `boxed/kind.rs` |
| started | whether the program ran, and why not | `start_stop.rs` |
| exit | the program's exit status; 137 when ended by three Ctrl+C | `serve/family_tty.rs` |

All paths are under `userland/capsule_linux/src/linux/`.

The nine kinds:

- `write`: a file written, made, renamed or removed outside the writable folders;
- `network`: a socket, bind, listen or stream the box gives no network for;
- `exec`: a program started without the right to start one;
- `jit`: executable memory made without the right to;
- `display`: the display reached without the right to;
- `models`: a model file read without the right to;
- `call`: a system call outside the calls the program's own policy declares;
- `barred`: a call no Linux program is ever given (`ptrace`, `mount`, `unshare`, ...);
- `unserved`: a call NONOS does not provide.

One system call counts once for each kind it was refused as. Every box gives a terminal and
starting programs, and each program a run starts is held to the same box, so what a child
process tries is counted in its parent's run.

## How the counts were checked against what a program really did

On a booted image, a python3 agent tried reads, writes inside and outside its folders, TCP and
UDP connections and a child shell, under `strict`, `offline` and `allow:/tmp/out`
([quickstart](../quickstart.md)). In each box, every refusal the agent saw was counted, the
child's included; nothing that succeeded was counted; and no other kind was. The report is held
to that by `capsule_linux_proofs` (the box, its kinds and its report) and `terminal_line_proofs`
(the report's channel and its screen), with mutants shown caught.

## Per run: integrity of the report

- The report travels on frames keyed by the Terminal for that run; the program is never told
  the key, and a `[BOX]` line the program prints never reaches the log
  (`capsule_terminal/src/jobs/box_hold.rs`).
- It is drawn after the program's output, on a screen put back to its defaults, so nothing the
  program left can hide or bend it (`box_screen.rs`).
- A report that never came whole is said, and logged as `[BOX-MISSING]`, with
  `capsule_failed=yes` when `capsule_linux` itself failed (`box_missing.rs`).

## Per machine: the platform

- The serial log's IOMMU line: which unit, whether it enforces, and how many device mappings go
  around it. The assessment runs only with `enforcing=1, unconfined grants=0`.

## Not measured in this release

- The kernel's sealed record of the run, its TPM quote, and a signature over the report: receipt
  v0, 0.9.4.
- A completed connection to an allowed destination: our lab has no route to the mixnet, so
  the report's `connected` reads `none`. Which destinations were allowed, and every attempt
  to reach another, are measured (`boxed/net_report.rs`, `net/connect/out.rs`).
- Anything an agent does outside NONOS, such as calls to your own systems.
