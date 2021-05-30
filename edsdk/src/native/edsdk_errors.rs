
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]


// Definition of error Codes

// ED-SDK Error Code Masks
pub const EDS_ISSPECIFIC_MASK   : u32 = 0x80000000u32;
pub const EDS_COMPONENTID_MASK  : u32 = 0x7F000000u32;
pub const EDS_RESERVED_MASK     : u32 = 0x00FF0000u32;
pub const EDS_ERRORID_MASK      : u32 = 0x0000FFFFu32;

// ED-SDK Base Component IDs
pub const EDS_CMP_ID_CLIENT_COMPONENTID : u32 = 0x01000000u32;
pub const EDS_CMP_ID_LLSDK_COMPONENTID  : u32 = 0x02000000u32;
pub const EDS_CMP_ID_HLSDK_COMPONENTID  : u32 = 0x03000000u32;

// ED-SDK Functin Success Code
pub const EDS_ERR_OK    : u32 = 0x00000000u32;

// ED-SDK Generic Error IDs
/* Miscellaneous errors */
pub const EDS_ERR_UNIMPLEMENTED         : u32 = 0x00000001u32;
pub const EDS_ERR_INTERNAL_ERROR        : u32 = 0x00000002u32;
pub const EDS_ERR_MEM_ALLOC_FAILED      : u32 = 0x00000003u32;
pub const EDS_ERR_MEM_FREE_FAILED       : u32 = 0x00000004u32;
pub const EDS_ERR_OPERATION_CANCELLED   : u32 = 0x00000005u32;
pub const EDS_ERR_INCOMPATIBLE_VERSION  : u32 = 0x00000006u32;
pub const EDS_ERR_NOT_SUPPORTED         : u32 = 0x00000007u32;
pub const EDS_ERR_UNEXPECTED_EXCEPTION  : u32 = 0x00000008u32;
pub const EDS_ERR_PROTECTION_VIOLATION  : u32 = 0x00000009u32;
pub const EDS_ERR_MISSING_SUBCOMPONENT  : u32 = 0x0000000Au32;
pub const EDS_ERR_SELECTION_UNAVAILABLE : u32 = 0x0000000Bu32;

/* File errors */
pub const EDS_ERR_FILE_IO_ERROR             : u32 = 0x00000020u32;
pub const EDS_ERR_FILE_TOO_MANY_OPEN        : u32 = 0x00000021u32;
pub const EDS_ERR_FILE_NOT_FOUND            : u32 = 0x00000022u32;
pub const EDS_ERR_FILE_OPEN_ERROR           : u32 = 0x00000023u32;
pub const EDS_ERR_FILE_CLOSE_ERROR          : u32 = 0x00000024u32;
pub const EDS_ERR_FILE_SEEK_ERROR           : u32 = 0x00000025u32;
pub const EDS_ERR_FILE_TELL_ERROR           : u32 = 0x00000026u32;
pub const EDS_ERR_FILE_READ_ERROR           : u32 = 0x00000027u32;
pub const EDS_ERR_FILE_WRITE_ERROR          : u32 = 0x00000028u32;
pub const EDS_ERR_FILE_PERMISSION_ERROR     : u32 = 0x00000029u32;
pub const EDS_ERR_FILE_DISK_FULL_ERROR      : u32 = 0x0000002Au32;
pub const EDS_ERR_FILE_ALREADY_EXISTS       : u32 = 0x0000002Bu32;
pub const EDS_ERR_FILE_FORMAT_UNRECOGNIZED  : u32 = 0x0000002Cu32;
pub const EDS_ERR_FILE_DATA_CORRUPT         : u32 = 0x0000002Du32;
pub const EDS_ERR_FILE_NAMING_NA            : u32 = 0x0000002Eu32;

/* Directory errors */
pub const EDS_ERR_DIR_NOT_FOUND         : u32 = 0x00000040u32;
pub const EDS_ERR_DIR_IO_ERROR          : u32 = 0x00000041u32;
pub const EDS_ERR_DIR_ENTRY_NOT_FOUND   : u32 = 0x00000042u32;
pub const EDS_ERR_DIR_ENTRY_EXISTS      : u32 = 0x00000043u32;
pub const EDS_ERR_DIR_NOT_EMPTY         : u32 = 0x00000044u32;

/* Property errors */
pub const EDS_ERR_PROPERTIES_UNAVAILABLE    : u32 = 0x00000050u32;
pub const EDS_ERR_PROPERTIES_MISMATCH       : u32 = 0x00000051u32;
pub const EDS_ERR_PROPERTIES_NOT_LOADED     : u32 = 0x00000053u32;

/* Function Parameter errors */
pub const EDS_ERR_INVALID_PARAMETER     : u32 = 0x00000060u32;
pub const EDS_ERR_INVALID_HANDLE        : u32 = 0x00000061u32;
pub const EDS_ERR_INVALID_POINTER       : u32 = 0x00000062u32;
pub const EDS_ERR_INVALID_INDEX         : u32 = 0x00000063u32;
pub const EDS_ERR_INVALID_LENGTH        : u32 = 0x00000064u32;
pub const EDS_ERR_INVALID_FN_POINTER    : u32 = 0x00000065u32;
pub const EDS_ERR_INVALID_SORT_FN       : u32 = 0x00000066u32;

