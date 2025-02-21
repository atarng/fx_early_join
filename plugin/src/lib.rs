#![feature(lazy_cell, ptr_sub_ptr)]

// use cobapi::*;
use engage::{
    eventsequence::EventSequence,
    force::*,
    gamedata::{ item::ItemData, Gamedata, unit::{Unit, Gender }, god::List, dispos::ChapterData },
    menu::BasicMenuItem,
    proc::ProcInst,
    script::*,
    sequence::gmap_sequence::{ GmapSpot, GmapSequence },
    unitpool::UnitPool,
    util::get_singleton_proc_instance,
};
use skyline::{ install_hook, patching::Patch, };

use unity::engine::Vector3;
use unity::prelude::*;

#[skyline::main(name = "recall_any_battle")]
pub fn main() {
    // If Not Fel or Divine Paralogue, nor Skirmish Mob, Provide Recall Battle for
    // Main/Side Story (Paralogue/Story Map)
    // 7101ccd3b8 40 fd 07 36     tbz        menuItemList,#0x0,LAB_7101ccd360
    Patch::in_text(0x01ccd3b8).bytes(&[0x40, 0xfd, 0x07, 0x36]).unwrap();
}