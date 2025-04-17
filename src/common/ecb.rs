use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::phx::Vector2f;
use smash::app::utility;
use crate::controls::util::get_player_number;
use smash::app::BattleObjectModuleAccessor;


static mut ECB_Y_OFFSETS: [f32; 8] = [0.0; 8];

pub unsafe fn fixed_ecbs(boma: &mut smash::app::BattleObjectModuleAccessor, category: i32, status_kind: i32, situation_kind: i32) {
    if CategoryModule::category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return;
    }

    let fighter_kind = utility::get_kind(boma);
    let entry_id = get_player_number(boma);

    let mut max_offset = if [
        FIGHTER_KIND_KIRBY,
        FIGHTER_KIND_PIKACHU,
        FIGHTER_KIND_NESS,
        FIGHTER_KIND_PURIN,
        FIGHTER_KIND_GAMEWATCH,
        FIGHTER_KIND_POPO,
        FIGHTER_KIND_NANA,
        FIGHTER_KIND_PICHU,
        FIGHTER_KIND_METAKNIGHT,
        FIGHTER_KIND_WARIO,
        FIGHTER_KIND_PZENIGAME,
        FIGHTER_KIND_PFUSHIGISOU,
        FIGHTER_KIND_LUCAS,
        FIGHTER_KIND_PIKMIN,
        FIGHTER_KIND_TOONLINK,
        FIGHTER_KIND_DUCKHUNT,
        FIGHTER_KIND_MURABITO,
        FIGHTER_KIND_INKLING,
        FIGHTER_KIND_SHIZUE,
    ].contains(&(fighter_kind)) {
        2.0
    } else if [
        FIGHTER_KIND_MARIO,
        FIGHTER_KIND_YOSHI,
        FIGHTER_KIND_LUIGI,
        FIGHTER_KIND_MARIOD,
        FIGHTER_KIND_YOUNGLINK,
        FIGHTER_KIND_PLIZARDON,
        FIGHTER_KIND_DIDDY,
        FIGHTER_KIND_DEDEDE,
        FIGHTER_KIND_ROCKMAN,
        FIGHTER_KIND_GEKKOUGA,
        FIGHTER_KIND_PACMAN,
        FIGHTER_KIND_KOOPAJR,
        FIGHTER_KIND_PACKUN,
        FIGHTER_KIND_MIIFIGHTER,
        FIGHTER_KIND_MIISWORDSMAN,
        FIGHTER_KIND_MIIGUNNER,
        FIGHTER_KIND_BUDDY,
    ].contains(&(fighter_kind)) {
        3.5
    } else if [
        FIGHTER_KIND_FOX,
        FIGHTER_KIND_FALCO,
        FIGHTER_KIND_DAISY,
        FIGHTER_KIND_MEWTWO,
        FIGHTER_KIND_PIT,
        FIGHTER_KIND_PITB,
        FIGHTER_KIND_SONIC,
        FIGHTER_KIND_LUCARIO,
        FIGHTER_KIND_ROBOT,
        FIGHTER_KIND_WOLF,
        FIGHTER_KIND_LITTLEMAC,
        FIGHTER_KIND_KROOL,
        FIGHTER_KIND_GAOGAEN,
    ].contains(&(fighter_kind)) {
        4.0
    } else if [
        FIGHTER_KIND_DONKEY,
        FIGHTER_KIND_LINK,
        FIGHTER_KIND_SAMUS,
        FIGHTER_KIND_SAMUSD,
        FIGHTER_KIND_CAPTAIN,
        FIGHTER_KIND_PEACH,
        FIGHTER_KIND_KOOPA,
        FIGHTER_KIND_SHEIK,
        FIGHTER_KIND_ZELDA,
        FIGHTER_KIND_MARTH,
        FIGHTER_KIND_LUCINA,
        FIGHTER_KIND_GANON,
        FIGHTER_KIND_ROY,
        FIGHTER_KIND_CHROM,
        FIGHTER_KIND_SZEROSUIT,
        FIGHTER_KIND_SNAKE,
        FIGHTER_KIND_IKE,
        FIGHTER_KIND_WIIFIT,
        FIGHTER_KIND_ROSETTA,
        FIGHTER_KIND_PALUTENA,
        FIGHTER_KIND_REFLET,
        FIGHTER_KIND_SHULK,
        FIGHTER_KIND_RYU,
        FIGHTER_KIND_KEN,
        FIGHTER_KIND_CLOUD,
        FIGHTER_KIND_KAMUI,
        FIGHTER_KIND_BAYONETTA,
        FIGHTER_KIND_RIDLEY,
        FIGHTER_KIND_SIMON,
        FIGHTER_KIND_RICHTER,
        FIGHTER_KIND_JACK,
        FIGHTER_KIND_BRAVE,
    ].contains(&(fighter_kind)) {
        5.0
    } else {
        0.0
    };

    if status_kind == *FIGHTER_STATUS_KIND_ENTRY
        || (StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_PASS
            && MotionModule::frame(boma) < 10.0)
    {
        max_offset = 0.0;
    }

    let vanilla_ecb = [
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
    ].contains(&status_kind)
        || [
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_THROWN,
        ].contains(&StatusModule::prev_status_kind(boma, 1));

    let air_trans = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;
    let mut offset = Vector2f { x: 0.0, y: 0.0 };

    if situation_kind == *SITUATION_KIND_AIR {
        if ECB_Y_OFFSETS[entry_id] < max_offset {
            ECB_Y_OFFSETS[entry_id] += 0.05;
        } else {
            ECB_Y_OFFSETS[entry_id] = max_offset;
        }

        offset.y = ECB_Y_OFFSETS[entry_id];
        if !(vanilla_ecb || air_trans) {
            GroundModule::set_rhombus_offset(boma, &offset);
        }
    } else if situation_kind == *SITUATION_KIND_GROUND {
        offset.y = 0.0;
        if !vanilla_ecb {
            GroundModule::set_rhombus_offset(boma, &offset);
        }
    } else {
        ECB_Y_OFFSETS[entry_id] = 0.0;
        offset.y = 0.0;
        if !vanilla_ecb {
            GroundModule::set_rhombus_offset(boma, &offset);
        }
    }
}