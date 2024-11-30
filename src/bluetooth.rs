/// Values are taken from Assigned Numbers.

use uuid::Uuid;

const BASE_UUID: u128 = 0x00000000_0000_1000_8000_00805f9b34fb;
const fn uuid_from_u16(v: u16) -> Uuid {
    Uuid::from_u128(BASE_UUID | ((v as u128) << 0x60))
}


/// 16-bit PSMs and SPSMs, from section 2.5 of Assigned Numbers.
pub mod psm {
    pub const SDP: u16 = 0x0001u16;
    pub const RFCOMM: u16 = 0x0003u16;
    pub const TCS_BIN: u16 = 0x0005u16;
    pub const TCS_BIN_CORDLESS: u16 = 0x0007u16;

    pub const BNEP: u16 = 0x000Fu16;
    pub const HID_CONTROL: u16 = 0x0011u16;
    pub const HID_INTERRUPT: u16 = 0x0013u16;
    pub const UPNP: u16 = 0x0015u16;
    pub const AVCTP: u16 = 0x0017u16;
    pub const AVDTP: u16 = 0x0019u16;
    pub const AVCTP_BROWSING: u16 = 0x001Bu16;
    pub const UDI_C_PLANE: u16 = 0x001Du16;
    pub const ATT: u16 = 0x001Fu16;
    pub const _3DSP: u16 = 0x0021u16;
    pub const LE_PSM_IPSP: u16 = 0x0023u16;
    pub const OTS: u16 = 0x0025u16;
    pub const EATT: u16 = 0x0027u16;
}

/// 16-bit protocol UUIDs, from section 3.1 of Assigned Numbers.
pub mod protocol {
    use uuid::Uuid;
    use super::uuid_from_u16;

    pub const SDP: Uuid = uuid_from_u16(0x0001);
    pub const UDP: Uuid = uuid_from_u16(0x0002);
    pub const RFCOMM: Uuid = uuid_from_u16(0x0003);
    pub const TCP: Uuid = uuid_from_u16(0x0004);
    pub const TCS_BIN: Uuid = uuid_from_u16(0x0005);
    pub const TCS_AT: Uuid = uuid_from_u16(0x0006);
    pub const ATT: Uuid = uuid_from_u16(0x0007);
    pub const OBEX: Uuid = uuid_from_u16(0x0008);
    pub const IP: Uuid = uuid_from_u16(0x0009);
    pub const FTP: Uuid = uuid_from_u16(0x000A);
    pub const HTTP: Uuid = uuid_from_u16(0x000C);

    pub const WSP: Uuid = uuid_from_u16(0x000E);
    pub const BNEP: Uuid = uuid_from_u16(0x000F);
    pub const UPNP: Uuid = uuid_from_u16(0x0010);
    pub const HID_PROTOCOL: Uuid = uuid_from_u16(0x0011);
    pub const HARDCOPY_CONTROL_CHANNEL: Uuid = uuid_from_u16(0x0012);

    pub const HARDCOPY_DATA_CHANNEL: Uuid = uuid_from_u16(0x0014);

    pub const HARDCOPY_NOTIFICATION_CHANNEL: Uuid = uuid_from_u16(0x0016);
    pub const AVCTP: Uuid = uuid_from_u16(0x0017);

    pub const AVDTP: Uuid = uuid_from_u16(0x0019);

    pub const CMTP: Uuid = uuid_from_u16(0x001B);
    pub const MCAP_CONTROL_CHANNEL: Uuid = uuid_from_u16(0x001E);
    pub const MCAP_DATA_CHANNEL: Uuid = uuid_from_u16(0x001F);

    pub const L2CAP: Uuid = uuid_from_u16(0x0100);
}

/// 16-bit browse group UUIDs, from section 3.2 of Assigned Numbers.
pub mod browse_group {
    use uuid::Uuid;
    use super::uuid_from_u16;

    pub const PUBLIC_BROWSE_ROOT: Uuid = uuid_from_u16(0x1002);
}

/// SDP service class and profile identifers, from section 3.3 of Assigned Numbers.
pub mod service_class {
    use uuid::Uuid;
    use super::uuid_from_u16;

