# CR-01 Data Dictionary

> **Retired — CR-01 historical reference only (2026-09-23).**
> Retired by explicit developer instruction during CR-02 closeout. This document
> is not an active requirements source; its original content is preserved below.
> Current authority: [Open Platform requirements, revision 26_09_23_02](../action-requests/CR-02-add-mode-manager/Open_Platform_HLRs_26_09_23_02.md).
> Current change context: [CR-02 sketch](../action-requests/CR-02-add-mode-manager/add-mode-manager-sketch.md)
> and [approved change plan](../action-requests/CR-02-add-mode-manager/change-plan.md).
> Requirement identifiers below retain their CR-01 meanings and must not be
> interpreted as current CR-02 identifiers.

| Item | Type / units / range | Definition and validity |
|---|---|---|
| Ethernet frame | `RawEthernetMessage`, 1600 bytes | Existing fixed-size carrier. Active length is derived from valid IPv4/UDP length fields; unused carrier bytes are not part of the MAVLink payload. |
| IPv4 total length | Unsigned 16-bit bytes; 20–9000 conceptually, bounded by the 1600-byte carrier in this system | Existing field at Ethernet offsets 16–17. It must not identify data beyond the received carrier. |
| UDP source port | Unsigned 16-bit, 0–65535 | ArduPilot ground-station traffic requires 14550. |
| UDP destination port | Unsigned 16-bit, 0–65535 | ArduPilot VM traffic requires 14562; retained direct UDP includes port 68. |
| UDP length | Unsigned 16-bit bytes; minimum 8 | Includes UDP header plus payload and must agree with the containing IPv4 length and available carrier bytes. |
| MAVLink payload slice | Byte sequence, 0–255 payload bytes plus version-specific framing | Exactly the UDP payload; must contain one complete frame and no trailing bytes. |
| `MAVLinkUDPMessage_Impl` | Struct `{ethernet_frame: RawEthernetMessage, payload_offset: Unsigned_16, payload_length: Unsigned_16}` | RxFirewall-to-MAVLinkFirewall carrier. `ethernet_frame` preserves the original packet; offset and length identify the validated UDP payload. |
| MAVLink payload offset | Unsigned 16-bit bytes, 0–1600 | Computed by RxFirewall as `14 + IPv4_IHL_bytes + 8`; must be within `ethernet_frame`. With the baseline no-options IPv4 policy (`IHL = 5`), the value is 42. |
| MAVLink payload length | Unsigned 16-bit bytes, bounded by UDP/frame length | Computed by RxFirewall as `UDP_length - 8`; offset plus length must not exceed the active IPv4 packet or 1600-byte carrier. |
| MAVLink version magic | Unsigned 8-bit | `0xFE` for v1, `0xFD` for v2. Other values are malformed. |
| MAVLink payload length | Unsigned 8-bit bytes, 0–255 | Number of transmitted MAVLink payload bytes; determines exact frame length with header, checksum, and optional v2 signature. MAVLink v2 may omit a trailing all-zero payload suffix up to the dialect maximum; MAVLink v1 uses the fixed dialect length. |
| MAVLink v1 message ID | Unsigned 8-bit, 0–255 | Identifies a message in the resolved dialect metadata. |
| MAVLink v2 message ID | Unsigned 24-bit, 0–16,777,215 | Little-endian three-byte message identifier in the v2 header. |
| MAVLink incompatibility flags | Unsigned 8-bit bitset | Unsupported set bits make the v2 frame invalid; the signature-present bit requires a complete 13-byte signature. |
| MAVLink checksum | X.25/CRC-16 plus dialect CRC extra | Two-byte checksum over the version-defined header/payload sequence and message-specific CRC extra. |
| MAVLink v2 signature | 13 bytes when signature-present flag is set | Structurally required when indicated. CR-01 checks structure, not cryptographic authenticity. |
| `FILE_TRANSFER_PROTOCOL` | MAVLink message ID 110 | Valid file-transfer messages are permitted by CR-01. |
| `MAV_CMD_FLASH_BOOTLOADER` | MAVLink command value 42650 | Firmware-flash activation command; denied when carried by a recognized command envelope. |
| Secure flash-bootloader operation | Secure-command operation value 7 | Firmware-flash activation operation; denied in its recognized secure-command envelope. |
| Lane | Integer index 0–3, unitless | One of four corresponding event-data paths. Input/output lane identity must be preserved; each port queue depth is 1. |
| Dispatch period | 1000 ms | Period for both RxFirewall and MAVLinkFirewall. |
| Compute-execution time | 300 ms | Configured execution time for both RxFirewall and MAVLinkFirewall. |
| Routing classification | Enumeration `{Drop, DirectVMM, MAVLinkFirewall}` | Result of networking-only policy in RxFirewall/`firewall_core`; exactly one result per input. |
| Drop reason | Implementation enumeration | At minimum distinguishes invalid Ethernet/IPv4/UDP, TCP denied, non-whitelisted traffic, malformed MAVLink framing/length/checksum, unsupported framing feature, and firmware-flash activation denied. |

Dialect metadata is derived from the CR-local `ardupilotmega.xml`, `common.xml`,
`standard.xml`, and `minimal.xml` files. Implementations shall resolve includes and
duplicate definitions deterministically and shall not treat descriptive occurrences of
the word “firmware” as deny rules.
