#![feature(lazy_cell, ptr_sub_ptr)]

// use cobapi::*;
use engage::{
    eventsequence::EventSequence,
    force::*,
    gamedata::{ item::ItemData, Gamedata, unit::Unit, },
    proc::ProcInst,
    script::*,
    unitpool::UnitPool,
};

use skyline::{ install_hook, };

use unity::engine::Vector3;
use unity::prelude::*;

#[unity::class("App", "MapCursor")]
pub struct MapCursor {
//     m_pos: Vector3<i32>,
//     m_old_pos: Vector3<i32>,
//     m_orig_pos: Vector3<i32>,
//     m_rotate: Vector3<i32>,
//     m_move: Vector3<i32>,
//     m_enter_pos: Vector3<i32>,
//     m_move_count: i32,
//     m_is_rotate_x_sound_playing, bool,
//     // transform
//     i32,
//     i32,
//     i32,
//     f32,
//     i32,
//     bool,
//     // GameObject,
//     // GameObject,
//     // MapPointerAnimManager*
//     // MapCursorAnimManager*
//     // InterpolatorVector3*
//     i32,
//     Color,
//     // FlagField
}

// void App.MapCursor$$Set(int32_t x,int32_t z,float speed,bool isUpdateEnterPos,MethodInfo *method)
// #[unity::from_offset("App", "MapCursor", "Set")]
#[skyline::hook(offset=0x029bc380)]
pub fn mapcursor_set1(x: i32, z: i32, speed: f32, is_update_enter_pos: bool, _method_info: u64) {
    println!("mapcursor_set/1] ({0}, {1}) spd: {2}, update: {3}", x, z, speed, is_update_enter_pos);
    call_original!(x, z, speed, is_update_enter_pos, _method_info);
}

// void App.MapCursor$$Set(UnityEngine_Vector3_o pos,float speed,bool isUpdateEnterPos, MethodInfo *method)
#[skyline::hook(offset=0x029bd0a0)]
pub fn mapcursor_set2(pos: Vector3<i32>, speed: f32, is_update_enter_pos: bool, _method_info: u64) {
    println!("mapcursor_set/2]  spd: {}, update: {} pos: ({}, {}, {})", speed, is_update_enter_pos, pos.x, pos.y, pos.z);
    call_original!(pos, speed, is_update_enter_pos, _method_info);
}

// void App.MapCursor$$Set(App_Unit_o *unit,float speed,bool isUpdateEnterPos,MethodInfo *method)
#[skyline::hook(offset=0x029bd180)]
pub fn mapcursor_set3(unit: Option<&Unit>, speed: f32, is_update_enter_pos: bool, _method_info: u64) {
    if let Some(person) = unit
    {
        println!("mapcursor_set/3] pid: {0} spd: {1}, update: {2}", person.get_pid(), speed, is_update_enter_pos);
    } else {
        println!("mapcursor_set/3] unit inaccessible");
    }

    call_original!(unit, speed, is_update_enter_pos, _method_info);
}

// UnityEngine_Vector3_o App.MapCursor$$GetMovePos
//           (App_MapCursor_o *__this,UnityEngine_Vector3_o prev,UnityEngine_Vector3_o next,float speed ,MethodInfo *method)
#[skyline::hook(offset=0x029aff00)]
pub fn mapcursor_getmovepos(this: Option<&MapCursor>, prev: Vector3<i32>, next: Vector3<i32>, speed: f32, _method_info: u64) {
    println!("mapcursor_getmovepos] prev: {}, {}, {} next: {}, {}, {} speed: {}", prev.x, prev.y, prev.z, next.x, next.y, next.z, speed);
    call_original!(this, prev, next, speed, _method_info);
}

// void App.EventSequence$$Opening(App_ProcInst_o *super,MethodInfo *method)
#[unity::hook("App", "EventSequence", "Opening")]
pub fn eventsequence_opening(supper: &ProcInst, method_info: OptionalMethod) {
    let event_script_instance = EventScript::get_instance();
    if let Some(exclude_fx_units_dv) = event_script_instance.get_func("ExcludeFxUnits") {
        EventSequence::try_create_bind(supper, exclude_fx_units_dv, None, None, None);
    }

    call_original!(supper, method_info);
}

// Maybe we should make this return the count... in the future.
extern "C" fn install_gender_check_script(event: &EventScript) {
    event.register_function("IsAlearFemale", is_alear_female);
}

extern "C" fn is_alear_female(args: &Il2CppArray<DynValue>, _method_info: OptionalMethod) -> &'static DynValue {
    let mut result: bool = false;
    if let Some(alear) = UnitPool::get_from_person_mut("PID_リュール".into(), false) {
        let person_gender: i32 = alear.get_gender();
        result = person_gender == 2;
        println!("[is_alear_female] alear: {} (gender: {})", result, person_gender);
    } else {
        println!("[is_alear_female] could not grab alear unit.");
    }
    
    // if let Some(absent_units) = Force::get(ForceType::Absent) {
    //     absent_units.iter().for_each(|unit| println!("[is_alear_female] absent-force_check: {}: {}", unit.get_pid(), unit.get_gender()) );
    // }
    // if let Some(player_units) = Force::get(ForceType::Player) {
    //     player_units.iter().for_each(|unit|  println!("[is_alear_female] player-force_check: {}: {}", unit.get_pid(), unit.get_gender()) );
    // }

    return DynValue::new_boolean(result);
}

#[skyline::main(name = "threesixty_camera_cursor")]
pub fn main() {
    let fn_ptr = install_gender_check_script as extern "C" fn(&engage::script::EventScript);
    cobapi::install_lua_command_registerer(fn_ptr);

    install_hook!(mapcursor_set1);
    install_hook!(mapcursor_set2);
    install_hook!(mapcursor_set3);
    install_hook!(mapcursor_getmovepos);

    install_hook!(eventsequence_opening);
}