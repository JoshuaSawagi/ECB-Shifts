use smash::app;
use smash::lib::lua_const::*;
use smash::app::utility;

//Aerial ECB Fixes, mainly for things like Link, Captain, Simon, and Richter (Credit to HDR)
extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;
}

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    smash::app::lua_bind::WorkModule::get_int(boma, *smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

extern "C"{
    #[link_name = "\u{1}_ZN3app14sv_information11is_ready_goEv"]
    pub fn is_ready_go() -> bool;
}

extern "C"{
    #[link_name = "\u{1}_ZN3app7utility8get_kindEPKNS_26BattleObjectModuleAccessorE"]
    pub fn get_kind(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32;
}
