

use memmap::Mmap;
use winapi::shared::minwindef::BOOL;
use winapi::shared::ntdef::LPCWSTR;
use winapi::um::winnt::SEMAPHORE_MODIFY_STATE;
use winapi::um::winnt::SYNCHRONIZE;
use windows::Win32::System::Diagnostics::Debug;
use core::panic;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io;  
use winapi::um::synchapi::{OpenSemaphoreW, ReleaseSemaphore, WaitForSingleObject};  
use winapi::um::winbase::INFINITE;  
use winapi::shared::ntdef::NULL;  
use std::ffi::OsStr;  
use std::os::windows::ffi::OsStrExt;  

// 引入依赖  
use serde::{Serialize, Deserialize};  
use serde_json::{from_str, to_string};  

pub fn get_message() -> Result<String, io::Error> {  
    let msg_file = File::open("D:\\pvzHE\\zzj_editor\\frontEnd\\msg_shared_memory")?;  
    let semaphore_ready = OsStr::new("Global\\data_ready").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();  
    let semaphore_received = OsStr::new("Global\\data_received").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();  
  
    // 打开或创建信号量
    let semaphore_ready = unsafe { 
        OpenSemaphoreW(
            SYNCHRONIZE | SEMAPHORE_MODIFY_STATE, 
            0x0 as BOOL, 
            semaphore_ready.as_ptr() as *const u16 as LPCWSTR
        )
    };  
  
    let semaphore_received = unsafe {  
        OpenSemaphoreW(
            SYNCHRONIZE | SEMAPHORE_MODIFY_STATE, 
            0x0 as BOOL, 
            semaphore_received.as_ptr() as *const u16 as LPCWSTR
        )  
    };  
    if semaphore_ready.is_null() || semaphore_received.is_null() {
        panic!("open semaphore failed!");
    }  
    // 等待前端写入数据  
    unsafe { WaitForSingleObject(semaphore_ready, INFINITE) };  
    
    // 打开并映射文件到内存  
    let mut mmap = unsafe { Mmap::map(&msg_file)? };  

    // 读取数据  
    let mut msg_length: [u8; 4] = [0; 4];
    msg_length.copy_from_slice(&mmap[..4]);
    let msg_length = u32::from_be_bytes(msg_length) as usize;
    let message = String::from_utf8_lossy(&mmap[4..(4 + msg_length)]);  
    println!("Received message from Python: {}", message);  
  
    // 通知前端已经被读取  
    unsafe { ReleaseSemaphore(semaphore_received, 1, NULL as *mut i32) };  

    Ok(message.to_string())  
}

#[derive(Debug)]
pub enum MISC {
    MODIFY_SUN,
    SET_PROCESS,
}
#[derive(Debug)]
pub enum PLANT {
    ALLOW_DUP,
    ALLOW_PENETRATION
}
#[derive(Debug)]
pub enum ZOMBIE {
    SET_ATTRACTED,
    REMOVE_ARMOR1,
    REMOVE_ARMOR2
}

#[derive(Debug)]
pub enum TASK {
    CLOSE,
    MISC_TASK(MISC),
    PLANT_TASK(PLANT),
    ZOMBIE_TASK(ZOMBIE), 
}

// 定义数据结构  
#[derive(Serialize, Deserialize, Debug)]  
struct TASK_MSG {  
    #[serde(rename="TASK")]
    task: String,  
    #[serde(rename="DATA")]
    data: String,
}

#[derive(Debug)]
pub struct TASK_DATA {
    pub task : TASK,
    pub data : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DATA_IN_MISC_SP {
    #[serde(rename="process_id")]
    pub process_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DATA_IN_MISC_MS {
    #[serde(rename="modify_sun_value")]
    pub modify_sun_value: u32,
}

pub fn parse_data_in_format<'a, DF: Serialize + Deserialize<'a>>(task:&'a TASK_DATA) -> DF {
    from_str::<'a>(task.data.as_str()).unwrap()
}

pub fn parse_msg_get_task(msg: &str) -> (TASK_DATA) {
    // 解析JSON  
    let user: serde_json::Result<TASK_MSG> = from_str(msg);  
  
    match user {  
        Ok(user) => {  
            println!("TASK: {}", user.task);  
            println!("DATA: {}", user.data);  
            match user.task.as_str() {  
                "SET_PROCESS" => {  
                    return TASK_DATA {   
                        task: TASK::MISC_TASK(MISC::SET_PROCESS),  
                        data: user.data   
                    };  
                },  
                "MODIFY_SUN_VALUE" => {
                    return TASK_DATA {   
                        task: TASK::MISC_TASK(MISC::MODIFY_SUN),  
                        data: user.data   
                    };  
                },
                "ATTRACT_ZOMBIE" => {
                    return TASK_DATA {
                        task: TASK::ZOMBIE_TASK(ZOMBIE::SET_ATTRACTED),
                        data: user.data
                    }
                },
                "REMOVE_ARMOR1" => {
                    return TASK_DATA {
                        task: TASK::ZOMBIE_TASK(ZOMBIE::REMOVE_ARMOR1),
                        data: user.data
                    }
                },
                "REMOVE_ARMOR2" => {
                    return TASK_DATA {
                        task: TASK::ZOMBIE_TASK(ZOMBIE::REMOVE_ARMOR2),
                        data: user.data
                    }
                },
                "ALLOW_DUPLICATION" => {
                    return TASK_DATA {
                        task: TASK::PLANT_TASK(PLANT::ALLOW_DUP),
                        data: user.data 
                    }
                },
                "ALLOW_PENETRATION" => {
                    return TASK_DATA {
                        task: TASK::PLANT_TASK(PLANT::ALLOW_PENETRATION),
                        data: user.data
                    }
                },
                _ => panic!("Unknown command!")  
            }
        },  
        Err(e) => panic!("Error parsing JSON: {}", e),  
    }  
}