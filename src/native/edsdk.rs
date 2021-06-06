use super::edsdk_types::*;
use std::ffi::c_void;

#[link(name="EDSDK")]
extern "stdcall" {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]

    //  Basic functions

    /// # Initializes the libraries. 
    /// When using the EDSDK libraries, you must call this API once  
    /// before using EDSDK APIs.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsInitializeSDK()->EdsError; // worked

    /// # Terminates use of the libraries. 
    /// This function muse be called when ending the SDK.
    /// Calling this function releases all resources allocated by the libraries.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsTerminateSDK()->EdsError; // worked

    // Reference-counter operating functions.

    /// # Increments the reference counter of existing objects.
    /// # Parameters:
    /// * inRef - The reference for the item.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsRetain(inRef : EdsBaseRef)->u32;

    /// # Decrements the reference counter to an object. 
    /// When the reference counter reaches 0, the object is released.
    /// # Parameters:
    /// * inRef - The reference of the item.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsRelease(inRef : EdsBaseRef)->u32; // worked

    // Item-tree operating functions
    /// # Gets the number of child objects of the designated object.
    /// Example: Number of files in a directory
    /// # Parameters:
    /// * inRef - The reference of the list.
    /// * outCount - Number of elements in this list.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetChildCount(inRef : EdsBaseRef, outCount : *mut u32)->EdsError; // worked

    /// # Gets an indexed child object of the designated object. 
    /// # Parameters:
    /// * inRef - The reference of the item.
    /// * inIndex -  The index that is passed in, is zero based.
    /// * outRef - The pointer which receives reference of the specified index .
    /// # Returns:    Any of the sdk errors.
