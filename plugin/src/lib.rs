#![feature(lazy_cell, ptr_sub_ptr)]

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

// Maybe we should make this return the count... in the future.
extern "C" fn install_gender_check_script(event: &EventScript) {
    event.register_function("IsAlearFemale", is_alear_female);
}

extern "C" fn is_alear_female(args: &Il2CppArray<DynValue>, _method_info: OptionalMethod) -> &'static DynValue {
    let mut result: bool = false;
    if let Some(alear) = UnitPool::get_from_person_mut("PID_リュール".into(), false) {
        let person_gender: i32 = alear.get_gender();
        result = person_gender == 2;
        println!("[is_alear_female] {} (gender: {})", result, person_gender);
    } else {
        println!("[is_alear_female] could not grab alear unit.");
    }
    return DynValue::new_boolean(result);
}

#[skyline::main(name = "is_alear_female")]
pub fn main() {
    let fn_ptr = install_gender_check_script as extern "C" fn(&engage::script::EventScript);
    cobapi::install_lua_command_registerer(fn_ptr);
}