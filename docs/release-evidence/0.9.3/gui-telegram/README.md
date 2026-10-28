# Telegram Desktop in the NØNOS frame

One TCG boot of a dev image of s2/debug-launch with s2/gui at 66858aee4a
merged: the /proc fix (71e34654ac) and WAYLAND_DEBUG on for every guest.
Telegram Desktop 5.0.1 (Qt 6) installed over the mirror relay in S3's
listing's box: WAYLAND, NET_ANON and FILES_HOME.

- telegram-desktop.png: about nine minutes after `linux telegram-desktop`
  on TCG, its window in the frame, the welcome screen drawn whole.
- telegram-window.png: the window alone.

Before 71e34654ac the window never came: libdispatch counts its runnable
workers by opening /proc/<tid>/stat for each one, the personality answered
only a process's own number, and libdispatch trapped (ud2) when the open
failed, ending the process on SIGILL.

Its network goes through the box: every connection it opens reaches the
SOCKS5 port, and the serial log shows each one refused because the Nym
and Anyone sessions never came up. This test machine reaches only the
package mirror, so neither network could fetch its directory. A sign-in
needs a boot with the network open.

Not shown yet: a sign-in, a message sent. Telegram draws its own title bar
inside the frame (Settings, Advanced, "Use system window frame" turns it
off).
