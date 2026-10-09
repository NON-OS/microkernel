# NONOS 0.9.2

The hardening release. Every change below was proven by building for `x86_64-nonos-user` (and
compile-checking the kernel) and by the host proof suites. None of it was booted in the session
that made it. What a boot must still confirm is listed at the end and, step by step, in
.

## The release-critical bugs

### Virtio behind the IOMMU

With QEMU's intel-iommu, virtio devices must carry `iommu_platform=on`, and QEMU then offers them
as modern-only virtio 1.0 functions. The drivers spoke only the legacy transport, the broker
refused their MSI-X BAR, and each driver retried bring-up in a tight loop.

- A shared modern PCI transport, `userland/nonos_virtio`, used by virtio-net, virtio-blk,
 virtio-rng and virtio-gpu. It walks the vendor capabilities with every length and offset
 checked, skips regions that touch the MSI-X table, negotiates VERSION_1 and ACCESS_PLATFORM,
 and programs queues with read-back. `virtio_transport_proofs` holds it with 70 tests.
- virtio-net uses the 12-byte header VERSION_1 requires and reads its MAC under the config
 generation. virtio-blk binds INTx or MSI-X with its vectors programmed and read back.
 virtio-rng reads each completion from its own used element (it served the first request's
 length for every later one) and drops a bad used id. virtio-gpu reads its config at the modern
 offsets (it read past the structure before) and compares the whole used id.
- Kernel side, in the VT-d unit: global command writes never resend the one-shot bits
 (`gcmd_with`), the write buffer is flushed before context and IOTLB invalidation where the
 unit needs it, and the IOVA allocator never places a grant in the MSI window
 (0xFEE00000 to 0xFEEFFFFF). A device runs unconfined only when no remapping unit is in
 service at all (`confine/posture.rs`).

### HTTPS over the Nym mixnet

Pages over Nym never finished: the reply path broke in several places.

- Reassembly held eight half-built replies, and a TLS flight interleaves dozens, so the ninth
 evicted the first and that stream waited forever. It is now a bounded pool (64 sets, 2 MiB,
 90 s staleness, the last 256 completed sets remembered so a late duplicate is dropped).
- The gateway frame buffer was 2413 bytes, so any reply in a larger Nym packet was dropped as if
 it had timed out, with the bytes behind it. It is 64 KiB, an oversized frame is stepped over,
 and buffered bytes survive a failed read.
- Reply-block keys sat in a ring of the newest 512 while the exit spends them oldest first, so
 replies were dropped once a page had made a few dozen requests. Keys are stored by digest and
 removed when used, and a per-session estimate keeps blocks ahead of a long answer.
- Each poll returned at most one exit message, net.nym split a long message and passed its tail
 on as whole, and net.socks5 held a poll for a second while the browser waited 60 ms. A batched
 read (`OP_RECV_BATCH`) returns everything that came back, whole, up to 32 KiB per answer, and
 net.socks5's inbox is bounded per stream and in total.
- net.socks5 recovers a session net.nym lost with its gateway link.
- The 21-byte frames seen live are most likely acknowledgements of this machine's own packets,
 which means requests arrived and replies did not.
- The terminal's mixnet path never worked: it sent raw SOCKS bytes, never polled, and went direct
 when net.socks5 was absent whatever the chosen network. It now speaks the numbered framing and
 refuses to go direct unless Direct is chosen.

`nym_reply_proofs` (44 tests, new) and `capsule_socks5_proofs` (73 to 108) hold all of this,
including 20,000 random replies through every parser.

### A store install of Qwen fetches its model

Installing a Qwen app failed with NotProvided because the installer looked the model up as a
package. The Linux personality's installer now treats the model as a declared dependency: a file
already verified on the data volume is not fetched again, a missing one is handed to
`tool.model-fetch`, and every file must match its pin before the tier counts as installed. Each
failure (no data volume, locked volume, no network, a hash mismatch, two installs at once, a tier
whose file names do not fit the volume) has its own reason and its own sentence in the store.
The model fetcher no longer leaves the chosen network when the setting cannot be read: an
unreadable setting means Nym, never a direct connection. The kernel answers ENODEV, not EIO, for
a disk with no data plan, so the store can tell a Live USB from a failing disk.

### No driver can spin

A driver whose device failed retried for ten seconds or forever, with yields between tries, and
each round was a broker claim, refusal and release.

- `nonos_libc::bring_up` and `start_driver`: a device that is absent exits at once with
 `EXIT_ABSENT` (2) and claims nothing; a present device gets seven attempts with a sleep that
 doubles from 100 ms to 3.2 s, then one give-up line and `EXIT_GAVE_UP` (6).
- All 19 drivers use it. Every failed attempt now releases what it claimed (ps2_input, iwlwifi,
 rtl8821ce, virtio-net and virtio-gpu kept claims or grants before, so every retry failed).
 Unbounded yield loops in i2c_hid, usb_hid, xhci, bga, usb_msc and the audio sink lookup are
 parked and bounded.
- The kernel prints `[EXIT] driver.<name> status 2` or `status 6` with words, so absence and
 give-up show on serial for drivers that hold no Debug capability.
- Nineteen serve loops went straight round when a receive failed at once; they now sleep first
 (`nonos_libc::recv_ready`).

### The anonymity dashboard

About has a Proofs screen, second in its list: the attestation of what is running and the route
this machine's traffic takes, with a verdict (anonymous, exposed, not established, unknown).
Route reports come only from the transports themselves: the attest service keeps a board of them
and accepts a report only from the process the kernel attests as `net.anon` or `net.nym`, with
the Network capability. net.anon reports its consensus, signatures and circuit stage, and
net.nym its signed topology, whether its gateway is authenticated, its reply blocks and its
cover traffic, each every five seconds or on a change. `route_proof_proofs` and `attest_doc_proofs` hold the wire format, the
authorisation and the verdict.

## Isolation

- **DMA after release.** A released device could keep writing into frames the kernel had already
 freed on a machine without an IOMMU. The broker now stops bus mastering before it frees a
 released device's DMA frames, and at shutdown stops every device before the zerostate wipe,
 which now also wipes live DMA buffers.
