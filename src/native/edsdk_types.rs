
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::ffi::c_void;

//  Definition of Constants
pub const EDS_MAX_NAME              : usize = 256;
pub const EDS_TRANSFER_BLOCK_SIZE   : usize = 512;

/// Error Types
pub type EdsError = u32;

pub type EdsBaseRef = *const c_void;

pub type EdsCameraListRef = EdsBaseRef;
pub type EdsCameraRef = EdsBaseRef;
pub type EdsVolumeRef = EdsBaseRef;
pub type EdsDirectoryItemRef = EdsBaseRef;

pub type EdsStreamRef = EdsBaseRef;
pub type EdsImageRef = EdsStreamRef;

pub type EdsEvfImageRef = EdsBaseRef;

/// Data Types
#[repr(u32)]
pub enum EdsDataType
{
    kEdsDataType_Unknown           = 0u32,
    kEdsDataType_Bool              = 1u32,
    kEdsDataType_String            = 2u32,
    kEdsDataType_Int8              = 3u32,
    kEdsDataType_UInt8             = 6u32,
    kEdsDataType_Int16             = 4u32,
    kEdsDataType_UInt16            = 7u32,
    kEdsDataType_Int32             = 8u32,
    kEdsDataType_UInt32            = 9u32,
    kEdsDataType_Int64             = 10u32,
    kEdsDataType_UInt64            = 11u32,
    kEdsDataType_Float             = 12u32,
    kEdsDataType_Double            = 13u32,
    kEdsDataType_ByteBlock         = 14u32,
    kEdsDataType_Rational          = 20u32,
    kEdsDataType_Point             = 21u32,
    kEdsDataType_Rect              = 22u32,
    kEdsDataType_Time              = 23u32,

    kEdsDataType_Bool_Array        = 30u32,
    kEdsDataType_Int8_Array        = 31u32,
    kEdsDataType_Int16_Array       = 32u32,
    kEdsDataType_Int32_Array       = 33u32,
    kEdsDataType_UInt8_Array       = 34u32,
    kEdsDataType_UInt16_Array      = 35u32,
    kEdsDataType_UInt32_Array      = 36u32,
    kEdsDataType_Rational_Array    = 37u32,

    kEdsDataType_FocusInfo         = 101u32,
    kEdsDataType_PictureStyleDesc,
}

/// Property IDs
pub type EdsPropertyID = u32;

// Camera Setting Properties
pub const kEdsPropID_Unknown                : EdsPropertyID = 0x0000ffffu32;
pub const kEdsPropID_ProductName            : EdsPropertyID = 0x00000002u32;
pub const kEdsPropID_OwnerName              : EdsPropertyID = 0x00000004u32;
pub const kEdsPropID_MakerName              : EdsPropertyID = 0x00000005u32;
pub const kEdsPropID_DateTime               : EdsPropertyID = 0x00000006u32;
pub const kEdsPropID_FirmwareVersion        : EdsPropertyID = 0x00000007u32;
pub const kEdsPropID_BatteryLevel           : EdsPropertyID = 0x00000008u32;
pub const kEdsPropID_CFn                    : EdsPropertyID = 0x00000009u32;
pub const kEdsPropID_SaveTo                 : EdsPropertyID = 0x0000000bu32;
pub const kEdsPropID_CurrentStorage         : EdsPropertyID = 0x0000000cu32;
pub const kEdsPropID_CurrentFolder          : EdsPropertyID = 0x0000000du32;
pub const kEdsPropID_MyMenu                 : EdsPropertyID = 0x0000000eu32;

pub const kEdsPropID_BatteryQuality         : EdsPropertyID = 0x00000010u32;

pub const kEdsPropID_BodyIDEx               : EdsPropertyID = 0x00000015u32;
pub const kEdsPropID_HDDirectoryStructure   : EdsPropertyID = 0x00000020u32;

// Image Properties
pub const kEdsPropID_ImageQuality           : EdsPropertyID = 0x00000100u32;
pub const kEdsPropID_JpegQuality            : EdsPropertyID = 0x00000101u32;
pub const kEdsPropID_Orientation            : EdsPropertyID = 0x00000102u32;
pub const kEdsPropID_ICCProfile             : EdsPropertyID = 0x00000103u32;
pub const kEdsPropID_FocusInfo              : EdsPropertyID = 0x00000104u32;
pub const kEdsPropID_DigitalExposure        : EdsPropertyID = 0x00000105u32;
pub const kEdsPropID_WhiteBalance           : EdsPropertyID = 0x00000106u32;
pub const kEdsPropID_ColorTemperature       : EdsPropertyID = 0x00000107u32;
pub const kEdsPropID_WhiteBalanceShift      : EdsPropertyID = 0x00000108u32;
pub const kEdsPropID_Contrast               : EdsPropertyID = 0x00000109u32;
pub const kEdsPropID_ColorSaturation        : EdsPropertyID = 0x0000010au32;
pub const kEdsPropID_ColorTone              : EdsPropertyID = 0x0000010bu32;
pub const kEdsPropID_Sharpness              : EdsPropertyID = 0x0000010cu32;
pub const kEdsPropID_ColorSpace             : EdsPropertyID = 0x0000010du32;
pub const kEdsPropID_ToneCurve              : EdsPropertyID = 0x0000010eu32;
pub const kEdsPropID_PhotoEffect            : EdsPropertyID = 0x0000010fu32;
pub const kEdsPropID_FilterEffect           : EdsPropertyID = 0x00000110u32;
pub const kEdsPropID_ToningEffect           : EdsPropertyID = 0x00000111u32;
pub const kEdsPropID_ParameterSet           : EdsPropertyID = 0x00000112u32;
pub const kEdsPropID_ColorMatrix            : EdsPropertyID = 0x00000113u32;
pub const kEdsPropID_PictureStyle           : EdsPropertyID = 0x00000114u32;
pub const kEdsPropID_PictureStyleDesc       : EdsPropertyID = 0x00000115u32;
pub const kEdsPropID_PictureStyleCaption    : EdsPropertyID = 0x00000200u32;

// Image Processing Properties
pub const kEdsPropID_Linear         : EdsPropertyID = 0x00000300u32;
pub const kEdsPropID_ClickWBPoint   : EdsPropertyID = 0x00000301u32;
pub const kEdsPropID_WBCoeffs       : EdsPropertyID = 0x00000302u32;