    pub const SERVICE_DISCOVERY_SERVER_SERVICE_CLASS_ID: Uuid = uuid_from_u16(0x0100);
    pub const BROWSE_GROUP_DESCRIPTOR_SERVICE_CLASS_ID: Uuid = uuid_from_u16(0x0101);

    pub const SERIAL_PORT: Uuid = uuid_from_u16(0x1101);
    pub const LAN_ACCESS_USING_PPP: Uuid = uuid_from_u16(0x1102);
    pub const DIAL_UP_NETWORKING: Uuid = uuid_from_u16(0x1103);
    pub const IRMC_SYNC: Uuid = uuid_from_u16(0x1104);
    pub const OBEX_OBJECT_PUSH: Uuid = uuid_from_u16(0x1105);
    pub const OBEX_FILE_TRANSFER: Uuid = uuid_from_u16(0x1106);
    pub const IRMC_SYNC_COMMAND: Uuid = uuid_from_u16(0x1107);
    pub const HEADSET: Uuid = uuid_from_u16(0x1108);
    pub const CORDLESS_TELEPHONY: Uuid = uuid_from_u16(0x1109);
    pub const AUDIO_SOURCE: Uuid = uuid_from_u16(0x110A);
    pub const AUDIO_SINK: Uuid = uuid_from_u16(0x110B);
    pub const AV_REMOTE_CONTROL_TARGET: Uuid = uuid_from_u16(0x110C);
    // const ADVANCED_AUDIO_DISTRIBUTION: Uuid = uuid_from_u16(0x110D);
    pub const AV_REMOTE_CONTROL: Uuid = uuid_from_u16(0x110E);
    pub const AV_REMOTE_CONTROL_CONTROLLER: Uuid = uuid_from_u16(0x110F);
    pub const INTERCOM: Uuid = uuid_from_u16(0x1110);
    pub const FAX: Uuid = uuid_from_u16(0x1111);
    pub const HEADSET_AUDIO_GATEWAY: Uuid = uuid_from_u16(0x1112);
    pub const WAP: Uuid = uuid_from_u16(0x1113);
    pub const WAP_CLIENT: Uuid = uuid_from_u16(0x1114);
    pub const PANU: Uuid = uuid_from_u16(0x1115);
    pub const NAP: Uuid = uuid_from_u16(0x1116);
    pub const GN: Uuid = uuid_from_u16(0x1117);
    pub const DIRECT_PRINTING: Uuid = uuid_from_u16(0x1118);
    pub const REFERENCE_PRINTING: Uuid = uuid_from_u16(0x1119);
    // const IMAGING: Uuid = uuid_from_u16(0x111A);
    pub const IMAGING_RESPONDER: Uuid = uuid_from_u16(0x111B);
    pub const IMAGING_AUTOMATIC_ARCHIVE: Uuid = uuid_from_u16(0x111C);
    pub const IMAGING_REFERENCED_OBJECTS: Uuid = uuid_from_u16(0x111D);
    pub const HANDS_FREE: Uuid = uuid_from_u16(0x111E);
    pub const AG_HANDS_FREE: Uuid = uuid_from_u16(0x111F);
    pub const DIRECT_PRINTING_REFERENCED_OBJECTS_SERVICE: Uuid = uuid_from_u16(0x1120);
    pub const REFLECTED_UI: Uuid = uuid_from_u16(0x1121);
    // const BASIC_PRINTING: Uuid = uuid_from_u16(0x1122);
    pub const PRINTING_STATUS: Uuid = uuid_from_u16(0x1123);
    pub const HID: Uuid = uuid_from_u16(0x1124);
    // const HARDCOPY_CABLE_REPlACEMENT: Uuid = uuid_from_u16(0x1125);
    pub const HCR_PRINT: Uuid = uuid_from_u16(0x1126);
    pub const HCR_SCAN: Uuid = uuid_from_u16(0x1127);
    pub const COMMON_ISDN_ACCESS: Uuid = uuid_from_u16(0x1128);

