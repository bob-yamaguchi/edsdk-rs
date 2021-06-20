

#[cfg(test)]
mod native_tests{
    use std::ffi::c_void;
    use std::io::Write;
    use edsdk::native::edsdk::*;
    use edsdk::native::edsdk_types::*;
    use edsdk::native::edsdk_errors::*;
    use edsdk::types::*;
    use std::ffi::CStr;

    static mut EVENT_END : bool = false;
    extern "C" fn dir_item_created(in_event: u32, in_ref: EdsBaseRef, _in_context: *const c_void)->EdsError{
        writeln!(&mut std::io::stdout(), "dir_item_created is called {}", in_event).unwrap();
        if in_event == kEdsObjectEvent_DirItemRequestTransfer{
            unsafe{
                EVENT_END = true;
                let stream : *mut c_void = std::ptr::null_mut();
                let mut dir_item_info = EdsDirectoryItemInfo{
                    size: 0,
                    isFolder: false,
                    groupID: 0,
                    option: 0,
                    szFileName: [0; EDS_MAX_NAME],
                    format: 0,
                    dateTime: 0
                };
                let result = EdsGetDirectoryItemInfo(in_ref, &mut dir_item_info);
                writeln!(&mut std::io::stdout(), "EdsGetDirectoryItemInfo()={}", result).unwrap();
                let path = std::env::current_dir().unwrap();
                let ch_dest = path.to_string_lossy() + "\\native_tests_strage.cr2";
                let result = EdsCreateFileStream(ch_dest.as_ptr(), EdsFileCreateDisposition::kEdsFileCreateDisposition_CreateAlways, EdsAccess::kEdsAccess_ReadWrite, &stream);
                writeln!(&mut std::io::stdout(), "EdsCreateFileStream({})={}", ch_dest, result).unwrap();
                let result = EdsDownload(in_ref, dir_item_info.size, stream);
                writeln!(&mut std::io::stdout(), "EdsDownload()={}", result).unwrap();
                let result = EdsDownloadComplete(in_ref);
                writeln!(&mut std::io::stdout(), "EdsDownloadComplete()={}", result).unwrap();
                let result = EdsRelease(stream);
                writeln!(&mut std::io::stdout(), "EdsRelease(stream)={}", result).unwrap();
                let result = EdsRelease(in_ref);
                writeln!(&mut std::io::stdout(), "EdsRelease(in_ref)={}", result).unwrap();
            }
          }
          else {
            unsafe{
                let result = EdsRelease(in_ref);
                writeln!(&mut std::io::stdout(), "EdsRelease(in_ref)={}", result).unwrap();
            }
        }

        return EDS_ERR_OK;
    }
    #[test]
    fn function_tests(){
        unsafe{
            let result = EdsInitializeSDK();
            writeln!(&mut std::io::stdout(), "EdsInitializeSDK() = {}", result).unwrap();
            if result != EDS_ERR_OK{
                writeln!(&mut std::io::stderr(), "try to run test with a option -- --test-threads=1").unwrap();
                panic!("Make this test fail");
            }

            let camera_list : *mut c_void = std::ptr::null_mut();
            let result = EdsGetCameraList(&camera_list);
            writeln!(&mut std::io::stdout(), "EdsGetCameraList() = {}", result).unwrap();

            let mut camera_count : u32 = 0;
            let result = EdsGetChildCount(camera_list, &mut camera_count);
            writeln!(&mut std::io::stdout(), "EdsGetChildCount() = {}, {}", result, camera_count).unwrap();

            if camera_count > 0{
                let camera : *mut c_void = std::ptr::null_mut();
                let result = EdsGetChildAtIndex(camera_list, 0, &camera);
                writeln!(&mut std::io::stdout(), "EdsGetChildAtIndex() = {}", result).unwrap();
                if result == EDS_ERR_OK{
                    let mut device_info = EdsDeviceInfo{
                        szPortName:[0; EDS_MAX_NAME],
                        szDeviceDescription:[0; EDS_MAX_NAME],
                        deviceSubType: 0,
                        reserved: 0
                    };
                    let result = EdsGetDeviceInfo(camera, &mut device_info);
                    writeln!(&mut std::io::stdout(), "EdsGetDeviceInfo() = {}", result).unwrap();
                    if result == EDS_ERR_OK{
                        let name = CStr::from_bytes_with_nul_unchecked(&device_info.szPortName);
                        writeln!(&mut std::io::stdout(), "port name = {}", name.to_str().unwrap()).unwrap();
                        let description = CStr::from_bytes_with_nul_unchecked(&device_info.szDeviceDescription);
                        writeln!(&mut std::io::stdout(), "device description = {}", description.to_str().unwrap()).unwrap();
                    }
                    let result = EdsOpenSession(camera);
                    writeln!(&mut std::io::stdout(), "EdsOpenSession() = {}", result).unwrap();
                    if result == EDS_ERR_OK{
                        let mut data_type : EdsDataType = EdsDataType::kEdsDataType_Unknown;
                        let mut data_size : u32 = 0;
                        let result = EdsGetPropertySize(camera, kEdsPropID_BatteryLevel, 0, &mut data_type, &mut data_size);
                        writeln!(&mut std::io::stdout(), "EdsGetPropertySize(kEdsPropID_BatteryLevel) = {},{},{}", result, data_type as u32, data_size).unwrap();

                        let mut buttery_level : i32 = 0;
                        let result = EdsGetPropertyData(camera, kEdsPropID_BatteryLevel, 0, data_size, &mut buttery_level as *mut i32 as *mut c_void);
                        writeln!(&mut std::io::stdout(), "EdsGetPropertyData(kEdsPropID_BatteryLevel) = {},{}", result, buttery_level).unwrap();

                        let mut property_desc = EdsPropertyDesc{
                            form : 0,
                            access : 0,
                            numElements : 0,
                            propDesc : [0; 128]
                        };
                        let result = EdsGetPropertyDesc(camera, kEdsPropID_ISOSpeed, &mut property_desc);
                        writeln!(&mut std::io::stdout(), "EdsGetPropertyDesc(kEdsPropID_ISOSpeed) = {},{},{},{},{:?}", result,
                        property_desc.form, property_desc.access, property_desc.numElements, property_desc.propDesc).unwrap();

                        let image_quality = EdsImageQuality::EdsImageQuality_SR as u32;
                        //let image_quality = EdsImageQuality::EdsImageQuality_LJF as u32;
                        let result = EdsSetPropertyData(camera, kEdsPropID_ImageQuality, 0, std::mem::size_of_val(&image_quality) as u32, &image_quality as *const u32 as *const c_void);
                        writeln!(&mut std::io::stdout(), "EdsSetPropertyData(kEdsPropID_ImageQuality) = {}", result).unwrap();

                        let save_to = EdsSaveTo::kEdsSaveTo_Host as u32;
                        let result = EdsSetPropertyData(camera, kEdsPropID_SaveTo, 0, std::mem::size_of_val(&save_to) as u32, &save_to as *const u32 as *const c_void);
                        writeln!(&mut std::io::stdout(), "EdsSetPropertyData(kEdsPropID_SaveTo) = {}", result).unwrap();

                        let result = EdsSendStatusCommand(camera, kEdsCameraStatusCommand_UILock, 0);
                        writeln!(&mut std::io::stdout(), "EdsSendStatusCommand(kEdsCameraStatusCommand_UILock) = {}", result).unwrap();

                        let capacity = EdsCapacity{
                            numberOfFreeClusters : 0x7FFFFFFFi32,
                            bytesPerSector : 0x1000i32,
                            reset : true
                        };
                        let result = EdsSetCapacity(camera, capacity);
                        writeln!(&mut std::io::stdout(), "EdsSetCapacity() = {}", result).unwrap();

                        let result = EdsSendStatusCommand(camera, kEdsCameraStatusCommand_UIUnLock, 0);
                        writeln!(&mut std::io::stdout(), "EdsSendStatusCommand(kEdsCameraStatusCommand_UIUnLock) = {}", result).unwrap();

                        let result = EdsSetObjectEventHandler(camera, kEdsObjectEvent_All, dir_item_created, std::ptr::null());
                        writeln!(&mut std::io::stdout(), "EdsSetObjectEventHandler(kEdsObjectEvent_All) = {}", result).unwrap();

                        let iso = ISOSpeed::ISO100 as u32;
                        let result = EdsSetPropertyData(camera, kEdsPropID_ISOSpeed, 0, std::mem::size_of_val(&iso) as u32, &iso as *const u32 as *const c_void);
                        writeln!(&mut std::io::stdout(), "EdsSetPropertyData(kEdsPropID_ISOSpeed) = {}", result).unwrap();

                        let av = ApertureValue::Av2_8 as u32;
                        let result = EdsSetPropertyData(camera, kEdsPropID_Av, 0, std::mem::size_of_val(&av) as u32, &av as *const u32 as *const c_void);
                        writeln!(&mut std::io::stdout(), "EdsSetPropertyData(kEdsPropID_Av) = {}", result).unwrap();

                        let tv = ShutterSpeed::Tv1_20th as u32;
                        let result = EdsSetPropertyData(camera, kEdsPropID_Tv, 0, std::mem::size_of_val(&tv) as u32, &tv as *const u32 as *const c_void);
                        writeln!(&mut std::io::stdout(), "EdsSetPropertyData(kEdsPropID_Tv) = {}", result).unwrap();

                        for _ in 0..100{
                            // if focusable objects don"t exist, this command will fail and EdsCloseSession waits to turn off camera.
                            let result = EdsSendCommand(camera, kEdsCameraCommand_TakePicture, 0);
                            writeln!(&mut std::io::stdout(), "EdsSendCommand(kEdsCameraCommand_TakePicture) = {}", result).unwrap();
                            if result == EDS_ERR_OK{
                                let mut ev_check = 0;
                                EVENT_END = false;
                                while !EVENT_END{
                                    let result = EdsGetEvent();
                                    writeln!(&mut std::io::stdout(), "EdsGetEvent() for consuming a event = {}", result).unwrap();
                                    std::thread::sleep(std::time::Duration::from_secs(1));
                                    ev_check += 1;
                                    if ev_check > 5{break;}
                                }
                                break;
                            }
                            std::thread::sleep(std::time::Duration::from_millis(10));
                        }
                    }
                    let result = EdsCloseSession(camera);
                    writeln!(&mut std::io::stdout(), "EdsCloseSession() = {}", result).unwrap();
                }

                let result = EdsRelease(camera);
                writeln!(&mut std::io::stdout(), "EdsRelease(camera) = {}", result).unwrap();
            }

            let result = EdsRelease(camera_list);
            writeln!(&mut std::io::stdout(), "EdsRelease(camera_list) = {}", result).unwrap();

            let result = EdsTerminateSDK();
            writeln!(&mut std::io::stdout(), "EdsTerminateSDK() = {}", result).unwrap();
        }
    }
}

