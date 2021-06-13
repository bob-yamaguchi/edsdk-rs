
use super::native::edsdk_types::*;
use super::native::edsdk_errors::*;
use super::native::edsdk::*;
use super::types::*;
use std::vec::Vec;
use std::result::Result;
use std::ffi::c_void;
use std::ffi::CStr;
//use std::ptr::{drop_in_place};

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
	pub date_time    : u32,
    pub file_name   : String
}

/// Camera for control
pub struct Camera{
    /// internal object
    device : *const std::ffi::c_void,
    /// # image transfer callback after remote release
    /// * `data`  slice to picture data
    /// * `size`  size of data
    memory_transfer_callback : Option<fn(data: &[u8], size : u64)->bool>,
    /// file name to save
    file_name : String,
    finish_transfer: bool
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
                                    memory_transfer_callback : None,
                                    file_name : String::new(),
                                    finish_transfer : false
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

impl Camera{
    /// callback function
    #[allow(non_upper_case_globals)]
    extern "C" fn dir_item_request_transfer_callback(event: u32, dir_item: *const c_void, context: *const c_void)->EdsError{
        match event{
            kEdsObjectEvent_DirItemRequestTransfer=>{
                let dir_item_info = get_directory_item_info(dir_item);
                if dir_item_info.is_err(){
                    return EDS_ERR_OK;
                }
                let dir_item_info = dir_item_info.unwrap();
                let camera_ptr : *mut Camera = context as *mut Camera;
                let camera : &mut Camera = unsafe{&mut *camera_ptr};
                camera.finish_transfer = true;
                if camera.memory_transfer_callback.is_some(){
                    // download image to memory stream
                }
                else if !camera.file_name.is_empty(){
                    // download image to strage
                    camera.file_transfer_callback(dir_item, dir_item_info.size);
                }
            },
            _=>{}
        }

        unsafe{EdsRelease(dir_item)};
        return EDS_ERR_OK;
    }
    /// get device info
    pub fn get_device_info(&self)->Result<DeviceInfo, ErrorId>{
        return get_device_info(self.device);
    }
    /// get iso speed desc
    pub fn get_iso_speed_desc(&self)->Result<Vec<ISOSpeed>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_ISOSpeed, converter);
    }
    /// get iso speed
    pub fn get_iso_speed(&self)->Result<ISOSpeed, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_property(kEdsPropID_ISOSpeed, converter);
    }
    // set iso speed
    pub fn set_iso_speed(&self, iso_speed : ISOSpeed)->Result<bool, ErrorId>{
        return self.set_4bytes_property(kEdsPropID_ISOSpeed, iso_speed as u32);
    }
    /// get Av desc
    pub fn get_av_desc(&self)->Result<Vec<ApertureValue>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_Av, converter);
    }
    /// get Av
    pub fn get_av(&self)->Result<ApertureValue, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_property(kEdsPropID_Av, converter);
    }
    /// set Av
    pub fn set_av(&self, av : ApertureValue)->Result<bool, ErrorId>{
        return self.set_4bytes_property(kEdsPropID_Av, av as u32);
    }
    /// get Tv desc
    pub fn get_tv_desc(&self)->Result<Vec<ShutterSpeed>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_Tv, converter);
    }
    /// get Tv
    pub fn get_tv(&self)->Result<ShutterSpeed, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_property(kEdsPropID_Tv, converter);
    }
    /// set Tv
    pub fn set_tv(&self, tv : ShutterSpeed)->Result<bool, ErrorId>{
        return self.set_4bytes_property(kEdsPropID_Tv, tv as u32);
    }
    /// get battery level
    pub fn get_battery_level(&self)->Result<EdsBatteryLevel2, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_property(kEdsPropID_BatteryLevel, converter);
    }
    /// get image quality desc
    pub fn get_image_quality_desc(&self)->Result<Vec<EdsImageQuality>, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_desc(kEdsPropID_ImageQuality, converter);
    }
    /// get image quality
    pub fn get_image_quality(&self)->Result<EdsImageQuality, ErrorId>{
        let converter = |v| unsafe { ::std::mem::transmute(v) };
        return self.get_4bytes_property(kEdsPropID_ImageQuality, converter);
    }
    /// set image quality
    pub fn set_image_quality(&self, image_quality : EdsImageQuality)->Result<bool, ErrorId>{
        return self.set_4bytes_property(kEdsPropID_ImageQuality, image_quality as u32);
    }
    /// remote release to strage
    pub fn take_picture_to_strage(&mut self, file_name: &str)->Result<bool, ErrorId>{
        self.file_name = String::from(file_name);
        self.finish_transfer = false;
        return self.do_something_between_session(
            ||{
                let result = set_property_data(self.device, kEdsPropID_SaveTo, 0, 4, &(EdsSaveTo::kEdsSaveTo_Host as u32) as *const u32 as *const c_void);
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
                let result = set_object_event_handler(self.device, kEdsObjectEvent_All, Camera::dir_item_request_transfer_callback, self as *const Camera as *const c_void);
                if result.is_err(){
                    return result;
                }
                let result = send_command(self.device, kEdsCameraCommand_TakePicture, 0);
                if result.is_ok(){
                    while !self.finish_transfer{
                        unsafe{EdsGetEvent()};
                        if !self.finish_transfer{
                            std::thread::sleep(std::time::Duration::from_millis(10));
                        }
                    }
                }
                return result;
            }
        );
    }


    fn do_something_between_session<F:Fn()->Result<bool, ErrorId>>(&self, function: F)->Result<bool, ErrorId>{
        let result = open_session(self.device);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let result = function();
        let _ = close_session(self.device);
        return result;
    }
    /// get 4 bytes desc
    fn get_4bytes_desc<T, F: Fn(i32)->T>(&self, property_id : u32, converter: F)->Result<Vec<T>, ErrorId>{
        let result = open_session(self.device);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let result = get_property_desc(self.device, property_id);
        let _ = close_session(self.device);
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
    /// get 4 bytes property
    fn get_4bytes_property<T, F: Fn(i32)->T>(&self, property_id : u32, converter: F)->Result<T, ErrorId>{
        let mut property = 0i32;
        let result = open_session(self.device);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let result = get_property_data(self.device, property_id, 0, 4, &mut property as *mut i32 as *mut c_void);
        let _ = close_session(self.device);
        return match result{
            Ok(true)=>Ok(converter(property)),
            _=>Err(result.unwrap_err())
        }
    }
    /// set 4 bytes property
    fn set_4bytes_property(&self, property_id : u32, value : u32)->Result<bool, ErrorId>{
        let result = open_session(self.device);
        if result.is_err(){
            return Err(result.unwrap_err());
        }
        let result = set_property_data(self.device, property_id, 0, 4, &value as *const u32 as *const c_void);
        let _ = close_session(self.device);
        return result;
    }

    /// save to file
    fn file_transfer_callback(&self, dir_item: *const c_void, size: u64)->bool{
        let result = create_file_stream(&self.file_name, EdsFileCreateDisposition::kEdsFileCreateDisposition_CreateAlways, EdsAccess::kEdsAccess_ReadWrite);
        if result.is_ok(){
            let stream = result.unwrap();
            let _ = download(dir_item, size, stream);
            let _ = download_complete(dir_item);
            unsafe{EdsRelease(stream)};
        }
        return true;
    }
}

