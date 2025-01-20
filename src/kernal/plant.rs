use std::collections::HashMap;
use std::fmt::Error;
use std::thread;
use std::time::Duration;

use winapi::um::winnt::PROCESS_DUP_HANDLE;
use windows::Win32::Foundation;

use crate::addr;
use crate::kernal::process;



pub fn create_plant(process_handle: Foundation::HANDLE, plant_type: u32, row: u32, col: u32, num: u32) -> Result<(), Error> {
    // 读取基表指针
    let mut base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, addr::BASE_ADDR, &mut base_ptr[..]).expect("Read Memory Failed");
    let base_ptr: u32 = unsafe { std::mem::transmute(base_ptr) };  

    // 获取游戏信息表指针
    let mut game_base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, base_ptr + addr::CUR_GAME_INFO_RADDR, &mut game_base_ptr[..]).expect("Read Memory Failed");
    let game_base_ptr: u32 = unsafe { std::mem::transmute(game_base_ptr) };
        
    // 获得植物序列指针
    let mut plants_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_LS_RADDR, &mut plants_ptr).expect("Read Memory Failed");
    let plants_ptr: u32 = unsafe { std::mem::transmute(plants_ptr) };


    let mut cur_plant_ptr = plants_ptr;
    let mut remaining_num = num;
    loop {
        if remaining_num == 0 {
            break;
        }
        // 当前植物是否存在，若不存在（或死亡），擦写复用该片内存
        let mut if_exist: [u8; 2] = [0; 2];
        process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
        let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };
        if true {
            // 擦除内存
            let mut clear_memory: [u8; addr::NEXT_PLANT_RADDR as usize] = [0x0; addr::NEXT_PLANT_RADDR as usize];
            process::write_memory_by_addr(process_handle, cur_plant_ptr, &mut clear_memory).unwrap();

            // 写入基址
            let mut base_ptr_writein = base_ptr.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_BASE_RADDR, &mut base_ptr_writein).unwrap();

            // 写入游戏信息表地址
            let mut game_base_ptr_writein = game_base_ptr.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_GAME_INFO_RADDR, &mut game_base_ptr_writein).unwrap();

            // 写入植物横坐标
            let x_loc: u32 = 40 + row * 80;
            let mut x_loc = x_loc.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_X_LOCATION_RADDR_INT, &mut x_loc).unwrap();

            // 写入植物纵坐标
            let y_loc: u32 = 80 + col * 100;
            let mut y_loc = y_loc.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_Y_LOCATION_RADDR_INT, &mut y_loc).unwrap();

            // 固定值
            let mut fixed_val: [u8; 8] = [0x50, 0, 0, 0, 0x50, 0, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_FIXED1_RADDR, &mut fixed_val).unwrap();

            // 设置为可见
            let mut if_visible: [u8; 4] = [0x1, 0, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IF_VISIBLE_RADDR, &mut if_visible).unwrap();

            // 写入植物所在行数
            let mut row_writein = row.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ROW_RADDR, &mut row_writein).unwrap();

            // 写入图像图层，见下方“获取下一个植物的编号，并加一”

            // 写入植物类型
            let mut plant_type_writein = plant_type.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_TYPE_RADDR, &mut plant_type_writein).unwrap();

            // 写入植物所在列数
            let mut col_writein = col.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_COL_RADDR, &mut col_writein).unwrap();

            // 写入植物动画倒计时
            let mut anima_clock: [u8; 4] = (10 as u32).to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ANIMA_CLOCK_RADDR, &mut anima_clock).unwrap();

            // 写入固定值
            let mut fixed_val2: [u8; 12] = [0x02, 00, 00, 00, 0x10, 00, 00, 00, 0x05, 00, 00, 00];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_FIXED2_RADDR, &mut fixed_val2).unwrap();

            // 写入植物状态
            let mut plant_status: [u8; 4] = [0x0; 4];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_STATUS_RADDR, &mut plant_status).unwrap();

            // 写入植物血量
            let mut cur_blood: [u8; 4] = [0x2C, 0x1, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_CURBLOOD_RADDR, &mut cur_blood).unwrap();

            // 写入血值上限（初始生命值）
            let mut max_blood: [u8; 4] = [0x2C, 0x1, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_MAXBLOOD_RADDR, &mut max_blood).unwrap();

            // 是否可以攻击？
            let mut attackable: [u8; 4] = [0x1, 0, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ALLOW_ATTACK_RADDR, &mut attackable).unwrap();

            // 属性倒计时
            let mut attribute_clock: [u8; 4] = [0x0, 0, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ATTRIBUTE_CLOCK_RADDR, &mut attribute_clock).unwrap();

            // 行为倒计时
            let mut action_clock: [u8; 4] = [0x64, 0, 0, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ACTION_CLOCK_RADDR, &mut action_clock).unwrap();

            // 写入大炮横坐标
            let mut bomb_x_location: [u8; 4] = [0xFF; 4];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::BOMB_X_LOCATION_RADDR, &mut bomb_x_location).unwrap();
            // 写入大炮纵坐标
            let mut bomb_y_location: [u8; 4] = [0xFF; 4];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::BOMB_Y_LOCATION_RADDR, &mut bomb_y_location).unwrap();

            // 动画编号1
            let mut animation1: [u8; 4] = (3585867785 as u32).to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ANIMATION1_RADDR, &mut animation1).unwrap();

            // 动画编号2
            let mut animation2: [u8; 4] = (3585933315 as u32).to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ANIMATION2_RADDR, &mut animation2).unwrap();

            // 行为倒计时
            let mut daction_clock: [u8; 2] = [0x00, 0x05];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_DACTION_CLOCK_RADDR, &mut daction_clock).unwrap();

            // 写入固定值（有可能与方向有关）
            let mut fixed_val3: [u8; 8] = [0xFF; 8];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_FIXED3_RADDR, &mut fixed_val3).unwrap();

            // 设置属性
            let mut assign_attributes: [u8; 5] = [0, 0, 0, 0, 1];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ATTRIBUTE_RADDR, &mut assign_attributes).unwrap();

            // 获取下一个植物的编号，并加一
            let mut next_plant_id: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_NEXTID_RADDR, &mut next_plant_id).unwrap();
            let mut next_plant_id: u32 = unsafe { std::mem::transmute(next_plant_id) };
            // 计算图像图层
            let mut image_layer = (302765 + 9920 * next_plant_id).to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IMAGE_LAYER_RADDR, &mut image_layer).unwrap();
            next_plant_id += 1;
            // 回写编号
            let mut next_plant_id = next_plant_id.to_le_bytes();
            process::write_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_NEXTID_RADDR, &mut next_plant_id).unwrap();

            // 获取最后一个植物的编号，加一回写，并写入到植物列表里新增植物内存
            let mut last_plant_id: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_LASTID_RADDR, &mut last_plant_id).unwrap();
            let mut last_plant_id: u32 = unsafe { std::mem::transmute(last_plant_id) };
            last_plant_id += 1;
            let mut last_plant_id = last_plant_id.to_le_bytes();
            process::write_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_LASTID_RADDR, &mut last_plant_id).unwrap();

            // 写入到新增植物内存
            let mut last_plant_id: [u8; 2] = unsafe { std::mem::transmute::<[u8; 4], u32>(last_plant_id) as u16}.to_le_bytes();
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ID_RADDR, &mut last_plant_id).unwrap();


            // 设置存在性
            let mut if_exist: [u8; 2] = [0x1, 0];
            process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IF_EXIST_RADDR, &mut if_exist).unwrap();


            remaining_num -= 1;
        }
        cur_plant_ptr = move_to_next_plant(cur_plant_ptr);
    }

    Ok(())
}