// Image GPS Properties
pub const kEdsPropID_GPSVersionID       : EdsPropertyID = 0x00000800u32;
pub const kEdsPropID_GPSLatitudeRef     : EdsPropertyID = 0x00000801u32;
pub const kEdsPropID_GPSLatitude        : EdsPropertyID = 0x00000802u32;
pub const kEdsPropID_GPSLongitudeRef    : EdsPropertyID = 0x00000803u32;
pub const kEdsPropID_GPSLongitude       : EdsPropertyID = 0x00000804u32;
pub const kEdsPropID_GPSAltitudeRef     : EdsPropertyID = 0x00000805u32;
pub const kEdsPropID_GPSAltitude        : EdsPropertyID = 0x00000806u32;
pub const kEdsPropID_GPSTimeStamp       : EdsPropertyID = 0x00000807u32;
pub const kEdsPropID_GPSSatellites      : EdsPropertyID = 0x00000808u32;
pub const kEdsPropID_GPSStatus          : EdsPropertyID = 0x00000809u32;
pub const kEdsPropID_GPSMapDatum        : EdsPropertyID = 0x00000812u32;
pub const kEdsPropID_GPSDateStamp       : EdsPropertyID = 0x0000081Du32;

// Property Mask
pub const kEdsPropID_AtCapture_Flag : EdsPropertyID = 0x80000000u32;

// Capture Properties
pub const kEdsPropID_AEMode                 : EdsPropertyID = 0x00000400u32;
pub const kEdsPropID_DriveMode              : EdsPropertyID = 0x00000401u32;
pub const kEdsPropID_ISOSpeed               : EdsPropertyID = 0x00000402u32;
pub const kEdsPropID_MeteringMode           : EdsPropertyID = 0x00000403u32;
pub const kEdsPropID_AFMode                 : EdsPropertyID = 0x00000404u32;
pub const kEdsPropID_Av                     : EdsPropertyID = 0x00000405u32;
pub const kEdsPropID_Tv                     : EdsPropertyID = 0x00000406u32;
pub const kEdsPropID_ExposureCompensation   : EdsPropertyID = 0x00000407u32;
pub const kEdsPropID_FlashCompensation      : EdsPropertyID = 0x00000408u32;
pub const kEdsPropID_FocalLength            : EdsPropertyID = 0x00000409u32;
pub const kEdsPropID_AvailableShots         : EdsPropertyID = 0x0000040au32;
pub const kEdsPropID_Bracket                : EdsPropertyID = 0x0000040bu32;
pub const kEdsPropID_WhiteBalanceBracket    : EdsPropertyID = 0x0000040cu32;
pub const kEdsPropID_LensName               : EdsPropertyID = 0x0000040du32;
pub const kEdsPropID_AEBracket              : EdsPropertyID = 0x0000040eu32;
pub const kEdsPropID_FEBracket              : EdsPropertyID = 0x0000040fu32;
pub const kEdsPropID_ISOBracket             : EdsPropertyID = 0x00000410u32;
pub const kEdsPropID_NoiseReduction         : EdsPropertyID = 0x00000411u32;
pub const kEdsPropID_FlashOn                : EdsPropertyID = 0x00000412u32;
pub const kEdsPropID_RedEye                 : EdsPropertyID = 0x00000413u32;
pub const kEdsPropID_FlashMode              : EdsPropertyID = 0x00000414u32;
pub const kEdsPropID_LensStatus             : EdsPropertyID = 0x00000416u32;
pub const kEdsPropID_Artist                 : EdsPropertyID = 0x00000418u32;
pub const kEdsPropID_Copyright              : EdsPropertyID = 0x00000419u32;
pub const kEdsPropID_DepthOfField           : EdsPropertyID = 0x0000041bu32;
pub const kEdsPropID_EFCompensation         : EdsPropertyID = 0x0000041eu32;
pub const kEdsPropID_AEModeSelect           : EdsPropertyID = 0x00000436u32;

// EVF Properties
pub const kEdsPropID_Evf_OutputDevice           : EdsPropertyID = 0x00000500u32;
pub const kEdsPropID_Evf_Mode                   : EdsPropertyID = 0x00000501u32;
pub const kEdsPropID_Evf_WhiteBalance           : EdsPropertyID = 0x00000502u32;
pub const kEdsPropID_Evf_ColorTemperature       : EdsPropertyID = 0x00000503u32;
pub const kEdsPropID_Evf_DepthOfFieldPreview    : EdsPropertyID = 0x00000504u32;

// EVF IMAGE DATA Properties
pub const kEdsPropID_Evf_Zoom               : EdsPropertyID = 0x00000507u32;
pub const kEdsPropID_Evf_ZoomPosition       : EdsPropertyID = 0x00000508u32;
pub const kEdsPropID_Evf_FocusAid           : EdsPropertyID = 0x00000509u32;
pub const kEdsPropID_Evf_Histogram          : EdsPropertyID = 0x0000050Au32;
pub const kEdsPropID_Evf_ImagePosition      : EdsPropertyID = 0x0000050Bu32;
pub const kEdsPropID_Evf_HistogramStatus    : EdsPropertyID = 0x0000050Cu32;
pub const kEdsPropID_Evf_AFMode             : EdsPropertyID = 0x0000050Eu32;

pub const kEdsPropID_Record                 : EdsPropertyID = 0x00000510u32;

pub const kEdsPropID_Evf_HistogramY         : EdsPropertyID = 0x00000515u32;
pub const kEdsPropID_Evf_HistogramR         : EdsPropertyID = 0x00000516u32;
pub const kEdsPropID_Evf_HistogramG         : EdsPropertyID = 0x00000517u32;
pub const kEdsPropID_Evf_HistogramB         : EdsPropertyID = 0x00000518u32;

pub const kEdsPropID_Evf_CoordinateSystem   : EdsPropertyID = 0x00000540u32;
pub const kEdsPropID_Evf_ZoomRect           : EdsPropertyID = 0x00000541u32;
pub const kEdsPropID_Evf_ImageClipRect      : EdsPropertyID = 0x00000545u32;

// Camera Commands
pub type EdsCameraCommand = u32;

// Send Commands
pub const kEdsCameraCommand_TakePicture         : EdsCameraCommand = 0x00000000u32;
pub const kEdsCameraCommand_ExtendShutDownTimer : EdsCameraCommand = 0x00000001u32;
pub const kEdsCameraCommand_BulbStart           : EdsCameraCommand = 0x00000002u32;
pub const kEdsCameraCommand_BulbEnd             : EdsCameraCommand = 0x00000003u32;
pub const kEdsCameraCommand_DoEvfAf             : EdsCameraCommand = 0x00000102u32;
pub const kEdsCameraCommand_DriveLensEvf        : EdsCameraCommand = 0x00000103u32;
pub const kEdsCameraCommand_DoClickWBEvf        : EdsCameraCommand = 0x00000104u32;

pub const kEdsCameraCommand_PressShutterButton  : EdsCameraCommand = 0x00000004u32;

#[repr(i32)]
pub enum EdsEvfAf
{
	kEdsCameraCommand_EvfAf_OFF    = 0i32,
	kEdsCameraCommand_EvfAf_ON     = 1i32,
}