//    pub fn EdsGetChildAtIndex(inRef : EdsBaseRef, inIndex : i32, outRef : *mut EdsBaseRef)->EdsError;
    pub fn EdsGetChildAtIndex(inRef : EdsBaseRef, inIndex : i32, outRef : &*mut c_void)->EdsError; // worked

    /// # Gets the parent object of the designated object.
    /// # Parameters:
    /// * inRef        - The reference of the item.
    /// * outParentRef - The pointer which receives reference.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetParent(inRef : EdsBaseRef, outParentRef : *mut EdsBaseRef)->EdsError;

    //  Property operating functions
    /// # Gets the byte size and data type of a designated property from a camera object or image object.
    /// # Parameters:
    /// * inRef - The reference of the item.
    /// * inPropertyID - The ProprtyID
    /// * inParam - Additional information of property.
    ///             We use this parameter in order to specify an index
    ///             in case there are two or more values over the same ID.
    /// * outDataType - Pointer to the buffer that is to receive the property type data.
    /// * outSize - Pointer to the buffer that is to receive the property size.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetPropertySize(inRef : EdsBaseRef, inPropertyID : EdsPropertyID, inParam : i32,
                                outDataType : *mut EdsDataType, outSize : *mut u32)->EdsError; // worked

    /// # Gets property information from the object designated in inRef.
    /// # Parameters:
    /// * inRef - The reference of the item.
    /// * inPropertyID - The ProprtyID
    /// * inParam - Additional information of property.
    ///             We use this parameter in order to specify an index
    ///             in case there are two or more values over the same ID.
    /// * inPropertySize - The number of bytes of the prepared buffer
    ///             for receive property-value.
    /// * outPropertyData - The buffer pointer to receive property-value.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetPropertyData(inRef : EdsBaseRef, inPropertyID : EdsPropertyID, inParam : i32, inPropertySize : u32,
                                outPropertyData : *mut c_void )->EdsError; // worked

    /// # Sets property data for the object designated in inRef. 
    /// # Parameters:
    /// * inRef - The reference of the item.
    /// * inPropertyID - The ProprtyID
    /// * inParam - Additional information of property.
    /// * inPropertySize - The number of bytes of the prepared buffer for set property-value.
    /// * inPropertyData - The buffer pointer to set property-value.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetPropertyData(inRef : EdsBaseRef, inPropertyID : EdsPropertyID, inParam : i32, inPropertySize : u32, inPropertyData : *const c_void)->EdsError; // worked

    /// # Gets a list of property data that can be set for the object designated in inRef, as well as maximum and minimum values. 
    /// This API is intended for only some shooting-related properties.
    /// #  Parameters:
    /// * inRef - The reference of the camera.
    /// * inPropertyID - The Property ID.
    /// * outPropertyDesc - Array of the value which can be set up.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetPropertyDesc(inRef : EdsBaseRef, inPropertyID : EdsPropertyID,
                                outPropertyDesc : *mut EdsPropertyDesc)->EdsError; // worked

    //  Device-list and device operating functions

    /// # Gets camera list objects.
    /// # Parameters:
    /// * outCameraListRef - Pointer to the camera-list.
    /// # Returns:    Any of the sdk errors.
 //   pub fn EdsGetCameraList(outCameraListRef : *mut EdsCameraListRef)->EdsError;
    pub fn EdsGetCameraList(outCameraListRef : &*mut c_void)->EdsError; // worked

    //  Camera operating functions

    /// # Gets device information, such as the device name.  
    /// Because device information of remote cameras is stored 
    /// on the host computer, you can use this API 
    /// before the camera object initiates communication
    /// (that is, before a session is opened). 
    /// # Parameters:
    /// * inCameraRef - The reference of the camera.
    /// * outDeviceInfo - Information as device of camera.
    ///  Returns:    Any of the sdk errors.
    pub fn EdsGetDeviceInfo(inCameraRef : EdsCameraRef, outDeviceInfo : *mut EdsDeviceInfo)->EdsError; // worked

    /// # Establishes a logical connection with a remote camera. 
    /// Use this API after getting the camera's EdsCamera object.
    /// # Parameters:
    /// * inCameraRef - The reference of the camera 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsOpenSession(inCameraRef : EdsCameraRef)->EdsError; // worked

    /// # Closes a logical connection with a remote camera.
    /// # Parameters:
    /// * inCameraRef - The reference of the camera 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCloseSession(inCameraRef : EdsCameraRef)->EdsError; // worked

    /// # Sends a command such as "Shoot" to a remote camera. 
    /// # Parameters:
    /// * inCameraRef - The reference of the camera which will receive the command.
    /// * inCommand - Specifies the command to be sent.
    /// * inParam -     Specifies additional command-specific information.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSendCommand(inCameraRef : EdsCameraRef, inCommand : EdsCameraCommand, inParam : i32)->EdsError; // worked

    /// # Sets the remote camera state or mode.
    /// # Parameters:
    /// * inCameraRef - The reference of the camera which will receive the command.
    /// * inStatusCommand - Specifies the command to be sent.
    /// * inParam -     Specifies additional command-specific information.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSendStatusCommand(inCameraRef : EdsCameraRef, inStatusCommand : EdsCameraStatusCommand, inParam : i32)->EdsError; // worked

    /// # Sets the remaining HDD capacity on the host computer
    /// (excluding the portion from image transfer),
    /// as calculated by subtracting the portion from the previous time. 
    /// Set a reset flag initially and designate the cluster length 
    /// and number of free clusters.
    /// Some type 2 protocol standard cameras can display the number of shots 
    /// left on the camera based on the available disk capacity 
    /// of the host computer. 
    /// For these cameras, after the storage destination is set to the computer, 
    /// use this API to notify the camera of the available disk capacity 
    /// of the host computer.
    /// # Parameters:
    /// * inCameraRef - The reference of the camera which will receive the command.
    /// * inCapacity -  The remaining capacity of a transmission place.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetCapacity(inCameraRef : EdsCameraRef, inCapacity : EdsCapacity)->EdsError; // worked

    //  Volume operating functions

    /// # Gets volume information for a memory card in the camera.
    /// # Parameters:
    /// * inVolumeRef - The reference of the volume.
    /// * outVolumeInfo - information of  the volume.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetVolumeInfo(inVolumeRef : EdsVolumeRef, outVolumeInfo : *mut EdsVolumeInfo)->EdsError;

    /// # Formats volumes of memory cards in a camera. 
    /// # Parameters:
    /// * inVolumeRef - The reference of volume .
    /// # Returns:    Any of the sdk errors.
    pub fn EdsFormatVolume(inVolumeRef : EdsVolumeRef)->EdsError;

    // Directory-item operating functions

    /// # Gets information about the directory or file objects on the memory card (volume) in a remote camera.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// * outDirItemInfo - information of the directory item.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetDirectoryItemInfo(inDirItemRef : EdsDirectoryItemRef,
                                    outDirItemInfo : *mut EdsDirectoryItemInfo)->EdsError; // worked

    /// # Deletes a camera folder or file.
    /// If folders with subdirectories are designated, all files are deleted 
    /// except protected files. 
    /// EdsDirectoryItem objects deleted by means of this API are implicitly 
    /// released by the EDSDK. Thus, there is no need to release them 
    /// by means of EdsRelease.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsDeleteDirectoryItem(inDirItemRef : EdsDirectoryItemRef)->EdsError;

    /// # Downloads a file on a remote camera (in the camera memory or on a memory card) to the host computer. 
    /// The downloaded file is sent directly to a file stream created in advance. 
    /// When dividing the file being retrieved, call this API repeatedly. 
    /// Also in this case, make the data block size a multiple of 512 (bytes), 
    /// excluding the final block.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// * inReadSize   - 
    /// * outStream    - The reference of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsDownload(inDirItemRef : EdsDirectoryItemRef, inReadSize : u64,
                        outStream : EdsStreamRef)->EdsError; // worked

    /// # Must be executed when downloading of a directory item is canceled. 
    /// Calling this API makes the camera cancel file transmission.
    /// It also releases resources. 
    /// This operation need not be executed when using EdsDownloadThumbnail. 
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsDownloadCancel(inDirItemRef : EdsDirectoryItemRef)->EdsError;

    /// # Must be called when downloading of directory items is complete. 
    /// Executing this API makes the camera 
    /// recognize that file transmission is complete. 
    /// This operation need not be executed when using EdsDownloadThumbnail.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsDownloadComplete(inDirItemRef : EdsDirectoryItemRef)->EdsError; // worked

    /// # Extracts and downloads thumbnail information from image files in a camera. 
    /// Thumbnail information in the camera's image files is downloaded 
    /// to the host computer. 
    /// Downloaded thumbnails are sent directly to a file stream created in advance.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// * outStream - The reference of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsDownloadThumbnail(inDirItemRef : EdsDirectoryItemRef,
                                outStream : EdsStreamRef)->EdsError;


    /// # Gets attributes of files on a camera.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// * outFileAttribute  - Indicates the file attributes. 
    ///     As for the file attributes, OR values of the value defined
    ///     by enum EdsFileAttributes can be retrieved. Thus, when 
    ///     determining the file attributes, you must check 
    ///     if an attribute flag is set for target attributes. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetAttribute(inDirItemRef : EdsDirectoryItemRef,
                            outFileAttribute : *mut EdsFileAttributes)->EdsError;

    /// # Changes attributes of files on a camera.
    /// # Parameters:
    /// * inDirItemRef - The reference of the directory item.
    /// * inFileAttribute  - Indicates the file attributes. 
    ///     As for the file attributes, OR values of the value 
    ///     defined by enum EdsFileAttributes can be retrieved. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetAttribute(inDirItemRef : EdsDirectoryItemRef, inFileAttribute : EdsFileAttributes)->EdsError;

    //  Stream operating functions

    /// # Creates a new file on a host computer (or opens an existing file) and creates a file stream for access to the file. 
    /// If a new file is designated before executing this API, 
    /// the file is actually created following the timing of writing 
    /// by means of EdsWrite or the like with respect to an open stream.
    /// # Parameters:
    /// * inFileName - Pointer to a null-terminated string that specifies the file name.
    /// * inCreateDisposition - Action to take on files that exist, 
    ///     and which action to take when files do not exist.  
    /// * inDesiredAccess - Access to the stream (reading, writing, or both).
    /// * outStream - The reference of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCreateFileStream(inFileName : *const u8, inCreateDisposition : EdsFileCreateDisposition, inDesiredAccess : EdsAccess,
                                outStream : &*mut c_void)->EdsError; // worked

    /// # Creates a stream in the memory of a host computer. 
    /// In the case of writing in excess of the allocated buffer size, 
    /// the memory is automatically extended.
    /// # Parameters:
    /// * inBufferSize - The number of bytes of the memory to allocate.
    /// * outStream - The reference of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCreateMemoryStream(inBufferSize : u64,
                                    outStream : &*mut c_void)->EdsError;

    /// # An extended version of EdsCreateStreamFromFile. 
    /// Use this function when working with Unicode file names.
    /// # Parameters:
    /// * inURL (for Macintosh) - Designate CFURLRef. 
    /// * inFileName (for Windows) - Designate the file name. 
    /// * inCreateDisposition - Action to take on files that exist, 
    ///     and which action to take when files do not exist.  
    /// * inDesiredAccess - Access to the stream (reading, writing, or both).
    /// * outStream - The reference of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCreateFileStreamEx(inFileName : *const u16, inCreateDisposition : EdsFileCreateDisposition, inDesiredAccess : EdsAccess,
                                    outStream : *mut EdsStreamRef)->EdsError;

    /// # Creates a stream from the memory buffer you prepare. 
    /// Unlike the buffer size of streams created by means of EdsCreateMemoryStream, 
    /// the buffer size you prepare for streams created this way does not expand.
    /// # Parameters:
    /// * inUserBuffer - Pointer to the buffer you have prepared.
    ///     Streams created by means of this API lead to this buffer. 
    /// * inBufferSize - The number of bytes of the memory to allocate.
    /// * outStream - The reference of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCreateMemoryStreamFromPointer(inUserBuffer : *const c_void, inBufferSize : u64,
                                            outStream : *mut EdsStreamRef)->EdsError;

    /// # Gets the pointer to the start address of memory managed by the memory stream. 
    /// As the EDSDK automatically resizes the buffer, the memory stream provides 
    /// you with the same access methods as for the file stream. 
    /// If access is attempted that is excessive with regard to the buffer size
    /// for the stream, data before the required buffer size is allocated 
    /// is copied internally, and new writing occurs. 
    /// Thus, the buffer pointer might be switched on an unknown timing. 
    /// Caution in use is therefore advised. 
    /// # Parameters:
    /// * inStream - Designate the memory stream for the pointer to retrieve. 
    /// * outPointer - If successful, returns the pointer to the buffer 
    ///     written in the memory stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetPointer(inStream : EdsStreamRef,
                            outPointer : *mut c_void )->EdsError;
