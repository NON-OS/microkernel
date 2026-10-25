# AgentProof quickstart: run an agent in a box and read what it tried

AgentProof runs a program you do not fully trust (an AI agent, or code an AI wrote) as an
ordinary Linux program in a box NONOS enforces. When it ends you get a report of everything
the box refused, by kind, with examples.

Everything on this page was run on a booted NONOS 0.9.3 development image: QEMU, Intel IOMMU
enforcing, the Terminal opened from the dock. The outputs below are copied from that boot.

## 1. Write the agent

Any program in the image's Linux tree runs in a box: `sh` and the BusyBox tools, `python3`,
`lua`, `perl`, `sqlite3` and the rest ([Linux programs](../using/linux-programs.md)). This
example agent tries what an agent with too much reach would: it reads a file, writes inside and
outside its folders, opens TCP and UDP connections and starts a shell that writes on its own.

```python
import os, socket, subprocess
def t(name, f):
    try:
        f(); print('TRY', name, 'ok')
    except Exception as e:
        print('TRY', name, 'refused:', e)
t('read /etc/passwd', lambda: open('/etc/passwd').read())
t('mkdir /tmp/out', lambda: os.makedirs('/tmp/out'))
for w in ['/tmp/a', '/tmp/out/b', '/home/c', '/etc/d']:
    t('write ' + w, lambda w=w: open(w, 'w').write('x'))
t('tcp 10.0.2.2:80', lambda: socket.create_connection(('10.0.2.2', 80), 3))
t('udp 10.0.2.2:53', lambda: socket.socket(2, 2).sendto(b'x', ('10.0.2.2', 53)))
t('spawn sh', lambda: subprocess.run(['/bin/sh', '-c', 'echo child; echo y > /home/e'], check=True))
```

Save it as `agent.py` in your home folder, with the text editor or line by line in the
Terminal (`echo "..." >> agent.py`).

## 2. Run it in a box

```
linux --box strict python3 - < agent.py
```

`< agent.py` hands the file to `python3` as its input; `-` tells Python to run what it reads.
The Terminal keeps eight words of a line, so give a long command as a script.

What the agent printed, under `strict`:

```
TRY read /etc/passwd ok
TRY mkdir /tmp/out ok
TRY write /tmp/a ok
TRY write /tmp/out/b ok
TRY write /home/c refused: [Errno 30] Read-only file system: '/home/c'
TRY write /etc/d refused: [Errno 30] Read-only file system: '/etc/d'
TRY tcp 10.0.2.2:80 refused: [Errno 13] Permission denied
TRY udp 10.0.2.2:53 refused: [Errno 13] Permission denied
child
/bin/sh: can't create /home/e: Read-only file system
TRY spawn sh refused: Command '[...]' returned non-zero exit status 1.
```

Then the report, after the program's own output:

```
BOX REPORT v1
  policy   strict 5811b081f826f154ce1ac5a7b558bff858ae0bacc27ad2db9d2e5f84ce05323a
  allowed  write /tmp
  allowed  read the image, read-only
  allowed  network none
  allowed  execve, each program held to this box
  refused  5
    write         3  /home/c, /etc/d, /home/e
    network       2  an internet socket, an internet socket
  exit     0
  counts are capsule_linux's enforcement, not the kernel's sealed histogram
```

Every refusal the agent saw is in the report, including `/home/e`, which the shell the agent
started tried: each program a boxed run starts is held to the same box.

## 3. Choose the box

The same agent, run in the three kinds of box:

- `strict`: it could write `/tmp`. Refused 5: write 3 (`/home/c`, `/etc/d`, `/home/e`),
  network 2.
- `offline`: it could write the run's own `/tmp /home /root /run /var/tmp /dev/shm`. Refused 3:
  write 1 (`/etc/d`), network 2.
- `allow:/tmp/out`: it could write `/tmp/out` only. Refused 6: write 4 (`/tmp/a`, `/home/c`,
  `/etc/d`, `/home/e`), network 2.

A report lists the first three examples of each kind; the count is every refusal. No box has
a network in this release. The run's folders are new when it starts and wiped when it ends.
[Run a program in a box](../using/linux-box.md) has every rule.

## 4. Keep the box as a file

Write the box as a `nonos-box 1` policy file and check it before you run anything:

```
nonos-box 1
base folders
write /tmp/out
network none
```

```
$ agentproof policy word out.box
allow:/tmp/out
$ agentproof policy hash out.box
2713c4db3da1c423003faebaa7beac7dba55687e9552455f935ee0f7bc8ad1cf
```

That hash is the one the booted run's report gave for `allow:/tmp/out`: the file and the box
are the same policy. `agentproof policy check` explains a file in plain words.
`agentproof` runs on your own machine: `cargo build --release` in `userland/agentproof_policy`.
[The format](policy-format.md).

## 5. For automation

A build with `capsule-serial-debug` writes one line per run to the serial log:

```
[BOX] v=1 preset=allow policy=2713c4db… refused=6 write=4 network=2 exec=0 jit=0 display=0
models=0 call=0 barred=0 unserved=0 started=yes exit=0 source=capsule_linux
```

(one line on the log, wrapped here). A run whose report did not arrive whole writes
`[BOX-MISSING] v=1 report=… capsule_failed=… exit=… source=terminal` instead. Treat that,
or no `[BOX]` line at all, as a failed run.

## What this does and does not prove

- The report is what `capsule_linux` refused, sent on frames keyed by the Terminal; a program
  cannot print, hide or change its own report.
- It is not yet a signed receipt. The kernel's sealed record of the run, a TPM quote and an
  offline verifier are planned for 0.9.4; until then, nothing on this page is signed.
- The counts are `capsule_linux`'s enforcement, not the kernel's sealed histogram.

## See also

- [Run a program in a box](../using/linux-box.md)
- [The policy file format](policy-format.md)
- [Linux programs](../using/linux-programs.md)