fn move_to_next_plant(plant_ptr: u32) -> u32 {
    plant_ptr + addr::NEXT_ZOMBIE_RADDR
}

fn set_alive_plant_pattern(process_handle: Foundation::HANDLE, raddr: u32, set_val: &mut [u8]) -> Result<(), Error> {
    // 读取基表指针
    let mut base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, addr::BASE_ADDR, &mut base_ptr[..]).expect("Read Memory Failed");
    let base_ptr: u32 = unsafe { std::mem::transmute(base_ptr) };  

    // 获取游戏信息表指针
    let mut game_base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, base_ptr + addr::CUR_GAME_INFO_RADDR, &mut game_base_ptr[..]).expect("Read Memory Failed");
    let game_base_ptr: u32 = unsafe { std::mem::transmute(game_base_ptr) };
        
    // 获得植物序列指针
    let mut plants_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_LS_RADDR, &mut plants_ptr).expect("Read Memory Failed");
    let plants_ptr: u32 = unsafe { std::mem::transmute(plants_ptr) };

    let plant_upper_num: u32 = 1000;
    let mut cur_plant_ptr = plants_ptr;
    for i in 0..plant_upper_num {
        // 是否存在
        let mut if_exist: [u8; 2] = [0; 2];
        process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
        let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };
        if if_exist != 0 {
            // 处理植物，如果存在的话
            process::write_memory_by_addr(process_handle, cur_plant_ptr + raddr, set_val).expect("set pattern failed");
        }
        cur_plant_ptr = move_to_next_plant(cur_plant_ptr);
    }
    Ok(())
}

