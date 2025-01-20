use std::collections::HashMap;
use std::fmt::Error;
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation;

use crate::addr;
use crate::kernal::{process, zombie};

pub fn get_zombie_id() -> HashMap<String, u32> {
    HashMap::from([
        (String::from("普通僵尸"), 0),
        (String::from("路障僵尸"), 2),
    ])
}
struct Zombie {
    x: u32,
    y: u32,
}
impl Zombie {
    pub fn new() -> Zombie {
        Zombie { x: 0, y : 0 }
    }
}


fn add_zombie(process_handle: Foundation::HANDLE, zombies_ptr: u32, zombie_id: u32, ins_row: u32) -> Result<(), String> {
    // 首先找到最后一个活着的僵尸
    let mut alive_zombie_num = get_active_zombie_num(process_handle, zombies_ptr).expect("get activate zombie num failed");
    let mut cur_zombie_ptr = zombies_ptr;
    while alive_zombie_num != 0 {
        let mut if_exist: [u8; 2] = [0; 2];
        process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_IF_EXIST_RADDR, &mut if_exist).expect("judge if exist failed");
        let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };
        if if_exist != 0 {
            alive_zombie_num = alive_zombie_num - 1;
        }
        cur_zombie_ptr = move_to_next_zombie(process_handle, cur_zombie_ptr);
    }

    // 设置新僵尸的Y坐标
    let mut y_int =  (50 + 100 * ins_row).to_le_bytes();
    let mut y_float = (50f32 + 100f32 * ins_row as f32).to_le_bytes();
    process::write_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_Y_LOCATION_RADDR_INT, &mut y_int).expect("write new zombie's y location failed(int)");
    process::write_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_Y_LOCATION_RADDR_FLOAT, &mut y_float).expect("write new zombie's y location failed(float)");
    
    // 设置新僵尸的X坐标
    let mut x_int =  500u32.to_le_bytes();
    let mut x_float = 500f32.to_le_bytes();
    process::write_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_X_LOCATION_RADDR_INT, &mut x_int).expect("write new zombie's x location failed(int)");
    process::write_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_X_LOCATION_RADDR_FLOAT, &mut x_float).expect("write new zombie's x location failed(float)");

    // 设置新僵尸的存在性
    let mut exist = 100u16.to_le_bytes();
    process::write_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_IF_EXIST_RADDR, &mut exist).expect("set alive failed");


    Ok(())
}

fn move_to_next_zombie(process_handle: Foundation::HANDLE, zombies_ptr: u32) -> u32 {
    // 获取下一个僵尸的指针
    zombies_ptr + addr::NEXT_ZOMBIE_RADDR
}

fn set_alive_zombie_pattern(process_handle: Foundation::HANDLE,
                            raddr: u32,
                            set_val: &mut [u8]) -> Result<(), Error> {
    // 读取基表指针
    let mut base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, addr::BASE_ADDR, &mut base_ptr[..]).expect("Read Memory Failed");
    let base_ptr: u32 = unsafe { std::mem::transmute(base_ptr) };  

    // 获取游戏信息表指针
    let mut game_base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, base_ptr + addr::CUR_GAME_INFO_RADDR, &mut game_base_ptr[..]).expect("Read Memory Failed");
    let game_base_ptr: u32 = unsafe { std::mem::transmute(game_base_ptr) };
        
    // 获得僵尸序列指针
    let mut zombies_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::ZOMBIE_LS_RADDR, &mut zombies_ptr).expect("Read Memory Failed");
    let zombies_ptr: u32 = unsafe { std::mem::transmute(zombies_ptr) };

    let zombie_upper_num: u32 = 1000;
    let mut cur_zombie_ptr = zombies_ptr;
    for i in 0..zombie_upper_num {
        // 是否存在
        let mut if_exist: [u8; 2] = [0; 2];
        process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
        let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };
        if if_exist != 0 {
            // 将僵尸魅惑，如果存在的话
            process::write_memory_by_addr(process_handle, cur_zombie_ptr + raddr, set_val).expect("set pattern failed");
        }
        cur_zombie_ptr = move_to_next_zombie(process_handle, cur_zombie_ptr);
    }
    Ok(())
}

