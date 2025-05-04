
pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    smash::app::lua_bind::WorkModule::get_int(boma, *smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

extern "C"{
    #[link_name = "\u{1}_ZN3app14sv_information11is_ready_goEv"]
    pub fn is_ready_go() -> bool;
}