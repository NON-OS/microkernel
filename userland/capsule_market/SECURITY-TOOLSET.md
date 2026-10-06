# A security toolset for NONOS

`security-packages.txt` is a curated list of security and networking tools
from Alpine v3.20 (main and community). It plugs into the marketplace the
same way the baseline list does: point `MARKET_LINUX_LIST` at it, and the
catalogue generator fetches each package, hashes it, and lists it.

## How a tool reaches the machine

1. `nonos-market-catalogue` reads the list, fetches each package from the
   distribution over the mirror relay, and records its BLAKE3 as a measured
   fact. A hash nobody computed is not an assertion, so a package that could
   not be fetched is left out.
2. The marketplace index is signed by the operator key and served by the
   market capsule.
3. In the terminal, `install <name>` looks the package up, fetches it again
   over the relay, holds the bytes to the distribution's own signature
   (Alpine's, Debian's or Kali's key), and unpacks it into a private tree.
4. The program then runs as a foreign guest at Authority 0: no hardware, no
   DMA, no reach into another process. It is confined whether or not it is
   trusted.

## What each tool needs before it runs

- A listing is held back until the package's measurement is enrolled (its
  `zk_trailer_hash` is empty until then), so the operator vouches for the
  exact bytes the machine will run.
- A tool that reaches a live network target (marked `(net)` in the list:
  nmap, masscan, the netcat family, hydra, nikto and the rest) needs the
  network capability granted to its guest. Until that grant exists it runs,
  but reaches nothing outside the sandbox. Reading a capture file, auditing
  a hash file, or examining a binary needs no network and works today.
- The store lists at most 128 entries and 48 MiB, and a dynamically linked
  tool brings its shared-library closure, so an image ships a chosen subset
  rather than the whole list.

## Proven

The catalogue generator fetched and hashed radare2 5.9.0, nmap 7.95,
openssl 3.3.7 and john 1.9.0 from Alpine, and listed each with the BLAKE3
of its real bytes (for example radare2, 4179014 bytes,
366f6463e579517c931cb6863850b07812bd04ecdd0adc34f04c4a2b1f9d9eb7). Each
listing carries `required_capabilities: [ForeignExec]` and an empty
`zk_trailer_hash`, held back until enrolled.