- **munmap over device windows.** `MkMunmap` over the broker's DMA or MMIO windows freed frames a
 device still held. Both windows are now refused to `munmap` and to a fixed `mmap`.
- **The attack suite** (`userland/capsule_attack`, `nonos-ci/attack_suite_check.py`) tries every
 row of the isolation matrix with raw syscalls: every call a capability gates, made without it
 (twenty of them); a send to each traffic service without Network; registering a service without
 RegisterService; kernel and foreign addresses to MkDebug, IPC, MkMmap, MkMunmap, MkProcStat and
 MkAttestStatus; the broker windows; and, last, a loop that never yields followed by a write to
 the null page. The checker reads the kernel's trap line for that fault and requires the log to
 go on. It now fails any matrix row that has no refusal line of its own.

- **Raw disk sectors.** Any capsule could reach a disk driver with MkIpcCall. NVMe and AHCI read
 and wrote raw sectors for any sender, and virtio-blk guarded only its writes, so any app could
 read every sector of the disk, another system's unencrypted partitions among them, and through
 NVMe or SATA overwrite them. Each disk driver now serves everything but a health check to the
 kernel's own client and to a StoreWrite holder, asked of the kernel on every request, and
 answers EACCES otherwise. The installer holds StoreWrite for the disk it writes. The attack
 capsule reads a raw sector and writes past the end of each disk that runs.
- **Sends by pid.** Every endpoint a process serves is read from its one inbox, and
 `MkIpcSendToPid` wrote into that inbox checking none of their gates. A capsule without Network
 could look up net.sockets' pid and write to it as if it held Network. A send by pid now passes
 the gate of every endpoint the process serves, as a send by name does.
- **Network cards.** Any capsule could send to a card's driver, which transmits what it is sent
 and hands back what it hears, past the stack, the chosen route and Network: raw frames on the
 wire from any app, and iwlwifi's firmware commands and raw transmit among them. The six cards'
 endpoints now take only the services that drive them, net.core and net.l2 for a wired card,
 net.core and the Settings and setup Wi-Fi panels for a Wi-Fi card, by name or by pid. The
 attack capsule tries net.sockets' inbox by pid and each running card by name and by pid.
- **The wallet's sealed key.** The wallet keeps its key sealed at /data/wallet.vault, under a
 record the keyring opens for whoever presents it, and any capsule holding FileSystem can read
 that file. A capsule could open it under its own pid and sign or export the wallet's key, or
 seal a key of its own and swap the file so the wallet showed the attacker's address for
 deposits. The keyring now seals and opens that record for the wallet alone, in any of its
 windows, asked of the registry on every request (`capsule_keyring/src/server/vault_gate`).
- **Every other device.** The same held for the rest: any capsule could call the USB and I2C
 keyboards' drivers and read keystrokes back, or feed USB key reports that arrive as typed
 keys; drive the USB and I2C controllers raw, a USB disk's sectors among what is behind them;
 put its own surface on the whole screen through the GPU; and play through the sound card. The
 keyboards, pointers, random source and USB storage now answer the kernel alone; the USB
 controller takes the USB keyboard and storage drivers, the I2C controller the I2C keyboard,
 the GPU the compositor, and the sound card the audio service. `kernel_proofs` fails if the
 kernel spawns a driver that is neither on the list nor gated in its own driver, as the disks
 are by StoreWrite.

## Privacy

Every capsule that holds the Network capability was checked for a path that leaves the machine
outside the network the person chose.

- **The app SDK.** `nonos_std::net` connected every app over a direct socket and looked every
 name up in the clear, whatever network was chosen. A connection now follows the chosen network
 as the system's own route client reads it: a direct socket on Direct, net.sockets' mixnet
 socket with the name sent to the exit unresolved on Nym, and the route client's tunnel on
 Anyone (one stream per app at a time, a second refused rather than ending the first). A chosen
 network that is not running refuses with its reason. A name lookup, a datagram socket and a
 listener on an address other than 127.0.0.0/8 are refused off Direct. `route_link_proofs`
 holds the rule over every choice.

- **One route client.** `userland/nonos_route_link` holds the one rule (Direct only when Direct is
 chosen; Anyone through net.anon; anything else, an unreadable setting included, through the
 Nym mixnet; a chosen network that is not running is no route) and a byte stream over it: SOCKS
 over IPC to net.socks5 or net.anon with the numbered framing, every wait bounded, every answer
 length checked. `route_link_proofs` (50 tests) runs it against a fake proxy that delays, splits,
 loses, floods and garbles.
- **git over HTTPS** went direct on every clone and fetch, with the host name in clear DNS. It now
 leaves through the chosen network, and a route that is down stops it before anything is sent.
- **The wallet** resolved its RPC host in clear DNS and connected directly whatever the setting,
 which tied this machine's address to the wallet's addresses at the RPC provider. Every request
 now leaves through the chosen network, with the host name sent unresolved to the exit. The
 status line names the route that carried it.
- **Time.** net.ntp asked a fixed time server at every boot. It now asks only when Direct is
 chosen; otherwise it keeps the RTC time and says why. ping, nslookup and pull in the terminal
 run only under Direct.
- **The model fetcher** fell back to a direct connection when the setting could not be read; it
 now treats that as Nym.
- **curl and wget in the terminal** went through net.socks5 whenever it ran, whatever the setting,
 and went direct when it did not. They now use the shared client: Anyone through net.anon,
 Direct only when chosen, an unreadable setting as Nym.
- **Linux guests' names.** A guest connecting by name on a mixnet socket had the name resolved in
 the clear through net.dns, which named every host it reached to the network. The mixnet carries
 addresses only, so net.sockets now refuses a name on a mixnet socket (the guest sees
 ENETUNREACH) and never asks net.dns. An address written as one still connects.
- **Anyone and the browser.** net.anon answered a repeated exchange number with the answer it had
 kept, even when new bytes came with it, so a request sent after a missed poll was dropped and a
 page could stall on its first exchange. New bytes are now carried, as net.socks5 already did.
