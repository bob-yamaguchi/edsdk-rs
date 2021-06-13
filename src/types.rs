use super::native::edsdk_errors::*;

#[repr(u32)]
pub enum ISOSpeed{
    ISOAuto     = 0x00000000u32,
    ISO50       = 0x00000040u32,
    ISO100      = 0x00000048u32,
    ISO125      = 0x0000004bu32,
    ISO160      = 0x0000004du32,
    ISO200      = 0x00000050u32,
    ISO250      = 0x00000053u32,
    ISO320      = 0x00000055u32,
    ISO400      = 0x00000058u32,
    ISO500      = 0x0000005bu32,
    ISO640      = 0x0000005du32,
    ISO800      = 0x00000060u32,
    ISO1000     = 0x00000063u32,
    ISO1250     = 0x00000065u32,
    ISO1600     = 0x00000068u32,
    ISO2000     = 0x0000006bu32,
    ISO2500     = 0x0000006du32,
    ISO3200     = 0x00000070u32,
    ISO4000     = 0x00000073u32,
    ISO5000     = 0x00000075u32,
    ISO6400     = 0x00000078u32,
    ISO8000     = 0x0000007bu32,
    ISO10000    = 0x0000007du32,
    ISO12800    = 0x00000080u32,
    ISO25600    = 0x00000088u32,
    ISO51200    = 0x00000090u32,
    ISO102400   = 0x00000098u32,
    ISO204800   = 0x000000a0u32,
    ISO409600   = 0x000000a8u32,
}

#[repr(u32)]
pub enum ApertureValue{
    /// Av 1.0
    Av1_0   = 0x00000008u32,
    /// Av 1.1
    Av1_1   = 0x0000000bu32,
    /// Av 1.2
    Av1_2   = 0x0000000cu32,
    /// Av 1.2(1/3)
    Av1_2a  = 0x0000000du32,
    /// Av 1.4
    Av1_4   = 0x00000010u32,
    /// Av 1.6
    Av1_6   = 0x00000013u32,
    /// Av 1.8
    Av1_8   = 0x00000014u32,
    /// Av 1.8(1/3)
    Av1_8a  = 0x00000015u32,
    /// Av 2.0
    Av2_0   = 0x00000018u32,
    /// Av 2.2
    Av2_2   = 0x0000001bu32,
    /// Av 2.5
    Av2_5   = 0x0000001cu32,
    /// Av 2.5(1/3)
    Av2_5a  = 0x0000001du32,
    /// Av 2.8
    Av2_8   = 0x00000020u32,
    /// Av 3.2
    Av3_2   = 0x00000023u32,
    /// Av 3.5
    Av3_5   = 0x00000024u32,
    /// Av 3.5(1/3)
    Av3_5a  = 0x00000025u32,
    /// Av 4.0
    Av4_0   = 0x00000028u32,
    /// Av 4.5
    Av4_5   = 0x0000002bu32,
    /// Av 4.5
    Av4_5a  = 0x0000002cu32,
    /// Av 5.0
    Av5_0   = 0x0000002du32,
    /// Av 5.6
    Av5_6   = 0x00000030u32,
    /// Av 6.3
    Av6_3   = 0x00000033u32,
    /// Av 6.7
    Av6_7   = 0x00000034u32,
    /// Av 7.1
    Av7_1   = 0x00000035u32,
    /// Av 8.0
    Av8_0   = 0x00000038u32,
    /// Av 9.0
    Av9_0   = 0x0000003bu32,
    /// Av 9.5
    Av9_5   = 0x0000003cu32,
    /// Av 10.0
    Av10_0  = 0x0000003du32,
    /// Av 11.0
    Av11_0  = 0x00000040u32,
    /// Av 13.0(1/3)
    Av13_0a = 0x00000043u32,
    /// Av 13.0
    Av13_0  = 0x00000044u32,
    /// Av 14.0
    Av14_0  = 0x00000045u32,
    /// Av 16.0
    Av16_0  = 0x00000048u32,
    /// Av 18.0
    Av18_0  = 0x0000004bu32,
    /// Av 19.0
    Av19_0  = 0x0000004cu32,
    /// Av 20.0
    Av20_0  = 0x0000004du32,
    /// Av 22.0
    Av22_0  = 0x00000050u32,
    /// Av 25.0
    Av25_0  = 0x00000053u32,
    /// Av 27.0
    Av27_0  = 0x00000054u32,
    /// Av 29.0
    Av29_0  = 0x00000055u32,
    /// Av 32.0
    Av32_0  = 0x00000058u32,
    /// Av 36.0
    Av36_0  = 0x0000005bu32,
    /// Av 38.0
    Av38_0  = 0x0000005cu32,
    /// Av 40.0
    Av40_0  = 0x0000005du32,
    /// Av 45.0
    Av45_0  = 0x00000060u32,
    /// Av 51.0
    Av51_0  = 0x00000063u32,
    /// Av 54.0
    Av54_0  = 0x00000064u32,
    /// Av 57.0
    Av57_0  = 0x00000065u32,
    /// Av 64.0
    Av64_0  = 0x00000068u32,
    /// Av 72.0
    Av72_0  = 0x0000006bu32,
    /// Av 76.0
    Av76_0  = 0x0000006cu32,
    /// Av 80.0
    Av80_0  = 0x0000006du32,
    /// Av 91.0
    Av91_0  = 0x00000070u32,
}

