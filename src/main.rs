mod kernal;
mod addr;

use core::panic;

use kernal::{
    message::{self, DATA_IN_MISC_MS, DATA_IN_MISC_SP, ZOMBIE}, plant, process, sun, zombie
};
use winapi::{ctypes::c_void, um::handleapi::CloseHandle};
use windows::Win32::Foundation::HANDLE;

fn main() -> Result<(), String> {  
    // let mut f_val: [u8; 4] = [0xA0, 0xAE, 0x98, 0x3E];
    // let f_val: f32 = unsafe { std::mem::transmute(f_val) };
    // println!("{}", f_val);
    // return Ok(());
    let mut process_id: u32 = 35156;  
    let mut process_handle: Option<HANDLE> = None;
    // let mut process_handle =  Option::Some(process::get_process_handle_by_pid(process_id).expect("failed to get handle"));

    while true {
        let msg = match message::get_message() {
            Ok(s) => s,
            Err(e) => {
                panic!("I don't know how to resolve, honestly. {:?}", e);
            }
        };
        let task_msg = message::parse_msg_get_task(&msg);
        match &task_msg.task {
        message::TASK::MISC_TASK(misc_task) => {
            match &misc_task {
            message::MISC::SET_PROCESS => {
                let data_in_misc_sp: DATA_IN_MISC_SP = message::parse_data_in_format(&task_msg);
                process_id = data_in_misc_sp.process_id;
                match process_handle {
                    Some(ph) => {unsafe { /* 不会释放 */ }},
                    None => {},
                }
                process_handle = Option::Some(process::get_process_handle_by_pid(process_id).expect("failed to get handle"));
                println!("set process id to: {}", process_id);
                let mut buf_ptr = [0xE9, 0xD6, 0xB7, 0x30, 0x00];
                process::write_memory_by_addr(process_handle.unwrap(), 0x489825u32, &mut buf_ptr).unwrap();
            },
            message::MISC::MODIFY_SUN => {
                let data_in_misc_ms: DATA_IN_MISC_MS = message::parse_data_in_format(&task_msg);
                sun::modify_sun_value(process_handle.unwrap(), &data_in_misc_ms.modify_sun_value)
                        .expect("modify sun value failed");
                println!("modify sun value to: {}", data_in_misc_ms.modify_sun_value);
            },
            _ => {},
            }
        },
        message::TASK::ZOMBIE_TASK(zombie_task) => {
            match &zombie_task {
            message::ZOMBIE::SET_ATTRACTED => { zombie::set_zombie_attracted(process_handle.unwrap()).expect("set zombie attracted failed"); },
            message::ZOMBIE::REMOVE_ARMOR1 => { zombie::remove_zombie_armor1(process_handle.unwrap()).expect("remove armor1 failed"); }
            message::ZOMBIE::REMOVE_ARMOR2 => { zombie::remove_zombie_armor2(process_handle.unwrap()).expect("remove armor2 failed"); }
            _ => {},
            }   
        },
        message::TASK::PLANT_TASK(plant_task) => {
            match &plant_task {
            message::PLANT::ALLOW_DUP => { plant::allow_duplicate(process_handle.unwrap()).unwrap(); }
            message::PLANT::ALLOW_PENETRATION => {  },
            _ => {},
            }
        },
        _ => panic!("Unknown task")
        }
    }

    // zombie::get_zombie_locations(process_handle.unwrap()).expect("zombie list failed");
    // plant::get_plant_locations(process_handle.unwrap()).expect("plants list failed");
    Ok(())  
}


