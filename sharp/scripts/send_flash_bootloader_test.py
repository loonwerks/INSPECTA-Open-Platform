#!/usr/bin/env python3
"""Send five MAV_CMD_FLASH_BOOTLOADER packets for the firewall test.

Requires: python3 -m pip install pymavlink
Usage: python3 send_flash_bootloader_test.py 172.27.60.153
Stop QGroundControl first if it owns UDP source port 14550.
Parameters are zeroed; the command ID alone should trigger the blacklist.
"""

import argparse
import ipaddress
import os
import socket
import time

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("destination_ip", type=ipaddress.IPv4Address,
                        help="destination IPv4 address")
    args = parser.parse_args()

    # Select MAVLink 2 before importing pymavlink.
    os.environ["MAVLINK20"] = "1"
    from pymavlink import mavutil

    destination = (str(args.destination_ip), 14562)
    mav = mavutil.mavlink.MAVLink(None, srcSystem=255, srcComponent=190)

    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.bind(("0.0.0.0", 14550))

        for i in range(5):
            mav.seq = i
            message = mav.command_long_encode(
                1, 1,       # Target vehicle system/component
                42650,      # MAV_CMD_FLASH_BOOTLOADER
                0,          # Confirmation
                0, 0, 0, 0, 0, 0, 0,
            )
            sock.sendto(message.pack(mav), destination)
            print(f"Sent {i + 1}/5")
            time.sleep(0.1)


if __name__ == "__main__":
    main()
