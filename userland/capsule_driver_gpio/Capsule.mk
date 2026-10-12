# x86 GPIO controller capsule. Hardware-facing only: claims the GPIO
# controllers the kernel published from ACPI, maps their windows, arms the
# _AEI pins and the serial-bus devices' GpioInt pins, and hands each
# pending pin to capsule_acpi or to its device's holder.

CAPSULE_SLUG             := driver-gpio
CAPSULE_HANDLE           := driver.gpio0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_gpio
CAPSULE_BIN_NAME         := driver_gpio
CAPSULE_FEATURE          := nonos-capsule-driver-gpio
CAPSULE_NAMESPACE        := systems.nonos.driver.gpio0
CAPSULE_SERVICE_ENDPOINT := service:4236:driver.gpio0
CAPSULE_REPLY_ENDPOINT   := reply:4237:endpoint.4294967392
# IPC|Memory|DeviceEnum|Driver|Mmio = 0x38018
CAPSULE_REQUIRED_CAPS    := 0x38018
CAPSULE_KERNEL_MIRROR    := src/hardware/gpio_capsule

include nonos-mk/capsule.mk