#[repr(u32)]
pub enum ShutterSpeed{
    /// Bulb
    TvBulb      = 0x0000000cu32,
    /// Tv 30"
    Tv30        = 0x00000010u32,
    /// Tv 25"
    Tv25        = 0x00000013u32,
    /// Tv 20"
    Tv20        = 0x00000014u32,
    /// Tv 20"(1/3)
    Tv20a       = 0x00000015u32,
    /// Tv 15"
    Tv15        = 0x00000018u32,
    /// Tv 13"
    Tv13        = 0x0000001bu32,
    /// Tv 10"
    Tv10        = 0x0000001cu32,
    /// Tv 10"(1/3)
    Tv10a       = 0x0000001du32,
    /// Tv 8"
    Tv8         = 0x00000020u32,
    /// Tv 6"(1/3)
    Tv6a        = 0x00000023u32,
    /// Tv 6"
    Tv6         = 0x00000024u32,
    /// Tv 5"
    Tv5         = 0x00000025u32,
    /// Tv 4"
    Tv4         = 0x00000028u32,
    /// Tv 3"2
    Tv3_2       = 0x0000002bu32,
    /// Tv 3"
    Tv3         = 0x0000002cu32,
    /// Tv 2"5
    Tv2_5       = 0x0000002du32,
    /// Tv 2"
    Tv2         = 0x00000030u32,
    /// Tv 1"6
    Tv1_6       = 0x00000033u32,
    /// Tv 1"5
    Tv1_5       = 0x00000034u32,
    /// Tv 1"3
    Tv1_3       = 0x00000035u32,
    /// Tv 1"
    Tv1         = 0x00000038u32,
    /// Tv 0"8
    Tv0_8       = 0x0000003bu32,
    /// Tv 0"7
    Tv0_7       = 0x0000003cu32,
    /// Tv 0"6
    Tv0_6       = 0x0000003du32,
    /// Tv 0"5
    Tv0_5       = 0x00000040u32,
    /// Tv 0"4
    Tv0_4       = 0x00000043u32,
    /// Tv 0"3
    Tv0_3       = 0x00000044u32,
    /// Tv 0"3(1/3)
    Tv0_3a      = 0x00000045u32,
    /// Tv 1/4
    Tv1_4th     = 0x00000048u32,
    /// Tv 1/5
    Tv1_5th     = 0x0000004bu32,
    /// Tv 1/6
    Tv1_6th     = 0x0000004cu32,
    /// Tv 1/6(1/3)
    Tv1_6tha    = 0x0000004du32,
    /// Tv 1/8
    Tv1_8th     = 0x00000050u32,
    /// Tv 1/10(1/3)
    Tv1_10tha   = 0x00000053u32,
    /// Tv 1/10
    Tv1_10th    = 0x00000054u32,
    /// Tv 1/13
    Tv1_13th    = 0x00000055u32,
    /// Tv 1/15
    Tv1_15th    = 0x00000058u32,
    /// Tv 1/20(1/3)
    Tv1_20tha   = 0x0000005bu32,
    /// Tv 1/20
    Tv1_20th    = 0x0000005cu32,
    /// Tv 1/25
    Tv1_25th    = 0x0000005du32,
    /// Tv 1/30
    Tv1_30th    = 0x00000060u32,
    /// Tv 1/40
    Tv1_40th    = 0x00000063u32,
    /// Tv 1/45
    Tv1_45th    = 0x00000064u32,
    /// Tv 1/50
    Tv1_50th    = 0x00000065u32,
    /// Tv 1/60
    Tv1_60th    = 0x00000068u32,
    /// Tv 1/80
    Tv1_80th    = 0x0000006bu32,
    /// Tv 1/90
    Tv1_90th    = 0x0000006cu32,
    /// Tv 1/100
    Tv1_100th   = 0x0000006du32,
    /// Tv 1/125
    Tv1_125th   = 0x00000070u32,
    /// Tv 1/160
    Tv1_160th   = 0x00000073u32,
    /// Tv 1/180
    Tv1_180th   = 0x00000074u32,
    /// Tv 1/200
    Tv1_200th   = 0x00000075u32,
    /// Tv 1/250
    Tv1_250th   = 0x00000078u32,
    /// Tv 1/320
    Tv1_320th   = 0x0000007bu32,
    /// Tv 1/350
    Tv1_350th   = 0x0000007cu32,
    /// Tv 1/400
    Tv1_400th   = 0x0000007du32,
    /// Tv 1/500
    Tv1_500th   = 0x00000080u32,
    /// Tv 1/640
    Tv1_640th   = 0x00000083u32,
    /// Tv 1/750
    Tv1_750th   = 0x00000084u32,
    /// Tv 1/800
    Tv1_800th   = 0x00000085u32,
    /// Tv 1/1000
    Tv1_1000th  = 0x00000088u32,
    /// Tv 1/1250
    Tv1_1250th  = 0x0000008bu32,
    /// Tv 1/1500
    Tv1_1500th  = 0x0000008cu32,
    /// Tv 1/1600
    Tv1_1600th  = 0x0000008du32,
    /// Tv 1/2000
    Tv1_2000th  = 0x00000090u32,
    /// Tv 1/2500
    Tv1_2500th  = 0x00000093u32,
    /// Tv 1/3000
    Tv1_3000th  = 0x00000094u32,
    /// Tv 1/3200
    Tv1_3200th  = 0x00000095u32,
    /// Tv 1/4000
    Tv1_4000th  = 0x00000098u32,
    /// Tv 1/5000
    Tv1_5000th  = 0x0000009bu32,
    /// Tv 1/6000
    Tv1_6000th  = 0x0000009cu32,
    /// Tv 1/6400
    Tv1_6400th  = 0x0000009du32,
    /// Tv 1/8000
    Tv1_8000th  = 0x000000a0u32,
}


