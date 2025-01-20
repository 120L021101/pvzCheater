use windows::Win32::Foundation;

use crate::addr;
use crate::kernal::process;

pub fn modify_sun_value(process_handle: Foundation::HANDLE, new_value: &u32) -> Result<(), String>{
    // 读取基表指针
    let mut base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, addr::BASE_ADDR, &mut base_ptr[..]).expect("Read Memory Failed");
    // 转换成指针
    let base_ptr: u32 = unsafe { std::mem::transmute(base_ptr) };  

    // 获取游戏信息表指针
    let mut game_base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, base_ptr + addr::CUR_GAME_INFO_RADDR, &mut game_base_ptr[..]).expect("Read Memory Failed");
    // 转换成指针
    let game_base_ptr: u32 = unsafe { std::mem::transmute(game_base_ptr) };  

    // 获取阳光地址
    let sun_val_addr = game_base_ptr + addr::SUN_RADDR;
    
    // 获取修改前阳光数值
    let mut sun_val: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, sun_val_addr, &mut sun_val[..]).expect("Read Memory Failed");
    let sun_val: u32 = unsafe { std::mem::transmute(sun_val) };
    println!("[modify_sun_value log] current sun val is {:?}", sun_val);

    // 修改阳光数值
    let mut new_value_bytes = new_value.to_le_bytes();
    process::write_memory_by_addr(process_handle, sun_val_addr, &mut new_value_bytes[..]).expect("Write Memory Failed");

    // 获取修改后阳光数值
    let mut sun_val: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, sun_val_addr, &mut sun_val[..]).expect("Read Memory Failed");
    let sun_val: u32 = unsafe { std::mem::transmute(sun_val) };
    println!("[modify_sun_value log] current sun val is {:?}", sun_val);

    Ok(())
}