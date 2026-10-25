# Run a program in a box

Run a Linux program, one you did not write or one a model wrote, in a sealed box, and read a report of everything it tried that the box refused.

## Run it

Type `linux --box`, the box, then the program and its arguments, in the Terminal:

```
linux --box strict sh -c 'echo ok > /tmp/out; echo no > /home/out; nc 10.0.0.1 80'
linux --box offline python3 script.py
linux --box allow:/tmp/out,/home/work python3 build.py
```

The Terminal keeps eight words of a line, so after `linux --box <box>` a program has five words for itself and its arguments; give a script with `sh -c '...'` as one word.

## The boxes

| Box | Network | What it may write |
|---|---|---|
| `strict` | none | only `/tmp`, the run's own scratch folder |
| `offline` | none | the run's own folders: `/tmp`, `/home`, `/root`, `/run`, `/var/tmp`, `/dev/shm` |
| `allow:<folder>,...` | none | only the folders named |

An `allow:` box names its folders with commas between and no spaces. A folder is a path from `/` inside one of the run's own folders, such as `/tmp/out` or `/home/work`, in printable ASCII. A folder anywhere else, a folder ending in `/`, or one with `.` or `..` in it is refused.

A run started from the Terminal has no network in this release, so every box has none. An `allow:` item naming a destination (`net:<address>:<port>`) is refused: `linux: --box refused: a terminal run has no network in this release`.

Every box lets the program use the terminal and start other programs. Each program it starts is held to the same box. The image itself, the Linux tree the programs come from, is always read-only.

A box only takes rights away: a program gets what both the box and its own policy allow. The run's own folders are new at its start and wiped at its end.

A box named wrongly is refused before anything runs, with the reason, such as `linux: --box refused: a folder is inside /tmp, /home, /root, /run, /var/tmp or /dev/shm`. `linux --box` with no box or no program prints the usage line.

## The report

When the program has ended, the Terminal shows its report. The report comes from `capsule_linux` on frames that carry a key the Terminal made for this run and the program is never told, so nothing the program prints is taken for the report. It is drawn only after the program's output has ended, on a screen put back to its defaults first (the main screen, the whole screen as the scroll region, normal video, plain characters, no insert mode), on a fresh line just below what the program printed, with everything below that line cleared. So the program's own output stays in view above the report, and nothing the program left behind can hide or bend it. A program can scroll the screen but cannot write, hide or change its own report.

```
BOX REPORT v1
  policy   strict 5811b081f826f154ce1ac5a7b558bff858ae0bacc27ad2db9d2e5f84ce05323a
  allowed  write /tmp
  allowed  read the image, read-only
  allowed  network none
  allowed  execve, each program held to this box
  refused  2
    write         1  /home/out
    network       1  an internet socket
  exit     0
  counts are capsule_linux's enforcement, not the kernel's sealed histogram
```

- `policy` is the box and its hash. Each box has one hash, whatever program runs in it and whatever order its items were named in; `strict` and `offline` always hash the same.
- `refused` is how many actions the box refused, then one line for each kind refused at least once: its count and the first three it refused.
- The kinds: `write` (a file written, made, renamed or removed outside the folders), `network` (a socket, a connection, a bind or a datagram the box does not allow), `exec`, `jit` (executable memory), `display`, `models`, `call` (a system call outside the program's declared calls), `barred` (calls no Linux program is ever given, such as `ptrace` and `mount`) and `unserved` (calls NONOS does not provide).
- One system call counts once for each kind it was refused as. Asking whether a file could be written, as `access` does, is not a refused action.
- `allowed` is what the program actually had: the box, narrowed by the program's own policy. A folder in `/home` or `/root` is listed only if the program's policy gives it a home.
- `started  no: <reason>` appears when the program never started (it was not found, or its proof was refused); nothing ran, so nothing was refused.
- `exit` is the program's exit status.

If no report arrives (the box word was refused, or the run was stopped or failed before its end), the Terminal says `linux: the box sent no report; the run ended before it could`. If `capsule_linux` itself failed (exit 134), it says `linux: the box sent no report: capsule_linux stopped on an internal error (exit 134), so what the program tried is not known`. If only part of it arrives, the Terminal shows that part and says `linux: the box's report arrived incomplete`. A program that ignores `Ctrl+C` is ended by pressing it three times within 2 seconds; it then ends as any run does, and its report shows `exit 137`. If the run is stopped with `Ctrl+C`, a report that had already arrived whole is still shown; otherwise the Terminal says `linux: stopped before the box's report arrived`.

A boxed run in the background that ends, or is stopped, while another program holds the screen keeps its report until that program has ended, then shows it.

The counts are made by `capsule_linux` as it refuses each action. They are not the kernel's sealed histogram, which counts every call by the error it returned and is what a proof of a run reads.

## For automation

The Terminal also writes the same summary to the serial log as one line, in a build that compiles `capsule-serial-debug`:

```
[BOX] v=1 preset=strict policy=<hash> refused=2 write=1 network=1 exec=0 jit=0 display=0 models=0 call=0 barred=0 unserved=0 started=yes exit=0 source=capsule_linux
```

Every kind is always present. `v=1` is the format; a reader should refuse any other number. The line is written whole, in pieces the log joins, however long the box's hash and counts make it; if the log refuses it, the Terminal says `linux: the [BOX] line could not be written to the log`. A build without `capsule-serial-debug`, such as a hardened release, has no such log: no line is written, and the Terminal says nothing of it. Only the Terminal writes this line, from the report's keyed frames: a `[BOX]` line the program prints itself never reaches the log.

When a boxed run's report does not arrive whole, the Terminal writes a line of its own instead, so a missing report is never read as a run that refused nothing:

```
[BOX-MISSING] v=1 report=none capsule_failed=yes exit=134 source=terminal
```

`report` is `none`, `incomplete` or `stopped` (Ctrl+C before the report came); `capsule_failed` is `yes` when no report came and `capsule_linux` ended with 134; every field is always present. A check of a run should fail it when it sees `[BOX-MISSING]`, or when it sees no `[BOX]` line at all.

## Where this comes from

- The boxes, the hash and the report: `userland/capsule_linux/src/linux/boxed/`.
- The report taken and drawn by the Terminal: `userland/capsule_terminal/src/jobs/box_hold.rs`, `box_screen.rs`, `box_draw.rs`, `box_wait.rs`, `box_log.rs` and `box_missing.rs`.
- How a box narrows a program's own policy: `userland/capsule_linux/src/linux/app/identity_box.rs`.
- The tests: `userland/capsule_linux_proofs/src/tests/boxed/`, `userland/terminal_line_proofs/src/jobs/box_*`, and the fuzz target `box_word` in `userland/capsule_linux_proofs/fuzz/`.

## See also

- [Linux programs](linux-programs.md)
- [Terminal](terminal.md)