#[repr(i32)]
pub enum EdsShutterButton
{
	kEdsCameraCommand_ShutterButton_OFF                = 0x00000000i32,
	kEdsCameraCommand_ShutterButton_Halfway            = 0x00000001i32,
	kEdsCameraCommand_ShutterButton_Completely         = 0x00000003i32,
	kEdsCameraCommand_ShutterButton_Halfway_NonAF      = 0x00010001i32,
	kEdsCameraCommand_ShutterButton_Completely_NonAF   = 0x00010003i32,
}

pub type EdsCameraStatusCommand = u32;

// Camera Status Commands
pub const kEdsCameraStatusCommand_UILock                : EdsCameraStatusCommand = 0x00000000u32;
pub const kEdsCameraStatusCommand_UIUnLock              : EdsCameraStatusCommand = 0x00000001u32;
pub const kEdsCameraStatusCommand_EnterDirectTransfer   : EdsCameraStatusCommand = 0x00000002u32;
pub const kEdsCameraStatusCommand_ExitDirectTransfer    : EdsCameraStatusCommand = 0x00000003u32;

// Camera Events

/// Property Event
pub type EdsPropertyEvent = u32;

/// Notifies all property events.
pub const kEdsPropertyEvent_All : EdsPropertyEvent = 0x00000100u32;

/// Notifies that a camera property value has been changed. 
/// The changed property can be retrieved from event data. 
/// The changed value can be retrieved by means of EdsGetPropertyData. 
/// In the case of type 1 protocol standard cameras, 
/// notification of changed properties can only be issued for custom functions (CFn). 
/// If the property type is 0x0000FFFF, the changed property cannot be identified. 
/// Thus, retrieve all required properties repeatedly.
pub const kEdsPropertyEvent_PropertyChanged : EdsPropertyEvent = 0x00000101u32;

/// Notifies of changes in the list of camera properties with configurable values. 
/// The list of configurable values for property IDs indicated in event data 
/// can be retrieved by means of EdsGetPropertyDesc. 
/// For type 1 protocol standard cameras, the property ID is identified as "Unknown"
/// during notification. 
/// Thus, you must retrieve a list of configurable values for all properties and
/// retrieve the property values repeatedly. 
/// (For details on properties for which you can retrieve a list of configurable
/// properties, 
/// see the description of EdsGetPropertyDesc).
pub const kEdsPropertyEvent_PropertyDescChanged : EdsPropertyEvent = 0x00000102u32;

/// Object Event
pub type EdsObjectEvent = u32;

/// Notifies all object events.
pub const kEdsObjectEvent_All : EdsObjectEvent = 0x00000200u32;

/// Notifies that the volume object (memory card) state (VolumeInfo)
/// has been changed. 
/// Changed objects are indicated by event data. 
/// The changed value can be retrieved by means of EdsGetVolumeInfo. 
/// Notification of this event is not issued for type 1 protocol standard cameras.
pub const kEdsObjectEvent_VolumeInfoChanged : EdsObjectEvent = 0x00000201u32;

/// Notifies if the designated volume on a camera has been formatted.
/// If notification of this event is received, get sub-items of the designated
/// volume again as needed. 
/// Changed volume objects can be retrieved from event data. 
/// Objects cannot be identified on cameras earlier than the D30
/// if files are added or deleted. 
/// Thus, these events are subject to notification.
pub const kEdsObjectEvent_VolumeUpdateItems : EdsObjectEvent = 0x00000202u32;

/// Notifies if many images are deleted in a designated folder on a camera.
/// If notification of this event is received, get sub-items of the designated
/// folder again as needed. 
/// Changed folders (specifically, directory item objects) can be retrieved
/// from event data.
pub const kEdsObjectEvent_FolderUpdateItems : EdsObjectEvent = 0x00000203u32;

/// Notifies of the creation of objects such as new folders or files
/// on a camera compact flash card or the like. 
/// This event is generated if the camera has been set to store captured
/// images simultaneously on the camera and a computer,
/// for example, but not if the camera is set to store images
/// on the computer alone. 
/// Newly created objects are indicated by event data. 
/// Because objects are not indicated for type 1 protocol standard cameras,
/// (that is, objects are indicated as NULL),
/// you must again retrieve child objects under the camera object to 
/// identify the new objects.
pub const kEdsObjectEvent_DirItemCreated : EdsObjectEvent = 0x00000204u32;

/// Notifies of the deletion of objects such as folders or files on a camera
/// compact flash card or the like. 
/// Deleted objects are indicated in event data. 
/// Because objects are not indicated for type 1 protocol standard cameras, 
/// you must again retrieve child objects under the camera object to
/// identify deleted objects.
pub const kEdsObjectEvent_DirItemRemoved : EdsObjectEvent = 0x00000205u32;

/// Notifies that information of DirItem objects has been changed. 
/// Changed objects are indicated by event data. 
/// The changed value can be retrieved by means of EdsGetDirectoryItemInfo. 
/// Notification of this event is not issued for type 1 protocol standard cameras.
pub const kEdsObjectEvent_DirItemInfoChanged : EdsObjectEvent = 0x00000206u32;

/// Notifies that header information has been updated, as for rotation information
/// of image files on the camera. 
/// If this event is received, get the file header information again, as needed. 
/// This function is for type 2 protocol standard cameras only.
pub const kEdsObjectEvent_DirItemContentChanged : EdsObjectEvent = 0x00000207u32;

/// Notifies that there are objects on a camera to be transferred to a computer. 
/// This event is generated after remote release from a computer or local release
/// from a camera. 
/// If this event is received, objects indicated in the event data must be downloaded.
/// Furthermore, if the application does not require the objects, instead
/// of downloading them,
/// execute EdsDownloadCancel and release resources held by the camera. 
/// The order of downloading from type 1 protocol standard cameras must be the order
/// in which the events are received.
pub const kEdsObjectEvent_DirItemRequestTransfer : EdsObjectEvent = 0x00000208u32;

/// Notifies if the camera's direct transfer button is pressed. 
/// If this event is received, objects indicated in the event data must be downloaded. 
/// Furthermore, if the application does not require the objects, instead of
/// downloading them, 
/// execute EdsDownloadCancel and release resources held by the camera. 
/// Notification of this event is not issued for type 1 protocol standard cameras.
pub const kEdsObjectEvent_DirItemRequestTransferDT : EdsObjectEvent = 0x00000209u32;

/// Notifies of requests from a camera to cancel object transfer 
/// if the button to cancel direct transfer is pressed on the camera. 
/// If the parameter is 0, it means that cancellation of transfer is requested for
/// objects still not downloaded,
/// with these objects indicated by kEdsObjectEvent_DirItemRequestTransferDT. 
/// Notification of this event is not issued for type 1 protocol standard cameras.
pub const kEdsObjectEvent_DirItemCancelTransferDT : EdsObjectEvent = 0x0000020au32;

