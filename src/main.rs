/*
Copyright © 2020, "rupansh" <rupanshsekar@hotmail.com>

 This software is licensed under the terms of the GNU General Public
 License version 3, as published by the Free Software Foundation, and
 may be copied, distributed, and modified under those terms.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 GNU General Public License for more details.

 Please maintain this if you use this script or any part of it
*/

extern crate winapi;
use winapi::shared::minwindef::DWORD;
use winapi::shared::basetsd::DWORD_PTR;
use winapi::shared::ntdef::NULL;

use std::thread;
use std::time::Duration;

mod hack;
use hack::*;
use hack::proc_mem::*;
use hack::offsets::*;
use hack::player_struct::*;


fn main() {
    let mut hack_vars: Game = Game {
        game_module: NULL as DWORD,
        client_state: NULL as DWORD,
        process_mem: ProcMem {
            h_process: NULL,
            dw_pid: 0,
            dw_protection: 0,
            dw_cave_addr: 0,
            b_pOn: false,
            b_iOn: false,
            b_prot: false,
        },
        local_entity: 0,
        local_player: PlayerStruct::default(),
        max_players: None,
    };

    println!("Searching for game");
    hack_vars.process_mem.process("csgo.exe".to_string());
    println!("Searching for panorama");
    let game_hmod = hack_vars.process_mem.module("client_panorama.dll".to_string());
    hack_vars.game_module = game_hmod as DWORD_PTR as u32;
    println!("Searching for engine");
    let engine = hack_vars.process_mem.module("engine.dll".to_string()) as DWORD_PTR as u32;
    hack_vars.process_mem.read_mem::<DWORD>(engine + OFFSETS_SIG.get(&"dwClientState").cloned().unwrap(), &mut hack_vars.client_state);


    loop {
        hack_vars.bhop();
        hack_vars.wallhax();
        hack_vars.aimbot();

        thread::sleep(Duration::from_millis(5))
    }
}