    pub const SIM_ACCESS: Uuid = uuid_from_u16(0x112D);
    pub const PHONEBOOK_ACCESS_CLIENT: Uuid = uuid_from_u16(0x112E);
    pub const PHONEBOOK_ACCESS_SERVER: Uuid = uuid_from_u16(0x112F);
    // const PHONEBOOK_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1130);
    pub const HEADSET_HS: Uuid = uuid_from_u16(0x1131);
    pub const MESSAGE_ACCESS_SERVER: Uuid = uuid_from_u16(0x1132);
    pub const MESSAGE_NOTIFICATION_SERVER: Uuid = uuid_from_u16(0x1133);
    // const MESSAGE_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1134);
    // const GNSS: Uuid = uuid_from_u16(0x1135);
    pub const GNSS_SERVER: Uuid = uuid_from_u16(0x1136);
    pub const _3D_DISPLAY: Uuid = uuid_from_u16(0x1137);
    pub const _3D_GLASSES: Uuid = uuid_from_u16(0x1138);
    // const _3D_SYNCH_PROFILE: Uuid = uuid_from_u16(0x1139);
    // const MULTI_PROFILE_SPECIFICATION: Uuid = uuid_from_u16(0x113A);
    pub const MPS: Uuid = uuid_from_u16(0x113B);
    pub const CTN_ACCESS_SERVICE: Uuid = uuid_from_u16(0x113C);
    pub const CTN_NOTIFICATION_SERVICE: Uuid = uuid_from_u16(0x113D);
    // const CALENDAR_TASKS_AND_NOTES_PROFILE: Uuid = uuid_from_u16(0x113E);

    pub const PNP_INFORMATION: Uuid = uuid_from_u16(0x1200);
    pub const GENERIC_NETWORKING: Uuid = uuid_from_u16(0x1201);
    pub const GENERIC_FILE_TRANSFER: Uuid = uuid_from_u16(0x1202);
    pub const GENERIC_AUDIO: Uuid = uuid_from_u16(0x1203);
    pub const GENERIC_TELEPHONY: Uuid = uuid_from_u16(0x1204);
    pub const UPNP_SERVICE: Uuid = uuid_from_u16(0x1205);
    pub const UPNP_IP_SERVICE: Uuid = uuid_from_u16(0x1206);

    pub const ESDP_UPNP_IP_PAN: Uuid = uuid_from_u16(0x1300);
    pub const ESDP_UPNP_IP_LAP: Uuid = uuid_from_u16(0x1301);
    pub const ESDP_UPNP_L2CAP: Uuid = uuid_from_u16(0x1302);
    pub const VIDEO_SOURCE: Uuid = uuid_from_u16(0x1303);
    pub const VIDEO_SINK: Uuid = uuid_from_u16(0x1304);
    // const VIDEO_DISTRIBUTION: Uuid = uuid_from_u16(0x1305);

    // const HDP: Uuid = uuid_from_u16(0x1400);
    pub const HDP_SOURCE: Uuid = uuid_from_u16(0x1401);
    pub const HDP_SINK: Uuid = uuid_from_u16(0x1402);
}

pub mod profile {
    use uuid::Uuid;
    use super::uuid_from_u16;

    pub const SERIAL_PORT: Uuid = uuid_from_u16(0x1101);
    pub const LAN_ACCESS_USING_PPP: Uuid = uuid_from_u16(0x1102);
    pub const DIAL_UP_NETWORKING: Uuid = uuid_from_u16(0x1103);
    pub const IRMC_SYNC: Uuid = uuid_from_u16(0x1104);
    pub const OBEX_OBJECT_PUSH: Uuid = uuid_from_u16(0x1105);
    pub const OBEX_FILE_TRANSFER: Uuid = uuid_from_u16(0x1106);
    // const IRMC_SYNC_COMMAND: Uuid = uuid_from_u16(0x1107);
    pub const HEADSET: Uuid = uuid_from_u16(0x1108);
    pub const CORDLESS_TELEPHONY: Uuid = uuid_from_u16(0x1109);

    pub const ADVANCED_AUDIO_DISTRIBUTION: Uuid = uuid_from_u16(0x110D);
    pub const AV_REMOTE_CONTROL: Uuid = uuid_from_u16(0x110E);

    pub const PANU: Uuid = uuid_from_u16(0x1115);
    pub const NAP: Uuid = uuid_from_u16(0x1116);
    pub const GN: Uuid = uuid_from_u16(0x1117);

    pub const IMAGING: Uuid = uuid_from_u16(0x111A);

    pub const HANDS_FREE: Uuid = uuid_from_u16(0x111E);