#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum ErrorId{
    // Miscellaneous errors
    UnImplemented                   = EDS_ERR_UNIMPLEMENTED,
    InternalError                   = EDS_ERR_INTERNAL_ERROR,
    MemAllocFailed                  = EDS_ERR_MEM_ALLOC_FAILED,
    MemFreeFailed                   = EDS_ERR_MEM_FREE_FAILED,
    OperationCancelled              = EDS_ERR_OPERATION_CANCELLED,
    IncompatibleVersion             = EDS_ERR_INCOMPATIBLE_VERSION,
    NotSupported                    = EDS_ERR_NOT_SUPPORTED,
    UnexpectedException             = EDS_ERR_UNEXPECTED_EXCEPTION,
    ProtectionViolation             = EDS_ERR_PROTECTION_VIOLATION,
    MissingSubcomponent             = EDS_ERR_MISSING_SUBCOMPONENT,
    SelectionUnavailable            = EDS_ERR_SELECTION_UNAVAILABLE,
    // File errors
    FileIoError                     = EDS_ERR_FILE_IO_ERROR,
    FileTooManyOpen                 = EDS_ERR_FILE_TOO_MANY_OPEN,
    FileNotFound                    = EDS_ERR_FILE_NOT_FOUND,
    FileOpenError                   = EDS_ERR_FILE_OPEN_ERROR,
    FileCloseError                  = EDS_ERR_FILE_CLOSE_ERROR,
    FileSeekError                   = EDS_ERR_FILE_SEEK_ERROR,
    FileTellError                   = EDS_ERR_FILE_TELL_ERROR,
    FileReadError                   = EDS_ERR_FILE_READ_ERROR,
    FileWriteError                  = EDS_ERR_FILE_WRITE_ERROR,
    FilePermissionError             = EDS_ERR_FILE_PERMISSION_ERROR,
    FileDiskFullError               = EDS_ERR_FILE_DISK_FULL_ERROR,
    FileAlreadyExists               = EDS_ERR_FILE_ALREADY_EXISTS,
    FileFormatUnrecognized          = EDS_ERR_FILE_FORMAT_UNRECOGNIZED,
    FileDataCorrupt                 = EDS_ERR_FILE_DATA_CORRUPT,
    FileNamingNa                    = EDS_ERR_FILE_NAMING_NA,
    // Directory errors
    DirNotFound                     = EDS_ERR_DIR_NOT_FOUND,
    DirIOError                      = EDS_ERR_DIR_IO_ERROR,
    DirEntryNotFound                = EDS_ERR_DIR_ENTRY_NOT_FOUND,
    DirEntryExists                  = EDS_ERR_DIR_ENTRY_EXISTS,
    DirNotEmpty                     = EDS_ERR_DIR_NOT_EMPTY,
    // Property errors
    PropertiesUnavailable           = EDS_ERR_PROPERTIES_UNAVAILABLE,
    PropertiesMismatch              = EDS_ERR_PROPERTIES_MISMATCH,
    PropertiesNotLoaded             = EDS_ERR_PROPERTIES_NOT_LOADED,
    // Function Parameter errors
    InvalidParameter                = EDS_ERR_INVALID_PARAMETER,
    InvalidHandle                   = EDS_ERR_INVALID_HANDLE,
    InvalidPointer                  = EDS_ERR_INVALID_POINTER,
    InvalidIndex                    = EDS_ERR_INVALID_INDEX,
    InvalidLength                   = EDS_ERR_INVALID_LENGTH,
    InvalidFnPointer                = EDS_ERR_INVALID_FN_POINTER,
    InvalidSortFn                   = EDS_ERR_INVALID_SORT_FN,
    // Device errors
    DeviceNotFound                  = EDS_ERR_DEVICE_NOT_FOUND,
    DeviceBusy                      = EDS_ERR_DEVICE_BUSY,
    DeviceInvalid                   = EDS_ERR_DEVICE_INVALID,
    DeviceEmergency                 = EDS_ERR_DEVICE_EMERGENCY,
    DeviceMemoryFull                = EDS_ERR_DEVICE_MEMORY_FULL,
    DeviceInternalError             = EDS_ERR_DEVICE_INTERNAL_ERROR,
    DeviceInvalidParameter          = EDS_ERR_DEVICE_INVALID_PARAMETER,
    DeviceNoDisk                    = EDS_ERR_DEVICE_NO_DISK,
    DeviceDiskError                 = EDS_ERR_DEVICE_DISK_ERROR,
    DeviceCfGateChanged             = EDS_ERR_DEVICE_CF_GATE_CHANGED,
    DeviceDialChanged               = EDS_ERR_DEVICE_DIAL_CHANGED,
    DeviceNotInstalled              = EDS_ERR_DEVICE_NOT_INSTALLED,
    DeviceStayAwake                 = EDS_ERR_DEVICE_STAY_AWAKE,
    DeviceNotReleased               = EDS_ERR_DEVICE_NOT_RELEASED,
    // Stream errors
    StreamIoError                   = EDS_ERR_STREAM_IO_ERROR,
    StreamNotOpen                   = EDS_ERR_STREAM_NOT_OPEN,
    StreamAlreadyOpen               = EDS_ERR_STREAM_ALREADY_OPEN,
    StreamOpenError                 = EDS_ERR_STREAM_OPEN_ERROR,
    StreamCloseError                = EDS_ERR_STREAM_CLOSE_ERROR,
    StreamSeekError                 = EDS_ERR_STREAM_SEEK_ERROR,
    StreamTellError                 = EDS_ERR_STREAM_TELL_ERROR,
    StreamReadError                 = EDS_ERR_STREAM_READ_ERROR,
    StreamWriteError                = EDS_ERR_STREAM_WRITE_ERROR,
    StreamPermissionError           = EDS_ERR_STREAM_PERMISSION_ERROR,
    StreamCouldntBeginThread        = EDS_ERR_STREAM_COULDNT_BEGIN_THREAD,
    StreamBadOptions                = EDS_ERR_STREAM_BAD_OPTIONS,
    StreamEndOfStream               = EDS_ERR_STREAM_END_OF_STREAM,
    // Communications errors
    CommPortIsInUse                 = EDS_ERR_COMM_PORT_IS_IN_USE,
    CommDisconnected                = EDS_ERR_COMM_DISCONNECTED,
    CommDeviceIncompatible          = EDS_ERR_COMM_DEVICE_INCOMPATIBLE,
    CommBufferFull                  = EDS_ERR_COMM_BUFFER_FULL,
    CommUsbBusErr                   = EDS_ERR_COMM_USB_BUS_ERR,
    // Lock/Unlock
    UsbDeviceLockError              = EDS_ERR_USB_DEVICE_LOCK_ERROR,
    UsbDeviceUnlockError            = EDS_ERR_USB_DEVICE_UNLOCK_ERROR,
    // STI/WIA
    StiUnknownError                 = EDS_ERR_STI_UNKNOWN_ERROR,
    StiInternalError                = EDS_ERR_STI_INTERNAL_ERROR,
    StiDeviceCreateError            = EDS_ERR_STI_DEVICE_CREATE_ERROR,
    StiDeviceReleaseError           = EDS_ERR_STI_DEVICE_RELEASE_ERROR,

    DeviceNotLaunched               = EDS_ERR_DEVICE_NOT_LAUNCHED,
    EnumNa                          = EDS_ERR_ENUM_NA,
    InvalidFnCall                   = EDS_ERR_INVALID_FN_CALL,
    HandleNotFound                  = EDS_ERR_HANDLE_NOT_FOUND,
    InvalidId                       = EDS_ERR_INVALID_ID,
    WaitTimeoutError                = EDS_ERR_WAIT_TIMEOUT_ERROR,
    // PTP
    SessionNotOpen                  = EDS_ERR_SESSION_NOT_OPEN,
    InvalidTransactionId            = EDS_ERR_INVALID_TRANSACTIONID,
    IncompleteTransfer              = EDS_ERR_INCOMPLETE_TRANSFER,
    InvalidStrageId                 = EDS_ERR_INVALID_STRAGEID,
    DevicePropNotSupported          = EDS_ERR_DEVICEPROP_NOT_SUPPORTED,
    InvalidObjectFormatCode         = EDS_ERR_INVALID_OBJECTFORMATCODE,
    SelfTestFailed                  = EDS_ERR_SELF_TEST_FAILED,
    PartialDeletion                 = EDS_ERR_PARTIAL_DELETION,
    SpecificationByFormatUnsupported    = EDS_ERR_SPECIFICATION_BY_FORMAT_UNSUPPORTED,
    NoValidObjectInfo               = EDS_ERR_NO_VALID_OBJECTINFO,
    InvalidCodeFormat               = EDS_ERR_INVALID_CODE_FORMAT,
    UnknownVendorCode               = EDS_ERR_UNKNOWN_VENDOR_CODE,
    CaptureAlreadyTerminated        = EDS_ERR_CAPTURE_ALREADY_TERMINATED,
    PtpDeviceBusy                   = EDS_ERR_PTP_DEVICE_BUSY,
    InvalidParentObject             = EDS_ERR_INVALID_PARENTOBJECT,
    InvalidDevicePropFormat         = EDS_ERR_INVALID_DEVICEPROP_FORMAT,
    InvalidDevicePropValue          = EDS_ERR_INVALID_DEVICEPROP_VALUE,
    SessionAlreadyOpen              = EDS_ERR_SESSION_ALREADY_OPEN,
    TransactionCancelled            = EDS_ERR_TRANSACTION_CANCELLED,
    SpecificationOfDestinationUnsupported   = EDS_ERR_SPECIFICATION_OF_DESTINATION_UNSUPPORTED,
    NotCameraSupportSdkVersion      = EDS_ERR_NOT_CAMERA_SUPPORT_SDK_VERSION,
    // PTP Vendor
    UnknownCommand                  = EDS_ERR_UNKNOWN_COMMAND,
    OperationRefused                = EDS_ERR_OPERATION_REFUSED,
    LensCoverClose                  = EDS_ERR_LENS_COVER_CLOSE,
    LowBattery                      = EDS_ERR_LOW_BATTERY,
    ObjectNotReady                  = EDS_ERR_OBJECT_NOTREADY,
    CannotMakeObject                = EDS_ERR_CANNOT_MAKE_OBJECT,
    MemoryStatusNotReady            = EDS_ERR_MEMORYSTATUS_NOTREADY,
    // Take Picture errors
    TakePictureAfNg                 = EDS_ERR_TAKE_PICTURE_AF_NG,
    TakePictureReserved             = EDS_ERR_TAKE_PICTURE_RESERVED,
    TakePictureMirrorUpNg           = EDS_ERR_TAKE_PICTURE_MIRROR_UP_NG,
    TakePictureSensorCleaningNg     = EDS_ERR_TAKE_PICTURE_SENSOR_CLEANING_NG,
    TakePictureSilenceNg            = EDS_ERR_TAKE_PICTURE_SILENCE_NG,
    TakePictureNoCardNg             = EDS_ERR_TAKE_PICTURE_NO_CARD_NG,
    TakePictureCardNg               = EDS_ERR_TAKE_PICTURE_CARD_NG,
    TakePictureCardProtectNg        = EDS_ERR_TAKE_PICTURE_CARD_PROTECT_NG,
    TakePictureMovieCropNg          = EDS_ERR_TAKE_PICTURE_MOVIE_CROP_NG,
    TakePictureStroboChargeNg       = EDS_ERR_TAKE_PICTURE_STROBO_CHARGE_NG,
    TakePictureNoLensNg             = EDS_ERR_TAKE_PICTURE_NO_LENS_NG,
    TakePictureSpecialMovieModeNg   = EDS_ERR_TAKE_PICTURE_SPECIAL_MOVIE_MODE_NG,
    TakePictureLvRelProhibitModeNg  = EDS_ERR_TAKE_PICTURE_LV_REL_PROHIBIT_MODE_NG,

    LastGenericErrorPlusOne         = EDS_ERR_LAST_GENERIC_ERROR_PLUS_ONE
}