pub const kEdsObjectEvent_VolumeAdded   : EdsObjectEvent = 0x0000020cu32;
pub const kEdsObjectEvent_VolumeRemoved : EdsObjectEvent = 0x0000020du32;

///  State Event
pub type EdsStateEvent = u32;

/// Notifies all state events.
pub const kEdsStateEvent_All : EdsStateEvent = 0x00000300u32;

/// Indicates that a camera is no longer connected to a computer, 
/// whether it was disconnected by unplugging a cord, opening
/// the compact flash compartment, 
/// turning the camera off, auto shut-off, or by other means.
pub const kEdsStateEvent_Shutdown : EdsStateEvent = 0x00000301u32;

/// Notifies of whether or not there are objects waiting to
/// be transferred to a host computer. 
/// This is useful when ensuring all shot images have been transferred 
/// when the application is closed. 
/// Notification of this event is not issued for type 1 protocol 
/// standard cameras.
pub const kEdsStateEvent_JobStatusChanged : EdsStateEvent = 0x00000302u32;

/// Notifies that the camera will shut down after a specific period. 
/// Generated only if auto shut-off is set. 
/// Exactly when notification is issued (that is, the number of
/// seconds until shutdown) varies depending on the camera model. 
/// To continue operation without having the camera shut down,
/// use EdsSendCommand to extend the auto shut-off timer.
/// The time in seconds until the camera shuts down is returned
/// as the initial value.
pub const kEdsStateEvent_WillSoonShutDown : EdsStateEvent = 0x00000303u32;

/// As the counterpart event to kEdsStateEvent_WillSoonShutDown,
/// this event notifies of updates to the number of seconds until
/// a camera shuts down. 
/// After the update, the period until shutdown is model-dependent.
pub const kEdsStateEvent_ShutDownTimerUpdate : EdsStateEvent = 0x00000304u32;

/// Notifies that a requested release has failed, due to focus
/// failure or similar factors.
pub const kEdsStateEvent_CaptureError : EdsStateEvent = 0x00000305u32;

/// Notifies of internal SDK errors. 
/// If this error event is received, the issuing device will probably
/// not be able to continue working properly,
/// so cancel the remote connection.
pub const kEdsStateEvent_InternalError      : EdsStateEvent = 0x00000306u32;

pub const kEdsStateEvent_AfResult           : EdsStateEvent = 0x00000309u32;

pub const kEdsStateEvent_BulbExposureTime   : EdsStateEvent = 0x00000310u32;



/// Drive Lens
#[repr(u32)]
pub enum EdsEvfDriveLens
{
	kEdsEvfDriveLens_Near1 = 0x00000001u32,
	kEdsEvfDriveLens_Near2 = 0x00000002u32,
	kEdsEvfDriveLens_Near3 = 0x00000003u32,
	kEdsEvfDriveLens_Far1  = 0x00008001u32,
	kEdsEvfDriveLens_Far2  = 0x00008002u32,
	kEdsEvfDriveLens_Far3  = 0x00008003u32,
}

/// Depth of Field Preview
#[repr(u32)]
pub enum EdsEvfDepthOfFieldPreview
{
	kEdsEvfDepthOfFieldPreview_OFF = 0x00000000u32,
	kEdsEvfDepthOfFieldPreview_ON  = 0x00000001u32,
}

/// Stream Seek Origins
#[repr(u32)]
pub enum EdsSeekOrigin
{
    kEdsSeek_Cur   = 0u32,
    kEdsSeek_Begin = 1u32,
    kEdsSeek_End   = 2u32,
}

/// File and Propaties Access
#[repr(u32)]
pub enum EdsAccess
{
    kEdsAccess_Read        = 0u32,
    kEdsAccess_Write       = 1u32,
    kEdsAccess_ReadWrite   = 2u32,
    kEdsAccess_Error       = 0xFFFFFFFFu32,
}

/// File Create Disposition
#[repr(u32)]
pub enum EdsFileCreateDisposition
{
    kEdsFileCreateDisposition_CreateNew            = 0u32,
    kEdsFileCreateDisposition_CreateAlways         = 1u32,
    kEdsFileCreateDisposition_OpenExisting         = 2u32,
    kEdsFileCreateDisposition_OpenAlways           = 3u32,
    kEdsFileCreateDisposition_TruncateExsisting    = 4u32,
}

/// Image Types
#[repr(u32)]
pub enum EdsImageType
{
    kEdsImageType_Unknown  = 0x00000000u32,
    kEdsImageType_Jpeg     = 0x00000001u32,
    kEdsImageType_CRW      = 0x00000002u32,
    kEdsImageType_RAW      = 0x00000004u32,
    kEdsImageType_CR2      = 0x00000006u32,
}

/// Image Size
#[repr(u32)]
pub enum EdsImageSize
{
    kEdsImageSize_Large    = 0u32,
    kEdsImageSize_Middle   = 1u32,
    kEdsImageSize_Small    = 2u32,
    kEdsImageSize_Middle1  = 5u32,
    kEdsImageSize_Middle2  = 6u32,
    kEdsImageSize_Small1   = 14u32,
    kEdsImageSize_Small2   = 15u32,
    kEdsImageSize_Small3   = 16u32,
	kEdsImageSize_Unknown  = 0xffffffffu32,
}

/// Image Compress Quality
#[repr(u32)]
pub enum EdsCompressQuality
{
    kEdsCompressQuality_Normal     = 2u32,
    kEdsCompressQuality_Fine       = 3u32,
    kEdsCompressQuality_Lossless   = 4u32,
    kEdsCompressQuality_SuperFine  = 5u32,
    kEdsCompressQuality_Unknown    = 0xffffffffu32,
}

/// Image Quality
#[repr(u32)]
pub enum EdsImageQuality
{
	/* Jpeg Only */
	EdsImageQuality_LJ     =	0x0010ff0fu32,	/* Jpeg Large */
	EdsImageQuality_M1J    =	0x0510ff0fu32,	/* Jpeg Middle1 */
	EdsImageQuality_M2J    =	0x0610ff0fu32,	/* Jpeg Middle2 */
	EdsImageQuality_SJ     =	0x0210ff0fu32,	/* Jpeg Small */
	EdsImageQuality_LJF    =	0x0013ff0fu32,	/* Jpeg Large Fine */
	EdsImageQuality_LJN    =	0x0012ff0fu32,	/* Jpeg Large Normal */
	EdsImageQuality_MJF    =	0x0113ff0fu32,	/* Jpeg Middle Fine */
	EdsImageQuality_MJN    =	0x0112ff0fu32,	/* Jpeg Middle Normal */
	EdsImageQuality_SJF    =	0x0213ff0fu32,	/* Jpeg Small Fine */
	EdsImageQuality_SJN    =	0x0212ff0fu32,	/* Jpeg Small Normal */
	EdsImageQuality_S1JF   =	0x0E13ff0fu32,	/* Jpeg Small1 Fine */
	EdsImageQuality_S1JN   =	0x0E12ff0fu32,	/* Jpeg Small1 Normal */
	EdsImageQuality_S2JF   =	0x0F13ff0fu32,	/* Jpeg Small2 */
	EdsImageQuality_S3JF   =	0x1013ff0fu32,	/* Jpeg Small3 */