    pub const BASIC_PRINTING: Uuid = uuid_from_u16(0x1122);

    pub const HID: Uuid = uuid_from_u16(0x1124);
    pub const HARDCOPY_CABLE_REPLACEMENT: Uuid = uuid_from_u16(0x1125);

    pub const COMMON_ISDN_ACCESS: Uuid = uuid_from_u16(0x1128);

    pub const SIM_ACCESS: Uuid = uuid_from_u16(0x112D);

    pub const PHONEBOOK_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1130);

    pub const MESSAGE_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1134);
    pub const GNSS: Uuid = uuid_from_u16(0x1135);

    pub const _3D_SYNCH_PROFILE: Uuid = uuid_from_u16(0x1139);
    pub const MULTI_PROFILE_SPECIFICATION: Uuid = uuid_from_u16(0x113A);

    pub const CALENDAR_TASKS_AND_NOTES_PROFILE: Uuid = uuid_from_u16(0x113E);

    pub const PNP_INFORMATION: Uuid = uuid_from_u16(0x1200);

    pub const VIDEO_DISTRIBUTION: Uuid = uuid_from_u16(0x1305);

    pub const HDP: Uuid = uuid_from_u16(0x1400);
}

/// 16-bit service discovery attribute IDs, from section 5.1 of Assigned Numbers.
pub mod attribute_id {
    pub const SERVICE_RECORD_HANDLE: u16 = 0x0000u16;
    pub const SERVICE_CLASS_ID_LIST: u16 = 0x0001u16;
    pub const SERVICE_RECORD_STATE: u16 = 0x0002u16;
    pub const SERVICE_ID: u16 = 0x0003u16;
    pub const PROTOCOL_DESCRIPTOR_LIST: u16 = 0x0004u16;
    pub const BROWSE_GROUP_LIST: u16 = 0x0005u16;
    pub const LANGUAGE_BASE_ATTRIBUTE_ID_LIST: u16 = 0x0006u16;
    pub const SERVICE_INFO_TIME_TO_LIVE: u16 = 0x0007u16;
    pub const SERVICE_AVAILABILITY: u16 = 0x0008u16;
    pub const BLUETOOTH_PROFILE_DESCRIPTOR_LIST: u16 = 0x0009u16;
    pub const DOCUMENTATION_URL: u16 = 0x000au16;
    pub const CLIENT_EXECUTABLE_URL: u16 = 0x000bu16;
    pub const ICON_URL: u16 = 0x000cu16;
    pub const ADDITIONAL_PROTOCOL_DESCRIPTOR_LISTS: u16 = 0x000du16;

    pub const SERVICE_NAME: u16 = 0x0100u16;
    pub const SERVICE_DESCRIPTION: u16 = 0x0101u16;
    pub const PROVIDER_NAME: u16 = 0x0102u16;

    pub mod hid {
        pub const HID_DEVICE_RELEASE_NUMBER: u16 = 0x0200u16;
        pub const HID_PARSER_VERSION: u16 = 0x0201u16;
        pub const HID_DEVICE_SUBCLASS: u16 = 0x0202u16;
        pub const HID_COUNTRY_CODE: u16 = 0x0203u16;
        pub const HID_VIRTUAL_CABLE: u16 = 0x0204u16;
        pub const HID_RECONNECT_INITIATE: u16 = 0x0205u16;
        pub const HID_DESCRIPTOR_LIST: u16 = 0x0206u16;
        pub const HID_LANG_BASE_ATTRIBUTE: u16 = 0x0207u16;
        pub const HID_SDP_DISABLE: u16 = 0x0208u16;
        pub const HID_BATTERY_POWER: u16 = 0x0209u16;
        pub const HID_REMOTE_WAKE: u16 = 0x020au16;
        pub const HID_PROFILE_VERSION: u16 = 0x020bu16;
        pub const HID_SUPERVISION_TIMEOUT: u16 = 0x020cu16;
        pub const HID_NORMALLY_CONNECTABLE: u16 = 0x020du16;
        pub const HID_BOOT_DEVICE: u16 = 0x020eu16;
        pub const HID_SSR_HOST_MAX_LATENCY: u16 = 0x020fu16;
        pub const HID_SSR_HOST_MIN_TIMEOUT: u16 = 0x0210u16;
    }
}