pub fn allow_duplicate(process_handle: Foundation::HANDLE) -> Result<(), Error> {
  
    // 创建一个新的线程，并将参数e传递给闭包  
    let handle = thread::spawn(move || {  
        // 原理，将所有的植物所在行数和列数都清成0, 0，这样可以规避叠放检查
        // 轮询
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100)); 
            let mut row: [u8; 4] = [0; 4];
            set_alive_plant_pattern(process_handle, addr::PLANT_ROW_RADDR, &mut row).unwrap();
            let mut col: [u8; 4] = [0; 4];
            set_alive_plant_pattern(process_handle, addr::PLANT_COL_RADDR, &mut col).unwrap();
        }
    });  
    
    Ok(())
}

pub fn allow_penetration(process_handle: Foundation::HANDLE) -> Result<(), Error> {
/*  等价Cheat Engine代码
46e429:
jmp L1

795100:
L1:
cmp [eax+5c], #0
je short L2
jmp 0046EB20
L2:
ret
*/

    let mut code_zhuru = [0xE9, 0xD2, 0x6C, 0x32, 0x00];
    let mut code_tail_str = "83 78 5C 00 74 05 E9 15 9A CD FF C3".split_whitespace();
    let mut code_tail: Vec<u8> = vec![];
    for code in code_tail_str {
        code_tail.push(u8::from_str_radix(code, 16).unwrap());
    }
    process::write_memory_by_addr(process_handle, 0x46e429, &mut code_zhuru).unwrap();
    process::write_memory_by_addr(process_handle, 0x46EB20, &mut code_tail).unwrap();

    Ok(())
}



