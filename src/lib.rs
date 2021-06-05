pub mod native;

#[cfg(test)]
mod tests{
    use std::ffi::c_void;
    use std::io::Write;
    use super::native::edsdk::*;
    use super::native::edsdk_types::*;
    use super::native::edsdk_errors::*;
    use std::ffi::CStr;

    #[test]
    fn function_tests(){
        unsafe{
            let result = EdsInitializeSDK();
            writeln!(&mut std::io::stdout(), "EdsInitializeSDK() = {}", result).unwrap();

            // let cameraList : *mut std::ffi::c_void;
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