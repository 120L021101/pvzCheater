use std::os::raw::c_void;

use windows::Win32::{
    Foundation, System::{Diagnostics::Debug, Threading::{self, PROCESS_VM_OPERATION}}
};

pub fn get_process_handle_by_pid(pid: u32) -> Result<Foundation::HANDLE, String> {
    
    let process_handle = unsafe {  
        Threading::OpenProcess(Threading::PROCESS_VM_READ | Threading::PROCESS_VM_WRITE | PROCESS_VM_OPERATION, false, pid)
            .expect("Failed to open process") 
    };

    // 检查进程句柄是否有效  
    if process_handle.is_invalid() {  
        return Err("Failed to open process".into());  
    }  

    // 附加debugger到pvz进程
    let result = unsafe { Debug::DebugActiveProcess(pid) }; // .expect("failed to debug it") };
    if result.is_err() {
        let result = result.unwrap_err();
        println!("{}", result.message());
    }
    Ok(process_handle)
}

pub fn read_memory_by_addr(process_handle: Foundation::HANDLE, address: u32, buf_ptr: &mut [u8]) -> Result<(), String> {
    let mut bytes_read: usize = 0;  
  
    // 读取进程的内存  
    let result = unsafe {  
        Debug::ReadProcessMemory(  
            process_handle,  
            address as *const usize as *const c_void,  
            buf_ptr.as_mut_ptr() as *mut c_void,  
            buf_ptr.len(),  
            Some(&mut bytes_read),  
        )  
    };  
    // 检查读取操作是否成功  
    return if result.is_err() {
        Err(format!("Failed to read process memory: {}", result.err().unwrap()))
    } else {
        Ok(())
    }
}

pub fn write_memory_by_addr(process_handle: Foundation::HANDLE, address: u32, buf_ptr: &mut [u8]) -> Result<(), String> {  
    // 读取进程的内存  
    let result = unsafe {  
        Debug::WriteProcessMemory(
            process_handle,
            address as *const usize as *const c_void,  
            buf_ptr.as_mut_ptr() as *mut c_void,  
            buf_ptr.len(),  
            Some(std::ptr::null_mut()),  
        )
    };  
    // 检查写入操作是否成功  
    if result.is_err() { 
        return Err(format!("Failed to write process memory: {}", result.err().unwrap()));  
    }  
    Ok(())
}