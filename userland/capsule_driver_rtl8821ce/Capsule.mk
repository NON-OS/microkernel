# RTL8821CE: Realtek 8821CE PCIe Wi-Fi. Hardware authority is brokered
# DeviceClaim/MMIO/DMA only, polled; Crypto draws the station address.
# 802.11 and WPA2/WPA3 run in the linked shared wifi core; the net stack
# stays in the upper network capsules. Signing,
# certificate, and manifest rules come from the shared capsule macro.

CAPSULE_SLUG             := driver-rtl8821ce
CAPSULE_HANDLE           := driver.rtl8821ce0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8821ce
CAPSULE_BIN_NAME         := driver_rtl8821ce
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8821ce
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8821ce0
CAPSULE_SERVICE_ENDPOINT := service:4234:driver.rtl8821ce0
CAPSULE_REPLY_ENDPOINT   := reply:4235:endpoint.4294967320
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038
# No Irq: the driver polls its rings and makes no MkIrq* call (and posts
# no input), the only calls Irq admits. A driver moved to interrupts
# takes the bit back.
# Debug (0x100) is the upper bound so a serial-debug kernel can grant it and the
# radio bring-up reports its progress; a hardened build grants a subset without
# it and the driver still spawns.
# Crypto (0x20) is what the CryptoRandom syscall is gated on. The station address
# is drawn rather than read out of the efuse, so without this the draw returns
# nothing and the PHY is left unconfigured with the radio down.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/rtl8821ce_capsule

include nonos-mk/capsule.mk
