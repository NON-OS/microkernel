# The AgentProof Gauntlet: run an agent in a box on every change

The Gauntlet builds a NONOS development image that holds your agent, boots it headless with
the IOMMU enforcing, runs the agent in the box you name, and fails when the box refused
anything. You get a pass or a fail, the reasons, the agent's output and the box's report.

```
python3 tools/agentproof-gauntlet --job path/to/job-folder --out target/agentproof
```

Exit 0 is a pass. Exit 1 is a fail, with why. Exit 2 means the image could not be built or
booted, so nothing was judged.

## The job folder

A folder with a `job` file and up to 15 other files, each at most 1 MiB. Name them with
letters, digits, `.`, `_` and `-`, not starting with `.`; any other name is refused. They
are placed in the image's Linux tree at `/usr/share/agentproof/`, packed and verified with
the rest of the store.

```
nonos-agentproof-job 1
box strict
run python3 /usr/share/agentproof/agent.py
```

- `nonos-agentproof-job 1` comes first: the format and its version.
- `box <word>` is the `linux --box` word: `strict`, `offline` or `allow:...`
  ([Run a program in a box](../using/linux-box.md)). An `agentproof policy word` of a
  `nonos-box 1` file is one ([the policy format](policy-format.md)).
- `run <program> [arguments]` is the program and at most four arguments.
- A job is at most 1024 bytes. A word is letters, digits and `/ . _ - = : , + @ %`, so no
  shell quoting is needed or taken.

## What a pass means

All of these, read from the serial log:

- the run finished: `[AGENTPROOF] v=1 status=done`;
- the kernel reported the IOMMU enforcing with 0 unconfined grants;
- exactly one `[BOX]` line and no `[BOX-MISSING]`, so the box's report arrived whole;
- the program started;
- the box refused no more than `--max-refused` actions (0 by default);
- the run's [receipt](receipt.md) came whole, `agentproof-receipt verify` took it, and a
  copy with one byte flipped was refused;
- on the machine, the receipt job's rule held: asked again, the same receipt; asked
  anything else, EBUSY; after the acknowledgement, the record gone.

With no `--trust`, the trust is pinned from that receipt on first use (`trust.pinned`).
That shows the receipt is whole and signed by the AK it carries, not that anyone vouched
for that AK or that release. Pass `--trust` with a file you keep to check later runs
against the first. A development image extends no PCR 14, so today its receipts are
refused at `release` and the run fails with that reason (letter server/14).

The lines it judges are written by the image, never by the agent. `[BOX]` comes from the
Terminal, out of the box's keyed report frames. `[AGENTPROOF]` comes from the headless
Terminal, and the IOMMU line from the kernel. The agent's own output reaches the log only as
`[AGENTPROOF-OUT] ` rows, which are never read as anything else. A run that prints its own
`[BOX]`, `[AGENTPROOF]` or `[AGENTPROOF-RECEIPT]` line changes nothing.

## What you get

`--out` gets `serial.log`, `result.json`, the receipt as `receipt.napr` and, when pinned,
`trust.pinned`. `result.json` holds:

- the verdict (`passed`, `why`);
- the fields of the `[BOX]` and `[AGENTPROOF]` lines;
- the agent's output rows;
- `receipt`: what the Terminal said of it, the rule as the machine showed it, whether it
  verified and what the verifier printed, and whether the flipped copy was refused;
- `job_sha256`: SHA-256 over each file's name, length and bytes, in name order;
- `image_commit` and `source_commit`;
- `keys`, which reads `test keys (development copy)`.

## What this does and does not prove

- It is a development image, signed with throwaway keys made for the copy it was built
  from. It is not a release image, and its receipt names a release only you pinned.
- The counts are `capsule_linux`'s enforcement, the same as the box's report, not the
  kernel's sealed histogram.
- It runs under QEMU. It says what the box let the agent do there, not what the agent would
  do elsewhere.

## In your own CI

[agentproof.yml](agentproof.yml) is a GitHub Actions workflow to copy into your own
repository. It checks out the NONOS tree at a ref you pin, builds the gauntlet image with
Nix, and boots it under QEMU. Building the image from nothing takes a long time, so run it
on a machine with a Nix store that is kept between runs.

## See also

- [The quickstart](quickstart.md)
- [Run a program in a box](../using/linux-box.md)
- [The policy file format](policy-format.md)