/* Device errors */
pub const EDS_ERR_DEVICE_NOT_FOUND          : u32 = 0x00000080u32;
pub const EDS_ERR_DEVICE_BUSY               : u32 = 0x00000081u32;
pub const EDS_ERR_DEVICE_INVALID            : u32 = 0x00000082u32;
pub const EDS_ERR_DEVICE_EMERGENCY          : u32 = 0x00000083u32;
pub const EDS_ERR_DEVICE_MEMORY_FULL        : u32 = 0x00000084u32;
pub const EDS_ERR_DEVICE_INTERNAL_ERROR     : u32 = 0x00000085u32;
pub const EDS_ERR_DEVICE_INVALID_PARAMETER  : u32 = 0x00000086u32;
pub const EDS_ERR_DEVICE_NO_DISK            : u32 = 0x00000087u32;
pub const EDS_ERR_DEVICE_DISK_ERROR         : u32 = 0x00000088u32;
pub const EDS_ERR_DEVICE_CF_GATE_CHANGED    : u32 = 0x00000089u32;
pub const EDS_ERR_DEVICE_DIAL_CHANGED       : u32 = 0x0000008Au32;
pub const EDS_ERR_DEVICE_NOT_INSTALLED      : u32 = 0x0000008Bu32;
pub const EDS_ERR_DEVICE_STAY_AWAKE         : u32 = 0x0000008Cu32;
pub const EDS_ERR_DEVICE_NOT_RELEASED       : u32 = 0x0000008Du32;

/* Stream errors */
pub const EDS_ERR_STREAM_IO_ERROR               : u32 = 0x000000A0u32;
pub const EDS_ERR_STREAM_NOT_OPEN               : u32 = 0x000000A1u32;
pub const EDS_ERR_STREAM_ALREADY_OPEN           : u32 = 0x000000A2u32;
pub const EDS_ERR_STREAM_OPEN_ERROR             : u32 = 0x000000A3u32;
pub const EDS_ERR_STREAM_CLOSE_ERROR            : u32 = 0x000000A4u32;
pub const EDS_ERR_STREAM_SEEK_ERROR             : u32 = 0x000000A5u32;
pub const EDS_ERR_STREAM_TELL_ERROR             : u32 = 0x000000A6u32;
pub const EDS_ERR_STREAM_READ_ERROR             : u32 = 0x000000A7u32;
pub const EDS_ERR_STREAM_WRITE_ERROR            : u32 = 0x000000A8u32;
pub const EDS_ERR_STREAM_PERMISSION_ERROR       : u32 = 0x000000A9u32;
pub const EDS_ERR_STREAM_COULDNT_BEGIN_THREAD   : u32 = 0x000000AAu32;
pub const EDS_ERR_STREAM_BAD_OPTIONS            : u32 = 0x000000ABu32;
pub const EDS_ERR_STREAM_END_OF_STREAM          : u32 = 0x000000ACu32;

/* Communications errors */
pub const EDS_ERR_COMM_PORT_IS_IN_USE       : u32 = 0x000000C0u32;
pub const EDS_ERR_COMM_DISCONNECTED         : u32 = 0x000000C1u32;
pub const EDS_ERR_COMM_DEVICE_INCOMPATIBLE  : u32 = 0x000000C2u32;
pub const EDS_ERR_COMM_BUFFER_FULL          : u32 = 0x000000C3u32;
pub const EDS_ERR_COMM_USB_BUS_ERR          : u32 = 0x000000C4u32;

/* Lock/Unlock */
pub const EDS_ERR_USB_DEVICE_LOCK_ERROR     : u32 = 0x000000D0u32;
pub const EDS_ERR_USB_DEVICE_UNLOCK_ERROR   : u32 = 0x000000D1u32;

/* STI/WIA */
pub const EDS_ERR_STI_UNKNOWN_ERROR         : u32 = 0x000000E0u32;
pub const EDS_ERR_STI_INTERNAL_ERROR        : u32 = 0x000000E1u32;
pub const EDS_ERR_STI_DEVICE_CREATE_ERROR   : u32 = 0x000000E2u32;
pub const EDS_ERR_STI_DEVICE_RELEASE_ERROR  : u32 = 0x000000E3u32;
pub const EDS_ERR_DEVICE_NOT_LAUNCHED       : u32 = 0x000000E4u32;
    
pub const EDS_ERR_ENUM_NA               : u32 = 0x000000F0u32;
pub const EDS_ERR_INVALID_FN_CALL       : u32 = 0x000000F1u32;
pub const EDS_ERR_HANDLE_NOT_FOUND      : u32 = 0x000000F2u32;
pub const EDS_ERR_INVALID_ID            : u32 = 0x000000F3u32;
pub const EDS_ERR_WAIT_TIMEOUT_ERROR    : u32 = 0x000000F4u32;

