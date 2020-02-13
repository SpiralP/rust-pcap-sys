//! LINK-LAYER HEADER TYPE VALUES [https://www.tcpdump.org/linktypes.html](https://www.tcpdump.org/linktypes.html)
//!
//! LINKTYPE_ name | LINKTYPE_ value | Corresponding DLT_ name | Description

/// DLT_NULL BSD loopback encapsulation; the link layer header is a 4-byte field, in host byte order, containing a value of 2 for IPv4 packets, a value of either 24, 28, or 30 for IPv6 packets, a value of 7 for OSI packets, or a value of 23 for IPX packets. All of the IPv6 values correspond to IPv6 packets; code reading files should check for all of them. Note that ``host byte order'' is the byte order of the machine on which the packets are captured; if a live capture is being done, ``host byte order'' is the byte order of the machine capturing the packets, but if a ``savefile'' is being read, the byte order is not necessarily that of the machine reading the capture file.
pub const NULL: i32 = 0;
/// DLT_EN10MB IEEE 802.3 Ethernet (10Mb, 100Mb, 1000Mb, and up); the 10MB in the DLT_ name is historical.
pub const ETHERNET: i32 = 1;
/// DLT_AX25 AX.25 packet, with nothing preceding it.
pub const AX25: i32 = 3;
/// DLT_IEEE802 IEEE 802.5 Token Ring; the IEEE802, without _5, in the DLT_ name is historical.
pub const IEEE802_5: i32 = 6;
/// DLT_ARCNET ARCNET Data Packets, as described by the ARCNET Trade Association standard ATA 878.1-1999, but without the Starting Delimiter, Information Length, or Frame Check Sequence fields, and with only the first ISU of the Destination Identifier. For most packet types, ARCNET Trade Association draft standard ATA 878.2 is also used. See also RFC 1051 and RFC 1201; for RFC 1051 frames, ATA 878.2 is not used.
pub const ARCNET_BSD: i32 = 7;
/// DLT_SLIP SLIP, encapsulated with a LINKTYPE_SLIP header.
pub const SLIP: i32 = 8;
/// DLT_PPP PPP, as per RFC 1661 and RFC 1662; if the first 2 bytes are 0xff and 0x03, it's PPP in HDLC-like framing, with the PPP header following those two bytes, otherwise it's PPP without framing, and the packet begins with the PPP header. The data in the frame is not octet-stuffed or bit-stuffed.
pub const PPP: i32 = 9;
/// DLT_FDDI FDDI, as specified by ANSI INCITS 239-1994.
pub const FDDI: i32 = 10;
/// DLT_PPP_SERIAL PPP in HDLC-like framing, as per RFC 1662, or Cisco PPP with HDLC framing, as per section 4.3.1 of RFC 1547; the first byte will be 0xFF for PPP in HDLC-like framing, and will be 0x0F or 0x8F for Cisco PPP with HDLC framing. The data in the frame is not octet-stuffed or bit-stuffed.
pub const PPP_HDLC: i32 = 50;
/// DLT_PPP_ETHER PPPoE; the packet begins with a PPPoE header, as per RFC 2516.
pub const PPP_ETHER: i32 = 51;
/// DLT_ATM_RFC1483 RFC 1483 LLC/SNAP-encapsulated ATM; the packet begins with an ISO 8802-2 (formerly known as IEEE 802.2) LLC header.
pub const ATM_RFC1483: i32 = 100;
/// DLT_RAW Raw IP; the packet begins with an IPv4 or IPv6 header, with the "version" field of the header indicating whether it's an IPv4 or IPv6 header.
pub const RAW: i32 = 101;
/// DLT_C_HDLC Cisco PPP with HDLC framing, as per section 4.3.1 of RFC 1547.
pub const C_HDLC: i32 = 104;
/// DLT_IEEE802_11 IEEE 802.11 wireless LAN.
pub const IEEE802_11: i32 = 105;
/// DLT_FRELAY Frame Relay LAPF frames, beginning with a ITU-T Recommendation Q.922 LAPF header starting with the address field, and without an FCS at the end of the frame.
pub const FRELAY: i32 = 107;
/// DLT_LOOP OpenBSD loopback encapsulation; the link-layer header is a 4-byte field, in network byte order, containing a value of 2 for IPv4 packets, a value of either 24, 28, or 30 for IPv6 packets, a value of 7 for OSI packets, or a value of 23 for IPX packets. All of the IPv6 values correspond to IPv6 packets; code reading files should check for all of them.
pub const LOOP: i32 = 108;
/// DLT_LINUX_SLL Linux "cooked" capture encapsulation.
pub const LINUX_SLL: i32 = 113;
/// DLT_LTALK Apple LocalTalk; the packet begins with an AppleTalk LocalTalk Link Access Protocol header, as described in chapter 1 of Inside AppleTalk, Second Edition.
pub const LTALK: i32 = 114;
/// DLT_PFLOG OpenBSD pflog; the link-layer header contains a "struct pfloghdr" structure, as defined by the host on which the file was saved. (This differs from operating system to operating system and release to release; there is nothing in the file to indicate what the layout of that structure is.)
pub const PFLOG: i32 = 117;
/// DLT_PRISM_HEADER Prism monitor mode information followed by an 802.11 header.
pub const IEEE802_11_PRISM: i32 = 119;
/// DLT_IP_OVER_FC RFC 2625 IP-over-Fibre Channel, with the link-layer header being the Network_Header as described in that RFC.
pub const IP_OVER_FC: i32 = 122;
/// DLT_SUNATM ATM traffic, encapsulated as per the scheme used by SunATM devices.
pub const SUNATM: i32 = 123;
/// DLT_IEEE802_11_RADIO Radiotap link-layer information followed by an 802.11 header.
pub const IEEE802_11_RADIOTAP: i32 = 127;
/// DLT_ARCNET_LINUX ARCNET Data Packets, as described by the ARCNET Trade Association standard ATA 878.1-1999, but without the Starting Delimiter, Information Length, or Frame Check Sequence fields, with only the first ISU of the Destination Identifier, and with an extra two-ISU "offset" field following the Destination Identifier. For most packet types, ARCNET Trade Association draft standard ATA 878.2 is also used; however, no exception frames are supplied, and reassembled frames, rather than fragments, are supplied. See also RFC 1051 and RFC 1201; for RFC 1051 frames, ATA 878.2 is not used.
pub const ARCNET_LINUX: i32 = 129;
/// DLT_APPLE_IP_OVER_IEEE1394 Apple IP-over-IEEE 1394 cooked header.
pub const APPLE_IP_OVER_IEEE1394: i32 = 138;
/// DLT_MTP2_WITH_PHDR Signaling System 7 Message Transfer Part Level 2, as specified by ITU-T Recommendation Q.703, preceded by a pseudo-header.
pub const MTP2_WITH_PHDR: i32 = 139;
/// DLT_MTP2 Signaling System 7 Message Transfer Part Level 2, as specified by ITU-T Recommendation Q.703.
pub const MTP2: i32 = 140;
/// DLT_MTP3 Signaling System 7 Message Transfer Part Level 3, as specified by ITU-T Recommendation Q.704, with no MTP2 header preceding the MTP3 packet.
pub const MTP3: i32 = 141;
/// DLT_SCCP Signaling System 7 Signalling Connection Control Part, as specified by ITU-T Recommendation Q.711, ITU-T Recommendation Q.712, ITU-T Recommendation Q.713, and ITU-T Recommendation Q.714, with no MTP3 or MTP2 headers preceding the SCCP packet.
pub const SCCP: i32 = 142;
/// DLT_DOCSIS DOCSIS MAC frames, as described by the DOCSIS 3.1 MAC and Upper Layer Protocols Interface Specification or earlier specifications for MAC frames.
pub const DOCSIS: i32 = 143;
/// DLT_LINUX_IRDA Linux-IrDA packets, with a LINKTYPE_LINUX_IRDA header, with the payload for IrDA frames beginning with by the IrLAP header as defined by IrDA Data Specifications, including the IrDA Link Access Protocol specification.
pub const LINUX_IRDA: i32 = 144;