- **The browser with its network gone.** When the chosen network's service was missing at a
 navigation, the browser turned its route off, and off meant direct. Fetches still running for
 the page before, images and relaunches, then opened sockets through net.sockets, looked their
 hosts up in clear DNS and connected direct while Nym or Anyone was chosen. A socket, a connect
 and a DNS lookup now go direct only when Direct is the reader's choice; a private choice with
 no route open is refused (`capsule_browser/src/browser/net/mixnet/leave.rs`).
- **Nym after the first hour.** A node list fetched from the API is good for an hour, but the
 fetch ran only until the list first held a gateway and an exit. An hour into the boot every
 send failed as expired and nothing fetched a new list, so Nym stopped for the rest of the
 boot. The list is now fetched again ten minutes before it expires, with the old one routing
 until the new one is installed; a fetch that keeps failing leaves nothing sent once the old
 list expires, rather than any fallback.
- **Anyone at the hour mark.** A consensus is fresh for an hour and valid for hours more, but the
 refetch dropped net.anon back to its cold bootstrap: until the new consensus and its
 microdescriptors were in, every stream open was refused, no circuit was built, the relay set
 was rebuilt from the few microdescriptors fetched so far, and the Proofs screen read not
 established. A refresh now serves from the relays, weights and expiry in hand and swaps the new
 set in once it is whole; once the old consensus is invalid nothing is built from it.
- **Anyone guards.** A guard that failed three dials was replaced by a draw that avoided nothing,
 so a heavily weighted guard unreachable from here came straight back, and two of them could
 take turns for the whole boot with no circuit ever built. The redraw now avoids the last eight
 guards given up on and their /16s, and starts over only once every guard has failed. A guard
 that took the link and dropped it within 30 seconds reset its failures at every open and was
 never replaced; its link breaking young now counts against it. A circuit failing further along
 the path does not, so a bad middle or exit relay cannot churn the guard.

## Capabilities

- `scripts/cap_audit.py` reads every capsule's code, its linked crates and the kernel's
 capability table, and reports each granted bit with no call behind it and each call its mask
 does not cover. It runs on 96 capsules and has its own self-test.
- `docs/release/0.9.2/capabilities.md` documents all 36 bits with the kernel line that checks
 each, and every capsule's mask with a verdict per bit.

- **Applied.** Every mask is held to what its code calls, with the kernel's grant changed in the
 same commit. login gained the display bits it waited on forever. Bits nothing uses were
 dropped: IO and StoreWrite from nonos-install, Keyring from vfs, Crypto from socks5, the
 display query from four capsules, Irq from four drivers that poll. Debug is optional in 27
 capsules and granted only by a `capsule-serial-debug` build; the Linux personality and the
 attack capsule keep it required. `scripts/cap_audit.py --strict` passes on the whole tree, with
 the attack capsule and toolkit's unused render op exempt.
- **FileSystem means something.** vfs served any capsule that could send it a message, so the
 FileSystem bit was a label. vfs now serves the kernel and holders of FileSystem alone, asked of
 the kernel on every request, and every capsule that reads or writes through it holds the bit:
 the shell, the installer, the market, policy, setup, the wallet, settings, net.core and the
 command-line tool sandbox. `fs_proofs` and `cap_audit` hold both sides, so the gate and the
 grants cannot drift apart.
- **virtio-net** draws a new station address every boot, as the wired drivers already did, and
 never takes the device's MAC feature.
- **CoreExec is held by what calls it.** The bit admits four syscalls: `MkGetPid`, `MkArgs`,
 `MkCapsuleLoad` and `MkCapsuleVerify`. 45 services called none of them and lose it, the kernel's
 spawn grant with them. crypto and market drop Crypto, which gates only the kernel's own
 primitives; both compute in process. `scripts/check_mirror_caps.py` evaluates every
 `requested_caps` a spawn file names and fails on a bit outside the manifest, since a mask cut in
 `Capsule.mk` and left in the mirror stops that capsule at boot. It also holds each README that
 quotes a mask to its manifest: 16 had drifted, some since before 0.9.2.

## The kernel's syscalls

Every syscall a capsule can make was reviewed for its pointers, lengths and arithmetic; the trust
path (signature checks, the capability check, the spawn gate) is unchanged. `kernel_proofs` went
from 136 to 189 tests.

- The clock scaled its counter to milliseconds in 64 bits, which overflows after about 71 days at
 3 GHz; with overflow checks on in the kernel, every clock read from any capsule would then have
 stopped the machine. It now scales in 128 bits and saturates.
- A thread spawn with a start outside the user half was published in the process table and then
 refused, and stayed there for good with its pid and kernel stack; any capsule could repeat it
 until pids or memory ran out. The start is now checked first (EINVAL), and a thread that cannot
 run after it was published is ended.
- A pid, port, endpoint or device field wider than its type was truncated onto another value
 (pid 2^32 + n became n; BAR 256 became BAR 0; flag bits a broker check should have refused were
 dropped). Each is now refused with EINVAL.
- Local sign and verify and capsule loads took caller-sized buffers (up to 64 MiB) infallibly, so a
 heap that could not hold one halted the machine. They now answer ENOMEM.
- Draining input events or a child's output with a bad buffer took the keystrokes or the line and
 then failed. The buffer is checked first.
- A Linux guest whose kernel stack or empty capability set could not be made was left in the
 process table with its pid, never run and never ended. It is now ended, as a guest that cannot
 be recorded already was.
- An IPC inbox counted messages, not bytes: 1024 messages of up to 1 MiB each could wait in one
 inbox of a 256 MiB kernel heap, so any capsule that sent faster than its peer read could halt
 the machine. Inboxes now hold at most 96 MiB together and 16 MiB each, and one sender at most
 half of any one inbox, so a capsule that floods a service leaves the other half to everyone
 else. A send past that is answered as a full inbox already was (EAGAIN, or EBUSY for a reply).
 Nothing caps all one sender has waiting: a reply waits in the inbox of the client that asked,
 so such a cap would let a client that never reads its replies spend a server's whole allowance.
 Each inbox's stats now show the bytes it holds.

## Services and requests they cannot read

