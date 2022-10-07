use winapi::um::winsvc::{QueryServiceStatusEx, OpenServiceA, OpenSCManagerA};
use std::ptr;



pub fn get_service_state(service_short_name: &str) -> i32 {
    unsafe {
        let sc_handle = OpenSCManagerA(ptr::null(), ptr::null(), 0xF003F);
        let open_service = OpenServiceA(sc_handle, service_short_name.as_ptr() as *const i8, 0xF003F);
        
        let insufficent_buffer: *mut u32 = 0 as *mut u32;
        let buffer: *mut u8 = 0 as *mut u8;
    
    
        let service_enabled: i32 = QueryServiceStatusEx(open_service, 0x00000010, buffer, 0, insufficent_buffer);
        return service_enabled;
    }

}