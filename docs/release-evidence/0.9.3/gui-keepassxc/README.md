# KeePassXC in the NØNOS frame

One TCG boot of a dev image of s2/debug-launch at 1f454e9354: s2/gui (with
FUTEX_WAKE_OP served, 152b4b5fec) plus S3's s3/window-wait, and
WAYLAND_DEBUG on for every guest (the protocol lines on the Terminal
behind). KeePassXC 2.7.8 (Qt 5.15, qt5-qtwayland by install_if) installed
over the mirror relay in its listing's box: WAYLAND and FILES_HOME, no
network.

- keepassxc-desktop.png: about two minutes after `linux keepassxc`, its
  window in the frame, the welcome screen drawn whole.
- keepassxc-window.png: the window alone.

Before 152b4b5fec the window never came: Qt 5's QSemaphore::release wakes
only through FUTEX_WAKE_OP, which the personality refused, so Qt's main
thread slept on its D-Bus thread's reply for ever.

Not shown yet: a database made, an entry copied and pasted, the clipboard
cleared.
