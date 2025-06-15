use smash::lib::LuaConst;

//COMMON

// A list of instance work IDs
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH: smash::lib::LuaConst = smash::lib::LuaConst::new(0x2000012A);

pub mod globals {
    //Vanilla Consts
    pub const UNK1: i32 = 0x0; //void value
    pub const UNK2: i32 = 0x1; //void value
    pub const FIGHTER_KIND: i32 = 0x2; //fighter kind, i32
    pub const OBJECT_ID: i32 = 0x3; //object id, i32
    pub const FIGHTER: i32 = 0x4; //ptr value, very similar to 0x6
    pub const MODULE_ACCESSOR: i32 = 0x5; //module accessor, ptr value
    pub const UNK4: i32 = 0x6; //void value
    pub const INIT_STATUS_FUNC: i32 = 0x7; //init status func, ptr value
    pub const IS_STOP: i32 = 0x8; //is stop, bool value
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9; //status kind interrupt, i32 value
    pub const PREV_STATUS_KIND: i32 = 0xA; //prev status kind, i32 value
    pub const STATUS_KIND: i32 = 0xB; //status kind, i32 value
    pub const STATUS_COUNT: i32 = 0xC; //status count, i32 value
    pub const UNK5: i32 = 0xD; //bool value
    pub const CURRENT_FRAME: i32 = 0xE; //current frame, f32 value
    pub const CURRENT_FRAME_NO_INTERP: i32 = 0xF; //current frame no interp, f32 value
    pub const UNK6: i32 = 0x10; //ptr value
    pub const UNK7: i32 = 0x11; //ptr value, equal to UNK8
    pub const UNK8: i32 = 0x12; //ptr value
    pub const SUB_STATUS3: i32 = 0x13; //sub status3, ptr/i32 value
    pub const PREV_SUB_STATUS: i32 = 0x14; //prev sub status, i32 value
    pub const SUB_STATUS: i32 = 0x15; //sub status, i32 value
    pub const SITUATION_KIND: i32 = 0x16; //situation kind, i32 value
    pub const PREV_SITUATION_KIND: i32 = 0x17; //prev situation kind, i32 value
    pub const PREV_STATUS_FRAME: i32 = 0x18; //prev status frame, f32 value
    pub const UNK9: i32 = 0x19; //i32 value, i32 value
    pub const STICK_X: i32 = 0x1A; //stick x, f32 value
    pub const STICK_Y: i32 = 0x1B; //stick y, f32 value
    pub const FLICK_X: i32 = 0x1C; //flick x, i32 value
    pub const FLICK_Y: i32 = 0x1D; //flick y, i32 value
    pub const FLICK_Y_DIR: i32 = 0x1E; //flick y dir, f32 value
    pub const PAD_FLAG: i32 = 0x1F; //pad flag, u64 value
    pub const CMD_CAT1: i32 = 0x20; //cmd cat1, u64 value
    pub const CMD_CAT2: i32 = 0x21; //cmd cat2, u64 value
    pub const CMD_CAT3: i32 = 0x22; //cmd cat3, u64 value
    pub const CMD_CAT4: i32 = 0x23; //cmd cat4, u64 value
    pub const UNK10: i32 = 0x24;
    pub const UNK11: i32 = 0x25;
    pub const CHECK_AIR_SPECIAL_UNIQ: i32 = 0x26; //check air special uniq
    pub const CHECK_GROUND_SPECIAL_UNIQ: i32 = 0x27; //check ground special uniq
    pub const CHECK_GROUND_ATTACK_UNIQ: i32 = 0x28; //check ground attack uniq
    pub const DASH_COMMON_UNIQ: i32 = 0x29; //dash common uniq
    pub const RUN_MAIN_UNIQ: i32 = 0x2A; //run main uniq
    pub const JUMP_SQUAT_MAIN_UNIQ: i32 = 0x2B; //jump squat main uniq
    pub const CHECK_AIR_LANDING_UNIQ: i32 = 0x2C; //check air landing uniq
    pub const CHECK_AIR_ITEM_THROW_UNIQ: i32 = 0x2D; //check air item throw uniq
    pub const CHECK_AIR_ATTACK_UNIQ: i32 = 0x2E; //check air attack uniq
    pub const CHECK_AIR_ESCAPE_UNIQ: i32 = 0x2F; //check air escape uniq
    pub const CHECK_AIR_TREAD_JUMP_UNIQ: i32 = 0x30; //check air tread jump uniq
    pub const CHECK_AIR_WALL_JUMP_UNIQ: i32 = 0x31; //check air wall jump uniq
    pub const CHECK_AIR_JUMP_UNIQ: i32 = 0x32; //check air jump uniq
    pub const CHECK_AIR_JUMP_AERIAL_UNIQ: i32 = 0x33; //check air jump aerial uniq
    pub const GUARD_CONT_UNIQ: i32 = 0x34; //guard cont uniq
    pub const TURN_UNIQ: i32 = 0x35; //turn uniq
    pub const CHECK_AIR_CLIFF_LASSO_UNIQ: i32 = 0x36; //check air cliff lasso uniq
    pub const LANDING_UNIQ_CHECK_STRANS_UNIQ: i32 = 0x37; //landing uniq check strans uniq
    pub const CHECK_SPECIAL_N_UNIQ: i32 = 0x38; //check special n uniq
    pub const CHECK_SPECIAL_S_UNIQ: i32 = 0x39; //check special s uniq
    pub const CHECK_SPECIAL_HI_UNIQ: i32 = 0x3A; //check special hi uniq
    pub const CHECK_SPECIAL_LW_UNIQ: i32 = 0x3B; //check special lw uniq
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C; //check special command
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D; //waza customize control
    pub const STATUS_END_CONTROL: i32 = 0x3E; //status end control
    pub const UNK12: i32 = 0x3F;
    pub const UNK13: i32 = 0x40;
    pub const UNK14: i32 = 0x41;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const SUB_UNIQ_DAMAGE_FLY_UNIQ: i32 = 0x43;
    pub const DOWN_DAMAGE_UNIQ: i32 = 0x44;
    pub const THROW_F_STATUS_KIND: i32 = 0x45;
    pub const THROW_B_STATUS_KIND: i32 = 0x46;
    pub const THROW_HI_STATUS_KIND: i32 = 0x47;
    pub const THROW_LW_STATUS_KIND: i32 = 0x48;
    pub const DAMAGE_STOP_MOTION_INTP_FRAME: i32 = 0x49;
    pub const SUB_REBIRTH_UNIQ_INIT_CORE_UNIQ: i32 = 0x4A;
    pub const SUB_REBIRTH_UNIQ_EXEC_UNIQ: i32 = 0x4B;
    pub const SUB_DEAD_UNIQ_INIT_UNIQ: i32 = 0x4C;
    pub const SUB_ROULETTE_SET_SETP_UNIQ: i32 = 0x4D;
    pub const FALL_BRAKE_UNIQ: i32 = 0x4E;
    pub const CHECK_GROUND_GUARD_UNIQ: i32 = 0x4F;
    pub const CHECK_GROUND_CATCH_UNIQ: i32 = 0x50;
    pub const CHECK_COMMAND_WALK_UNIQ: i32 = 0x51;
    pub const CHECK_GROUND_JUMP_MINI_ATTACK: i32 = 0x52;
    pub const CHECK_AIR_ITEM_THROW_POST: i32 = 0x53;
    pub const IS_ITEM_SHOOT_STATUS_UNIQ: i32 = 0x54;
    pub const CHECK_ATTACK_3_UNIQ: i32 = 0x55;
    pub const CHECK_ATTACK_N_UNIQ: i32 = 0x56;
    pub const CHECK_ATTACK_S4_UNIQ: i32 = 0x57;
    pub const CHECK_ATTACK_HI4_UNIQ: i32 = 0x58;
    pub const CHECK_ATTACK_LW4_UNIQ: i32 = 0x59;
    pub const SQUAT_COMMON_UNIQ: i32 = 0x5A;

    //Offsets
    pub const CONSTANT_OFFSET : usize = 0x3728030;
    pub const FLOAT_OFFSET: usize = 0x4E53E0;
}