- **Every service answers.** 26 services and drivers dropped a request they could not decode
 without a reply: wm, input_router, policy, net.dns, net.sockets, net.ip, net.tcp, net.udp,
 net.dhcp, net.l2, net.nym, net.core, toolkit, keyring, installer, payment, ramfs,
 wallpaper_catalog, audio (every request while no sink was attached), virtio_gpu, usb_msc,
 usb_hid, i2c_pci, i2c_hid, iwlwifi, hda and rtl8821ce. The caller waited out its whole timeout,
 and the call kept its place in the kernel's queue of replies that service owes, 64 deep, so a
 program sending junk could leave every other caller of the service refused as busy. Each now
 answers with an error under the op and request id the frame carried.
- **One caller, one share.** A call's place in that queue is kept until the service answers, even
 after the caller stopped waiting, so a late answer is discarded instead of being taken for the
 caller's next call. A service that never answers a request kept the place until the caller
 exited. One caller now holds at most 8 of a service's 64 places; past that only its own calls
 are refused. `kernel_proofs` holds the share.
- The toolkit's button paint added a button's width to its position unchecked; a button at the
 edge of the coordinate range panicked the toolkit. It saturates.
- Every request-serving capsule's request decode is driven with 200,000 seeded hostile inputs
 and a named boundary set on the host; apps are covered through app_skeleton's delivery decode.
 New proof crates: clipboard, policy, wallpaper_catalog, seq_wire, udp, dhcp, l2, net_core,
 net_anon and service_header, with image_codec's decode in `image_codec_proofs`.

## Recovery

- **A core service that ends is started again.** The kernel kept a restart policy for every
 capsule it spawned, but nothing called it, so one crash in net.sockets left the machine without
 a network, in vfs without files, and in the compositor without a screen, until a reboot. Init
 now watches 29 core services (the network stack and both anonymity networks, files, keyring,
 entropy, crypto, policy, the store, attestation, audio, clipboard, input routing, compositor,
 window manager, desktop shell, wallpaper and image codec) and restarts one that ended: two
 seconds apart, at most eight times, each logged on serial, and a ninth end is reported once
 and left. Drivers are not watched, since a driver exits on purpose when its device is absent;
 nor are apps, setup, the installer or the Linux personality. `kernel_proofs` holds the rule, the
 policy, and that every watched name is one the spawn plan registers.

## Networking

Each fix below came with a test that fails on the old code.

- **TLS.** The subjectAltName was found by searching the whole certificate for its OID, so a
 certificate legitimately issued for one domain, with a modulus ground to carry another name,
 matched any host. It is now read through the extension list. A second Certificate after the
 CertificateVerify is refused, the chain walk is capped at ten certificates, and records over
 the RFC 8446 limits end the session. The wallet had its own copy of the subjectAltName bug,
 which put every RPC call open to a man in the middle; it now uses the fixed matcher.
- **TCP.** Two reordered full segments crashed net.tcp. A listener's connection never sent first.
 A blind reset could kill a connection, an ACK reopening a zero window was ignored, data after
 our FIN was dropped, and segments from broadcast or impossible sources were taken. The peer's
 MSS is read and honoured, and the reassembly buffer drains across the sequence wrap. A closed
 window holding data back with nothing in flight is probed (RFC 9293 3.8.6.1), from the
 retransmission timeout doubling to 60 s; a peer that answers is kept however long its reader
 takes, and one that leaves eight probes unanswered is dropped. Before, a lost reopening left the
 queued data unsent for good. A read that at least doubles a small window announces the room at
 once, where a peer that saw the window closed used to wait for its own probe. A SYN to a
 listening port left an entry that never ended, and every application's sockets reach net.tcp
 as one owner, so at most 31 forged SYNs refused every later connection on the machine, outgoing
 ones included. A listener now holds at most eight half-open connections, the oldest giving way to
 a new SYN, and each ends 30 s after its SYN unless the peer finished it. For the same reason a
 closed connection's minute in TIME-WAIT held one of those places, so the 33rd connection closed
 within a minute refused every new one, and a peer that never sent its FIN held one for good in
 FIN-WAIT-2. TIME-WAIT no longer counts and gives way, oldest first, when the table is full;
 FIN-WAIT-2 ends after 60 s.
- **Sockets.** net.sockets kept a client's sockets after the client ended, each with the stream,
 port or mixnet connection behind it, so a client crashing in a loop filled its 256 slots and
 every later socket on the machine was refused; one client could also open all 256. The sockets
 of an ended client are now freed and their transports released, and one client holds at most
 half the table. `sockets_proofs` holds both. net.tcp does the same for the capsules that use
 it directly, the terminal among them: an ended owner's connections are reset, so their peers
 stop waiting, and freed. net.udp kept an ended client's ports, so a service that came back
 could not bind its own port again; a bind that finds its port or the last place taken now frees
 ended clients' ports first, and one client binds at most half of the 64. net.anon's 32
 streams were shared the same way: one caller could open them all, and a caller that ended left
 its streams, ended by the exit or not, for good. One caller now opens at most half, and every
 two seconds an ended caller's streams are ended at the exit and dropped. net.socks5, the Nym
 route for the browser, the terminal and the wallet, held a slot, a tunnel and up to 1 MiB for
 each of its 32 callers, and one that crashed held them for good: after 32 crashes the Nym route
 refused every program. It also kept one reply for every caller ever seen. An ended caller's
 slot, tunnel, held bytes and kept reply are now freed.
- **net.nym and net.core.** net.nym's 32 mixnet sessions, each with a key and up to 2 MiB of
 replies, were never freed when a client ended, and a dead client's session held the directory
 refresh back for good, since the refresh waits for an empty table. An ended client's sessions
 are now closed and their keys wiped, and one client opens at most 16. In the desktop build
 net.core is the TCP and UDP stack: an ended client's connections are released as its close
 would release them and its ports unbound, and one client, net.sockets included, holds at most
 half of each table. Switching between wired and Wi-Fi rebuilt the stack but kept the clients'
 handles into the old one, so a client's bytes could reach another client's connection or end
 net.core; the old stack's connections and ports are now forgotten, and their next call is told
 the socket is gone.
