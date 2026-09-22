# Copyright 2025, UNSW
# SPDX-License-Identifier: BSD-2-Clause
import argparse
import struct
import xml.etree.ElementTree as ET
from random import randint
from dataclasses import dataclass
from typing import List, Tuple, Optional
from sdfgen import SystemDescription, Sddf, DeviceTree, LionsOs
from importlib.metadata import version

# This file will not be overwritten if HAMR codegen is rerun

assert int(version('sdfgen').split(".")[1]) >= 30, f"Requires sdfgen >= 0.30, found {version('sdfgen')}"

from sdfgen_helper import *

ProtectionDomain = SystemDescription.ProtectionDomain
MemoryRegion = SystemDescription.MemoryRegion
Map = SystemDescription.Map
Channel = SystemDescription.Channel
IrqConventional = SystemDescription.IrqConventional
VirtualMachine = SystemDescription.VirtualMachine

@dataclass
class Board:
    name: str
    arch: SystemDescription.Arch
    paddr_top: int
    serial: str
    timer: str
    ethernet: str
    i2c: Optional[str]


BOARDS: List[Board] = [
    Board(
        name="qemu_virt_aarch64",
        arch=SystemDescription.Arch.AARCH64,
        paddr_top=0x6_0000_000,
        serial="pl011@9000000",
        timer="timer",
        ethernet="virtio_mmio@a003e00",
        i2c=None,
    ),
]

def schedule(*entries):
    """
    entries: sequence of (channel, timeslice_ns)
    """
    part_ch, part_timeslices, is_user_partition = zip(*entries)
    return UserSchedule(list(part_timeslices), list(part_ch), list(is_user_partition))

