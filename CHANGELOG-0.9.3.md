# NØNOS 0.9.3

The release where Linux programs get windows. A program whose box grants a window opens on the NØNOS desktop, and the box still decides everything else it can reach. The full account, with the evidence behind each line, is in [docs/release/0.9.3-what-is-new.md](docs/release/0.9.3-what-is-new.md). What does not work yet is in [docs/release/0.9.3-known-issues.md](docs/release/0.9.3-known-issues.md) and [docs/release/0.9.3-security-limits.md](docs/release/0.9.3-security-limits.md).

Unless a line says otherwise, the boots behind these changes are development images run under QEMU.

## Added

- A Wayland bridge in the Linux personality. KeePassXC, Telegram Desktop, swayimg, foot, mpv, zathura and DarkFi's chat app are listed with a window; the compositor and window manager draw it with a title bar like any other.
- Pseudo-terminals for Linux programs, so interactive tools and shells behave as on Linux.
- The Monero wallet, reaching its node over Nym or Anyone.
- Installing Linux apps from the Marketplace: 31 recipes, 25 from Alpine and six built from source or pinned (SimpleX Chat and its core, DarkFi's `drk`, `darkirc`, `darkfid` and chat app).
- Update in place. A disk installed by 0.9.3 keeps its encrypted data volume across a later release. The volume's key is random and wrapped under a key the TPM gives only to a kernel the release approved.
- Power Off and the Power key ask to wipe memory, then power off through the wipe.
- Drivers: on QEMU the e1000, e1000e, igb, RTL8139, PCnet, vmxnet3 and Tulip drivers each got a DHCP lease; new Wi-Fi drivers for the RTL8822CE, RTL8852BE, MediaTek MT7921 and MT7922, and a RTL8821CE report from a maintainer's laptop that has no log.
- A 13-step first-boot setup, with the Appearance step choosing the desktop wallpaper.

## Changed

- Capabilities are 40 bits and native syscalls 183, all documented.
- The 0.9.2 receive fault of the e1000 and RTL8139 is shown fixed on a boot.
- Setup, About and the Terminal show the release from `VERSION`.
- The documentation is checked against the tree: figures, counts and links.

## Known gaps

- A wallet created on a machine with no TPM keeps its keys unwrapped; the known-issues page describes it.
- `setuid(0)` inside a Linux box is answered, not refused. The box, not the user id, is the boundary.
- Only seven of the 31 Linux programs have a window.
- No Ethernet card has a report from real hardware.

See [docs/release/0.9.3-known-issues.md](docs/release/0.9.3-known-issues.md) for the rest.