- **HTTP.** Bodies are framed as RFC 9112 6.3 says: conflicting Content-Length values are
 refused, the last transfer coding decides chunking, 204 and 304 carry no body, interim
 responses are skipped, every chunk's CRLF is checked, field names are held to the token
 grammar ("Content-Length : 5" was a smuggling vector), and a response head is capped at 64 KiB.
- **DNS.** The answer is the one for the name asked, along its CNAME chain, with compression
 pointers that can only go backwards. Garbage from the server's address no longer ends a lookup,
 and no answer is cached longer than a day.
- **NTP.** A reply whose transmit time had the top bit set but fell before 1970 was subtracted
 unchecked. The release build wrapped it to a clock thousands of years off and set it; any
 server, or anyone on the path of the direct route NTP takes, could send one. It is refused. The
 proof that should have caught it passed only because CI builds proofs in release, where the
 proof's own arithmetic wrapped the same way.
- **DHCP lease.** In builds with the split stack, any program that could reach net.dhcp could
 release the machine's address, taking it off the network, or have it broadcast its MAC in a
 fresh exchange as often as it liked. Request, renew and release are now the Settings app's
 alone (E_PERM to anyone else); net.dhcp takes its lease at boot by itself, as before, and
 every client can still read the lease.
- **IP, DHCP, ARP.** No reply to a broadcast echo, no datagram from an impossible source, a
 malformed frame dropped without ending the poll, DHCP replies' IP and UDP framing checked, and
 the ARP cache learned only from a real neighbour.

## The Linux personality

Each fix below came with a test that fails on the old logic; `capsule_linux_proofs` went from 134
to 295 tests.

- **What a guest can make the personality and the store hold.** A guest's /tmp and other private
 directories were bounded at half its family's memory and not at all in names, so a guest could
 fill the store's 2048 names, leaving no capsule able to create a file, or write enough to run
 capsule_vfs out of memory and take the store down for everyone. They now hold 16 MiB and 128
 names (ENOSPC past either, and statfs says so); a hard link across mounts is EXDEV instead of a
 copy of an installed library; and files being written count against the same quota while they
 live in memory. Wayland frames are capped at 16 MiB and allocated fallibly, a client that never
 reads its events is held as a full socket holds it, and a batch of requests is walked once
 instead of cut from the front one at a time. Empty datagrams are charged as Linux charges them,
 record locks stop at 4096 (ENOLCK), and a family stops at 512 tasks (EAGAIN from fork and clone,
 reported as RLIMIT_NPROC), so a fork bomb ends. A punched hole after a refused resize no longer
 allocates the refused length, which aborted the family.
- A process's /proc image record goes when the process is reaped, so a long shell loop no longer
 grows the personality. A bad path answers ENAMETOOLONG, EBADF, ENOTDIR or ENOENT as Linux does,
 and an empty one is refused (`rmdir("")` removed the working directory). An execve vector too
 large is E2BIG instead of cut short.

- A supervisor could make guests and guest threads without limit, each a kernel process with a
 pid and a kernel stack, until the machine had none left. One supervisor now holds at most 1024
 (EAGAIN past that), twice what the personality allows a family.
- A guest ended from another CPU, as its terminal closed say, had its page tables and memory freed
 on a later tick while its supervisor could still be copying into it, from its own CPU, through
 the kernel's mapping: the write then landed in memory already handed to another process. The
 kernel now frees a guest's memory only with no such copy in progress, and a copy checks that
 the guest is still its own inside the same lock it copies under.
- mmap, mprotect, mremap and munmap check their arguments in Linux's order; mremap's end can no
 longer wrap; MAP_FIXED below the minimum address is refused; the region list is capped at
 65530, as vm.max_map_count is.
- futex decodes its operation as Linux does, and a bitset wake wakes only matching waiters, so
 wakeups are no longer lost.
- sigaltstack, SA_RESTART and rt_sigaction keep state as Linux does; a restarted pipe write no
 longer repeats the part that completed. ppoll, pselect6 and epoll_pwait wait under the mask
 they are given. rt_sigtimedwait never takes SIGKILL or SIGSTOP.
- clone refuses the flag sets Linux refuses. poll, select and epoll report and refuse as Linux
 does, with loops and nesting past four refused.
- What a guest can make the personality hold is bounded: descriptor slots, SCM_RIGHTS
 descriptors and a cmsg walk that used to loop forever, message gather, DNS sizes, Wayland
 objects, the symlink table. Error paths no longer leak pipes, eventfds, timerfds or signalfds.
- Terminal ioctls answer in the family's numbers; TIOCGPGRP no longer leaks the kernel's pid, and
 TIOCSWINSZ raises SIGWINCH. readv and writev check the descriptor before the iovec array and
 every length in it; one pwritev2 call crashed the capsule before. A timer near the top of the
 clock no longer fires in a busy loop. wait4 and waitid read their options as an int, so musl's
 __WCLONE is accepted.
- A guest's outbound streams follow the network the person chose. Under Anyone they go through
 net.anon's stream front, and with net.anon stopped a connect fails as unreachable with a
 `[LINUX]` line naming the Anyone network; nothing else is tried. Under Nym, or a default that
 cannot be read, they stay on the Nym mixnet as before, and under Direct too, since a guest never
 reaches the network directly. Before, every guest stream took the mixnet, so under Anyone each
 connect failed and a guest's traffic left on a network nobody chose. Past net.anon's share of
 16 streams a guest gets ENOBUFS. `capsule_linux_proofs` holds the route over every choice, each
 stream closed exactly once, a bounded receive buffer and hostile replies, 203 to 241 tests.
- The syscall scanner now sees calls routed through every number-module alias: 17 served calls
 had no disclosure line. Disclosure shows 220 of 220.
