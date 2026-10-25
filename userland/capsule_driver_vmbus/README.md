# capsule_driver_vmbus

## Role

`capsule_driver_vmbus` is the VMBus core: the guest's side of Hyper-V's
device bus, which carries every synthetic device (network, SCSI, video,
keyboard, mouse) and the integration services. It also runs on QEMU's
`vmbus-bridge`. The core:
- contacts the host through the kernel's SynIC calls;
- agrees a VMBus version;
- takes the host's channel offers;
- keeps the bus: offers added later are kept, withdrawn ones released.

It opens no channel. A storage or network channel is given a record of its
own (`MkHvChannel`) for its driver to claim and drive; from then on the
kernel keeps the core from that channel. No host service is ever given, so
none is on.

```text
host --SynIC SINT 2 (MkHvRecv)--> driver.vmbus0 --MkHvPost--> host
       contact, version, offers, withdrawals
driver.vmbus0 --MkHvChannel(relid)--> kernel: a channel record for its driver
```

## Microkernel contract

- `MkDeviceList` finds the VMBus root: a `VIRT` record, vendor 0x1414,
  device 0x5642. The kernel registers it once the Hyper-V SynIC is up
  (`src/hardware/broker/vmbus.rs`).
- `MkDeviceClaim` on it; `MkDmaMap` of one coherent 12 KiB grant for the
  contact's pages. The record has no PCI address, so the grant's device
  address is the guest-physical one the host is told.
- `MkHvPost`, `MkHvRecv` on the root. The kernel posts only the bus's
  messages (the contact, the request for offers, a released relid, the
  unload), aimed at VP 0 and SINT 2, naming only pages in the caller's
  grants on the root (`src/arch/x86_64/hypervisor/hv/vmbus_out.rs`). A
  channel's open, close and GPADLs are its driver's alone, and its answers
  never reach the core.
- `MkHvChannel` for each network, SCSI or IDE offer: a record, vendor
  0x1414, device "NE", "SC" or "ID", held for `driver.netvsc0` or
  `driver.storvsc0`. The kernel learns each channel from the host's offer
  itself, and refuses any other interface.
- No `MkIrqBind` yet: the SynIC's vector, 0xC1, has no gate until
  integration. The core polls; the kernel takes a message off the SynIC on
  VP 0, where its end-of-message counts.
- `service:4302:driver.vmbus0` is held to the kernel alone; nothing calls
  the core yet.

## Interface contract

No IPC is served yet. The console lists the bus:
- `driver.vmbus0: VMBus 5.3 on connection 4, 9 channels offered, 3 given to drivers`;
- then one line per channel, for example
  `driver.vmbus0: channel 2: scsi, given to its driver`, or
  `..., sub-channel 1` for a sub-channel;
- a channel the kernel would not give names why (`already given`,
  `not known to the kernel`, `refused`).

Channels are named by interface alone: network, scsi, ide, video,
keyboard, mouse, heartbeat, shutdown, time sync, key-value exchange,
backup (VSS), dynamic memory, file copy, pci, or unknown. Instance GUIDs
are never printed, because they identify the virtual machine.

## Authority

