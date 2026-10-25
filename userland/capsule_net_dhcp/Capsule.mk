# net_dhcp: DHCPv4 client. Sends and reads raw Ethernet frames through
# `capsule_net_l2` for the bootp transport, drives the
# DISCOVER/OFFER/REQUEST/ACK ladder, installs the lease into
# `capsule_net_ip` once acknowledged.

CAPSULE_SLUG             := net-dhcp
CAPSULE_HANDLE           := net.dhcp.client
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_dhcp
CAPSULE_BIN_NAME         := net_dhcp
CAPSULE_FEATURE          := nonos-capsule-net-dhcp
CAPSULE_NAMESPACE        := systems.nonos.net.dhcp.client
CAPSULE_SERVICE_ENDPOINT := service:4440:net.dhcp.client
CAPSULE_REPLY_ENDPOINT   := reply:4441:endpoint.net.dhcp.client.reply
CAPSULE_REQUIRED_CAPS    := 0x0003c
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(). It carries one line a
# bound lease, "[DHCP] lease ...", which is how a boot reads that the card
# received; without it the lease binds all the same.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_dhcp

include nonos-mk/capsule.mk