- **Twelve Linux tools, built in the tree, in every image.** `sqlite3` 3.53.4, `qjs` (QuickJS
 2026-06-04), `jq` 1.8.2, `tclsh` 8.6.18, `mruby` 3.4.0, `rg` 15.2.0, `fd` 10.5.0, `gojq`
 0.12.19, `perl` 5.44.0, `nano` 9.2, `make` 4.4.1 and the `openssl` 3.5.9 tool are in every
 image's store with their proofs, so each runs on a fresh boot with no network and no install.
 Each is built from a pinned upstream source (each pin says what it was checked against), as a
 static, non-PIE executable with no build path in it, and a rebuild gives the same bytes. Each
 program's build now sees only its own recipe and pins (`tools/linux-userland/`), so an edit to
 one no longer rebuilds Python. `docs/handbook/linux/tools-0.9.2.md` lists every tool with its
 pin, size, licence and how it reaches a machine.
- **An install leaves through the network the person chose, to a named mirror.** Package fetches
 went over the Nym mixnet whatever the setting, to Alpine's CDN by the literal address
 151.101.66.132. They now go through `Route::chosen` like every capsule holding Network, to
 `dl-cdn.alpinelinux.org`, which the route resolves where it leaves. A chosen network that is
 not running stops an install before anything is fetched, as `NoNetwork`, instead of reading
 as a mirror that refused; nothing falls back to another network. A mirror on the local network
 is still dialled directly.
- **In-tree packages can install from the Linux tab.** A later in-tree tool declared as a package
 is listed as `linux.nonos-<tool>`, pinned by the BLAKE3 of its content and served by the NONOS
 package mirror `nonos.toml` names (`linux_packages`; empty by default, which lists none). The
 path on the mirror is the pin, so the mirror is trusted with nothing. 0.9.2 declares none.
 `capsule_linux_proofs` holds the reader, the mirror's naming and the tool names, and reads the
 seal's own content tar back with the personality's tar reader: 310 to 330 tests.

## Filesystem and store

- One damaged store entry used to fail the whole store; it is now left out alone and the load
 reports it. Names in the table must be normalised absolute paths.
- **The store carries every Linux tool.** Its loaded budget is 96 MiB (from 60), its table 512
 entries (from 128) and its streamed budget 20 MiB (from 48; the wallpaper collection is 11.6
 MiB), which still fit below the disk plan with a full table. vfs reads a table longer than one
 block request in pieces, and its heap is 320 MiB, backed only as it is touched. The standard
 store is 218 entries and 85.6 MiB loaded, leaving 10.4 MiB and 294 entries for what a person
 keeps.
- **A kept file draws on the budget the next boot counts it against.** vfs checked every write
 against all the store's bytes, the streamed wallpaper collection included, so on a standard
 image no file could be kept: the wallet's vault, Settings and the market catalogue all failed.
 A wallpaper written past the streamed budget was taken and then refused at the next boot.
- `tools/nonos-store-check IMAGE` reads an image's store as vfs does and names each entry it
 would refuse, and why; a boot says only "store status 9".
- An append that grew the table wrote over the first payload on every packed store; the payload
 is now moved first. A removal cut short by a power loss could serve one name with another
 file's bytes; it now leaves the old store, the new one, or one slot refused.
- No disk, a faulting disk, a damaged store and a full store are four different errnos. A store
 write with no disk answers ENODEV, as a read does.
- ramfs bounds each file and the whole of `/ram`, so a large lseek no longer aborts it.
- **The store cannot be run out of memory.** vfs keeps every file in its 192 MiB heap, and a
 write grew its file with an allocation that could not fail gracefully, as did a copy, a
 truncate and an install: any client asking for more than the heap held aborted the store for
 the whole machine. All of them now reserve what they add fallibly and within 160 MiB for all
 files together, and answer ENOSPC past it while the store goes on. One client creates at most
 512 of the 2048 names, so nobody can leave the rest unable to create a file; the boot seed and
 the packages staged from disk are not held to it.
- A Linux run that crashed left its private files under /linux-private for good, since the
 personality aborts on a panic before it can remove them. vfs now removes and wipes the private
 files of a run that ended, within two seconds, as it closes an ended client's handles.
- vfs's 256 file handles and ramfs's 1024 were never closed when the client that held them
 ended, so a client crashing in a loop filled the table and every later open on the machine was
 refused; one client could also take every handle. An ended client's handles are now closed and
 one client holds at most half; the kernel's own handles are never taken.
- The ESP the installer writes is read back by an independent FAT checker. It found three
 writer faults, now fixed.

`fs_proofs` went from 227 to 307 tests, `nonos_disk` from 25 to 38.

## Apps

Every app now says in its own window when a service it needs is missing, a view is empty, or the
input is too large, instead of drawing nothing, keeping stale content, or freezing. A new
`apps_proofs` crate (63 tests) holds the decisions for the apps that had no proofs.

- **Process manager.** The kill prompt, its outcome, a protected process and an unreadable table
 were never drawn. They are now, and an unreadable table clears its rows instead of keeping a
 kill armed on stale ones.
- **Settings.** A silent policy service froze the first paint for about eight seconds and
 defaults passed for stored values. The first read stops at the first timeout and says the
 values shown are not stored.
- **Files.** A failed listing showed as an empty folder, or kept the previous folder's entries
 under the new name, and its retry never fired. It now says the files are not available and why,
 and retries on the next key or click.
- **Text editor.** A file over 256 KiB on a mount where stat fails opened cut short, and the next
 save would have shortened it on disk; it is now refused. Explorer failures show over a tree
 with rows.
- **Audio player.** With no audio service, play sat in Playing forever; it now stops and says so.
 A track that will not load is named, and the previous track no longer plays in its place. A
 track loads in bounded memory, so an ordinary four-minute song no longer runs the heap out.
- **Image viewer.** On-demand windows claimed the wrong pid with the file store and were refused
 every read. Images over 20 megapixels are refused before the copy, and an unlistable store is
 told apart from one with no images.
- **Video player, clock, calculator, mdview.** An unlistable library no longer reads as empty.
 The clock says when the system time is not set and times its stopwatch and timer since boot.
 The calculator names its error and its readout holds the widest value it can compute. An empty
 readme is named.

- **Audio streams are their opener's.** Stream ids are sequential and a stream answered any
 client, so a program could feed, pause or close another's stream by guessing its id, and a
 player that crashed kept its slot for good: four crashes and no program could play a stream.
 A stream now answers only the client that opened it, one client holds at most two of the four,
 and an ended client's streams are closed. `audio_proto_proofs` holds all three.