pub fn set_zombie_attracted(process_handle: Foundation::HANDLE) -> Result<(), Error> {
    let mut set_val: [u8; 1] = [1; 1];
    set_alive_zombie_pattern(process_handle,
                             addr::ZOMBIE_ATTRACTED_RADDR,
                             &mut set_val)
}

pub fn remove_zombie_armor1(process_handle: Foundation::HANDLE) -> Result<(), Error> {
    let mut set_val: [u8; 4] = [0xFF; 4];
    set_alive_zombie_pattern(process_handle,
                             addr::ZOMBIE_ARMOR1_BLOOD_RADDR,
                             &mut set_val).unwrap();
    let mut set_val: [u8; 4] = [0x0; 4];
    set_alive_zombie_pattern(process_handle,
                             addr::ZOMBIE_ARMOR1_RADDR,
                             &mut set_val)
}

pub fn remove_zombie_armor2(process_handle: Foundation::HANDLE) -> Result<(), Error> {
    let mut set_val: [u8; 4] = [0xFF; 4];
    set_alive_zombie_pattern(process_handle,
                             addr::ZOMBIE_ARMOR2_BLOOD_RADDR,
                             &mut set_val).unwrap();
    let mut set_val: [u8; 4] = [0x0; 4];
    set_alive_zombie_pattern(process_handle,
                             addr::ZOMBIE_ARMOR2_RADDR,
                             &mut set_val)
}

fn get_active_zombie_num(process_handle: Foundation::HANDLE, zombies_ptr: u32) -> Result<usize, String> {
    let zombie_upper_num: u32 = 10;
    let mut active_num: usize = 0;
    let mut cur_zombie_ptr = zombies_ptr;
    for i in 0..zombie_upper_num {
        // 是否存在
        let mut if_exist: [u8; 2] = [0; 2];
        process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
        let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };
        if true {
            // 僵尸是否存在
            println!("If This Zombie Exist?: {}", if_exist);

            // 僵尸所在行数
            let mut zombie_line: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_LINE_RADDR, &mut zombie_line);
            let zombie_line: u32 = unsafe { std::mem::transmute(zombie_line) };
            println!("\tZombie Line: {}", zombie_line);

            // 读取僵尸的类型
            let mut zombie_type: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_TYPE_RADDR, &mut zombie_type).expect("read zombie type failed");
            let zombie_type: u32 = unsafe { std::mem::transmute(zombie_type) };
            println!("\tZombie{} Type: {}", active_num, zombie_type);

            // 读取僵尸的X坐标
            let mut zombie_x_int: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_X_LOCATION_RADDR_INT, &mut zombie_x_int).expect("read zombie x location failed");
            let zombie_x_int: u32 = unsafe { std::mem::transmute(zombie_x_int) };
            
            // 读取僵尸的Y坐标
            let mut zombie_y_int: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_Y_LOCATION_RADDR_INT, &mut zombie_y_int).expect("read zombie x location failed");
            let zombie_y_int: u32 = unsafe { std::mem::transmute(zombie_y_int) };

            println!("\tZombie Location: ({}, {})", zombie_x_int, zombie_y_int);

            // 读取僵尸状态
            let mut zombie_status: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_STATUS_RADDR, &mut zombie_status).unwrap();
            let zombie_status: u32 = unsafe { std::mem::transmute(zombie_status) };
            println!("\tZombie Status is {}", zombie_status);

            // 读取僵尸已存在时间
            let mut zombie_exist_time: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_EXIST_TIME_RADDR, &mut zombie_exist_time);
            let zombie_exist_time: u32 = unsafe { std::mem::transmute(zombie_exist_time) };
            println!("\tZombie Exist Time: {}", zombie_exist_time);            
            
            // 读取僵尸是否被魅惑
            let mut zombie_attracted: [u8; 1] = [0; 1];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_ATTRACTED_RADDR, &mut zombie_attracted);
            let zombie_attracted: u8 = unsafe { std::mem::transmute(zombie_attracted) };
            println!("\tZombie Attracted: {}", zombie_attracted);

            // 读取僵尸护甲1血量
            let mut zombie_armor1: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_ARMOR1_BLOOD_RADDR, &mut zombie_armor1).expect("read blood1 failed");
            let zombie_armor1: u32 = unsafe { std::mem::transmute(zombie_armor1) };
            println!("\tZombie armor1 blood: {}", zombie_armor1);

            // 读取僵尸护甲2血量
            let mut zombie_armor2: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_zombie_ptr + addr::ZOMBIE_ARMOR2_BLOOD_RADDR, &mut zombie_armor2).expect("read zombie armor2 blood failed");
            let zombie_armor2: u32 = unsafe { std::mem::transmute(zombie_armor2) };
            println!("\tZombie armor2 blood: {}", zombie_armor2);

            // remove_zombie_armor1(process_handle);
            // remove_zombie_armor2(process_handle);

            active_num += 1;
        }
        cur_zombie_ptr = move_to_next_zombie(process_handle, cur_zombie_ptr);
    }
    Ok(active_num)
}

