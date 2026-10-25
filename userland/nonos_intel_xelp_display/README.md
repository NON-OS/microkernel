# nonos_intel_xelp_display

Mode setting for Intel's Gen11 and Gen12 (Xe-LP) display engines, written
from Intel's public Tiger Lake Programmer's Reference Manual. Every register
and step is cited beside it by volume and page: "Vol 2c" is
IHD-OS-TGL-Vol 2c-12.21 (registers), and "Vol 12" is IHD-OS-TGL-Vol 12-12.21
(the display engine and its sequences). `no_std`; it depends only on
`nonos_edid` and `nonos_intel_display_hw`, and reaches the hardware only
through that crate's `Mmio` and `Clock`.

The Intel display capsule (`capsule_driver_intel_display`) will call it
through `nonos_intel_display_hw::modeset::ModeSet` for the Ice Lake, Tiger
Lake and Alder Lake families, once its proofs meet the bars agreed with the
capsule's owner. It is not wired in yet, and `ModeSet` is not implemented
yet.

What is here so far (gpu-2a):

- `regs::trans`: the transcoder timing registers, PIPE_SRCSZ and TRANS_CONF
  for transcoders A to D; Tiger Lake has no eDP transcoder;
- `timing::check`: every limit Vol 2c puts on a timing, checked before any
  write (progressive modes only; DisplayPort and HDMI limits);
- `timing::words` and `timing::write`: the register words, written only
  into a transcoder that reads off;
- `timing::frame`: a frame's length, for waits given in frame times;
- `transcoder`: TRANS_CONF on and off, each waiting on the state bit under a
  clock deadline, and an enable that never lights turned back off;
- `wait`: a register's bits awaited until a deadline.

And the combo PHY's PLLs (gpu-2b):

- `regs::pll` and `regs::clka`: DPLL0, DPLL1 and DPLL4 (absent on Y and U
  parts), DPCLKA_CFGCR0 and DSSM's reference clock;
- `pll::dp`: Vol 12's DisplayPort table for 1.62 to 8.1 Gbit/s, as given;
- `pll::hdmi` and `pll::split`: Vol 12's HDMI divider search;
- `pll::sequence` and `pll::lock`: the enable and disable sequences, with
  the voltage switch left between `configure` and `lock` for the caller;
- `pll::map`: a combo port's clock select and gate, as two writes.

And the combo PHY and DDI buffer (gpu-2c):

- `regs::phy` and `regs::ddi`: the PHY's port, group and lane registers,
  DDI_BUF_CTL, PHY_MISC and the driver's DDI IO power well;
- `phy::init`: the PHY initialisation sequence, with Vol 12's procmon
  values picked by the PHY's reported voltage and process;
- `phy::swing`, `phy::signal` and the swing tables: the voltage swing
  sequence for DP up to 2.7 Gbit/s, eDP up to 5.4 and HDMI. DP above 2.7 is
  refused, because Vol 12 prints two unlabelled "DP HBR2" tables;
- `phy::lanes` and `phy::power`: loadgen, the lanes powered, and IO power;
- `ddi::buf`: the DDI buffer on and off.

Proved by `intel_xelp_display_proofs`. Nothing here has run on a machine:
no QEMU device models these engines, so it is tested against register
models until the owner boots it on real hardware.
