# RTL8852BE: Realtek 8852BE PCIe Wi-Fi (rtw89 8852B, 2x2, run as a 20 MHz
# legacy station). Hardware authority is brokered DeviceClaim/MMIO/DMA only,
# polled; Crypto draws the station address. 802.11 and WPA2/WPA3 run in the
# linked shared wifi core; the net stack stays in the upper network capsules.
# Signing, certificate, and manifest rules come from the shared capsule macro.

CAPSULE_SLUG             := driver-rtl8852be
CAPSULE_HANDLE           := driver.rtl8852be0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8852be
CAPSULE_BIN_NAME         := driver_rtl8852be
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8852be
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8852be0
CAPSULE_SERVICE_ENDPOINT := service:4268:driver.rtl8852be0
CAPSULE_REPLY_ENDPOINT   := reply:4269:endpoint.4294967374
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038, as the RTL8821CE:
# no Irq (the rings are polled and no MkIrq* call is made), Crypto for the
# drawn station address (the efuse address is never read out).
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/rtl8852be_capsule

include nonos-mk/capsule.mk
