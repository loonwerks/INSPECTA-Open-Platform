# QEMU board additions kept outside HAMR's generated system.mk.
override SUPPORTED_BOARDS := qemu_virt_aarch64 zcu102
include $(TOP_DIR)/system.mk

include ${SDDF}/drivers/network/${NET_DRIV_DIR}/eth_driver.mk
include ${SDDF}/network/components/network_components.mk

QEMU_NETWORK_IMAGES := \
	eth_driver_virtio.elf \
	network_virt_rx.elf \
	network_virt_tx.elf \
	network_copy.elf

QEMU_NET_CLIENT_OBJ := qemu_virtio_net_client.o
QEMU_NETWORK_CONFIG_STAMP := .qemu_network_configured

DOCKARDU_URL ?= https://github.com/dornerworks/ardupilot-container/releases/download/inspecta-phase-2-udp-specific-port/dockardu.tar
DOCKARDU_TAR := $(TOP_BUILD_DIR)/dockardu.tar
ARDUPILOT_QEMU_BOARD_DIR := $(TOP_DIR)/components/seL4_ArduPilot_ArduPilot/board/qemu_virt_aarch64
ARDUPILOT_COMPOSE_TEMPLATE := $(ARDUPILOT_QEMU_BOARD_DIR)/ardupilot/docker-compose.yml.in
ARDUPILOT_START_SCRIPT := $(ARDUPILOT_QEMU_BOARD_DIR)/ardupilot/startArdu.sh
ARDUPILOT_DATA_IMAGE_SCRIPT := $(ARDUPILOT_QEMU_BOARD_DIR)/create-ardupilot-data-image.sh
ARDUPILOT_DATA_IMAGE := $(TOP_BUILD_DIR)/ardupilot-data.img
ARDUPILOT_DATA_IMAGE_SIZE ?= 4G
QGC_HOST_IP := $(shell getent hosts host.docker.internal 2>/dev/null | awk 'NR == 1 { print $$1 }')
QGC_HOST_CONFIG := $(TOP_BUILD_DIR)/qgc-host-ip

$(QEMU_NETWORK_IMAGES): libsddf_util_debug.a ${CHECK_FLAGS_BOARD_MD5}

# The generated link rule already links every object prerequisite. Add the
# board-specific sDDF client without modifying that generated rule.
seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.elf: $(QEMU_NET_CLIENT_OBJ)

# Ensure all images referenced by the QEMU system description exist before
# sdfgen emits the system and its binary configuration data.
$(SYSTEM_FILE): $(MSD) $(QEMU_NETWORK_IMAGES)

$(QEMU_NETWORK_CONFIG_STAMP): $(SYSTEM_FILE) $(QEMU_NETWORK_IMAGES) \
		seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.elf
	$(OBJCOPY) --update-section .device_resources=eth_driver_device_resources.data eth_driver_virtio.elf
	$(OBJCOPY) --update-section .net_driver_config=net_driver.data eth_driver_virtio.elf
	$(OBJCOPY) --update-section .net_virt_rx_config=net_virt_rx.data network_virt_rx.elf
	$(OBJCOPY) --update-section .net_virt_tx_config=net_virt_tx.data network_virt_tx.elf
	$(OBJCOPY) --update-section .net_copy_config=net_copy_low_level_net_copier.data network_copy.elf
	$(OBJCOPY) --update-section .net_client_config=net_client_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.data seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.elf
	touch $@

$(IMAGE_FILE) $(REPORT_FILE): $(QEMU_NETWORK_CONFIG_STAMP)

# ArduPilot sends telemetry through QEMU's user network to the Docker host on
# UDP port 14550, where QGroundControl runs on the same machine.
# QGroundControl replies over that established UDP path, and the model routes
# its source-14550, destination-14562 frames through the receive firewalls.
override QEMU_ARCH_ARGS += $(QEMU_NET_ARGS) \
	-netdev user,id=netdev0 \
	-drive if=none,file=$(ARDUPILOT_DATA_IMAGE),format=raw,id=ardupilot_data \
	-device virtio-blk-device,drive=ardupilot_data,bus=virtio-mmio-bus.8

$(DOCKARDU_TAR):
	curl --fail --location $(DOCKARDU_URL) -o $@.tmp
	mv $@.tmp $@

.PHONY: qgc-host-config-force
qgc-host-config-force:

$(QGC_HOST_CONFIG): qgc-host-config-force
	@test -n "$(QGC_HOST_IP)" || \
		(echo "Could not resolve host.docker.internal"; exit 1)
	@printf '%s\n' "$(QGC_HOST_IP)" > $@.tmp
	@cmp -s $@.tmp $@ || mv $@.tmp $@
	@rm -f $@.tmp

$(ARDUPILOT_DATA_IMAGE): $(DOCKARDU_TAR) $(ARDUPILOT_COMPOSE_TEMPLATE) \
		$(ARDUPILOT_START_SCRIPT) $(QGC_HOST_CONFIG) \
		$(ARDUPILOT_DATA_IMAGE_SCRIPT)
	$(ARDUPILOT_DATA_IMAGE_SCRIPT) \
		$@ $(ARDUPILOT_DATA_IMAGE_SIZE) $(DOCKARDU_TAR) \
		$(ARDUPILOT_COMPOSE_TEMPLATE) $(QGC_HOST_IP) \
		$(ARDUPILOT_START_SCRIPT)

qemu: $(ARDUPILOT_DATA_IMAGE)

clean::
	$(RM) -f $(QEMU_NETWORK_CONFIG_STAMP) $(QEMU_NET_CLIENT_OBJ)
