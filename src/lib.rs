pub mod native;

#[cfg(test)]
mod tests{
    use std::ffi::c_void;
    use std::io::Write;
    use super::native::edsdk::*;
    use super::native::edsdk_types::*;

    #[test]
    fn function_tests(){
        unsafe{
            let result = EdsInitializeSDK();
            writeln!(&mut std::io::stdout(), "EdsInitializeSDK() = {}", result).unwrap();

            // let cameraList : *mut std::ffi::c_void;
            let camera_list : *mut c_void = std::ptr::null_mut();
            let result = EdsGetCameraList(&camera_list);
            writeln!(&mut std::io::stdout(), "EdsGetCameraList() = {}", result).unwrap();

            let result = EdsTerminateSDK();
            writeln!(&mut std::io::stdout(), "EdsTerminateSDK() = {}", result).unwrap();
        }
    }
}