// LINKTYPE_USER0-LINKTYPE-USER15 147-162 DLT_USER0-DLT_USER15 Reserved for private use; see above.

/// DLT_IEEE802_11_RADIO_AVS AVS monitor mode information followed by an 802.11 header.
pub const IEEE802_11_AVS: i32 = 163;
/// DLT_BACNET_MS_TP BACnet MS/TP frames, as specified by section 9.3 MS/TP Frame Format of ANSI/ASHRAE Standard 135, BACnet® - A Data Communication Protocol for Building Automation and Control Networks, including the preamble and, if present, the Data CRC.
pub const BACNET_MS_TP: i32 = 165;
/// DLT_PPP_PPPD PPP in HDLC-like encapsulation, like LINKTYPE_PPP_HDLC, but with the 0xff address byte replaced by a direction indication - 0x00 for incoming and 0x01 for outgoing.
pub const PPP_PPPD: i32 = 166;
/// DLT_GPRS_LLC General Packet Radio Service Logical Link Control, as defined by 3GPP TS 04.64.
pub const GPRS_LLC: i32 = 169;
/// DLT_GPF_T Transparent-mapped generic framing procedure, as specified by ITU-T Recommendation G.7041/Y.1303.
pub const GPF_T: i32 = 170;
/// DLT_GPF_F Frame-mapped generic framing procedure, as specified by ITU-T Recommendation G.7041/Y.1303.
pub const GPF_F: i32 = 171;
/// DLT_LINUX_LAPD Link Access Procedures on the D Channel (LAPD) frames, as specified by ITU-T Recommendation Q.920 and ITU-T Recommendation Q.921, captured via vISDN, with a LINKTYPE_LINUX_LAPD header, followed by the Q.921 frame, starting with the address field.
pub const LINUX_LAPD: i32 = 177;
/// DLT_MFR FRF.16.1 Multi-Link Frame Relay frames, beginning with an FRF.12 Interface fragmentation format fragmentation header.
pub const MFR: i32 = 182;
/// DLT_BLUETOOTH_HCI_H4 Bluetooth HCI UART transport layer; the frame contains an HCI packet indicator byte, as specified by the UART Transport Layer portion of the most recent Bluetooth Core specification, followed by an HCI packet of the specified packet type, as specified by the Host Controller Interface Functional Specification portion of the most recent Bluetooth Core Specification.
pub const BLUETOOTH_HCI_H4: i32 = 187;
/// DLT_USB_LINUX USB packets, beginning with a Linux USB header, as specified by the struct usbmon_packet in the Documentation/usb/usbmon.txt file in the Linux source tree. Only the first 48 bytes of that header are present. All fields in the header are in host byte order. When performing a live capture, the host byte order is the byte order of the machine on which the packets are captured. When reading a pcap file, the byte order is the byte order for the file, as specified by the file's magic number; when reading a pcapng file, the byte order is the byte order for the section of the pcapng file, as specified by the Section Header Block.
pub const USB_LINUX: i32 = 189;
/// DLT_PPI Per-Packet Information information, as specified by the Per-Packet Information Header Specification, followed by a packet with the LINKTYPE_ value specified by the pph_dlt field of that header.
pub const PPI: i32 = 192;
/// DLT_IEEE802_15_4_WITHFCS IEEE 802.15.4 Low-Rate Wireless Networks, with each packet having the FCS at the end of the frame.
pub const IEEE802_15_4_WITHFCS: i32 = 195;
/// DLT_SITA Various link-layer types, with a pseudo-header, for SITA.
pub const SITA: i32 = 196;
/// DLT_ERF Various link-layer types, with a pseudo-header, for Endace DAG cards; encapsulates Endace ERF records.
pub const ERF: i32 = 197;
/// DLT_BLUETOOTH_HCI_H4_WITH_PHDR Bluetooth HCI UART transport layer; the frame contains a 4-byte direction field, in network byte order (big-endian), the low-order bit of which is set if the frame was sent from the host to the controller and clear if the frame was received by the host from the controller, followed by an HCI packet indicator byte, as specified by the UART Transport Layer portion of the most recent Bluetooth Core specification, followed by an HCI packet of the specified packet type, as specified by the Host Controller Interface Functional Specification portion of the most recent Bluetooth Core Specification.
pub const BLUETOOTH_HCI_H4_WITH_PHDR: i32 = 201;
/// DLT_AX25_KISS AX.25 packet, with a 1-byte KISS header containing a type indicator.
pub const AX25_KISS: i32 = 202;
/// DLT_LAPD Link Access Procedures on the D Channel (LAPD) frames, as specified by ITU-T Recommendation Q.920 and ITU-T Recommendation Q.921, starting with the address field, with no pseudo-header.
pub const LAPD: i32 = 203;
/// DLT_PPP_WITH_DIR PPP, as per RFC 1661 and RFC 1662, preceded with a one-byte pseudo-header with a zero value meaning "received by this host" and a non-zero value meaning "sent by this host"; if the first 2 bytes are 0xff and 0x03, it's PPP in HDLC-like framing, with the PPP header following those two bytes, otherwise it's PPP without framing, and the packet begins with the PPP header. The data in the frame is not octet-stuffed or bit-stuffed.
pub const PPP_WITH_DIR: i32 = 204;
/// DLT_C_HDLC_WITH_DIR Cisco PPP with HDLC framing, as per section 4.3.1 of RFC 1547, preceded with a one-byte pseudo-header with a zero value meaning "received by this host" and a non-zero value meaning "sent by this host".
pub const C_HDLC_WITH_DIR: i32 = 205;
/// DLT_FRELAY_WITH_DIR Frame Relay LAPF frames, beginning with a one-byte pseudo-header with a zero value meaning "received by this host" (DCE->DTE) and a non-zero value meaning "sent by this host" (DTE->DCE), followed by an ITU-T Recommendation Q.922 LAPF header starting with the address field, and without an FCS at the end of the frame.
pub const FRELAY_WITH_DIR: i32 = 206;
/// DLT_LAPB_WITH_DIR Link Access Procedure, Balanced (LAPB), as specified by ITU-T Recommendation X.25, preceded with a one-byte pseudo-header with a zero value meaning "received by this host" (DCE->DTE) and a non-zero value meaning "sent by this host" (DTE->DCE).
pub const LAPB_WITH_DIR: i32 = 207;
/// DLT_IPMB_LINUX IPMB over an I2C circuit, with a Linux-specific pseudo-header.
pub const IPMB_LINUX: i32 = 209;
/// DLT_IEEE802_15_4_NONASK_PHY IEEE 802.15.4 Low-Rate Wireless Networks, with each packet having the FCS at the end of the frame, and with the PHY-level data for the O-QPSK, BPSK, GFSK, MSK, and RCC DSS BPSK PHYs (4 octets of 0 as preamble, one octet of SFD, one octet of frame length + reserved bit) preceding the MAC-layer data (starting with the frame control field).
pub const IEEE802_15_4_NONASK_PHY: i32 = 215;
/// DLT_USB_LINUX_MMAPPED USB packets, beginning with a Linux USB header, as specified by the struct usbmon_packet in the Documentation/usb/usbmon.txt file in the Linux source tree. All 64 bytes of the header are present. All fields in the header are in host byte order. When performing a live capture, the host byte order is the byte order of the machine on which the packets are captured. When reading a pcap file, the byte order is the byte order for the file, as specified by the file's magic number; when reading a pcapng file, the byte order is the byte order for the section of the pcapng file, as specified by the Section Header Block. For isochronous transfers, the ndesc field specifies the number of isochronous descriptors that follow.
pub const USB_LINUX_MMAPPED: i32 = 220;
/// DLT_FC_2 Fibre Channel FC-2 frames, beginning with a Frame_Header.
pub const FC_2: i32 = 224;
/// DLT_FC_2_WITH_FRAME_DELIMS Fibre Channel FC-2 frames, beginning an encoding of the SOF, followed by a Frame_Header, and ending with an encoding of the SOF. The encodings represent the frame delimiters as 4-byte sequences representing the corresponding ordered sets, with K28.5 represented as 0xBC, and the D symbols as the corresponding byte values; for example, SOFi2, which is K28.5 - D21.5 - D1.2 - D21.2, is represented as 0xBC 0xB5 0x55 0x55.
pub const FC_2_WITH_FRAME_DELIMS: i32 = 225;
/// DLT_IPNET Solaris ipnet pseudo-header, followed by an IPv4 or IPv6 datagram.
pub const IPNET: i32 = 226;
/// DLT_CAN_SOCKETCAN CAN (Controller Area Network) frames, with a pseudo-header followed by the frame payload.
pub const CAN_SOCKETCAN: i32 = 227;
/// DLT_IPV4 Raw IPv4; the packet begins with an IPv4 header.
pub const IPV4: i32 = 228;
/// DLT_IPV6 Raw IPv6; the packet begins with an IPv6 header.
pub const IPV6: i32 = 229;
/// DLT_IEEE802_15_4_NOFCS IEEE 802.15.4 Low-Rate Wireless Network, without the FCS at the end of the frame.
pub const IEEE802_15_4_NOFCS: i32 = 230;
/// DLT_DBUS Raw D-Bus messages, starting with the endianness flag, followed by the message type, etc., but without the authentication handshake before the message sequence.
pub const DBUS: i32 = 231;
/// DLT_DVB_CI DVB-CI (DVB Common Interface for communication between a PC Card module and a DVB receiver), with the message format specified by the PCAP format for DVB-CI specification.
pub const DVB_CI: i32 = 235;
/// DLT_MUX27010 Variant of 3GPP TS 27.010 multiplexing protocol (similar to, but not the same as, 27.010).
pub const MUX27010: i32 = 236;
/// DLT_STANAG_5066_D_PDU D_PDUs as described by NATO standard STANAG 5066, starting with the synchronization sequence, and including both header and data CRCs. The current version of STANAG 5066 is backwards-compatible with the 1.0.2 version, although newer versions are classified.
pub const STANAG_5066_D_PDU: i32 = 237;
/// DLT_NFLOG Linux netlink NETLINK NFLOG socket log messages.
pub const NFLOG: i32 = 239;
/// DLT_NETANALYZER Pseudo-header for Hilscher Gesellschaft für Systemautomation mbH netANALYZER devices, followed by an Ethernet frame, beginning with the MAC header and ending with the FCS.
pub const NETANALYZER: i32 = 240;
/// DLT_NETANALYZER_TRANSPARENT Pseudo-header for Hilscher Gesellschaft für Systemautomation mbH netANALYZER devices, followed by an Ethernet frame, beginning with the preamble, SFD, and MAC header, and ending with the FCS.
pub const NETANALYZER_TRANSPARENT: i32 = 241;
/// DLT_IPOIB IP-over-InfiniBand, as specified by RFC 4391 section 6.
pub const IPOIB: i32 = 242;
/// DLT_MPEG_2_TS MPEG-2 Transport Stream transport packets, as specified by ISO 13818-1/ITU-T Recommendation H.222.0 (see table 2-2 of section 2.4.3.2 "Transport Stream packet layer").
pub const MPEG_2_TS: i32 = 243;
/// DLT_NG40 Pseudo-header for ng4T GmbH's UMTS Iub/Iur-over-ATM and Iub/Iur-over-IP format as used by their ng40 protocol tester, followed by frames for the Frame Protocol as specified by 3GPP TS 25.427 for dedicated channels and 3GPP TS 25.435 for common/shared channels in the case of ATM AAL2 or UDP traffic, by SSCOP packets as specified by ITU-T Recommendation Q.2110 for ATM AAL5 traffic, and by NBAP packets for SCTP traffic.
pub const NG40: i32 = 244;
/// DLT_NFC_LLCP Pseudo-header for NFC LLCP packet captures, followed by frame data for the LLCP Protocol as specified by NFCForum-TS-LLCP_1.1.
pub const NFC_LLCP: i32 = 245;
/// DLT_INFINIBAND Raw InfiniBand frames, starting with the Local Routing Header, as specified in Chapter 5 "Data packet format" of InfiniBand™ Architectural Specification Release 1.2.1 Volume 1 - General Specifications.
pub const INFINIBAND: i32 = 247;
/// DLT_SCTP SCTP packets, as defined by RFC 4960, with no lower-level protocols such as IPv4 or IPv6.
pub const SCTP: i32 = 248;
/// DLT_USBPCAP USB packets, beginning with a USBPcap header.
pub const USBPCAP: i32 = 249;
/// DLT_RTAC_SERIAL Serial-line packet header for the Schweitzer Engineering Laboratories "RTAC" product, followed by a payload for one of a number of industrial control protocols.
pub const RTAC_SERIAL: i32 = 250;
/// DLT_BLUETOOTH_LE_LL Bluetooth Low Energy air interface Link Layer packets, in the format described in section 2.1 "PACKET FORMAT" of volume 6 of the Bluetooth Specification Version 4.0 (see PDF page 2200), but without the Preamble.
pub const BLUETOOTH_LE_LL: i32 = 251;
/// DLT_NETLINK Linux Netlink capture encapsulation.
pub const NETLINK: i32 = 253;
/// DLT_BLUETOOTH_LINUX_MONITOR Bluetooth Linux Monitor encapsulation of traffic for the BlueZ stack.
pub const BLUETOOTH_LINUX_MONITOR: i32 = 254;
/// DLT_BLUETOOTH_BREDR_BB Bluetooth Basic Rate and Enhanced Data Rate baseband packets.
pub const BLUETOOTH_BREDR_BB: i32 = 255;
/// DLT_BLUETOOTH_LE_LL_WITH_PHDR Bluetooth Low Energy link-layer packets.
pub const BLUETOOTH_LE_LL_WITH_PHDR: i32 = 256;
/// DLT_PROFIBUS_DL PROFIBUS data link layer packets, as specified by IEC standard 61158-4-3, beginning with the start delimiter, ending with the end delimiter, and including all octets between them.
pub const PROFIBUS_DL: i32 = 257;
/// DLT_PKTAP Apple PKTAP capture encapsulation.
pub const PKTAP: i32 = 258;
/// DLT_EPON Ethernet-over-passive-optical-network packets, starting with the last 6 octets of the modified preamble as specified by 65.1.3.2 "Transmit" in Clause 65 of Section 5 of IEEE 802.3, followed immediately by an Ethernet frame.
pub const EPON: i32 = 259;
/// DLT_IPMI_HPM_2 IPMI trace packets, as specified by Table 3-20 "Trace Data Block Format" in the PICMG HPM.2 specification. The time stamps for packets in this format must match the time stamps in the Trace Data Blocks.
pub const IPMI_HPM_2: i32 = 260;
/// DLT_ZWAVE_R1_R2 Z-Wave RF profile R1 and R2 packets, as specified by ITU-T Recommendation G.9959, with some MAC layer fields moved.
pub const ZWAVE_R1_R2: i32 = 261;
/// DLT_ZWAVE_R3 Z-Wave RF profile R3 packets, as specified by ITU-T Recommendation G.9959, with some MAC layer fields moved.
pub const ZWAVE_R3: i32 = 262;
/// DLT_WATTSTOPPER_DLM Formats for WattStopper Digital Lighting Management (DLM) and Legrand Nitoo Open protocol common packet structure captures.
pub const WATTSTOPPER_DLM: i32 = 263;
/// DLT_ISO_14443 Messages between ISO 14443 contactless smartcards (Proximity Integrated Circuit Card, PICC) and card readers (Proximity Coupling Device, PCD), with the message format specified by the PCAP format for ISO14443 specification.
pub const ISO_14443: i32 = 264;
/// DLT_RDS Radio data system (RDS) groups, as per IEC 62106, encapsulated in this form.
pub const RDS: i32 = 265;
/// DLT_USB_DARWIN USB packets, beginning with a Darwin (macOS, etc.) USB header.
pub const USB_DARWIN: i32 = 266;
/// DLT_SDLC SDLC packets, as specified by Chapter 1, "DLC Links", section "Synchronous Data Link Control (SDLC)" of Systems Network Architecture Formats, GA27-3136-20, without the flag fields, zero-bit insertion, or Frame Check Sequence field, containing SNA path information units (PIUs) as the payload.
pub const SDLC: i32 = 268;
/// DLT_LORATAP LoRaTap pseudo-header, followed by the payload, which is typically the PHYPayload from the LoRaWan specification.
pub const LORATAP: i32 = 270;
/// DLT_VSOCK Protocol for communication between host and guest machines in VMware and KVM hypervisors.
pub const VSOCK: i32 = 271;
/// DLT_NORDIC_BLE Messages to and from a Nordic Semiconductor nRF Sniffer for Bluetooth LE packets, beginning with a pseudo-header.
pub const NORDIC_BLE: i32 = 272;
/// DLT_DOCSIS31_XRA31 DOCSIS packets and bursts, preceded by a pseudo-header giving metadata about the packet.
pub const DOCSIS31_XRA31: i32 = 273;
/// DLT_ETHERNET_MPACKET mPackets, as specified by IEEE 802.3br Figure 99-4, starting with the preamble and always ending with a CRC field.
pub const ETHERNET_MPACKET: i32 = 274;
/// DLT_DISPLAYPORT_AUX DisplayPort AUX channel monitoring data as specified by VESA DisplayPort(DP) Standard preceeded by a pseudo-header.
pub const DISPLAYPORT_AUX: i32 = 275;
/// DLT_LINUX_SLL2 Linux "cooked" capture encapsulation v2.
pub const LINUX_SLL2: i32 = 276;
/// DLT_OPENVIZSLA Openvizsla FPGA-based USB sniffer.
pub const OPENVIZSLA: i32 = 278;
/// DLT_EBHSCR Elektrobit High Speed Capture and Replay (EBHSCR) format.
pub const EBHSCR: i32 = 279;
/// DLT_VPP_DISPATCH Records in traces from the http://fd.io VPP graph dispatch tracer, in the the graph dispatcher trace format.
pub const VPP_DISPATCH: i32 = 280;
/// DLT_DSA_TAG_BRCM Ethernet frames, with a switch tag inserted between the source address field and the type/length field in the Ethernet header.
pub const DSA_TAG_BRCM: i32 = 281;
/// DLT_DSA_TAG_BRCM_PREPEND Ethernet frames, with a switch tag inserted before the destination address in the Ethernet header.
pub const DSA_TAG_BRCM_PREPEND: i32 = 282;
/// DLT_IEEE802_15_4_TAP IEEE 802.15.4 Low-Rate Wireless Networks, with a pseudo-header containing TLVs with metadata preceding the 802.15.4 header.
pub const IEEE802_15_4_TAP: i32 = 283;
/// DLT_DSA_TAG_DSA Ethernet frames, with a switch tag inserted between the source address field and the type/length field in the Ethernet header.
pub const DSA_TAG_DSA: i32 = 284;
/// DLT_DSA_TAG_EDSA Ethernet frames, with a programmable Ethernet type switch tag inserted between the source address field and the type/length field in the Ethernet header.
pub const DSA_TAG_EDSA: i32 = 285;
/// DLT_ELEE Payload of lawful intercept packets using the ELEE protocol. The packet begins with the ELEE header; it does not include any transport-layer or lower-layer headers for protcols used to transport ELEE packets.
pub const ELEE: i32 = 286;