- **The keyring and login.** An owner's keys stayed in memory and kept their places after the
 owner ended, so eight ended wallets filled the keyring. They are now wiped and dropped. A login
 session whose owner ended stayed open, so every later login answered busy; it is now locked.

## The trust path, tested harder

No signature check, attestation verifier, STARK verifier or kernel capability check changed in
this release. Their tests grew.

- The attestation path check and the v4 trailer parse meet 20,000 arbitrary byte strings, every
 truncation, 20,000 paths with up to eight bytes damaged, and 20,000 single-byte hits anywhere
 in a v4 trailer. None panics, and nothing but the enrolled path, unchanged, verifies for its
 own slot (`nonos-attest-path`, 38 to 43 tests).
- The manifest and NONOS-ID certificate decoders refuse every truncation of every committed
 artifact and a byte appended to each, and take 30,000 arbitrary inputs and random damage
 without a panic (`admission_proofs`, 3 to 7 tests).

## Hardware

- hda found no controller where firmware left the legacy interrupt line at 0xFF, which is common
 on laptops that expect MSI. Discovery no longer asks about the interrupt; the bind asks for INTx
 only when a line was routed, then MSI-X, then polls.
- Malformed-input proofs for parsers that had none: rtl8139's receive gate, rtl8169's receive
 descriptors, ps2's aux packets, ahci's request header and body.
- virtio-net read a used entry whose descriptor id named no primed slot from the slot that id
 folded onto, which the device might still own, and then posted the wild id back to the
 device as a free buffer. The entry is now consumed and dropped, and nothing is refilled.
- NVMe: the enable, disable and completion waits and the identify parse run on the host against
 a hostile controller, with a 200,000-round fuzz, and six faults were fixed. CAP.DSTRD, which the
 device chooses, placed doorbells that were never checked against the mapped registers, so a
 stride of 15 wrote past the window. The waits ignored CAP.TO and gave every controller 5 s; it
 now gets what CAP.TO asks, up to 127.5 s. A completion from another submission queue finished
 the command, and a stray completion at the head stalled the queue until a reboot; both now
 need the phase, SQ id and command id to match, and a stray entry is consumed. Namespace formats
 with metadata, or an FLBAS past NLBAF, were taken on trust and are refused. MDTS was parsed and
 never used; each command now keeps within it. A controller left in fatal status was refused
 before the reset that clears it could act; the disable wait now waits for the reset.
- AHCI: IDENTIFY, the command span, the PRD and the completion wait run on the host, with fuzzes
 and named edges, and five faults were fixed. A capacity of 2^48 sectors or more let a request
 past 2^48 through, and the FIS dropped the high bits, so the I/O landed near LBA 0; a zero
 count, or one with no 48-bit feature set, was served too. A disk is now served only on a 48-bit
 count from 1 to 2^48-1 with the feature set supported and enabled. The logical sector size was
 never read, so a 4Kn disk was served in the wrong units; only 512 is served. A command that
 failed between the status read and the slot read passed as done and its buffer went to the
 client; it is now done only on a clean status read after its slot clears. Recovery after a
 failed command left FIS receive off, so on QEMU one error failed every later command until a
 reboot. A port named past the mapped ABAR window faulted the capsule.
- xHCI: the event ring, and every wait and poll that reads it, run on the host over a model of
 the controller's producer, with a 200,000-round hostile fuzz. They found three faults, each
 fixed. The driver zeroed each event it read, so once the ring wrapped, every cleared slot read
 as a new event: from the 64th event on it read phantom events, ran ahead of the controller and
 timed out on real ones. An interrupt-IN residual larger than the request handed up the stale
 buffer as a new report. A transfer event completed a transfer on its pointer alone; it must now
 name the transfer's slot and endpoint too.
- USB HID: the descriptor walk and every report decode have host proofs against a hostile
 device, a 200,000-round fuzz per surface among them, and they found eight faults, each fixed.
 An endpoint after a cut interface record bound to the interface before it. Endpoint 0 and
 packet sizes that cannot hold a report were bound. A key named in two slots of one report was
 pressed twice. A rollover report released every held key and the next report typed them again.
 Reserved and modifier usages in key slots were posted as keys. A short keyboard read was padded
 with zeros and released every held key. Tablet positions past the logical range went through.
 An endpoint whose packets are over 8 bytes bound but was never read, since the xHCI driver
 refuses longer reads.
- Every driver README carries a real-hardware bring-up checklist.
- `xhci_proofs` no longer fails half its runs under `--release`: the port proof waited a count of
 reads for its model thread and now waits by time.

## Wi-Fi

- **iwlwifi on laptops.** Discovery skipped any Intel adapter whose firmware left the legacy
 interrupt line at 0xFF or the pin at 0, which is common on UEFI laptops and is likely where
 "no Wi-Fi" came from. It now finds the adapter whatever the line says, and binds INTx when
 routed, then MSI-X, then runs polled. Its staging DMA grant asked for 2 MiB where the broker
 allows 64 pages, so it failed on every adapter; on AX211 (SO) platforms it now picks its
 firmware by MAC and RF type, boots it to ALIVE in grants of 64 pages or less, and scans
 passively.
- **iwlwifi joins.** On AX211 (SO) the Intel driver now joins WPA2-PSK and WPA3-SAE networks
 (hash-to-element and hunting-and-pecking) and carries data both ways through net.core's link
 protocol, with the same shared MLME, SAE, supplicant and CCMP code as the RTL8821CE; the
 pairwise and group keys are installed in the firmware and group rekeys follow. It finds a
 network by listening only, so a hidden network is not joined, and before the link opens it
 sends nothing but authentication, association and EAPOL. It now holds Crypto (mask 0xF8038):
 it draws a fresh, locally administered station address every boot, and each join's nonces and
 SAE secrets; without randomness it only scans, from a fixed address that never reaches the
 air. It answers net.core's link probes, so the stack no longer waits out a timeout on an Intel
 card. The Wi-Fi client sends it joins, the RTL8821CE still asked first on a machine with both,
 and net.core's autojoin uses it. Every firmware command it sends was checked against Linux
 v6.12's headers with gcc, 133 sizes and offsets, all matching
