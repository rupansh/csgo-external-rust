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

extern crate cc;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::iter::FromIterator;

#[path = "src/offsets.rs"]
pub mod offsets;
use offsets::OFFSETS_NET;


fn match_offset(offset_name: &str, var_type: &mut String, default_val: &mut String) -> u32 {
    let var_size;
    *default_val =  "0".to_string();
    if offset_name.contains("m_iItem") {
        *var_type = "u8".to_string();
        var_size = std::mem::size_of::<u8>();
    } else if offset_name.contains("m_i") {
        *var_type = "i32".to_string();
        var_size = std::mem::size_of::<i32>();
    } else if offset_name.contains("m_vec") {
        *var_type = "D3DVECTOR".to_string();
        *default_val = "D3DVECTOR {x: 0.0, y: 0.0, z: 0.0}".to_string();
        var_size = 12;
    } else if offset_name.contains("m_fl") {
        *var_type = "f32".to_string();
        *default_val = "0.0".to_string();
        var_size = std::mem::size_of::<f32>();
    } else if offset_name.contains("m_b") {
        *var_type = "u8".to_string();
        var_size = std::mem::size_of::<u8>();
    } else if offset_name.contains("m_dw") {
        *var_type = "u32".to_string();
        var_size = std::mem::size_of::<u32>();
    } else if offset_name.contains("m_MoveType") {
        *var_type = "u8".to_string();
        var_size = std::mem::size_of::<u8>();
    } else if offset_name.contains("m_h") {
        *var_type = "i32".to_string();
        var_size = std::mem::size_of::<i32>();
    } else if offset_name.contains("m_lifeState") {
        *var_type = "u8".to_string();
        var_size = std::mem::size_of::<u8>();
    } else if offset_name.contains("m_aimPunchAngleVel") {
        *var_type = "i32".to_string();
        var_size = std::mem::size_of::<i32>();
    } else if offset_name.contains("m_aimPunchAngle") {
        *var_type = "D3DVECTOR".to_string();
        *default_val = "D3DVECTOR {x: 0.0, y: 0.0, z: 0.0}".to_string();
        var_size = 12;
    } else if offset_name.contains("m_fFlags") {
        *var_type = "i32".to_string();
        var_size = std::mem::size_of::<i32>();
    } else {
        *var_type = "u32".to_string();
        var_size = std::mem::size_of::<u32>();
    }

    return var_size as u32;
}


fn main() {
    let path = Path::new("src/player_struct_skel.txt");


    let mut player_file = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", path.display(),
                                                 e.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match player_file.read_to_string(&mut s){
        Err(e) => panic!("couldn't open {}: {}", path.display(),
                                                 e.description()),
        Ok(_) => (),
    };

    let mut player_dict: Vec<String> = Vec::new();
    let mut default_dict: Vec<String> = Vec::new();

    let mut offsets_vec = Vec::from_iter(OFFSETS_NET.entries());
    offsets_vec.sort_by(|(_, a), (_, b)| a.cmp(b));
    let mut size: u32 = 0;
    for i in 0..offsets_vec.len() {
        let mut var_type = String::new();
        let mut default_value = String::new();
        let var_size = match_offset(*offsets_vec[i].0, &mut var_type, &mut default_value);
        size += var_size;
        player_dict.push(format!("    pub {}: {},", *offsets_vec[i].0, var_type).to_string());
        default_dict.push(format!("        {}: {},", *offsets_vec[i].0, default_value).to_string());
        if i != offsets_vec.len() - 1{
            if size != *offsets_vec[i+1].1 {
                if size > *offsets_vec[i+1].1 {
                    panic!("NIBBER {}, {}", size, *offsets_vec[i+1].1);
                }
                player_dict.push(format!("    pub _p{}: [u8; {}],", i, *offsets_vec[i+1].1 - size).to_string());
                default_dict.push(format!("        _p{}: [0; {}],", i, *offsets_vec[i+1].1 - size).to_string());
                size = *offsets_vec[i+1].1;
            }
        }
    }

    let joined_player = player_dict.join("\n");
    let joined_default = default_dict.join("\n");
    s = s.replace("pls_skeleton", &joined_player);
    s = s.replace("default_skeleton", &joined_default);

    let path = Path::new("src/player_struct.rs");
    let mut player_file = match File::create(&path) {
        Err(e) => panic!("couldn't open {}: {}", path.display(), e.description()),
        Ok(file) => file,
    };

    match player_file.write_all(s.as_bytes()) {
        Err(e) => panic!("couldn't write to {}: {}", path.display(), e.description()),
        Ok(_) => (),
    }

}