def generate(sdf_path: str, output_dir: str, dtb: DeviceTree):
    timer_node = dtb.node(board.timer)
    assert timer_node is not None

    timer_driver = ProtectionDomain("timer_driver", "timer_driver.elf", priority=201)
    timer_system = Sddf.Timer(sdf, timer_node, timer_driver)

    scheduler = ProtectionDomain("scheduler", "scheduler.elf", priority=200)


    # BEGIN META MARKER

    #######################################
    # PARTITION PROTECTION DOMAINS
    #######################################
    seL4_MavlinkFirewall_MavlinkFirewall_MON = ProtectionDomain(
      name="seL4_MavlinkFirewall_MavlinkFirewall_MON",
      program_image="seL4_MavlinkFirewall_MavlinkFirewall_MON.elf",
      priority=150,
      passive=True)
    scheduler.add_child_pd(seL4_MavlinkFirewall_MavlinkFirewall_MON)

    seL4_MavlinkFirewall_MavlinkFirewall = ProtectionDomain(
      name="seL4_MavlinkFirewall_MavlinkFirewall",
      program_image="seL4_MavlinkFirewall_MavlinkFirewall.elf",
      priority=140,
      passive=True,
      stack_size=0x100_000)
    seL4_MavlinkFirewall_MavlinkFirewall_MON.add_child_pd(seL4_MavlinkFirewall_MavlinkFirewall, child_id=1)

    seL4_RxFirewall_RxFirewall_MON = ProtectionDomain(
      name="seL4_RxFirewall_RxFirewall_MON",
      program_image="seL4_RxFirewall_RxFirewall_MON.elf",
      priority=150,
      passive=True)
    scheduler.add_child_pd(seL4_RxFirewall_RxFirewall_MON)

    seL4_RxFirewall_RxFirewall = ProtectionDomain(
      name="seL4_RxFirewall_RxFirewall",
      program_image="seL4_RxFirewall_RxFirewall.elf",
      priority=140,
      passive=True,
      stack_size=0x100_000)
    seL4_RxFirewall_RxFirewall_MON.add_child_pd(seL4_RxFirewall_RxFirewall, child_id=1)

    seL4_TxFirewall_TxFirewall_MON = ProtectionDomain(
      name="seL4_TxFirewall_TxFirewall_MON",
      program_image="seL4_TxFirewall_TxFirewall_MON.elf",
      priority=150,
      passive=True)
    scheduler.add_child_pd(seL4_TxFirewall_TxFirewall_MON)

    seL4_TxFirewall_TxFirewall = ProtectionDomain(
      name="seL4_TxFirewall_TxFirewall",
      program_image="seL4_TxFirewall_TxFirewall.elf",
      priority=140,
      passive=True,
      stack_size=0x100_000)
    seL4_TxFirewall_TxFirewall_MON.add_child_pd(seL4_TxFirewall_TxFirewall, child_id=1)

    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON = ProtectionDomain(
      name="seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON",
      program_image="seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON.elf",
      priority=150,
      passive=True)
    scheduler.add_child_pd(seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON)

    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver = ProtectionDomain(
      name="seL4_LowLevelEthernetDriver_LowLevelEthernetDriver",
      program_image="seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.elf",
      priority=140,
      passive=True,
      stack_size=0x100_000)
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON.add_child_pd(seL4_LowLevelEthernetDriver_LowLevelEthernetDriver, child_id=1)

    seL4_ArduPilot_ArduPilot_MON = ProtectionDomain(
      name="seL4_ArduPilot_ArduPilot_MON",
      program_image="seL4_ArduPilot_ArduPilot_MON.elf",
      priority=150,
      passive=True)
    scheduler.add_child_pd(seL4_ArduPilot_ArduPilot_MON)

    seL4_ArduPilot_ArduPilot = ProtectionDomain(
      name="seL4_ArduPilot_ArduPilot",
      program_image="seL4_ArduPilot_ArduPilot.elf",
      priority=140,
      passive=True,
      stack_size=0x100_000)
    seL4_ArduPilot_ArduPilot_MON.add_child_pd(seL4_ArduPilot_ArduPilot, child_id=1)
    seL4_ArduPilot_ArduPilot.add_irq(IrqConventional(irq=33, id=1))

    #######################################
    # MEMORY REGIONS
    #######################################
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM", 0x10_000_000, paddr=0x40_000_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM)
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_GIC = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_GIC", 0x1_000, paddr=0x8_040_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_GIC)
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Serial = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Serial", 0x1_000, paddr=0x9_000_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Serial)
    ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out0_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out0_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out0_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out1_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out1_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out1_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out2_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out2_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out2_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out3_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out3_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out3_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut0_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut0_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut0_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut1_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut1_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut1_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut2_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut2_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut2_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut3_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut3_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut3_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut0_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut0_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut0_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut1_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut1_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut1_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut2_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut2_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut2_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut3_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut3_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut3_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut0_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut0_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut0_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut1_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut1_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut1_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut2_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut2_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut2_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut3_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut3_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut3_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx0_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx0_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx0_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx1_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx1_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx1_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx2_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx2_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx2_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx3_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx3_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx3_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx0_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx0_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx0_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx1_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx1_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx1_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx2_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx2_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx2_1_Memory_Region)
    ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx3_1_Memory_Region = MemoryRegion(sdf, "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx3_1_Memory_Region", 0x1_000)
    sdf.add_mr(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx3_1_Memory_Region)

    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM, 0x40_000_000, perms="rw", setvar_vaddr="ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM_vaddr"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out0_1_Memory_Region, 0x10_000_000, perms="rw", setvar_vaddr="Out0_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out0_1_Memory_Region, 0x20_000_000, perms="r", setvar_vaddr="MavlinkRx0_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out1_1_Memory_Region, 0x10_001_000, perms="rw", setvar_vaddr="Out1_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out1_1_Memory_Region, 0x20_001_000, perms="r", setvar_vaddr="MavlinkRx1_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out2_1_Memory_Region, 0x10_002_000, perms="rw", setvar_vaddr="Out2_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out2_1_Memory_Region, 0x20_002_000, perms="r", setvar_vaddr="MavlinkRx2_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out3_1_Memory_Region, 0x10_003_000, perms="rw", setvar_vaddr="Out3_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_MavlinkFirewall_MavlinkFirewall_Out3_1_Memory_Region, 0x20_003_000, perms="r", setvar_vaddr="MavlinkRx3_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut0_1_Memory_Region, 0x10_000_000, perms="rw", setvar_vaddr="VmmOut0_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut0_1_Memory_Region, 0x20_004_000, perms="r", setvar_vaddr="FirewallRx0_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut1_1_Memory_Region, 0x10_001_000, perms="rw", setvar_vaddr="VmmOut1_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut1_1_Memory_Region, 0x20_005_000, perms="r", setvar_vaddr="FirewallRx1_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut2_1_Memory_Region, 0x10_002_000, perms="rw", setvar_vaddr="VmmOut2_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut2_1_Memory_Region, 0x20_006_000, perms="r", setvar_vaddr="FirewallRx2_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut3_1_Memory_Region, 0x10_003_000, perms="rw", setvar_vaddr="VmmOut3_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_VmmOut3_1_Memory_Region, 0x20_007_000, perms="r", setvar_vaddr="FirewallRx3_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut0_1_Memory_Region, 0x10_004_000, perms="r", setvar_vaddr="In0_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut0_1_Memory_Region, 0x10_004_000, perms="rw", setvar_vaddr="MavlinkOut0_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut1_1_Memory_Region, 0x10_005_000, perms="r", setvar_vaddr="In1_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut1_1_Memory_Region, 0x10_005_000, perms="rw", setvar_vaddr="MavlinkOut1_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut2_1_Memory_Region, 0x10_006_000, perms="r", setvar_vaddr="In2_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut2_1_Memory_Region, 0x10_006_000, perms="rw", setvar_vaddr="MavlinkOut2_queue_1"))
    seL4_MavlinkFirewall_MavlinkFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut3_1_Memory_Region, 0x10_007_000, perms="r", setvar_vaddr="In3_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_RxFirewall_RxFirewall_MavlinkOut3_1_Memory_Region, 0x10_007_000, perms="rw", setvar_vaddr="MavlinkOut3_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut0_1_Memory_Region, 0x10_000_000, perms="rw", setvar_vaddr="EthernetFramesTxOut0_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut0_1_Memory_Region, 0x10_000_000, perms="r", setvar_vaddr="EthernetFramesTx0_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut1_1_Memory_Region, 0x10_001_000, perms="rw", setvar_vaddr="EthernetFramesTxOut1_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut1_1_Memory_Region, 0x10_001_000, perms="r", setvar_vaddr="EthernetFramesTx1_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut2_1_Memory_Region, 0x10_002_000, perms="rw", setvar_vaddr="EthernetFramesTxOut2_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut2_1_Memory_Region, 0x10_002_000, perms="r", setvar_vaddr="EthernetFramesTx2_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut3_1_Memory_Region, 0x10_003_000, perms="rw", setvar_vaddr="EthernetFramesTxOut3_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_TxFirewall_TxFirewall_EthernetFramesTxOut3_1_Memory_Region, 0x10_003_000, perms="r", setvar_vaddr="EthernetFramesTx3_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx0_1_Memory_Region, 0x10_008_000, perms="r", setvar_vaddr="EthernetFramesRxIn0_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx0_1_Memory_Region, 0x10_004_000, perms="rw", setvar_vaddr="EthernetFramesRx0_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx1_1_Memory_Region, 0x10_009_000, perms="r", setvar_vaddr="EthernetFramesRxIn1_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx1_1_Memory_Region, 0x10_005_000, perms="rw", setvar_vaddr="EthernetFramesRx1_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx2_1_Memory_Region, 0x10_00A_000, perms="r", setvar_vaddr="EthernetFramesRxIn2_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx2_1_Memory_Region, 0x10_006_000, perms="rw", setvar_vaddr="EthernetFramesRx2_queue_1"))
    seL4_RxFirewall_RxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx3_1_Memory_Region, 0x10_00B_000, perms="r", setvar_vaddr="EthernetFramesRxIn3_queue_1"))
    seL4_LowLevelEthernetDriver_LowLevelEthernetDriver.add_map(Map(ZCU102_Impl_Instance_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_EthernetFramesRx3_1_Memory_Region, 0x10_007_000, perms="rw", setvar_vaddr="EthernetFramesRx3_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx0_1_Memory_Region, 0x10_004_000, perms="r", setvar_vaddr="EthernetFramesTxIn0_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx0_1_Memory_Region, 0x20_008_000, perms="rw", setvar_vaddr="EthernetFramesTx0_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx1_1_Memory_Region, 0x10_005_000, perms="r", setvar_vaddr="EthernetFramesTxIn1_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx1_1_Memory_Region, 0x20_009_000, perms="rw", setvar_vaddr="EthernetFramesTx1_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx2_1_Memory_Region, 0x10_006_000, perms="r", setvar_vaddr="EthernetFramesTxIn2_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx2_1_Memory_Region, 0x20_00A_000, perms="rw", setvar_vaddr="EthernetFramesTx2_queue_1"))
    seL4_TxFirewall_TxFirewall.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx3_1_Memory_Region, 0x10_007_000, perms="r", setvar_vaddr="EthernetFramesTxIn3_queue_1"))
    seL4_ArduPilot_ArduPilot.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_EthernetFramesTx3_1_Memory_Region, 0x20_00B_000, perms="rw", setvar_vaddr="EthernetFramesTx3_queue_1"))

    #######################################
    # VMMs
    #######################################
    seL4_ArduPilot_ArduPilot_VM_vm = VirtualMachine("seL4_ArduPilot_ArduPilot_VM", [VirtualMachine.Vcpu(id=0)])
    seL4_ArduPilot_ArduPilot_VM_vm.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM, 0x40_000_000, perms="rwx"))
    seL4_ArduPilot_ArduPilot_VM_vm.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_GIC, 0x8_010_000, perms="rw", cached=False))
    seL4_ArduPilot_ArduPilot_VM_vm.add_map(Map(ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Serial, 0x9_000_000, perms="rw", cached=False))
    seL4_ArduPilot_ArduPilot.set_virtual_machine(seL4_ArduPilot_ArduPilot_VM_vm)

    #######################################
    # CHANNELS
    #######################################
    channel_seL4_MavlinkFirewall_MavlinkFirewall_MON = 6
    channel_seL4_RxFirewall_RxFirewall_MON = 5
    channel_seL4_TxFirewall_TxFirewall_MON = 3
    channel_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON = 4
    channel_seL4_ArduPilot_ArduPilot_MON = 2

    sdf.add_channel(Channel(a=scheduler, a_id=6, b=seL4_MavlinkFirewall_MavlinkFirewall_MON, b_id=0))
    sdf.add_channel(Channel(a=seL4_MavlinkFirewall_MavlinkFirewall_MON, a_id=1, b=seL4_MavlinkFirewall_MavlinkFirewall, b_id=0))
    sdf.add_channel(Channel(a=scheduler, a_id=5, b=seL4_RxFirewall_RxFirewall_MON, b_id=0))
    sdf.add_channel(Channel(a=seL4_RxFirewall_RxFirewall_MON, a_id=1, b=seL4_RxFirewall_RxFirewall, b_id=0))
    sdf.add_channel(Channel(a=scheduler, a_id=3, b=seL4_TxFirewall_TxFirewall_MON, b_id=0))
    sdf.add_channel(Channel(a=seL4_TxFirewall_TxFirewall_MON, a_id=1, b=seL4_TxFirewall_TxFirewall, b_id=0))
    sdf.add_channel(Channel(a=scheduler, a_id=4, b=seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON, b_id=0))
    sdf.add_channel(Channel(a=seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON, a_id=1, b=seL4_LowLevelEthernetDriver_LowLevelEthernetDriver, b_id=0))
    sdf.add_channel(Channel(a=scheduler, a_id=2, b=seL4_ArduPilot_ArduPilot_MON, b_id=0))
    sdf.add_channel(Channel(a=seL4_ArduPilot_ArduPilot_MON, a_id=1, b=seL4_ArduPilot_ArduPilot, b_id=0))

    #######################################
    # SCHEDULE
    #######################################
    ts_seL4_ArduPilot_ArduPilot_MON = (channel_seL4_ArduPilot_ArduPilot_MON, 600000000, True)
    ts_seL4_TxFirewall_TxFirewall_MON = (channel_seL4_TxFirewall_TxFirewall_MON, 300000000, True)
    ts_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON = (channel_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON, 300000000, True)
    ts_seL4_RxFirewall_RxFirewall_MON = (channel_seL4_RxFirewall_RxFirewall_MON, 300000000, True)
    ts_seL4_MavlinkFirewall_MavlinkFirewall_MON = (channel_seL4_MavlinkFirewall_MavlinkFirewall_MON, 300000000, True)
    ts_pad = (0, 150000000, False)

    user_schedule = schedule(
      ts_seL4_ArduPilot_ArduPilot_MON,
      ts_seL4_TxFirewall_TxFirewall_MON,
      ts_seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_MON,
      ts_seL4_RxFirewall_RxFirewall_MON,
      ts_seL4_MavlinkFirewall_MavlinkFirewall_MON,
      ts_pad
    )

    # END META MARKER

    sdf.add_pd(timer_driver)
    sdf.add_pd(scheduler)
    timer_system.add_client(scheduler)

    assert timer_system.connect()
    assert timer_system.serialise_config(output_dir)

    data_path = output_dir + "/schedule_config.data"
    with open(data_path, "wb+") as f:
        f.write(user_schedule.serialise())
    update_elf_section(obj_copy, scheduler.program_image,
                       user_schedule.section_name,
                       data_path)

    # Post-process sdf.render() to add page_size attributes not yet
    # supported by sdfgen's Python API.
    def add_page_size(xml_str, mappings):
        root = ET.fromstring(xml_str)
        for mr in root.iter('memory_region'):
            name = mr.get('name')
            if name in mappings:
                mr.set('page_size', mappings[name])
        ET.indent(root, space='  ')
        return ET.tostring(root, encoding='unicode', xml_declaration=True)

    page_size_mappings = {
        "ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM": "0x200_000"
    }

    with open(f"{output_dir}/{sdf_path}", "w+") as f:
        f.write(add_page_size(sdf.render(), page_size_mappings))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--dtb", required=True)
    parser.add_argument("--sddf", required=True)
    parser.add_argument("--board", required=True, choices=[b.name for b in BOARDS])
    parser.add_argument("--output", required=True)
    parser.add_argument("--sdf", required=True)
    parser.add_argument("--objcopy", required=True)

    args = parser.parse_args()

    # Import the config structs module from the build directory
    sys.path.append(args.output)
    from config_structs import *

    board = next(filter(lambda b: b.name == args.board, BOARDS))

    sdf = SystemDescription(board.arch, board.paddr_top)
    sddf = Sddf(args.sddf)

    global obj_copy
    obj_copy = args.objcopy

    with open(args.dtb, "rb") as f:
        dtb = DeviceTree(f.read())

    generate(args.sdf, args.output, dtb)
