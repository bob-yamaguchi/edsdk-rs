
use super::native::edsdk_types::*;
use super::native::edsdk_errors::*;
use super::native::edsdk::*;
use super::types::*;
use std::vec::Vec;
use std::result::Result;
use std::ffi::c_void;
use std::ffi::CStr;

//use std::io::Write;

/// Device infomation
pub struct DeviceInfo{
    pub description : String,
    pub port        : String,
}

pub struct DirectoryItemInfo
{
    pub size        : u64,
    pub is_folder   : bool,
    pub group_id    : u32,
    pub option      : u32,
	pub format      : u32,
	pub date_time   : u32,
    pub file_name   : String
}
#[derive(Debug)]
pub struct Session{
    device  : EdsCameraRef,
}

struct TakePicturesContext{
    /// # image transfer callback after remote release
    /// * `data`  slice to picture data
    memory_transfer_callback_func : Option<fn(data: &[u8])>,
    /// file name to save
    file_name : String,
    finish_transfer: bool
}

impl Session{
    /// callback function
    #[allow(non_upper_case_globals)]
    extern "C" fn dir_item_request_transfer_callback(event: u32, dir_item: EdsBaseRef, context: *const c_void)->EdsError{
        match event{
            kEdsObjectEvent_DirItemRequestTransfer=>{
                let dir_item_info = get_directory_item_info(dir_item);
                if dir_item_info.is_err(){
                    return EDS_ERR_OK;
                }
                let dir_item_info = dir_item_info.unwrap();
                let context_ptr = context as *mut TakePicturesContext;
                let context_ref : &mut TakePicturesContext = unsafe{&mut *context_ptr};
                context_ref.finish_transfer = true;
                if context_ref.memory_transfer_callback_func.is_some(){
                    // download image to memory stream
                    Self::sub_memory_transfer(context_ref, dir_item, dir_item_info.size);
                }
                else if !context_ref.file_name.is_empty(){
                    // download image to strage
                    Self::sub_file_transfer(context_ref, dir_item, dir_item_info.size);
                }
            },
            _=>{}
        }

        unsafe{EdsRelease(dir_item)};
        return EDS_ERR_OK;
    }
    /// save to file
    fn sub_file_transfer(context : &TakePicturesContext, dir_item: EdsDirectoryItemRef, size: u64){
        let result = create_file_stream_scoped(&context.file_name, EdsFileCreateDisposition::kEdsFileCreateDisposition_CreateAlways, EdsAccess::kEdsAccess_ReadWrite);
        if result.is_ok(){
            let stream = result.unwrap();
            let _ = download(dir_item, size, stream.stream);
            let _ = download_complete(dir_item);
        }
        else{
            let _ = download_cancel(dir_item);
        }
    }
    /// save to memory
    fn sub_memory_transfer(context : &TakePicturesContext, dir_item: EdsDirectoryItemRef, size: u64){
        let result = create_memory_stream_scoped(size);
        if result.is_ok(){
            let stream = result.unwrap();
            let _ = download(dir_item, size, stream.stream);
            let _ = download_complete(dir_item);
            let data = get_pointer(stream.stream);
            let length = get_length(stream.stream);
            if data.is_ok() && length.is_ok(){
                context.memory_transfer_callback_func.unwrap()(unsafe{std::slice::from_raw_parts(data.unwrap() as *const u8, length.unwrap() as usize)});
            }
        }
        else{
            let _ = download_cancel(dir_item);
        }
    }
    /// get iso speed desc
    pub fn get_iso_speed_desc(&self)->Result<Vec<ISOSpeed>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_ISOSpeed, converter);
    }
    /// get iso speed
    pub fn get_iso_speed(&self)->Result<ISOSpeed, ErrorId>{
        return get_property_data::<ISOSpeed>(self.device, kEdsPropID_ISOSpeed, 0);
    }
    // set iso speed
    pub fn set_iso_speed(&self, iso_speed : ISOSpeed)->Result<bool, ErrorId>{
        return set_property_data(self.device, kEdsPropID_ISOSpeed, 0, &(iso_speed as u32));
    }
    /// get Av desc
    pub fn get_av_desc(&self)->Result<Vec<ApertureValue>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_Av, converter);
    }
    /// get Av
    pub fn get_av(&self)->Result<ApertureValue, ErrorId>{
        return get_property_data::<ApertureValue>(self.device, kEdsPropID_Av, 0);
    }
    /// set Av
    pub fn set_av(&self, av : ApertureValue)->Result<bool, ErrorId>{
        return set_property_data(self.device, kEdsPropID_Av, 0, &(av as u32));
    }
    /// get Tv desc
    pub fn get_tv_desc(&self)->Result<Vec<ShutterSpeed>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_Tv, converter);
    }
    /// get Tv
    pub fn get_tv(&self)->Result<ShutterSpeed, ErrorId>{
        return get_property_data::<ShutterSpeed>(self.device, kEdsPropID_Tv, 0);
    }
    /// set Tv
    pub fn set_tv(&self, tv : ShutterSpeed)->Result<bool, ErrorId>{
        return set_property_data(self.device, kEdsPropID_Tv, 0, &(tv as u32));
    }
    /// get battery level
    pub fn get_battery_level(&self)->Result<EdsBatteryLevel2, ErrorId>{
        return get_property_data::<EdsBatteryLevel2>(self.device, kEdsPropID_BatteryLevel, 0);
    }
    /// get image quality desc
    pub fn get_image_quality_desc(&self)->Result<Vec<EdsImageQuality>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_ImageQuality, converter);
    }
    /// get image quality
    pub fn get_image_quality(&self)->Result<EdsImageQuality, ErrorId>{
        return get_property_data::<EdsImageQuality>(self.device, kEdsPropID_ImageQuality, 0);
    }
    /// set image quality
    pub fn set_image_quality(&self, image_quality : EdsImageQuality)->Result<bool, ErrorId>{
        return set_property_data(self.device, kEdsPropID_ImageQuality, 0, &(image_quality as u32));
    }
    /// remote release to strage
    pub fn take_picture_to_strage(&self, file_name: &str)->Result<bool, ErrorId>{
        return self.take_picture_core(&mut TakePicturesContext{
            memory_transfer_callback_func: None,
            file_name: String::from(file_name),
            finish_transfer: false,
        });
    }
    /// remote release to memory
    pub fn take_picture_to_memory(&self, processor: fn(data: &[u8]))->Result<bool, ErrorId>{
        return self.take_picture_core(&mut TakePicturesContext{
            memory_transfer_callback_func: Some(processor),
            file_name: String::new(),
            finish_transfer: false,
        });
    }

    pub fn start_live_preview(&self)->Result<bool, ErrorId>{
        let evf_mode = EdsEvfAf::kEdsCameraCommand_EvfAf_ON as i32;
        let result = set_property_data(self.device, kEdsPropID_Evf_Mode, 0, &evf_mode);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let result = get_property_data::<i32>(self.device, kEdsPropID_Evf_OutputDevice, 0);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let mut out_device = result.unwrap();
        out_device |= EdsEvfOutputDevice::kEdsEvfOutputDevice_PC as i32;
        return set_property_data(self.device, kEdsPropID_Evf_OutputDevice, 0, &out_device);
    }
    pub fn stop_live_preview(&self)->Result<bool, ErrorId>{
        let result = get_property_data::<i32>(self.device, kEdsPropID_Evf_OutputDevice, 0);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let mut out_device = result.unwrap();
        out_device &= !(EdsEvfOutputDevice::kEdsEvfOutputDevice_PC as i32);
        return set_property_data(self.device, kEdsPropID_Evf_OutputDevice, 0, &out_device);
    }
    /// get live preview
    pub fn take_live_preview(&self, processor: fn(data: &[u8]))->Result<bool, ErrorId>{
        let result = create_memory_stream_scoped(0u64);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let stream = result.unwrap();
        {
            let result = create_evf_image_ref_scoped(stream.stream);
            if result.is_err(){
                return Err(result.unwrap_err());
            }
            let image_ref = result.unwrap();
            loop{
                let result = download_evf_image(self.device, image_ref.image_ref);
                match result{
                    Err(ErrorId::ObjectNotReady)=>{
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        continue;
                    },
                    Ok(true)=>{break},
                    _=>return Err(result.unwrap_err())
                }
            }
            let data = get_pointer(stream.stream);
            let length = get_length(stream.stream);
            if data.is_ok() && length.is_ok(){
                processor(unsafe{std::slice::from_raw_parts(data.unwrap() as *const u8, length.unwrap() as usize)});
            }
        }
        Ok(true)
    }

    /// get 4 bytes desc
    fn get_4bytes_desc<T, F: Fn(i32)->T>(&self, property_id : u32, converter: F)->Result<Vec<T>, ErrorId>{
        let result = get_property_desc(self.device, property_id);
        if result.is_ok(){
            let result = result.unwrap();
            let mut desc = Vec::with_capacity(result.len());
            for v in result{
                desc.push(converter(v));
            }
            Ok(desc)
        }
        else{
            Err(result.unwrap_err())
        }
    }
    // common function to take picture
    fn take_picture_core(&self, context : &mut TakePicturesContext)->Result<bool, ErrorId>{
        let result = set_property_data(self.device, kEdsPropID_SaveTo, 0, &(EdsSaveTo::kEdsSaveTo_Host as u32));
        if result.is_err(){
            return result;
        }
        let result = send_status_command(self.device, kEdsCameraStatusCommand_UILock, 0);
        if result.is_err(){
            return result;
        }
        let result = set_capacity(self.device, 
            EdsCapacity{
                numberOfFreeClusters : 0x7FFFFFFFi32,
                bytesPerSector : 0x1000i32,
                reset : true
            }
        );
        if result.is_err(){
            return result;
        }
        let result = send_status_command(self.device, kEdsCameraStatusCommand_UIUnLock, 0);
        if result.is_err(){
            return result;
        }
        let result = set_object_event_handler(self.device, kEdsObjectEvent_All, Self::dir_item_request_transfer_callback, context as *const TakePicturesContext as *const c_void);
        if result.is_err(){
            return result;
        }
        let result = send_command(self.device, kEdsCameraCommand_TakePicture, 0);
        if result.is_ok(){
            while !context.finish_transfer{
                unsafe{EdsGetEvent()};
                if !context.finish_transfer{
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
        return result;
    }
}

impl Drop for Session{
    fn drop(&mut self){
        if self.device != std::ptr::null(){
            let _ = close_session(self.device);
        }
    }
}

#[derive(Debug)]
struct ScopedStream
{
    pub stream      : EdsStreamRef
}
impl Drop for ScopedStream{
    fn drop(&mut self){
        if self.stream != std::ptr::null(){
            unsafe{EdsRelease(self.stream)};
        }
    }
}

#[derive(Debug)]
struct ScopedEvfImageRef{
    pub image_ref   : EdsEvfImageRef
}
impl Drop for ScopedEvfImageRef{
    fn drop(&mut self){
        if self.image_ref != std::ptr::null(){
            unsafe{EdsRelease(self.image_ref)};
        }
    }
}

/// Camera for control
pub struct Camera{
    /// internal object
    device : EdsCameraRef,
}
impl Camera{
    /// get device info
    pub fn get_device_info(&self)->Result<DeviceInfo, ErrorId>{
        return get_device_info(self.device);
    }
    /// open session
    pub fn open_session(&self)->Result<Session, ErrorId>{
        let result = open_session(self.device);
        return match result{
            Ok(true)=>Ok(
                Session{
                    device : self.device,
                }),
            _=>Err(result.unwrap_err())
        }
    }
}

impl Drop for Camera{
    fn drop(&mut self){
        if self.device != std::ptr::null_mut(){
            unsafe{EdsRelease(self.device);}
        }
    }
}

/// Edsdk wrapper
pub struct Library{
    initialized : bool
}

impl Drop for Library{
    fn drop(&mut self){
        let _ = terminate_sdk();
    }
}

impl Library{
    /// Initialize EDSDK and return the instance that can access the library functions.
    /// If you call multiple initialize() it will fail.
    pub fn initialize()->Result<Library, ErrorId>{
        let result = initialize_sdk();
        if result.is_ok(){
            let mut library = Library::new();
            library.initialized = true;
            Ok(library)
        }
        else{
            Err(result.unwrap_err())
        }
    }
    fn new()->Self{
        Library{initialized : false}
    }

    pub fn get_device_list(&self)->Vec<Camera>{
        let mut devices = Vec::new();
        if self.initialized{
            let camera_list = get_camera_list();
            if camera_list.is_ok(){
                let camera_list = camera_list.unwrap();
                let count = get_child_count(camera_list);
                if count.is_ok(){
                    for i in 0 .. count.unwrap(){
                        let device = get_child_at_index(camera_list, i);
                        if device.is_ok(){
                            devices.push(
                                Camera{
                                    device : device.unwrap(),
                                }
                            );
                        }
                    }
                }
            }
        }
        return devices;
    }
}


// wrap function

fn mask_error(v: u32)->u32{
    return v & EDS_ERRORID_MASK;
}
fn onvert_error(v: u32)->ErrorId{
    return unsafe { ::std::mem::transmute(v) };
}

/// EdsInitializeSDK wrapper
fn initialize_sdk()->Result<bool,ErrorId>{
    let result = unsafe{EdsInitializeSDK()};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsTerminateSDK wrapper
fn terminate_sdk()->Result<bool,ErrorId>{
    let result = unsafe{EdsTerminateSDK()};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetCameraList wrapper
fn get_camera_list()->Result<EdsCameraListMutRef, ErrorId>{
    let camera_list : EdsCameraListMutRef = std::ptr::null_mut();
    let result = unsafe{EdsGetCameraList(&camera_list)};
    return match result{
        EDS_ERR_OK=>Ok(camera_list),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsGetChildCount wrapper
fn get_child_count(camera_list : EdsCameraListRef)->Result<u32, ErrorId>{
    let mut count = 0u32;
    let result = unsafe{EdsGetChildCount(camera_list, &mut count)};
    return match result{
        EDS_ERR_OK=>Ok(count),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetChildAtIndex wrapper
fn get_child_at_index(camera_list : EdsCameraListRef, index : u32)->Result<EdsCameraRef, ErrorId>{
    let device : *mut c_void = std::ptr::null_mut();
    let result = unsafe{EdsGetChildAtIndex(camera_list, index as i32, &device)};
    return match result{
        EDS_ERR_OK=>Ok(device),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsGetDeviceInfo wrapper
fn get_device_info(device: EdsCameraRef)->Result<DeviceInfo, ErrorId>{
    let mut device_info = EdsDeviceInfo{
        szPortName:[0; EDS_MAX_NAME],
        szDeviceDescription:[0; EDS_MAX_NAME],
        deviceSubType: 0,
        reserved: 0
    };
    let result = unsafe{EdsGetDeviceInfo(device, &mut device_info)};
    if result == EDS_ERR_OK{
        let port;
        let description;
        unsafe{
            port = CStr::from_bytes_with_nul_unchecked(&device_info.szPortName);
            description = CStr::from_bytes_with_nul_unchecked(&device_info.szDeviceDescription);
        }
        let device_info = DeviceInfo{
            description : String::from(description.to_str().unwrap()),
            port : String::from(port.to_str().unwrap())
        };
        Ok(device_info)
    }
    else{
        Err(onvert_error(mask_error(result)))
    }
}
/// EdsOpenSession wrapper
fn open_session(device: EdsCameraRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsOpenSession(device)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsCloseSession wrapper
fn close_session(device: EdsCameraRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsCloseSession(device)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetPropertySize wrapper
#[allow(dead_code)]
fn get_property_size(device: EdsCameraRef, property_id : EdsPropertyID, param : i32)->Result<(EdsDataType, u32), ErrorId>{
    let mut data_type = EdsDataType::kEdsDataType_Unknown;
    let mut data_size = 0;
    let result = unsafe{EdsGetPropertySize(device, property_id, param, &mut data_type, &mut data_size)};
    return match result{
        EDS_ERR_OK=>Ok((data_type, data_size)),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetPropertyData wrapper
fn get_property_data<T>
(device: EdsCameraRef, property_id : EdsPropertyID, param : i32)->Result<T, ErrorId>
where
T:std::default::Default
{
    let mut val : T = std::default::Default::default();
    let result = unsafe{EdsGetPropertyData(device, property_id, param, std::mem::size_of_val(&val) as u32, &mut val as *mut T as *mut c_void)};
    return match result{
        EDS_ERR_OK=>Ok(val),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSetPropertyData wrapper
fn set_property_data<T>(device: EdsCameraRef, property_id : EdsPropertyID, param : i32, property_data : &T)->Result<bool, ErrorId>{
    let result = unsafe{EdsSetPropertyData(device, property_id, param, std::mem::size_of_val(property_data) as u32, property_data as *const T as *const c_void)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetPropertyDesc wrapper
fn get_property_desc(device: EdsCameraRef, property_id : EdsPropertyID)->Result<Vec<i32>, ErrorId>{
    let mut property_desc = EdsPropertyDesc{
        form : 0,
        access : 0,
        numElements : 0,
        propDesc : [0; 128]
    };
    let result = unsafe{EdsGetPropertyDesc(device, property_id, &mut property_desc)};
    return match result{
        EDS_ERR_OK=>Ok((&property_desc.propDesc[..property_desc.numElements as usize]).to_vec()),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSetObjectEventHandler wrapper
fn set_object_event_handler(device: EdsCameraRef, event : EdsObjectEvent, object_event_handler : EdsObjectEventHandler, context : *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsSetObjectEventHandler(device, event, object_event_handler, context)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSetCapacity wrapper
fn set_capacity(device: EdsCameraRef, capacity : EdsCapacity)->Result<bool, ErrorId>{
    let result = unsafe{EdsSetCapacity(device, capacity)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSendStatusCommand wrapper
fn send_status_command(device: EdsCameraRef, status_command : EdsCameraStatusCommand, param : i32)->Result<bool, ErrorId>{
    let result = unsafe{EdsSendStatusCommand(device, status_command, param)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSendCommand wrapper
fn send_command(device: EdsCameraRef, command : EdsCameraCommand, param : i32)->Result<bool, ErrorId>{
    let result = unsafe{EdsSendCommand(device, command, param)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetDirectoryItemInfo wrapper
fn get_directory_item_info(dir_item : EdsDirectoryItemRef)->Result<DirectoryItemInfo, ErrorId>{
    let mut dir_item_info = EdsDirectoryItemInfo{
        size: 0,
        isFolder: false,
        groupID: 0,
        option: 0,
        szFileName: [0; EDS_MAX_NAME],
        format: 0,
        dateTime: 0
    };
    let result = unsafe{EdsGetDirectoryItemInfo(dir_item, &mut dir_item_info)};
    return match result{
        EDS_ERR_OK=>Ok(
            DirectoryItemInfo{
                size: dir_item_info.size,
                is_folder: dir_item_info.isFolder,
                group_id: dir_item_info.groupID,
                option: dir_item_info.option,
                format: dir_item_info.format,
                date_time: dir_item_info.dateTime,
                file_name: String::from(unsafe{CStr::from_bytes_with_nul_unchecked(&dir_item_info.szFileName)}.to_str().unwrap())
            }),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsCreateFileStream wrapper
#[allow(dead_code)]
fn create_file_stream(file_name : &String, create_disposition : EdsFileCreateDisposition, desired_access : EdsAccess)->Result<EdsStreamRef, ErrorId>{
    let stream : EdsStreamMutRef = std::ptr::null_mut();
    let result = unsafe{EdsCreateFileStream(file_name.as_ptr(), create_disposition, desired_access, &stream)};
    return match result{
        EDS_ERR_OK=>Ok(stream),
        _=>Err(onvert_error(mask_error(result)))
    }
}
fn create_file_stream_scoped(file_name : &String, create_disposition : EdsFileCreateDisposition, desired_access : EdsAccess)->Result<ScopedStream, ErrorId>{
    let stream : EdsStreamMutRef = std::ptr::null_mut();
    let result = unsafe{EdsCreateFileStream(file_name.as_ptr(), create_disposition, desired_access, &stream)};
    return match result{
        EDS_ERR_OK=>Ok(ScopedStream{stream : stream}),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsCreateMemoryStream wrapper
#[allow(dead_code)]
fn create_memory_stream(buffer_size : u64)->Result<EdsStreamRef, ErrorId>{
    let stream : EdsStreamMutRef = std::ptr::null_mut();
    let result = unsafe{EdsCreateMemoryStream(buffer_size, &stream)};
    return match result{
        EDS_ERR_OK=>Ok(stream),
        _=>Err(onvert_error(mask_error(result)))
    }
}
fn create_memory_stream_scoped(buffer_size : u64)->Result<ScopedStream, ErrorId>{
    let stream : EdsStreamMutRef = std::ptr::null_mut();
    let result = unsafe{EdsCreateMemoryStream(buffer_size, &stream)};
    return match result{
        EDS_ERR_OK=>Ok(ScopedStream{stream : stream}),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsDownload wrapper
fn download(dir_item : EdsDirectoryItemRef, read_size : u64, stream : EdsStreamRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownload(dir_item, read_size, stream)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsDownloadCancel wrapper
fn download_cancel(dir_item : EdsDirectoryItemRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownloadCancel(dir_item)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsDownloadComplete wrapper
fn download_complete(dir_item : EdsDirectoryItemRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownloadComplete(dir_item)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsGetPointer wrapper
fn get_pointer(stream : EdsStreamRef)->Result<*const c_void, ErrorId>{
    let data = std::ptr::null_mut();
    let result = unsafe{EdsGetPointer(stream, &data)};
    return match result{
        EDS_ERR_OK=>Ok(data),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsGetLength wrapper
fn get_length(stream : EdsStreamRef)->Result<u64, ErrorId>{
    let mut length = 0u64;
    let result = unsafe{EdsGetLength(stream, &mut length)};
    return match result{
        EDS_ERR_OK=>Ok(length),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsCreateEvfImageRef wrapper
#[allow(dead_code)]
fn create_evf_image_ref(stream : EdsStreamRef)->Result<EdsEvfImageRef, ErrorId>{
    let evf_image = std::ptr::null_mut();
    let result = unsafe{EdsCreateEvfImageRef(stream, &evf_image)};
    return match result{
        EDS_ERR_OK=>Ok(evf_image),
        _=>Err(onvert_error(mask_error(result)))
    }
}
fn create_evf_image_ref_scoped(stream : EdsStreamRef)->Result<ScopedEvfImageRef, ErrorId>{
    let evf_image = std::ptr::null_mut();
    let result = unsafe{EdsCreateEvfImageRef(stream, &evf_image)};
    return match result{
        EDS_ERR_OK=>Ok(ScopedEvfImageRef{image_ref : evf_image}),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsDownloadEvfImage wrapper
fn download_evf_image(camera_ref : EdsCameraRef, evf_image_ref : EdsEvfImageRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownloadEvfImage(camera_ref, evf_image_ref)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