impl Drop for Camera{
    fn drop(&mut self){
        if self.device != std::ptr::null_mut(){
            unsafe{
                EdsRelease(self.device);
            }
        }
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
fn get_camera_list()->Result<*mut c_void, ErrorId>{
    let camera_list : *mut c_void = std::ptr::null_mut();
    let result = unsafe{EdsGetCameraList(&camera_list)};
    return match result{
        EDS_ERR_OK=>Ok(camera_list),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsGetChildCount wrapper
fn get_child_count(camera_list : *const c_void)->Result<u32, ErrorId>{
    let mut count = 0u32;
    let result = unsafe{EdsGetChildCount(camera_list, &mut count)};
    return match result{
        EDS_ERR_OK=>Ok(count),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetChildAtIndex wrapper
fn get_child_at_index(camera_list : *const c_void, index : u32)->Result<*const c_void, ErrorId>{
    let device : *mut c_void = std::ptr::null_mut();
    let result = unsafe{EdsGetChildAtIndex(camera_list, index as i32, &device)};
    return match result{
        EDS_ERR_OK=>Ok(device),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsGetDeviceInfo wrapper
fn get_device_info(device: *const c_void)->Result<DeviceInfo, ErrorId>{
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
fn open_session(device: *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsOpenSession(device)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsCloseSession wrapper
fn close_session(device: *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsCloseSession(device)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetPropertySize wrapper
#[allow(dead_code)]
fn get_property_size(device: *const c_void, property_id : EdsPropertyID, param : i32)->Result<(EdsDataType, u32), ErrorId>{
    let mut data_type = EdsDataType::kEdsDataType_Unknown;
    let mut data_size = 0;
    let result = unsafe{EdsGetPropertySize(device, property_id, param, &mut data_type, &mut data_size)};
    return match result{
        EDS_ERR_OK=>Ok((data_type, data_size)),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetPropertyData wrapper
fn get_property_data(device: *const c_void, property_id : EdsPropertyID, param : i32, property_size : u32,
    property_data : *mut c_void )->Result<bool, ErrorId>{
    let result = unsafe{EdsGetPropertyData(device, property_id, param, property_size, property_data)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSetPropertyData wrapper
fn set_property_data(device: *const c_void, property_id : EdsPropertyID, param : i32, property_size : u32, property_data : *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsSetPropertyData(device, property_id, param, property_size, property_data)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetPropertyDesc wrapper
fn get_property_desc(device: *const c_void, property_id : EdsPropertyID)->Result<Vec<i32>, ErrorId>{
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
fn set_object_event_handler(device: *const c_void, event : EdsObjectEvent, object_event_handler : EdsObjectEventHandler, context : *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsSetObjectEventHandler(device, event, object_event_handler, context)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSetCapacity wrapper
fn set_capacity(device: *const c_void, capacity : EdsCapacity)->Result<bool, ErrorId>{
    let result = unsafe{EdsSetCapacity(device, capacity)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSendStatusCommand wrapper
fn send_status_command(device: *const c_void, status_command : EdsCameraStatusCommand, param : i32)->Result<bool, ErrorId>{
    let result = unsafe{EdsSendStatusCommand(device, status_command, param)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsSendCommand wrapper
fn send_command(device: *const c_void, command : EdsCameraCommand, param : i32)->Result<bool, ErrorId>{
    let result = unsafe{EdsSendCommand(device, command, param)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}

/// EdsGetDirectoryItemInfo wrapper
fn get_directory_item_info(dir_item : *const c_void)->Result<DirectoryItemInfo, ErrorId>{
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
fn create_file_stream(file_name : &String, create_disposition : EdsFileCreateDisposition, desired_access : EdsAccess)->Result<*const c_void, ErrorId>{
    let stream : *mut c_void = std::ptr::null_mut();
    let result = unsafe{EdsCreateFileStream(file_name.as_ptr(), create_disposition, desired_access, &stream)};
    return match result{
        EDS_ERR_OK=>Ok(stream),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsDownload wrapper
fn download(dir_item : *const c_void, read_size : u64, stream : EdsStreamRef)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownload(dir_item, read_size, stream)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsDownloadCancel wrapper
#[allow(dead_code)]
fn download_cancel(dir_item : *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownloadCancel(dir_item)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
/// EdsDownloadComplete wrapper
fn download_complete(dir_item : *const c_void)->Result<bool, ErrorId>{
    let result = unsafe{EdsDownloadComplete(dir_item)};
    return match result{
        EDS_ERR_OK=>Ok(true),
        _=>Err(onvert_error(mask_error(result)))
    }
}
