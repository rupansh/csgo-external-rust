/*
Copyright © 2019, "rupansh" <rupanshsekar@hotmail.com>

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
use winapi::shared::ntdef::NULL;

use std::string::String;
use std::time::Duration;
use std::thread;


mod hack;
use hack::*;
use hack::proc_mem::*;
use hack::offsets::*;

const PROCESS: &[u8; 8] = b"csgo.exe";
const DLL: &[u8; 19] = b"client_panorama.dll";
const EDLL: &[u8; 10] = b"engine.dll";


fn main() {
    let mut hack_vars: HackVars = HackVars {
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
        local_player: NULL as DWORD,
    };

    println!("Searching for game");
    hack_vars.process_mem.process(String::from_utf8(PROCESS.to_vec()).unwrap());
    println!("Searching for panorama");
    hack_vars.game_module = hack_vars.process_mem.module(String::from_utf8(DLL.to_vec()).unwrap());
    println!("Searching for engine");
    let engine = hack_vars.process_mem.module(String::from_utf8(EDLL.to_vec()).unwrap());
    hack_vars.process_mem.read_mem::<DWORD>(engine + OFFSETS_SIG.dwClientState, &mut hack_vars.client_state);

    loop {
        hack_vars.bhop();
        hack_vars.wallhax();
        hack_vars.aimbot();
        thread::sleep(Duration::from_millis(10))
    }
}

