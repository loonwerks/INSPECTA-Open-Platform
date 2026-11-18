# Extend HAMR's board list without editing the generated build rules.
override SUPPORTED_BOARDS := qemu_virt_aarch64 zcu102
include $(TOP_DIR)/system.mk
