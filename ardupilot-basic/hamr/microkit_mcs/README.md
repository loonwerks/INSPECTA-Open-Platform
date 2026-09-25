# Microkit MCS build and run guide

This directory is based on the SysML model in
[`sysml_mcs`](../../sysml_mcs).

Run these commands from this directory. The first build downloads dependencies and creates a
4 GiB ArduPilot data image, so it takes longer than later builds.

## Start an Interactive Docker Container:
From this directory run:

```sh
   docker compose -f docker_compose.yml run -P microkit_provers
   ```

Then you can either (a) run the QEMU simulation, (b) run on the ZCU102 board, or (c) run tests and Verus verification:

### (a) Run the QEMU board

1. Inside the container, build and run QEMU:

   ```sh
   RUST_MAKE_TARGET=build-release make CONFIG=qemu.mk qemu
   ```

   ***NOTE:*** Allow <u>*several minutes*</u> after QEMU starts for the Linux guest and ArduPilot
   to finish booting.

2. Choose any MAVLink-compatible application from the
   [ArduPilot Ground-Station list](https://ardupilot.org/copter/docs/common-choosing-a-ground-station.html),
   then configure it on the host:

      - Connection: UDP listener
      - Local UDP port: `14550`
      - Vehicle: Multi-Rotor / ArduCopter
      - Vehicle system ID: `1` (normally discovered automatically)

   The guest sends telemetry to the Docker host automatically. Allow incoming
   UDP traffic if the host operating system asks.

3. Stop QEMU with `Ctrl-A`, then `X`. Exit the container with `exit`.

### (b) Run the ZCU102 Board

1. Inside the container, build the ZCU102 image:

   ```sh
   RUST_MAKE_TARGET=build-release make CONFIG=zcu102.mk
   ```

2. The resulting image is located at:

   ```text
   build/loader.img
   ```

3. Collect the four files needed for the SD card:

   ```sh
   mkdir -p build/zcu102-sd
   cp build/loader.img build/zcu102-sd/
   curl -L \
     https://github.com/dornerworks/u-boot-xlnx/releases/download/inspecta-phase-1/BOOT.BIN \
     -o build/zcu102-sd/BOOT.BIN
   curl -L \
     https://github.com/dornerworks/ardupilot-container/releases/download/inspecta-phase-2-udp-specific-port/dockardu.tar \
     -o build/zcu102-sd/dockardu.tar
   curl -L \
     https://raw.githubusercontent.com/dornerworks/ardupilot-container/refs/tags/inspecta-phase-2-udp-specific-port/docker-compose.yml \
     -o build/zcu102-sd/docker-compose.yml
   ```

4. In `build/zcu102-sd/docker-compose.yml`, replace the literal `${GCS_IP}`
   with the IP address of the host running the ground station. The host and
   ZCU102 must be able to reach each other over Ethernet.

5. Format an SD card with these two partitions (see the [Xilinx Wiki](https://xilinx-wiki.atlassian.net/wiki/spaces/A/pages/18842385/How+to+format+SD+card+for+SD+boot)):

   1. A FAT boot partition
   2. An ext4 data partition

6. Copy the deployment files to the mounted partitions. Replace
   `<boot-mount>` and `<data-mount>` with their actual mount points:

   ```sh
   cp build/zcu102-sd/BOOT.BIN build/zcu102-sd/loader.img <boot-mount>/
   mkdir -p <data-mount>/docker/containers
   cp build/zcu102-sd/docker-compose.yml <data-mount>/docker/docker-compose.yml
   cp build/zcu102-sd/dockardu.tar <data-mount>/docker/containers/dockardu.tar
   sync
   ```

7. With the board powered off:

   - Insert the SD card into slot `J100`.
   - Set boot-mode switch `SW6` for SD boot: switch 1 ON and switches 2-4 OFF
     (`SW6[4:1] = OFF, OFF, OFF, ON`).
   - Connect the USB UART port `J83` to the host and open its serial console at
     `115200 8-N-1`.
   - Connect the ZCU102 Ethernet port to the host or to the same network as the
     host.

   Locate the USB serial devices and open the one that displays the
   U-Boot console. 

8. Power on the board with `SW1`. At the U-Boot `ZynqMP>` prompt, load and
   start the Microkit image:

   ```text
   fatload mmc 0 0x40000000 loader.img && go 0x40000000
   ```

   ***NOTE:*** Allow <u>*several minutes*</u> after boot for the Linux guest and ArduPilot
   to finish booting.

9.  Choose any MAVLink-compatible application from the
   [ArduPilot Ground-Station list](https://ardupilot.org/copter/docs/common-choosing-a-ground-station.html),
   then configure it on the host:

      - Connection: UDP listener
      - Local UDP port: `14550`
      - Vehicle: Multi-Rotor / ArduCopter
      - Vehicle system ID: `1` (normally discovered automatically)

     If the application asks for a remote endpoint, use the ZCU102 address reported by `ifconfig`; applications
    that listen for telemetry should listen on local UDP port `14550`.

10. Exit the build container with `exit` when finished.

### (c) Test and Verify

1. Inside the container, Run the component tests for the QEMU configuration:

   ```sh
   make CONFIG=qemu.mk test
   ```

2. Inside the container, run Verus verification for the QEMU configuration:

   ```sh
   make CONFIG=qemu.mk verus
   ```

3. Inside the container, run the component tests for the ZCU102 configuration:

   ```sh
   make CONFIG=zcu102.mk test
   ```

4. Inside the container, run Verus verification for the ZCU102 configuration:

   ```sh
   make CONFIG=zcu102.mk verus
   ```

\
\
ℹ️ ***NOTE:*** When switching back from ZCU102 to QEMU and vice versa, run `make clobber`
before rebuilding with a new `CONFIG` option.

ℹ️ ***NOTE:*** The commands above use Cargo to build the release binaries without running
Verus. Omit `RUST_MAKE_TARGET=build-release` to require verification during the
image build.
