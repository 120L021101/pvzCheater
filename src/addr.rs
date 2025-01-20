pub const BASE_ADDR : u32 = 0x6A9EC0;
    pub const CUR_GAME_INFO_RADDR : u32 = 0x768;
        pub const PLANT_LS_RADDR : u32 = 0xAC;
            pub const PLANT_BASE_RADDR : u32 = 0x0;
            pub const PLANT_GAME_INFO_RADDR : u32 = 0x4;
            pub const PLANT_X_LOCATION_RADDR_INT : u32 = 0x8;
            pub const PLANT_Y_LOCATION_RADDR_INT : u32 = 0xC;
            pub const PLANT_FIXED1_RADDR : u32 = 0x10;
            pub const PLANT_IF_VISIBLE_RADDR : u32 = 0x18;
            pub const PLANT_ROW_RADDR: u32 = 0x1C;
            pub const PLANT_IMAGE_LAYER_RADDR: u32 = 0x20;
            pub const PLANT_TYPE_RADDR : u32 = 0x24;
            pub const PLANT_COL_RADDR : u32 = 0x28;
            pub const PLANT_ANIMA_CLOCK_RADDR : u32 = 0x2C;
            pub const PLANT_FIXED2_RADDR : u32 = 0x30;
            pub const PLANT_STATUS_RADDR : u32 = 0x3C;
            pub const PLANT_CURBLOOD_RADDR : u32 = 0x40;
            pub const PLANT_MAXBLOOD_RADDR : u32 = 0x44;
            pub const PLANT_ALLOW_ATTACK_RADDR : u32 = 0x48;
            pub const PLANT_ATTRIBUTE_CLOCK_RADDR : u32 = 0x54;
            pub const PLANT_ACTION_CLOCK_RADDR : u32 = 0x5C;
            pub const BOMB_X_LOCATION_RADDR : u32 = 0x80;
            pub const BOMB_Y_LOCATION_RADDR : u32 = 0x84;
            pub const PLANT_ANIMATION1_RADDR : u32 = 0x94;
            pub const PLANT_ANIMATION2_RADDR : u32 = 0x98;
            pub const PLANT_DACTION_CLOCK_RADDR : u32 = 0x128;
            pub const PLANT_FIXED3_RADDR : u32 = 0x138;
            pub const PLANT_ATTRIBUTE_RADDR : u32 = 0x140;
            pub const PLANT_IF_EXIST_RADDR : u32 = 0x148;
            pub const PLANT_ID_RADDR : u32 = 0x14A;
            pub const NEXT_PLANT_RADDR : u32 = 0x204;


        pub const ZOMBIE_LS_RADDR : u32 = 0x90;
            pub const ZOMBIE_X_LOCATION_RADDR_INT : u32 = 0x08;
            pub const ZOMBIE_Y_LOCATION_RADDR_INT : u32 = 0x0C; 
            pub const ZOMBIE_LINE_RADDR : u32 = 0x1C;
            pub const ZOMBIE_TYPE_RADDR : u32 = 0x24;
            pub const ZOMBIE_STATUS_RADDR : u32 = 0x28;
            pub const ZOMBIE_X_LOCATION_RADDR_FLOAT : u32 = 0x2C;
            pub const ZOMBIE_Y_LOCATION_RADDR_FLOAT : u32 = 0x30; 
            pub const ZOMBIE_EXIST_TIME_RADDR : u32 = 0x60;
            pub const ZOMBIE_ATTRACTED_RADDR : u32 = 0xB8;
            pub const ZOMBIE_DEFENSE1_RADDR : u32 = 0xC4;
            pub const ZOMBIE_CUR_BLOOD_RADDR : u32 = 0xC8;
            pub const ZOMBIE_IF_EXIST_RADDR : u32 = 0x15A; 

            pub const ZOMBIE_ARMOR1_RADDR: u32 = 0xC4;
            pub const ZOMBIE_ARMOR1_BLOOD_RADDR: u32 = 0xD0;
            pub const ZOMBIE_ARMOR1_BLOOD_UPPER_RADDR: u32 = 0xD4;
            pub const ZOMBIE_ARMOR2_RADDR: u32 = 0xD8;
            pub const ZOMBIE_ARMOR2_BLOOD_RADDR: u32 = 0xDC;
            pub const ZOMBIE_ARMOR2_BLOOD_UPPER_RADDR: u32 = 0xE0;

            pub const NEXT_ZOMBIE_RADDR : u32 = 0x204;

        pub const ZOMBIE_NUM_UPPER_LIMIT_RADDR : u32 = 0x98;

        pub const ZOMBIE_NUM_RADDR : u32 = 0xA0;

        pub const PLANT_NEXTID_RADDR: u32 = 0xB8;
        pub const PLANT_LASTID_RADDR: u32 = 0xC0;

        pub const SUN_RADDR : u32 = 0x5560;