// EdsVoid**               outPointer

    /// # Reads data the size of inReadSize into the outBuffer buffer, starting at the current read or write position of the stream. 
    /// The size of data actually read can be designated in outReadSize.
    /// # Parameters:
    /// * inStreamRef - The reference of the stream or image.
    /// * inReadSize -  The number of bytes to read.
    /// * outBuffer - Pointer to the user-supplied buffer that is to receive
    ///     the data read from the stream. 
    /// * outReadSize - The actually read number of bytes.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsRead(inStreamRef : EdsStreamRef, inReadSize : u64,
                    outBuffer : *mut c_void, outReadSize : *mut u64)->EdsError;

    /// # Writes data of a designated buffer to the current read or write position of the stream. 
    /// # Parameters:
    /// * inStreamRef  - The reference of the stream or image.
    /// * inWriteSize - The number of bytes to write.
    /// * inBuffer - A pointer to the user-supplied buffer that contains 
    ///     the data to be written to the stream.
    /// * outWrittenSize - The actually written-in number of bytes.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsWrite(inStreamRef : EdsStreamRef, inWriteSize : u64, inBuffer : *const c_void,
                        outWrittenSize : *mut c_void)->EdsError;

    /// # Moves the read or write position of the stream(that is, the file position indicator).
    /// # Parameters:
    /// * inStreamRef  - The reference of the stream or image. 
    /// * inSeekOffset - Number of bytes to move the pointer. 
    /// * inSeekOrigin - Pointer movement mode. Must be one of the following values.
    ///         - 'kEdsSeek_Cur'     Move the stream pointer inSeekOffset bytes 
    ///                 from the current position in the stream. 
    ///         - 'kEdsSeek_Begin'   Move the stream pointer inSeekOffset bytes
    ///                 forward from the beginning of the stream. 
    ///         - 'kEdsSeek_End'     Move the stream pointer inSeekOffset bytes
    ///                 from the end of the stream. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSeek(inStreamRef : EdsStreamRef, inSeekOffset : i64, inSeekOrigin : EdsSeekOrigin)->EdsError;

    /// # Gets the current read or write position of the stream (that is, the file position indicator).
    /// # Parameters:
    /// * inStreamRef - The reference of the stream or image.
    /// * outPosition - The current stream pointer.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetPosition(inStreamRef : EdsStreamRef,
                            outPosition : *mut u64)->EdsError;

    /// # Gets the stream size.
    /// # Parameters:
    /// * inStreamRef - The reference of the stream or image.
    /// * outLength - The length of the stream.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetLength(inStreamRef : EdsStreamRef,
                            outLength : *mut u64 )->EdsError;

    /// # Copies data from the copy source stream to the copy destination stream. 
    /// The read or write position of the data to copy is determined from 
    /// the current file read or write position of the respective stream. 
    /// After this API is executed, the read or write positions of the copy source 
    /// and copy destination streams are moved an amount corresponding to 
    /// inWriteSize in the positive direction. 
    /// # Parameters:
    /// * inStreamRef - The reference of the stream or image.
    /// * inWriteSize - The number of bytes to copy.
    /// * outStreamRef - The reference of the stream or image.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCopyData(inStreamRef : EdsStreamRef, inWriteSize : u64,
                        outStreamRef : EdsStreamRef )->EdsError;

    /// # Register a progress callback function. 
    /// An event is received as notification of progress during processing that 
    /// takes a relatively long time, such as downloading files from a
    /// remote camera. 
    /// If you register the callback function, the EDSDK calls the callback
    /// function during execution or on completion of the following APIs. 
    /// This timing can be used in updating on-screen progress bars, for example.
    ///  Parameters:
    /// * inRef - The reference of the stream or image.
    /// * inProgressCallback - Pointer to a progress callback function.
    /// * inProgressOption - The option about progress is specified.
    ///     Must be one of the following values.
    ///     kEdsProgressOption_Done 
    ///     When processing is completed,a callback function
    ///     is called only at once.
    ///     kEdsProgressOption_Periodically
    ///     A callback function is performed periodically.
    /// * inContext - Application information, passed in the argument 
    ///     when the callback function is called. Any information 
    ///     required for your program may be added. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetProgressCallback(inRef : EdsBaseRef, inProgressCallback : EdsProgressCallback, inProgressOption : EdsProgressOption, inContext : *const c_void)->EdsError;

    //  Image operating functions

    /// # Creates an image object from an image file. 
    /// Without modification, stream objects cannot be worked with as images. 
    /// Thus, when extracting images from image files, 
    /// you must use this API to create image objects. 
    /// The image object created this way can be used to get image information 
    /// (such as the height and width, number of color components, and
    /// resolution), thumbnail image data, and the image data itself.
    /// # Parameters:
    /// * inStreamRef - The reference of the stream.
    /// * outImageRef - The reference of the image.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCreateImageRef(inStreamRef : EdsStreamRef, outImageRef : *mut EdsImageRef)->EdsError;

    /// # Gets image information from a designated image object. 
    /// Here, image information means the image width and height, 
    /// number of color components, resolution, and effective image area.
    /// # Parameters:
    /// * inStreamRef - Designate the object for which to get image information. 
    /// * inImageSource - Of the various image data items in the image file,
    ///     designate the type of image data representing the 
    ///     information you want to get. Designate the image as
    ///     defined in Enum EdsImageSource. 
    ///
    ///     - 'kEdsImageSrc_FullView'
    ///         The image itself (a full-sized image) 
    ///     - 'kEdsImageSrc_Thumbnail'
    ///         A thumbnail image 
    ///     - 'kEdsImageSrc_Preview'
    ///         A preview image
    ///     - 'kEdsImageSrc_RAWThumbnail'
    ///         A RAW thumbnail image 
    ///     - 'kEdsImageSrc_RAWFullView'
    ///         A RAW full-sized image 
    /// * outImageInfo - Stores the image data information designated in inImageSource. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetImageInfo(inImageRef : EdsImageRef, inImageSource : EdsImageSource,
                                        outImageInfo : *mut EdsImageInfo)->EdsError;

    /// # Gets designated image data from an image file, in the form of a designated rectangle. 
    /// Returns uncompressed results for JPEGs and processed results 
    /// in the designated pixel order (RGB, Top-down BGR, and so on) for
    /// RAW images. 
    /// Additionally, by designating the input/output rectangle, 
    /// it is possible to get reduced, enlarged, or partial images. 
    /// However, because images corresponding to the designated output rectangle 
    /// are always returned by the SDK, the SDK does not take the aspect 
    /// ratio into account. 
    /// To maintain the aspect ratio, you must keep the aspect ratio in mind 
    /// when designating the rectangle. 
    /// # Parameters:
    /// * inImageRef - Designate the image object for which to get the image data.
    /// * inImageSource - Designate the type of image data to get from
    ///     the image file (thumbnail, preview, and so on). 
    ///     Designate values as defined in Enum EdsImageSource. 
    /// * inImageType - Designate the output image type. Because
    ///     the output format of EdGetImage may only be RGB, only
    ///     kEdsTargetImageType_RGB or kEdsTargetImageType_RGB16
    ///     can be designated. 
    ///     However, image types exceeding the resolution of 
    ///     inImageSource cannot be designated. 
    /// * inSrcRect - Designate the coordinates and size of the rectangle
    ///     to be retrieved (processed) from the source image. 
    /// * inDstSize - Designate the rectangle size for output. 
    /// * outStreamRef - Designate the memory or file stream for output of the image.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetImage(inImageRef : EdsImageRef, inImageSource : EdsImageSource, inImageType : EdsTargetImageType,
                            inSrcRect : EdsRect, inDstSize : EdsSize,
                            outStreamRef : EdsStreamRef)->EdsError;

    /// # Saves as a designated image type after RAW processing. 
    /// When saving with JPEG compression, 
    /// the JPEG quality setting applies with respect to EdsOptionRef.
    /// # Parameters:
    /// * inImageRef - Designate the image object for which to produce the file. 
    /// * inImageType - Designate the image type to produce. Designate the 
    ///     following image types.
    ///     - 'kEdsTargetImageType' - Jpeg  JPEG
    ///     - 'kEdsTargetImageType' - TIFF  8-bit TIFF
    ///     - 'kEdsTargetImageType' - TIFF16    16-bit TIFF
    /// * inSaveSetting - Designate saving options, such as JPEG image quality.
    /// * outStreamRef - Specifies the output file stream. The memory stream cannot be specified here.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSaveImage(inImageRef : EdsImageRef, inImageType : EdsTargetImageType, inSaveSetting : EdsSaveImageSetting,
                            outStreamRef : EdsStreamRef)->EdsError;

    /// # Switches a setting on and off for creation of an image cache in the SDK for a designated image object during extraction (processing) of the image data. 
    /// Creating the cache increases the processing speed, starting from
    /// the second time.
    /// # Parameters:
    /// * inImageRef - The reference of the image.
    /// * inUseCache - If cache image data or not
    ///     If set to FALSE, the cached image data will released.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCacheImage(inImageRef : EdsImageRef, inUseCache : bool)->EdsError;

    /// # Incorporates image object property changes (effected by means of EdsSetPropertyData) in the stream. 
    /// # Parameters:
    /// * inImageRef - The reference of the image.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsReflectImageProperty(inImageRef : EdsImageRef)->EdsError;

    /// # Creates an object used to get the live view image data set. 
    /// # Parameters:
    /// * inStreamRef - The stream reference which opened to get EVF JPEG image.
    /// * outEvfImageRef - The EVFData reference.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsCreateEvfImageRef (inStreamRef : EdsStreamRef,
                                    outEvfImageRef : *mut EdsEvfImageRef)->EdsError;

    /// # Downloads the live view image data set for a camera currently in live view mode.
    /// Live view can be started by using the property ID:kEdsPropertyID_Evf_OutputDevice and
    /// data:EdsOutputDevice_PC to call EdsSetPropertyData.
    /// In addition to image data, information such as zoom, focus position, and histogram data
    /// is included in the image data set. Image data is saved in a stream maintained by EdsEvfImageRef.
    /// EdsGetPropertyData can be used to get information such as the zoom, focus position, etc.
    /// Although the information of the zoom and focus position can be obtained from EdsEvfImageRef,
    /// settings are applied to EdsCameraRef.
    /// # Parameters:
    /// * inCameraRef - The Camera reference.
    /// * inEvfImageRef - The EVFData reference.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsDownloadEvfImage (inCameraRef : EdsCameraRef, inEvfImageRef : EdsEvfImageRef)->EdsError;

    // Event handler registering functions

    /// # Registers a callback function for when a camera is detected.
    /// # Parameters:
    /// * inCameraAddedHandler - Pointer to a callback function
    ///     called when a camera is connected physically
    /// * inContext - Specifies an application-defined value to be sent to
    ///     the callback function pointed to by CallBack parameter.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetCameraAddedHandler(inCameraAddedHandler : EdsCameraAddedHandler, inContext : *const c_void)->EdsError;

    /// # Registers a callback function for receiving status change notification events for property states on a camera.
    /// # Parameters:
    /// * inCameraRef - Designate the camera object. 
    /// * inEvent - Designate one or all events to be supplemented.
    /// * inPropertyEventHandler - Designate the pointer to the callback
    ///     function for receiving property-related camera events.
    /// * inContext - Designate application information to be passed by 
    ///     means of the callback function. Any data needed for
    ///     your application can be passed. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetPropertyEventHandler(inCameraRef : EdsCameraRef, inEvnet : EdsPropertyEvent, inPropertyEventHandler : EdsPropertyEventHandler, inContext : *const c_void)->EdsError;

    /// # Registers a callback function for receiving status change notification events for objects on a remote camera. 
    /// Here, object means volumes representing memory cards, files and directories, 
    /// and shot images stored in memory, in particular. 
    /// # Parameters:
    /// * inCameraRef - Designate the camera object. 
    /// * inEvent - Designate one or all events to be supplemented.
    ///     To designate all events, use kEdsObjectEvent_All. 
    /// * inObjectEventHandler - Designate the pointer to the callback function
    ///     for receiving object-related camera events.
    /// * inContext - Passes inContext without modification,
    ///     as designated as an EdsSetObjectEventHandler argument. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetObjectEventHandler(inCameraRef : EdsCameraRef, inEvnet : EdsObjectEvent, inObjectEventHandler : EdsObjectEventHandler, inContext : *const c_void)->EdsError; // worked

    /// # Registers a callback function for receiving status change notification events for property states on a camera.
    /// # Parameters:
    /// * inCameraRef - Designate the camera object. 
    /// * inEvent - Designate one or all events to be supplemented.
    ///     To designate all events, use kEdsStateEvent_All. 
    /// * inStateEventHandler - Designate the pointer to the callback function
    ///     for receiving events related to camera object states.
    /// * inContext - Designate application information to be passed
    ///     by means of the callback function. Any data needed for
    ///     your application can be passed. 
    /// # Returns:    Any of the sdk errors.
    pub fn EdsSetCameraStateEventHandler(inCameraRef : EdsCameraRef, inEvnet : EdsStateEvent, inStateEventHandler : EdsStateEventHandler, inContext : *const c_void)->EdsError;

    pub fn EdsCreateStream(inStream : *const EdsIStream, outStreamRef : *mut EdsStreamRef)->EdsError;

    /// # This function acquires an event. 
    /// In console application, please call this function regularly to acquire
    /// the event from a camera.
    /// # Returns:    Any of the sdk errors.
    pub fn EdsGetEvent()->EdsError; // worked

}