	/* RAW + Jpeg */
	EdsImageQuality_LR     =	0x0064ff0fu32,	/* RAW */
	EdsImageQuality_LRLJF  =	0x00640013u32,	/* RAW + Jpeg Large Fine */
	EdsImageQuality_LRLJN  =	0x00640012u32,	/* RAW + Jpeg Large Normal */
	EdsImageQuality_LRMJF  =	0x00640113u32,	/* RAW + Jpeg Middle Fine */
	EdsImageQuality_LRMJN  =	0x00640112u32,	/* RAW + Jpeg Middle Normal */
	EdsImageQuality_LRSJF  =	0x00640213u32,	/* RAW + Jpeg Small Fine */
	EdsImageQuality_LRSJN  =	0x00640212u32,	/* RAW + Jpeg Small Normal */
	EdsImageQuality_LRS1JF =	0x00640E13u32,	/* RAW + Jpeg Small1 Fine */
	EdsImageQuality_LRS1JN =	0x00640E12u32,	/* RAW + Jpeg Small1 Normal */
	EdsImageQuality_LRS2JF =	0x00640F13u32,	/* RAW + Jpeg Small2 */
	EdsImageQuality_LRS3JF =	0x00641013u32,	/* RAW + Jpeg Small3 */

	EdsImageQuality_LRLJ   =	0x00640010u32,	/* RAW + Jpeg Large */
	EdsImageQuality_LRM1J  =	0x00640510u32,	/* RAW + Jpeg Middle1 */
	EdsImageQuality_LRM2J  =	0x00640610u32,	/* RAW + Jpeg Middle2 */
	EdsImageQuality_LRSJ   =	0x00640210u32,	/* RAW + Jpeg Small */

	/* MRAW(SRAW1) + Jpeg */
	EdsImageQuality_MR     =	0x0164ff0fu32,	/* MRAW(SRAW1) */
	EdsImageQuality_MRLJF  =	0x01640013u32,	/* MRAW(SRAW1) + Jpeg Large Fine */
	EdsImageQuality_MRLJN  =	0x01640012u32,	/* MRAW(SRAW1) + Jpeg Large Normal */
	EdsImageQuality_MRMJF  =	0x01640113u32,	/* MRAW(SRAW1) + Jpeg Middle Fine */
	EdsImageQuality_MRMJN  =	0x01640112u32,	/* MRAW(SRAW1) + Jpeg Middle Normal */
	EdsImageQuality_MRSJF  =	0x01640213u32,	/* MRAW(SRAW1) + Jpeg Small Fine */
	EdsImageQuality_MRSJN  =	0x01640212u32,	/* MRAW(SRAW1) + Jpeg Small Normal */
	EdsImageQuality_MRS1JF =	0x01640E13u32,	/* MRAW(SRAW1) + Jpeg Small1 Fine */
	EdsImageQuality_MRS1JN =	0x01640E12u32,	/* MRAW(SRAW1) + Jpeg Small1 Normal */
	EdsImageQuality_MRS2JF =	0x01640F13u32,	/* MRAW(SRAW1) + Jpeg Small2 */
	EdsImageQuality_MRS3JF =	0x01641013u32,	/* MRAW(SRAW1) + Jpeg Small3 */

	EdsImageQuality_MRLJ   =	0x01640010u32,	/* MRAW(SRAW1) + Jpeg Large */
	EdsImageQuality_MRM1J  =	0x01640510u32,	/* MRAW(SRAW1) + Jpeg Middle1 */
	EdsImageQuality_MRM2J  =	0x01640610u32,	/* MRAW(SRAW1) + Jpeg Middle2 */
	EdsImageQuality_MRSJ   =	0x01640210u32,	/* MRAW(SRAW1) + Jpeg Small */

	/* SRAW(SRAW2) + Jpeg */
	EdsImageQuality_SR     =	0x0264ff0fu32,	/* SRAW(SRAW2) */
	EdsImageQuality_SRLJF  =	0x02640013u32,	/* SRAW(SRAW2) + Jpeg Large Fine */
	EdsImageQuality_SRLJN  =	0x02640012u32,	/* SRAW(SRAW2) + Jpeg Large Normal */
	EdsImageQuality_SRMJF  =	0x02640113u32,	/* SRAW(SRAW2) + Jpeg Middle Fine */
	EdsImageQuality_SRMJN  =	0x02640112u32,	/* SRAW(SRAW2) + Jpeg Middle Normal */
	EdsImageQuality_SRSJF  =	0x02640213u32,	/* SRAW(SRAW2) + Jpeg Small Fine */
	EdsImageQuality_SRSJN  =	0x02640212u32,	/* SRAW(SRAW2) + Jpeg Small Normal */
	EdsImageQuality_SRS1JF =	0x02640E13u32,	/* SRAW(SRAW2) + Jpeg Small1 Fine */
	EdsImageQuality_SRS1JN =	0x02640E12u32,	/* SRAW(SRAW2) + Jpeg Small1 Normal */
	EdsImageQuality_SRS2JF =	0x02640F13u32,	/* SRAW(SRAW2) + Jpeg Small2 */
	EdsImageQuality_SRS3JF =	0x02641013u32,	/* SRAW(SRAW2) + Jpeg Small3 */

	EdsImageQuality_SRLJ   =	0x02640010u32,	/* SRAW(SRAW2) + Jpeg Large */
	EdsImageQuality_SRM1J  =	0x02640510u32,	/* SRAW(SRAW2) + Jpeg Middle1 */
	EdsImageQuality_SRM2J  =	0x02640610u32,	/* SRAW(SRAW2) + Jpeg Middle2 */
	EdsImageQuality_SRSJ   =	0x02640210u32,	/* SRAW(SRAW2) + Jpeg Small */

	EdsImageQuality_Unknown    = 0xffffffffu32,
}