fn get_active_plant_num(process_handle: Foundation::HANDLE, plants_ptr: u32) -> Result<usize, String> {
    let zombie_upper_num: u32 = 10;
    let mut active_num: usize = 0;
    let mut cur_plant_ptr = plants_ptr;
    // create_plant(process_handle, 0x0, 0x2, 0x2, 0x1).unwrap();
    for i in 0..zombie_upper_num {
        // 是否存在
        let mut if_exist: [u8; 2] = [0;2];
        process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
        let if_exist: u16 = unsafe { std::mem::transmute(if_exist) };
        // if if_exist {
        if true {
            // 植物是否存在
            println!("If This Plant Exist?: {}", if_exist);
            
            // 获取植物编号
            let mut plant_id: [u8; 2] = [0; 2];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ID_RADDR, &mut plant_id).unwrap();
            let plant_id: u16 = unsafe { std::mem::transmute(plant_id) };
            println!("plant id: {}", plant_id);

            // 获取植物类型
            let mut plant_type: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_TYPE_RADDR, &mut plant_type).unwrap();
            let plant_id: u32 = unsafe { std::mem::transmute(plant_type) };
            println!("plant type: {}", plant_id);


            // 获取植物横坐标
            let mut x_location: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_X_LOCATION_RADDR_INT, &mut x_location).unwrap();
            let x_location: u32 = unsafe { std::mem::transmute(x_location) };
            // 获取植物纵坐标
            let mut y_location: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_Y_LOCATION_RADDR_INT, &mut y_location).unwrap();
            let y_location: u32 = unsafe { std::mem::transmute(y_location) };
            println!("location: ({}, {})", x_location, y_location);

            // 获取植物行数
            let mut row: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ROW_RADDR, &mut row).unwrap();
            let row: u32 = unsafe { std::mem::transmute(row) };
            // 获取植物列数
            let mut col: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_COL_RADDR, &mut col).unwrap();
            let col: u32 = unsafe { std::mem::transmute(col) };
            println!("position: ({}, {})", row, col);

            // 获取植物图象图层
            let mut image_layer: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IMAGE_LAYER_RADDR, &mut image_layer).unwrap();
            let image_layer: u32 = unsafe { std::mem::transmute(image_layer) };
            println!("image layer: {}", image_layer);

            // 获取植物图像倒计时
            let mut anima_clock: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ANIMA_CLOCK_RADDR, &mut anima_clock).unwrap();
            let anima_clock: u32 = unsafe { std::mem::transmute(anima_clock) };
            println!("anima clock: {}", anima_clock);

            // 获取植物是否可以攻击
            let mut attackable: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ALLOW_ATTACK_RADDR, &mut attackable).unwrap();
            let attackable: u32 = unsafe { std::mem::transmute(attackable) };
            println!("can attack?: {}", attackable);

            // 获取属性倒计时
            let mut attribute_clock: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ATTRIBUTE_CLOCK_RADDR, &mut attribute_clock).unwrap();
            let attribute_clock: u32 = unsafe { std::mem::transmute(attribute_clock) };
            println!("attribute clock: {}", attribute_clock);

            // 行为倒计时
            let mut action_clock: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ACTION_CLOCK_RADDR, &mut action_clock).unwrap();
            let action_clock: u32 = unsafe { std::mem::transmute(action_clock) };
            println!("action clock: {}", action_clock);
            
            // 植物动画编号1
            let mut animation1: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ANIMATION1_RADDR, &mut animation1).unwrap();
            let animation1: u32 = unsafe { std::mem::transmute(animation1) };
            println!("animation1: {}", animation1);

            // 植物动画编号2
            let mut animation2: [u8; 4] = [0; 4];
            process::read_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_ANIMATION2_RADDR, &mut animation2).unwrap();
            let animation2: u32 = unsafe { std::mem::transmute(animation2) };
            println!("animation2: {}", animation2);

            // 清除植物
            // let mut if_exist: [u8; 2] = [0;2];
            // process::write_memory_by_addr(process_handle, cur_plant_ptr + addr::PLANT_IF_EXIST_RADDR, &mut if_exist).expect("read if exist failed");
        }
        cur_plant_ptr = move_to_next_plant(cur_plant_ptr);
    }
    Ok(active_num)
}


pub fn get_plant_locations(process_handle: Foundation::HANDLE) -> Result<(), String> {
    // 读取基表指针
    let mut base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, addr::BASE_ADDR, &mut base_ptr[..]).expect("Read Memory Failed");
    let base_ptr: u32 = unsafe { std::mem::transmute(base_ptr) };  

    // 获取游戏信息表指针
    let mut game_base_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, base_ptr + addr::CUR_GAME_INFO_RADDR, &mut game_base_ptr[..]).expect("Read Memory Failed");
    let game_base_ptr: u32 = unsafe { std::mem::transmute(game_base_ptr) };

    // 获取下一个植物编号
    let mut plant_id: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_NEXTID_RADDR, &mut plant_id).unwrap();
    let plant_id: u32 = unsafe { std::mem::transmute(plant_id) };
    println!("next plant id: {}", plant_id);

    // 获取最后一个植物编号
    let mut plant_id: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_LASTID_RADDR, &mut plant_id).unwrap();
    let plant_id: u32 = unsafe { std::mem::transmute(plant_id) };
    println!("last plant id: {}", plant_id);


    // 获得植物序列指针
    let mut plants_list_ptr: [u8; 4] = [0; 4];
    process::read_memory_by_addr(process_handle, game_base_ptr + addr::PLANT_LS_RADDR, &mut plants_list_ptr).expect("Read Memory Failed");
    

    loop {
        thread::sleep(Duration::from_secs(1));
        let mut plants_ptr: u32 = unsafe { std::mem::transmute(plants_list_ptr) };
        println!("Active Plant Number: {}", get_active_plant_num(process_handle, plants_ptr).expect("load activate number failed"));
    }
    
    Ok(())
}

