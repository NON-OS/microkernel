# The exposure map: `audit`

`audit` answers one question about a running machine: which capsule can reach
the network, storage or devices right now, and on whose signature. It is a
Terminal built-in (`userland/capsule_terminal/src/command/builtin/audit/`).

Every word it prints comes from the kernel, not from any capsule's account of
itself: the process table (`MkProcStat`) for names, and the attestation registry
(`MkAttestEntries`) for each capsule's capability mask and signer, the same
entries `receipt` lists and a TPM-signed attestation folds.

## Reading the table

The ACPI capsule's row on a standard image, as its spawn site requests it
(`src/userspace/capsule_acpi/spawn.rs`); its pid differs per boot:

```
audit v1
  pid  name                  network  storage  devices*  signed by  capabilities
  ---  --------------------  -------  -------  --------  ---------  ------------
    7  acpi                  no       no       YES       vendor     IPC, Memory, Debug, Admin, DeviceEnum, Driver, Irq
```

- `network`: `YES` when the capsule holds `Network`, the only capability the
  kernel lets open a socket.
- `storage`: `YES` when it holds `FileSystem`, `StoreWrite` or `StreamImport`.
- `devices*`: `YES` when it holds `Driver`, `Mmio`, `Irq`, `Dma` or `Pio`. This
  column is capability bits, what the kernel would let the capsule claim, not
  devices it has claimed now. Live DMA, MMIO and IRQ grants come in a later
  release.
- `Admin`, `Hardware` and `DeviceEnum` open none of the three: the kernel's
  checks take their own capability and nothing stands in for it.
- `signed by`: who signed the ceiling the kernel recorded when the spawn gate
  admitted the capsule: `vendor`, `publisher`, or `developer` for a key enrolled
  on this machine.
- A process the registry does not list reads `not in the kernel's registry: no
  signed ceiling recorded`, and its reach is not counted.
- `; debug withheld (holds InputSource)` after the capabilities: the kernel's
  spawn gate withheld the `Debug` its manifest offered, because it holds
  `InputSource`, `Keyring`, `DeviceSecret` or `InputConsumer` and this is not
  the dev image. The missing bit is the gate's, not a change to the signed
  manifest. The process table says so (`MkProcStat`'s `flags`, bit 0), and
  `receipt` prints the same note.

Under the table, one line per column counts the capsules that reach it, of those
with a signed ceiling, and one line counts the processes without one.

A name the kernel holds that is not plain printable ASCII is shown with `?` in
place of each other byte, so no capsule can move the cursor or forge a row with
its name.

## One capsule: `audit --agent <name>`

The same capsule; the measurement is the first six bytes of its own, in hex:

```
audit v1
acpi, pid 7
  authority  its signed manifest's ceiling, signed by vendor, measurement <12 hex digits>
  network    no
  storage    no
  devices*   YES, through Driver, Irq
  holds      IPC, Memory, Debug, Admin, DeviceEnum, Driver, Irq
```

Each area names the capabilities that open it. A capsule whose `Debug` the
kernel withheld has one more line, `debug      withheld by the kernel, as it
holds <capabilities>`. A name running more than once gets one block per pid.

## The v1 format

The first line is always `audit v1`. A script that reads the table should check
it and refuse any other. In v1:

- the heading's columns start at character 2 (`pid`), 7 (`name`), 29
  (`network`), 38 (`storage`), 47 (`devices*`), 57 (`signed by`) and 68
  (`capabilities`); `pid` is right-aligned in five places; a name longer than 20
  characters is cut;
- a reach cell is `YES` or `no`; capabilities are the kernel's names, joined by
  `, `, in bit order, and a bit this build has no name for prints as `unknown
  bit N`, never dropped (so do `receipt` and `capsules`);
- a row the gate withheld `Debug` from ends `; debug withheld (holds <names>)`;
- the agent block is a `<name>, pid <pid>` line, then the `authority`,
  `network`, `storage`, `devices*` and `holds` lines, and `debug` when withheld,
  two spaces in, the value at character 13.

A change to any of these is v2.

## When it refuses

| Line | Why |
|---|---|
| `usage: audit [--agent <name>]` | `--agent` without a name, or with more than one word |
| `audit: unexpected <word>; usage: ...` | any other word after `audit` |
| `audit: the kernel did not hand over its registry` | `MkAttestEntries` refused |
| `audit: the kernel did not hand over its process table` | `MkProcStat` answered nothing |
| `audit: the kernel's registry is not whole entries; refused` | the registry's length is not a multiple of its 45-byte entry, so this build and the kernel disagree |
| `audit: no running capsule has that name; ...` | `--agent` named no running process |

## What it does not show

- Reach through another capsule: a capsule without `Network` may still ask one
  that has it over IPC. `audit` shows what each capsule can do itself.
- Live device grants (see `devices*` above).

## How this is checked

`exposure_proofs` runs the shipping sources on the host: every verdict against
the kernel's own capability bits, the join over seeded and hostile tables
(registries of any length, names of any bytes), the v1 heading and refusals, and
the About and attest holder checks ([What a running machine shows about
itself](../security/checking-the-claims-what-a-running.md)).