#[repr(u32)]
pub enum EdsImageQualityForLegacy
{
	kEdsImageQualityForLegacy_LJ       =	0x001f000fu32,	/* Jpeg Large */
	kEdsImageQualityForLegacy_M1J      =	0x051f000fu32,	/* Jpeg Middle1 */
	kEdsImageQualityForLegacy_M2J      =	0x061f000fu32,	/* Jpeg Middle2 */
	kEdsImageQualityForLegacy_SJ       =	0x021f000fu32,	/* Jpeg Small */
	kEdsImageQualityForLegacy_LJF      =	0x00130000u32,	/* Jpeg Large Fine */
	kEdsImageQualityForLegacy_LJN      =	0x00120000u32,	/* Jpeg Large Normal */
	kEdsImageQualityForLegacy_MJF      =	0x01130000u32,	/* Jpeg Middle Fine */
	kEdsImageQualityForLegacy_MJN      =	0x01120000u32,	/* Jpeg Middle Normal */
	kEdsImageQualityForLegacy_SJF      =	0x02130000u32,	/* Jpeg Small Fine */
	kEdsImageQualityForLegacy_SJN      =	0x02120000u32,	/* Jpeg Small Normal */

	kEdsImageQualityForLegacy_LR       =	0x00240000u32,	/* RAW */
	kEdsImageQualityForLegacy_LRLJF    =	0x00240013u32,	/* RAW + Jpeg Large Fine */
	kEdsImageQualityForLegacy_LRLJN    =	0x00240012u32,	/* RAW + Jpeg Large Normal */
	kEdsImageQualityForLegacy_LRMJF    =	0x00240113u32,	/* RAW + Jpeg Middle Fine */
	kEdsImageQualityForLegacy_LRMJN    =	0x00240112u32,	/* RAW + Jpeg Middle Normal */
	kEdsImageQualityForLegacy_LRSJF    =	0x00240213u32,	/* RAW + Jpeg Small Fine */
	kEdsImageQualityForLegacy_LRSJN    =	0x00240212u32,	/* RAW + Jpeg Small Normal */

	kEdsImageQualityForLegacy_LR2      =	0x002f000fu32,	/* RAW */
	kEdsImageQualityForLegacy_LR2LJ    =	0x002f001fu32,	/* RAW + Jpeg Large */
	kEdsImageQualityForLegacy_LR2M1J   =	0x002f051fu32,	/* RAW + Jpeg Middle1 */
	kEdsImageQualityForLegacy_LR2M2J   =	0x002f061fu32,	/* RAW + Jpeg Middle2 */
	kEdsImageQualityForLegacy_LR2SJ    =	0x002f021fu32,	/* RAW + Jpeg Small */

	kEdsImageQualityForLegacy_Unknown  = 0xffffffffu32,
}

/// Image Source
#[repr(u32)]
pub enum EdsImageSource
{
    kEdsImageSrc_FullView      = 0u32,
    kEdsImageSrc_Thumbnail     = 1u32,
    kEdsImageSrc_Preview       = 2u32,
    kEdsImageSrc_RAWThumbnail  = 3u32,
    kEdsImageSrc_RAWFullView   = 4u32,
}

/// Target Image Types
#[repr(u32)]
pub enum EdsTargetImageType
{
    kEdsTargetImageType_Unknown    = 0x00000000u32,
    kEdsTargetImageType_Jpeg       = 0x00000001u32,
    kEdsTargetImageType_TIFF       = 0x00000007u32,
    kEdsTargetImageType_TIFF16     = 0x00000008u32,
    kEdsTargetImageType_RGB        = 0x00000009u32,
    kEdsTargetImageType_RGB16      = 0x0000000Au32,
    kEdsTargetImageType_DIB        = 0x0000000Bu32
}

/// Progress Option
#[repr(u32)]
pub enum EdsProgressOption
{
    kEdsProgressOption_NoReport        = 0u32,
    kEdsProgressOption_Done            = 1u32,
    kEdsProgressOption_Periodically    = 2u32,
}

/// File attribute
#[repr(u32)]
pub enum EdsFileAttributes
{
    kEdsFileAttribute_Normal   = 0x00000000u32,
    kEdsFileAttribute_ReadOnly = 0x00000001u32,
    kEdsFileAttribute_Hidden   = 0x00000002u32,
    kEdsFileAttribute_System   = 0x00000004u32,
    kEdsFileAttribute_Archive  = 0x00000020u32,
}

/// Battery level
#[repr(u32)]
pub enum EdsBatteryLevel2
{
   kEdsBatteryLevel2_Empty     = 0u32,
   kEdsBatteryLevel2_Low       = 9u32,
   kEdsBatteryLevel2_Half      = 49u32,
   kEdsBatteryLevel2_Normal    = 80u32,
   kEdsBatteryLevel2_Hi        = 69u32,
   kEdsBatteryLevel2_Quarter   = 19u32,
 //  kEdsBatteryLevel2_Error     = 0u32,
 //  kEdsBatteryLevel2_BCLevel   = 0u32,
   kEdsBatteryLevel2_AC        = 0xFFFFFFFFu32,
}

/// Save To
#[repr(u32)]
pub enum EdsSaveTo
{
    kEdsSaveTo_Camera  = 1u32,
    kEdsSaveTo_Host    = 2u32,
    kEdsSaveTo_Both    = (EdsSaveTo::kEdsSaveTo_Camera as u32) | (EdsSaveTo::kEdsSaveTo_Host as u32),
}

/// StorageType
#[repr(u32)]
pub enum EdsStorageType
{
    kEdsStorageType_Non    = 0u32,
    kEdsStorageType_CF     = 1u32,
    kEdsStorageType_SD     = 2u32,
    kEdsStorageType_HD     = 4u32,
	kEdsStorageType_CFast  = 5u32,
}

/// White Balance
#[repr(i32)]
pub enum EdsWhiteBalance
{
    kEdsWhiteBalance_Auto          = 0i32,
    kEdsWhiteBalance_Daylight      = 1i32,
    kEdsWhiteBalance_Cloudy        = 2i32,
    kEdsWhiteBalance_Tangsten      = 3i32,
    kEdsWhiteBalance_Fluorescent   = 4i32,
    kEdsWhiteBalance_Strobe        = 5i32,
    kEdsWhiteBalance_WhitePaper    = 6i32,
    kEdsWhiteBalance_Shade         = 8i32,
    kEdsWhiteBalance_ColorTemp     = 9i32,
    kEdsWhiteBalance_PCSet1        = 10i32,
    kEdsWhiteBalance_PCSet2        = 11i32,
    kEdsWhiteBalance_PCSet3        = 12i32,
	kEdsWhiteBalance_WhitePaper2   = 15i32,
	kEdsWhiteBalance_WhitePaper3   = 16i32,
	kEdsWhiteBalance_WhitePaper4   = 18i32,
	kEdsWhiteBalance_WhitePaper5   = 19i32,
    kEdsWhiteBalance_PCSet4        = 20i32,
    kEdsWhiteBalance_PCSet5        = 21i32,
	kEdsWhiteBalance_AwbWhite      = 23i32,
	kEdsWhiteBalance_Click         = -1i32,
    kEdsWhiteBalance_Pasted        = -2i32,
}

/// Photo Effects
#[repr(u32)]
pub enum EdsPhotoEffect
{
    kEdsPhotoEffect_Off        = 0u32,
    kEdsPhotoEffect_Monochrome = 5u32,

}