fn get_next_zombie(process_handle: Foundation::HANDLE, zombies_ptr: u32) -> Result<Zombie, ()> {
    let mut next_zombie = Zombie::new();

    // println!("下一个僵尸的指针地址: {:x}", zombies_ptr);
    // 获取X坐标
    let mut x_pos: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, zombies_ptr + addr::ZOMBIE_X_LOCATION_RADDR_INT, &mut x_pos).expect("load x location failed");
    let x_pos: u32 = unsafe { std::mem::transmute(x_pos) };

    // 获取Y坐标
    let mut y_pos: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, zombies_ptr + addr::ZOMBIE_Y_LOCATION_RADDR_INT, &mut y_pos).expect("load y location failed");
    let y_pos: u32 = unsafe { std::mem::transmute(y_pos) };

    // 存在时间
    let mut exist_time: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, zombies_ptr + addr::ZOMBIE_EXIST_TIME_RADDR, &mut exist_time).expect("load exist time failed");
    let exist_time: u32 = unsafe { std::mem::transmute(exist_time) };

    // 是否存在
    let mut if_exist: [u8; 2] = [0; 2];
    process::read_memory_by_addr(process_handle, zombies_ptr + addr::ZOMBIE_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
    let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };

    // println!("该僵尸是否存在: {}", { if_exist });
    // println!("next zombie location: ({}, {})", x_pos, y_pos);
    // println!("Zombie exist: {}", exist_time);

    Ok(next_zombie)
}

pub fn get_zombie_locations(process_handle: Foundation::HANDLE) -> Result<(), String> {
    // 读取基表指针
    let mut base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, addr::BASE_ADDR, &mut base_ptr[..]).expect("Read Memory Failed");
    let base_ptr: u32 = unsafe { std::mem::transmute(base_ptr) };  

    // 获取游戏信息表指针
    let mut game_base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, base_ptr + addr::CUR_GAME_INFO_RADDR, &mut game_base_ptr[..]).expect("Read Memory Failed");
    let game_base_ptr: u32 = unsafe { std::mem::transmute(game_base_ptr) };
        
    // 获得僵尸序列指针
    let mut zombie_list_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::ZOMBIE_LS_RADDR, &mut zombie_list_ptr).expect("Read Memory Failed");
    

    loop {
        thread::sleep(Duration::from_secs(1));
        let mut zombies_ptr: u32 = unsafe { std::mem::transmute(zombie_list_ptr) };
        println!("Active Zombie Number: {}", get_active_zombie_num(process_handle, zombies_ptr).expect("load activate number failed"));
        // for i in 0..zombie_num {
        //     get_next_zombie(process_handle, zombies_ptr);
        //     // backwards_zombie(process_handle, zombies_ptr);
        //     // zombies_ptr = move_to_next_zombie(process_handle, zombies_ptr);
        // }
        // add_zombie(process_handle, zombies_ptr, zombie_id, 1).expect("add zombie failed");
        // let_zombie_attacted(process_handle, zombies_ptr).expect("let zombie attracted failed");
    }
    
    Ok(())
}