/* PTP */
pub const EDS_ERR_SESSION_NOT_OPEN                          : u32 = 0x00002003;
pub const EDS_ERR_INVALID_TRANSACTIONID                     : u32 = 0x00002004;
pub const EDS_ERR_INCOMPLETE_TRANSFER                       : u32 = 0x00002007;
pub const EDS_ERR_INVALID_STRAGEID                          : u32 = 0x00002008;
pub const EDS_ERR_DEVICEPROP_NOT_SUPPORTED                  : u32 = 0x0000200A;
pub const EDS_ERR_INVALID_OBJECTFORMATCODE                  : u32 = 0x0000200B;
pub const EDS_ERR_SELF_TEST_FAILED                          : u32 = 0x00002011;
pub const EDS_ERR_PARTIAL_DELETION                          : u32 = 0x00002012;
pub const EDS_ERR_SPECIFICATION_BY_FORMAT_UNSUPPORTED       : u32 = 0x00002014;
pub const EDS_ERR_NO_VALID_OBJECTINFO                       : u32 = 0x00002015;
pub const EDS_ERR_INVALID_CODE_FORMAT                       : u32 = 0x00002016;
pub const EDS_ERR_UNKNOWN_VENDOR_CODE                       : u32 = 0x00002017;
pub const EDS_ERR_CAPTURE_ALREADY_TERMINATED                : u32 = 0x00002018;
pub const EDS_ERR_PTP_DEVICE_BUSY                           : u32 = 0x00002019;
pub const EDS_ERR_INVALID_PARENTOBJECT                      : u32 = 0x0000201A;
pub const EDS_ERR_INVALID_DEVICEPROP_FORMAT                 : u32 = 0x0000201B;
pub const EDS_ERR_INVALID_DEVICEPROP_VALUE                  : u32 = 0x0000201C;
pub const EDS_ERR_SESSION_ALREADY_OPEN                      : u32 = 0x0000201E;
pub const EDS_ERR_TRANSACTION_CANCELLED                     : u32 = 0x0000201F;
pub const EDS_ERR_SPECIFICATION_OF_DESTINATION_UNSUPPORTED  : u32 = 0x00002020;
pub const EDS_ERR_NOT_CAMERA_SUPPORT_SDK_VERSION            : u32 = 0x00002021;

/* PTP Vendor */
pub const EDS_ERR_UNKNOWN_COMMAND       : u32 = 0x0000A001;
pub const EDS_ERR_OPERATION_REFUSED     : u32 = 0x0000A005;
pub const EDS_ERR_LENS_COVER_CLOSE      : u32 = 0x0000A006;
pub const EDS_ERR_LOW_BATTERY           : u32 = 0x0000A101;
pub const EDS_ERR_OBJECT_NOTREADY       : u32 = 0x0000A102;
pub const EDS_ERR_CANNOT_MAKE_OBJECT    : u32 = 0x0000A104;
pub const EDS_ERR_MEMORYSTATUS_NOTREADY : u32 = 0x0000A106;

/* Take Picture errors */ 
pub const EDS_ERR_TAKE_PICTURE_AF_NG                    : u32 = 0x00008D01u32;
pub const EDS_ERR_TAKE_PICTURE_RESERVED                 : u32 = 0x00008D02u32;
pub const EDS_ERR_TAKE_PICTURE_MIRROR_UP_NG             : u32 = 0x00008D03u32;
pub const EDS_ERR_TAKE_PICTURE_SENSOR_CLEANING_NG       : u32 = 0x00008D04u32;
pub const EDS_ERR_TAKE_PICTURE_SILENCE_NG               : u32 = 0x00008D05u32;
pub const EDS_ERR_TAKE_PICTURE_NO_CARD_NG               : u32 = 0x00008D06u32;
pub const EDS_ERR_TAKE_PICTURE_CARD_NG                  : u32 = 0x00008D07u32;
pub const EDS_ERR_TAKE_PICTURE_CARD_PROTECT_NG          : u32 = 0x00008D08u32;
pub const EDS_ERR_TAKE_PICTURE_MOVIE_CROP_NG            : u32 = 0x00008D09u32;
pub const EDS_ERR_TAKE_PICTURE_STROBO_CHARGE_NG         : u32 = 0x00008D0Au32;
pub const EDS_ERR_TAKE_PICTURE_NO_LENS_NG               : u32 = 0x00008D0Bu32;
pub const EDS_ERR_TAKE_PICTURE_SPECIAL_MOVIE_MODE_NG    : u32 = 0x00008D0Cu32;
pub const EDS_ERR_TAKE_PICTURE_LV_REL_PROHIBIT_MODE_NG  : u32 = 0x00008D0Du32;


pub const EDS_ERR_LAST_GENERIC_ERROR_PLUS_ONE   : u32 = 0x000000F5u32;