/// Color Matrix
#[repr(u32)]
pub enum EdsColorMatrix
{
    kEdsColorMatrix_Custom = 0u32,
    kEdsColorMatrix_1      = 1u32,
    kEdsColorMatrix_2      = 2u32,
    kEdsColorMatrix_3      = 3u32,
    kEdsColorMatrix_4      = 4u32,
    kEdsColorMatrix_5      = 5u32,
    kEdsColorMatrix_6      = 6u32,
    kEdsColorMatrix_7      = 7u32,
}

/// Filter Effects
#[repr(u32)]
pub enum EdsFilterEffect
{
    kEdsFilterEffect_None      = 0u32,
    kEdsFilterEffect_Yellow    = 1u32,
    kEdsFilterEffect_Orange    = 2u32,
    kEdsFilterEffect_Red       = 3u32,
    kEdsFilterEffect_Green     = 4u32,
}

/// Toning Effects
#[repr(u32)]
pub enum EdsTonigEffect
{
    kEdsTonigEffect_None   = 0u32,
    kEdsTonigEffect_Sepia  = 1u32,
    kEdsTonigEffect_Blue   = 2u32,
    kEdsTonigEffect_Purple = 3u32,
    kEdsTonigEffect_Green  = 4u32,
}

/// Color Space
#[repr(u32)]
pub enum EdsColorSpace
{
    kEdsColorSpace_sRGB        = 1u32,
    kEdsColorSpace_AdobeRGB    = 2u32,
    kEdsColorSpace_Unknown     = 0xffffffffu32,
}

/// PictureStyle
#[repr(u32)]
pub enum EdsPictureStyle
{
    kEdsPictureStyle_Standard      = 0x0081u32,
    kEdsPictureStyle_Portrait      = 0x0082u32,
    kEdsPictureStyle_Landscape     = 0x0083u32,
    kEdsPictureStyle_Neutral       = 0x0084u32,
    kEdsPictureStyle_Faithful      = 0x0085u32,
    kEdsPictureStyle_Monochrome    = 0x0086u32,
    kEdsPictureStyle_Auto          = 0x0087u32,
	kEdsPictureStyle_FineDetail    = 0x0088u32,
	kEdsPictureStyle_User1         = 0x0021u32,
    kEdsPictureStyle_User2         = 0x0022u32,
    kEdsPictureStyle_User3         = 0x0023u32,
    kEdsPictureStyle_PC1           = 0x0041u32,
    kEdsPictureStyle_PC2           = 0x0042u32,
    kEdsPictureStyle_PC3           = 0x0043u32,
}

/// Transfer Option
#[repr(u32)]
pub enum EdsTransferOption 
{
    kEdsTransferOption_ByDirectTransfer    = 1u32,
    kEdsTransferOption_ByRelease           = 2u32,
    kEdsTransferOption_ToDesktop           = 0x00000100u32,
}

/// AE Mode
#[repr(u32)]
pub enum EdsAEMode
{
    kEdsAEMode_Program                 = 0u32,
    kEdsAEMode_Tv                      = 1u32,
    kEdsAEMode_Av                      = 2u32,
    kEdsAEMode_Manual                  = 3u32,
    kEdsAEMode_Bulb                    = 4u32,
    kEdsAEMode_A_DEP                   = 5u32,
    kEdsAEMode_DEP                     = 6u32,
    kEdsAEMode_Custom                  = 7u32,
    kEdsAEMode_Lock                    = 8u32,
    kEdsAEMode_Green                   = 9u32,
    kEdsAEMode_NightPortrait           = 10u32,
    kEdsAEMode_Sports                  = 11u32,
    kEdsAEMode_Portrait                = 12u32,
    kEdsAEMode_Landscape               = 13u32,
    kEdsAEMode_Closeup                 = 14u32,
    kEdsAEMode_FlashOff                = 15u32,
    kEdsAEMode_CreativeAuto            = 19u32,
	kEdsAEMode_Movie                   = 20u32,
	kEdsAEMode_PhotoInMovie		       = 21u32,
	kEdsAEMode_SceneIntelligentAuto    = 22u32,
	kEdsAEMode_SCN                     = 25u32,
	kEdsAEMode_NightScenes             = 23u32,
	kEdsAEMode_BacklitScenes           = 24u32,
	kEdsAEMode_Children                = 26u32,
	kEdsAEMode_Food                    = 27u32,
	kEdsAEMode_CandlelightPortraits    = 28u32,
	kEdsAEMode_CreativeFilter          = 29u32,
	kEdsAEMode_RoughMonoChrome         = 30u32,
	kEdsAEMode_SoftFocus               = 31u32,
	kEdsAEMode_ToyCamera               = 32u32,
	kEdsAEMode_Fisheye                 = 33u32,
	kEdsAEMode_WaterColor              = 34u32,
	kEdsAEMode_Miniature               = 35u32,
	kEdsAEMode_Hdr_Standard            = 36u32,
	kEdsAEMode_Hdr_Vivid               = 37u32,
	kEdsAEMode_Hdr_Bold                = 38u32,
	kEdsAEMode_Hdr_Embossed            = 39u32,
	kEdsAEMode_Movie_Fantasy           = 40u32,
	kEdsAEMode_Movie_Old               = 41u32,
	kEdsAEMode_Movie_Memory            = 42u32,
	kEdsAEMode_Movie_DirectMono        = 43u32,
	kEdsAEMode_Movie_Mini              = 44u32,
	kEdsAEMode_Unknown                 = 0xffffffffu32,
}

/// Bracket
#[repr(u32)]
pub enum EdsBracket
{
    kEdsBracket_AEB        = 0x01u32,
    kEdsBracket_ISOB       = 0x02u32,
    kEdsBracket_WBB        = 0x04u32,
    kEdsBracket_FEB        = 0x08u32,
    kEdsBracket_Unknown    = 0xffffffffu32,
}

/// EVF Output Device [Flag]
#[repr(u32)]
pub enum EdsEvfOutputDevice
{
	kEdsEvfOutputDevice_TFT        = 1u32,
	kEdsEvfOutputDevice_PC         = 2u32,
	kEdsEvfOutputDevice_MOBILE     = 4u32,
	kEdsEvfOutputDevice_MOBILE2    = 8u32,
}

/// EVF Zoom
#[repr(u32)]
pub enum EdsEvfZoom
{
	kEdsEvfZoom_Fit    = 1u32,
	kEdsEvfZoom_x5     = 5u32,
	kEdsEvfZoom_x10    = 10u32,
}

/// EVF AF Mode
#[repr(u32)]
pub enum EdsEvfAFMode
{
	Evf_AFMode_Quick       = 0u32,
	Evf_AFMode_Live        = 1u32,
	Evf_AFMode_LiveFace    = 2u32,
	Evf_AFMode_LiveMulti   = 3u32,
}