#[cfg(test)]
mod wrapper_tests{
    use std::io::{BufWriter, Write};
    use std::fs::File;
    use edsdk::native::edsdk_types::*;
    use edsdk::types::*;
    use edsdk::wrap::*;


    #[test]
    fn wrapper_tests(){
        let library = Library::initialize();
        if library.is_ok(){
            writeln!(&mut std::io::stdout(), "Library initialized succeeded").unwrap();
        }
        else{
            let err = library.err().unwrap();
            writeln!(&mut std::io::stderr(), "Library initialized failed {:?}", err).unwrap();
            writeln!(&mut std::io::stderr(), "try to run test with a option -- --test-threads=1").unwrap();
            panic!("Make this test fail");
        }
        let library = library.unwrap();
        let devices = library.get_device_list();
        let mut count = 0;
        for device in &devices{
            let device_info = device.get_device_info();
            if device_info.is_err(){
                let err = device_info.err().unwrap();
                writeln!(&mut std::io::stdout(), "get_device_info failed {:?}", err).unwrap();
                continue;
            }
            let device_info = device_info.unwrap();
            writeln!(&mut std::io::stdout(), "device[{}]", count).unwrap();
            writeln!(&mut std::io::stdout(), "description {}", device_info.description).unwrap();
            writeln!(&mut std::io::stdout(), "port {}", device_info.port).unwrap();
            count += 1;
        }
        for device in devices{
            let result = device.open_session();
            if result.is_ok(){
                let session = result.unwrap();
                let result = session.get_battery_level();
                writeln!(&mut std::io::stdout(), "get_battery_level {:?}", result).unwrap();
                let result = session.set_iso_speed(ISOSpeed::ISO100);
                writeln!(&mut std::io::stdout(), "set_iso_speed {:?}", result).unwrap();
                let result = session.set_av(ApertureValue::Av2_8);
                writeln!(&mut std::io::stdout(), "set_av {:?}", result).unwrap();
                let result = session.set_tv(ShutterSpeed::Tv1_20th);
                writeln!(&mut std::io::stdout(), "set_tv {:?}", result).unwrap();
                let result = session.set_image_quality(EdsImageQuality::EdsImageQuality_LR);
                writeln!(&mut std::io::stdout(), "set_image_quality {:?}", result).unwrap();
                let path = std::env::current_dir().unwrap();
                let ch_dest = path.to_string_lossy() + "\\wrapper_tests_strage.cr2";
                let result = session.take_picture_to_strage(ch_dest.as_ref());
                writeln!(&mut std::io::stdout(), "take_picture_to_strage {:?}", result).unwrap();
                let result = session.take_picture_to_memory(|data|{
                    let path = std::env::current_dir().unwrap();
                    let ch_dest = path.to_string_lossy() + "\\wrapper_tests_mem.cr2";
                    let mut writer = BufWriter::new(File::create(ch_dest.as_ref()).unwrap());
                    let _ = writer.write_all(data);
                    let _ = writer.flush();
                });
                writeln!(&mut std::io::stdout(), "take_picture_to_memory {:?}", result).unwrap();
                let result = session.start_live_preview();
                writeln!(&mut std::io::stdout(), "start_live_preview {:?}", result).unwrap();
                let result = session.take_live_preview(|data|{
                    let path = std::env::current_dir().unwrap();
                    let ch_dest = path.to_string_lossy() + "\\wrapper_tests_live.jpg";
                    let mut writer = BufWriter::new(File::create(ch_dest.as_ref()).unwrap());
                    let _ = writer.write_all(data);
                    let _ = writer.flush();
                });
                writeln!(&mut std::io::stdout(), "take_live_preview {:?}", result).unwrap();
            }
            break;
        }
    }
}