`IPC`, `Memory`, `DeviceEnum`, `Driver`, `Dma`
(`CAPSULE_REQUIRED_CAPS = 0x98018`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Mmio`, `Pio`, `Irq`, `Network` or
`FileSystem`.

```text
allowed:   the VMBus root; the contact, offer requests, relid releases, unload;
           giving a storage or network channel its record
forbidden: opening a channel, GPADLs, any host service, any channel's answers;
           messages the kernel's check refuses
```

## Privacy and persistence

The host learns that the guest speaks VMBus, the version it agreed, and
where the guest's three pages are. Nothing else is sent by the core. Only
storage and network drivers open channels, each its own. Heartbeat,
shutdown, time sync, key-value exchange, VSS and dynamic memory stay closed:
the kernel never gives them a record. Ballooning stays off.

Every host message is held to its length before a field is read. At most
64 channels are kept and 16 messages handled a look. Nothing is stored.

## Runtime lifecycle

1. The claim and the grant.
2. INITIATE_CONTACT, versions newest first: 5.3, 5.2, 5.1, 5.0, 4.1, 4.0,
   3.0, 2.4. From 5.0 it goes on connection 4 and names SINT 2. Before 5.0
   it goes on connection 1 and names the interrupt page. Both forms name
   the two monitor pages and aim at VP 0.
3. The first version the host takes, and the message connection it names:
   1 or 4, or 4 when it names none. Another is refused, because the kernel
   would refuse the posts.
4. REQUESTOFFERS, then the offers gathered until ALLOFFERS_DELIVERED. A
   withdrawal among them is answered with RELID_RELEASED.
5. Each network, SCSI or IDE offer given a record (`MkHvChannel`).
6. The serve loop: a new offer is kept, named and given, and a withdrawn
   one is forgotten and its relid released. The kernel retires its record,
   revoking what its driver held.

## Failure model

Every bring-up step names its failure and gives back the grant and the
claim. A host contacted but whose offers do not all arrive is sent UNLOAD
first, so it forgets the contact. A host that does not answer the contact
in 5 s, or the offers in 10 s, fails that attempt. Attempts follow the
shared bounded schedule (`nonos_libc::start_driver`).

Until the SynIC's gate lands, a message reaches the core only when it
polls from VP 0. That is always the case on a one-CPU guest.

## Current implemented surface

- VMBus 2.4 to 5.3, the contact in both forms, offers and withdrawals.
- The channel table, by relid, with interface, instance, sub-channel,
  connection, monitor and the record given.
- Storage and network channels given records for their drivers.

## Wire format

SynIC messages of type 1, 16 bytes of header and up to 240 of payload. The
channel messages are Linux's `vmbus_channel_*` layouts: an 8-byte header,
then the fields.
- `initiate_contact`: 40 bytes.
- `version_response`: 16 bytes.
- `offer_channel`: 196 bytes. The interface GUID is at 8, the instance at
  24, the sub-channel index at 180, the relid at 184, the monitor at
  188/189, and the connection at 192.
- `rescind`: 12 bytes.
- `relid_released`: 12 bytes.

## State ownership

The host owns the channels; the core owns its view of them, and the
contact's pages for its life.

## Operating rules

- Open no channel and start no service from here; give only storage and
  network channels, each once.
- Print interface names, never instance GUIDs.
- Release a withdrawn relid at once.

## Release target

On Hyper-V (generation 1 or 2) and on QEMU with `-device vmbus-bridge` and
`-cpu ...,hv-vpindex,hv-synic`, the boot lists the host's channels.

## Release evidence

`vmbus_proofs` runs the shipping source against a model of a VMBus host
(QEMU's `hw/hyperv/vmbus.c`), with the kernel's check included by path in
the shim and the caller's grants as the pages it holds. It covers:
- the root found by its record, and claim, grant, contact and offers in
  order;
- what the contact names in both forms, and versions walked down;
- 2.4 as the oldest asked, the named connection, and one the kernel
  refuses;
- QEMU's versions;
- refusals that give everything back, a silent host, offers never ended
  (unloaded), a busy host and one busy too long;
- withdrawals during and after the offers, and offers added and replaced;
- the per-look bound and the table bound;
- the console, with no instance GUID;
- messages short, cut, or of another type;
- storage and network channels given records, others not, a channel
  offered later given and a withdrawn one given afresh, a refusal said;
- a given channel's answers never reaching the core;
- every message number the core uses pinned to QEMU's vmbus-proto.h.

A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_vmbus` keys).
- `vmbus_proofs` passes.
- Hyper-V or QEMU's vmbus lane: the boot lists the channels.

## Explicit non-goals today

Opening channels, which is each channel's driver's (storvsc, netvsc); the
integration services; keyboard, mouse and video channels, until their
drivers exist.

## Verification

- Build: `make -B nonos-mk-driver-vmbus`
- Proofs: `cd userland/vmbus_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