/// Strobo Mode
#[repr(u32)]
pub enum EdsStroboMode
{
	kEdsStroboModeInternal         = 0u32,
	kEdsStroboModeExternalETTL     = 1u32,
	kEdsStroboModeExternalATTL     = 2u32,
	kEdsStroboModeExternalTTL      = 3u32,
	kEdsStroboModeExternalAuto     = 4u32,
	kEdsStroboModeExternalManual   = 5u32,
	kEdsStroboModeManual           = 6u32,
}

/// ETTL-II Mode
#[repr(u32)]
pub enum EdsETTL2Mode
{
	kEdsETTL2ModeEvaluative    = 0u32,
	kEdsETTL2ModeAverage       = 1u32,
}

// Definition of base Structures

/// Point
#[repr(C)]
pub struct EdsPoint
{
    pub x   : i32,
    pub y   : i32
}

/// Size
#[repr(C)]
pub struct EdsSize
{
    pub width   : i32,
    pub height  : i32
}

/// Rectangle
#[repr(C)]
pub struct EdsRect
{
    pub point   : EdsPoint,
    pub size    : EdsSize
}

/// Rational
#[repr(C)]
pub struct EdsRational
{
    pub numerator   : i32,
    pub denominator : u32
}

/// Time
#[repr(C)]
pub struct EdsTime
{
    pub year            : u32,
    pub month           : u32,
    pub day             : u32,
    pub hour            : u32,
    pub minute          : u32,
    pub second          : u32,
    pub milliseconds    : u32
}

/// Device Info
#[repr(C)]
pub struct EdsDeviceInfo
{
    pub szPortName          : [u8; EDS_MAX_NAME],
    pub szDeviceDescription : [u8; EDS_MAX_NAME],
    pub deviceSubType       : u32,
	pub reserved            : u32
}

/// Volume Info
#[repr(C)]
pub struct EdsVolumeInfo
{
    pub storageType         : u32,
    pub access              : EdsAccess,
    pub maxCapacity         : u64,
    pub freeSpaceInBytes    : u64,
    pub szVolumeLabel       : [u8; EDS_MAX_NAME]
}

/// DirectoryItem Info
#[repr(C)]
pub struct EdsDirectoryItemInfo
{
    pub size        : u64,
    pub isFolder    : bool,
    pub groupID     : u32,
    pub option      : u32,
    pub szFileName  : [u8; EDS_MAX_NAME],

	pub format      : u32,
	pub dateTime    : u32
}

/// Image Info
#[repr(C)]
pub struct EdsImageInfo
{
    pub width           : u32,
    pub height          : u32,
    pub numOfComponents : u32,
    pub componentDepth  : u32,
    pub effectiveRect   : EdsRect,
    pub reserved1       : u32,
    pub reserved2       : u32,
}

/// SaveImage Setting
#[repr(C)]
pub struct EdsSaveImageSetting
{
    pub JPEGQuality         : u32,
    pub iccProfileStream    : EdsStreamRef,
    pub reserved            : u32
}

/// Property Desc
#[repr(C)]
pub struct EdsPropertyDesc
{
    pub form        : i32,
    pub access      : i32,
    pub numElements : i32,
    pub propDesc    : [i32; 128]
}

/// Picture Style Desc
#[repr(C)]
pub struct EdsPictureStyleDesc
{
    pub contrast        : i32,
    pub sharpness       : u32,
    pub saturation      : i32,
    pub colorTone       : i32,
    pub filterEffect    : u32,
    pub toningEffect    : u32,
	pub sharpFineness   : u32,
	pub sharpThreshold  : u32
}

/// Focus Info
#[repr(C)]
pub struct EdsFocusPoint
{
    pub valid       : u32,
	pub selected    : u32,
    pub justFocus   : u32,
    pub rect        : EdsRect,
    pub reserved    : u32
}

#[repr(C)]
pub struct EdsFocusInfo
{
    pub imageRect   : EdsRect,
    pub pointNumber : u32,
    pub focusPoint  : [EdsFocusPoint; 128],
	pub executeMode : u32
}

/// User WhiteBalance (PC set1,2,3)/ User ToneCurve / User PictureStyle dataset 
#[repr(C)]
pub struct EdsUsersetData
{
    pub valid       : u32,
    pub dataSize    : u32,
    pub szCaption   : [i8; 32],
    pub data        : [u8; 1]
}

/// Capacity
#[repr(C)]
pub struct EdsCapacity
{
    pub numberOfFreeClusters    : i32,
    pub bytesPerSector          : i32,
    pub reset                   : bool
}

/// FramePoint
#[repr(C)]
pub struct EdsFramePoint
{
	pub x   : i32,
	pub y   : i32
}

// Callback Functions

/// EdsProgressCallback
pub type EdsProgressCallback = extern "C" fn(inPercent : u32, inContext : *const c_void, outCancel : *mut bool)->EdsError;
                    
/// EdsCameraAddedHandler
pub type EdsCameraAddedHandler = extern "C" fn(inContext : *const c_void)->EdsError;

/// EdsPropertyEventHandler
pub type EdsPropertyEventHandler = extern "C" fn(inEvent : EdsPropertyEvent, inPropertyID : EdsPropertyID, inParam : u32, inContext : *const c_void)->EdsError;

/// EdsObjectEventHandler
pub type EdsObjectEventHandler = extern "C" fn(inEvent : EdsObjectEvent, inRef : EdsBaseRef, inContext : *const c_void)->EdsError;

/// EdsStateEventHandler
pub type EdsStateEventHandler = extern "C" fn(inEvent : EdsStateEvent, inEventData : u32, inContext : *const c_void)->EdsError;

pub type EdsReadStream = extern "C" fn(inContext: *const c_void, inReadSize : u32, outBuffer : *mut c_void, outReadSize : *mut u32)->EdsError;
pub type EdsWriteStream = extern "C" fn(inContext: *const c_void, inWriteSize : u32, inBuffer : *const c_void, outWrittenSize : *mut u32)->EdsError;
pub type EdsSeekStream = extern "C" fn(inContext : *const c_void, inSeekOffset : i32, inSeekOrigin : EdsSeekOrigin)->EdsError;
pub type EdsTellStream = extern "C" fn(inContext : *const c_void, outPosition : *mut i32)->EdsError;
pub type EdsGetStreamLength = extern "C" fn(inContext : *const c_void, outLength : *mut u32)->EdsError;

#[repr(C)]
pub struct EdsIStream
{
    pub context     : *const c_void,

    pub read        : EdsReadStream,
    pub write       : EdsWriteStream,
    pub seek        : EdsSeekStream,
    pub tell        : EdsTellStream,
    pub getLength   : EdsGetStreamLength
}