.
- **The shared Wi-Fi core** runs the four-way and group key handshakes as 802.11-2020 12.7 says
 (group rekey, a repeated message 1, the RSNE in message 3 checked against the beacon, a forged
 MIC no longer ending the join, a hex PSK not run through PBKDF2), and WPA3-SAE over group 19
 with hash-to-element and hunting-and-pecking, held to the IEEE Annex J.10 vectors. A network
 saved as WPA3 is never joined over WPA2. Every received data frame is checked against the
 association: from the BSSID only, no fragments or A-MSDUs, no unprotected frame on a protected
 link, replay counters per TID.
- **rtl8821ce** joins WPA2 and WPA3 through that core, checks every received frame, installs group
 rekeys, handles deauthentication, and sends a directed probe only for a network saved as hidden.

`nonos_wifi_core_proofs` went from 14 to 70 tests, `iwlwifi_proofs` from 79 to 181,
`rtl8821ce_proofs` from 116 to 150, `wifi_panel_proofs` from 6 to 19.

## Desktop

- One window order. The compositor drew the focused window on top and the rest in table order,
 while the window manager hit-tested its own stack, so the window on top was not always the one
 a click reached. Both now stack by raise order, and a press on a window raises and focuses it
 in the same press that starts a drag on its title bar.
- A title-bar drag keeps the pointer until the release, so the terminal's tab strip can no longer
 steal it. The resize band no longer starts on the title bar's right end.
- The dock's window stays under application windows, so a press on a maximised window's lower
 edge no longer launches a dock app. A moved or closed window leaves nothing behind, and a dead
 app's window goes within a second.
- One program opens at most half the window manager's 256 windows, and a tray app that ended
 loses its tray items; one app holds at most half the tray.
- A surface's slot was freed only when its owner exited; unmapping a surface kept its slot. The
 machine has 256. image_codec registers one per decoded image and never exits, the image viewer
 one per image it sends, and an app one per resize, so a few hundred images or resizes left no
 slot for any new window. An owner that unmaps the whole of a surface now gives it up: its slot
 is freed at once, or once the last process that attached it lets go. image_codec also kept
 every decoded image, up to 64 MiB each, for good. It now lets an image go once the client asks
 for the next one, has ended or has waited 30 s, and holds at most four.
- The compositor raised any program's window to the top when asked, from any sender, so a program
 could put its window over another's at will, over a password prompt among them, while keys
 still went to the window underneath. Only the window manager, which owns the stacking order,
 may raise a window now; anyone else gets E_PERM.
- Setup and the installer lay out from the canvas on one scale (1, 1.5, 2) and one 8 px rhythm,
 proven against real font widths at 1366x768 to 3840x2160. The installer window is 704 px tall
 so every screen fits; the Qwen list scrolls around its selection on short screens.

## Before release

- Every capsule rebuilt from this tree is re-signed and re-enrolled under ek's keys, as at every
 release. The installer and its command line also changed masks (StoreWrite added), and 46
 services shed CoreExec or Crypto, so their committed manifests in `nonos-data/trust/capsules`
 are stale until then.

- Twelve new Linux userland programs (`linux-userland-sqlite3` through `-openssl`, ports 5164 to
 5187) are in the capsule catalogue and are enrolled and signed at the seal like every capsule.
 `tools/nonos-store-check <out>/nonos.img` after the seal should read 218 entries and nothing
 refused.

## Known gaps

- The kernel profiles (`microkernel-full-gui`, `microkernel-desktop-gui`, through
 `microkernel-desktop-base`) still turn on `capsule-serial-debug`. The build's `hardened` and
 `airgapped` profiles (`tools/nix/config.nix`) take it out of everything those turn on, so an
 image built from either grants no capsule Debug; the default `standard` profile keeps it, and
 capsules on it still write serial lines. A release image is built `hardened`.
- Admin is still held outside init by policy, power, install and install-cli; narrower kernel
 bits for reboot and policy push would let them drop it.
- iwlwifi joins and carries data on AX211 (SO) platforms only; older Intel families have no air
 path. It has no rate scaling (data goes at the highest basic rate), no QoS, HT or aggregation,
 no power save, no beacon-loss detection, and services no interrupt (no MSI-X IVAR setup). A
 firmware assert takes the radio down until reboot: there is no firmware restart. Its older NIWF
 protocol answers only net.core and the Wi-Fi panels, and nothing in the tree speaks it.
- A Linux guest never leaves direct, so under Direct its streams stay on the Nym mixnet and fail
 as unreachable while net.nym is stopped. That is deliberate: the personality runs programs
 nobody here wrote. Only the install role holds Network; a guest in the run or terminal role
 reaches no network under any choice.
- DMA escape is proven by construction and by host proofs of the IOVA allocator; the refusal
 itself needs a boot with the vIOMMU on.
- Most driver manifests lack the Debug capability, so a driver's own lines (beyond the kernel's
 `[EXIT]` line) do not reach serial.
- The login capsule's START_SESSION cannot succeed: it asks the keyring to unlock a key for the
 caller, and the keyring unlocks only for the program that asks it. Nothing in the tree starts a
 session today, so no flow depends on it; fixing it changes how the keyring hands keys to
 another program, which is left for a reviewed change.
- The payment capsule is not spawned by any profile. Its nonce map is keyed by a pid the caller
 states and grows with each new one; it needs bounding before anything spawns it. The power
 capsule is not spawned either, and reboots or shuts down for any sender; it needs a sender
 check first.
- The hardware broker hands out the address ranges it maps device registers and DMA buffers at
 from two 64 GiB windows and never reuses a range. A driver maps its buffers once when it
 starts, so a window runs out only after thousands of driver restarts; then a map is refused
 until reboot.
- A distribution package installed on a live boot is unvouched, as any installed package is
 there, so the exec gate refuses to run it; every tool the image carries runs, since each proves
 itself through its publisher (`docs/handbook/linux/tools-0.9.2.md`, "Trust").
- An install shows Queued, Installing, then the result; there is no byte count while a package
 downloads, which needs a progress field in the kernel's install status.
