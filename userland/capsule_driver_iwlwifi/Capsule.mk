# iwlwifi: Intel Wi-Fi PCIe capsule. Hardware authority is brokered
# DeviceClaim/MMIO/IRQ/DMA only; Crypto draws the station address and
# the join's secrets. The WPA2/WPA3 join runs here through the linked
# nonos_wifi_core; DHCP, IP, and sockets stay in upper network capsules.

CAPSULE_SLUG             := driver-iwlwifi
CAPSULE_HANDLE           := driver.iwlwifi0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_iwlwifi
CAPSULE_BIN_NAME         := driver_iwlwifi
CAPSULE_FEATURE          := nonos-capsule-driver-iwlwifi
CAPSULE_NAMESPACE        := systems.nonos.driver.iwlwifi0
CAPSULE_SERVICE_ENDPOINT := service:4228:driver.iwlwifi0
CAPSULE_REPLY_ENDPOINT   := reply:4229:endpoint.4294967317
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8038
# Crypto (0x20): CryptoRandom draws the per-boot station address and each join's nonces and SAE secrets.
CAPSULE_REQUIRED_CAPS    := 0xF8038
CAPSULE_KERNEL_MIRROR    := src/hardware/iwlwifi_capsule

include nonos-mk/capsule.mk
