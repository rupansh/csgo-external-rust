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
use winapi::shared::d3d9types::D3DVECTOR;
use winapi::shared::minwindef::{DWORD, BOOL, FALSE, TRUE};
use winapi::shared::ntdef::NULL;
use winapi::um::winuser::{VK_LBUTTON, VK_SPACE, GetAsyncKeyState};

#[path = "offsets.rs"]
pub mod offsets;
use offsets::{OFFSETS_NET, OFFSETS_SIG};
#[path = "proc_mem.rs"]
pub mod proc_mem;
use proc_mem::ProcMem;

pub struct HackVars {
    pub game_module: DWORD,
    pub client_state: DWORD,
    pub process_mem: ProcMem,
    pub local_player: DWORD,
}

impl HackVars{
    fn set_local_player(&mut self){
        self.process_mem.read_mem::<DWORD>(self.game_module + OFFSETS_SIG.dwLocalPlayer, &mut self.local_player);
    }

    fn local_get_punch_angles(&self) -> D3DVECTOR {
        let mut pos = D3DVECTOR { x: 0.0, y: 0.0, z: 0.0};
        self.process_mem.read_mem::<D3DVECTOR>(self.local_player + OFFSETS_NET.m_aimPunchAngle, &mut pos);
        return pos;
    }

    fn local_get_view_offset(&self) -> D3DVECTOR {
        let mut off = D3DVECTOR {x: 0.0, y: 0.0, z: 0.0};
        self.process_mem.read_mem::<D3DVECTOR>(self.local_player + OFFSETS_NET.m_vecViewOffset, &mut off);
        return off
    }

    fn local_set_view_angle(&self, view_angle: &mut D3DVECTOR) {
        self.process_mem.write_mem::<D3DVECTOR>(self.client_state + OFFSETS_SIG.dwClientState_ViewAngles, view_angle);
    }

    fn local_get_flags(&self) -> i32 {
        let mut flags = 0;
        self.process_mem.read_mem::<i32>(self.local_player + OFFSETS_NET.m_fFlags, &mut flags);
        return flags
    }

    fn get_entity(&self, idx: u32) -> DWORD {
        let mut entity: DWORD = NULL as DWORD;
        self.process_mem.read_mem::<DWORD>(self.game_module + OFFSETS_SIG.dwEntityList + idx*0x10, &mut entity);
        return entity;
    }

    fn entity_get_hp(&self, entity: DWORD) -> i32 {
        let mut entity_health: i32 = NULL as i32;
        self.process_mem.read_mem::<i32>(entity + OFFSETS_NET.m_iHealth, &mut entity_health);
        return entity_health;
    }

    fn entity_get_team(&self, entity: DWORD) -> i32 {
        let mut entity_team: i32 = NULL as i32;
        self.process_mem.read_mem::<i32>(entity + OFFSETS_NET.m_iTeamNum, &mut entity_team);
        return entity_team;
    }

    fn entity_is_valid(&self, entity: DWORD) -> bool {
        let mut dormant: BOOL = TRUE;
        self.process_mem.read_mem::<BOOL>(entity + OFFSETS_SIG.m_bDormant, &mut dormant);
        return (self.entity_get_hp(entity) > 0 && self.entity_get_hp(entity) < 101 && self.entity_get_team(entity) != 0) && dormant == FALSE
    }

    fn entity_is_immune(&self, entity: DWORD) -> bool {
        let mut stat: BOOL = FALSE;
        self.process_mem.read_mem::<BOOL>(entity + OFFSETS_NET.m_bGunGameImmunity, &mut stat);
        return stat == TRUE;
    }

    fn entity_is_spotted(&self, entity: DWORD) -> bool {
        let mut spot: BOOL = FALSE;
        self.process_mem.read_mem::<BOOL>(entity + OFFSETS_NET.m_bSpotted, &mut spot);
        return spot == TRUE;
    }

    fn entity_get_bone_matrix(&self, entity: DWORD) -> DWORD {
        let mut bone_matrix: DWORD = NULL as DWORD;
        self.process_mem.read_mem::<DWORD>(entity + OFFSETS_NET.m_dwBoneMatrix, &mut bone_matrix);
        return bone_matrix;
    }

    fn entity_get_bone_pos(&self, entity: DWORD, bone_id: u32) -> D3DVECTOR {

        let matrix = self.entity_get_bone_matrix(entity);
        let mut retval = D3DVECTOR { x: 0.0, y: 0.0, z: 0.0 };
        self.process_mem.read_mem::<f32>(matrix + 0x30 * bone_id + 0x0C, &mut retval.x);
        self.process_mem.read_mem::<f32>(matrix + 0x30 * bone_id + 0x1C, &mut retval.y);
        self.process_mem.read_mem::<f32>(matrix + 0x30 * bone_id + 0x2C, &mut retval.z);
        return retval;
    }

    fn entity_get_pos(&self, entity: DWORD) -> D3DVECTOR {
        let mut pos = D3DVECTOR { x: 0.0, y: 0.0, z: 0.0};
        self.process_mem.read_mem::<D3DVECTOR>(entity + OFFSETS_NET.m_vecOrigin, &mut pos);
        return pos;
    }

    fn entity_get_max_p(&self) -> i32 {
        let mut num = 0;
        self.process_mem.read_mem::<i32>(self.client_state + OFFSETS_SIG.dwClientState_MaxPlayer, &mut num);
        return num
    }

    pub fn wallhax(&mut self) {
        let mut glow_model: DWORD = NULL as DWORD;
        self.set_local_player();
        while self.local_player == NULL as DWORD {
            self.set_local_player();
        }
        self.process_mem.read_mem::<DWORD>(self.game_module + OFFSETS_SIG.dwGlowObjectManager, &mut glow_model);
        let my_team: i32 = self.entity_get_team(self.local_player);
    
        for i in 0..(self.entity_get_max_p() as u32) {
            let entity: DWORD = self.get_entity(i);
            if entity != NULL as DWORD {
                let mut glow_index: i32 = NULL as i32;
                self.process_mem.read_mem::<i32>(entity + OFFSETS_NET.m_iGlowIndex, &mut glow_index);
                let entity_team: i32 = self.entity_get_team(entity);
                let _entity_health: i32 = self.entity_get_hp(entity); // TODO
                if entity_team != NULL as i32 && entity_team != my_team{
                    //println!("{} entity team", entity_team);
                    let mut val: f32 = 2.0;
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0x4) as u32, &mut val);
                    val = 0.0;
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0x8) as u32, &mut val);
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0xC) as u32, &mut val);
                    val = 0.5;
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0x10) as u32, &mut val);
                } else {
                    let mut val: f32 = 0.0;
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0x4) as u32, &mut val);
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0x8) as u32, &mut val);
                    val = 2.0;
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0xC) as u32, &mut val);
                    val = 0.5;
                    self.process_mem.write_mem::<f32>(glow_model + ((glow_index * 0x38) + 0x10) as u32, &mut val);
                }
                let mut boolval = true;
                self.process_mem.write_mem::<bool>(glow_model + ((glow_index * 0x38) + 0x24) as u32, &mut boolval);
                boolval = false;
                self.process_mem.write_mem::<bool>(glow_model + ((glow_index * 0x38) + 0x25) as u32, &mut boolval);
            }
        }
    }

    fn find_player(&self) -> DWORD {
        let mut player_hd: DWORD = NULL as DWORD;
        let mut low_dist: f32 = 0.0;
        let mut skip = false;
        for i in 0..(self.entity_get_max_p() as u32) {
            let entity = self.get_entity(i);
            if entity != self.local_player && entity != NULL as DWORD && self.entity_get_team(entity) != self.entity_get_team(self.local_player) && self.entity_is_valid(entity) && !self.entity_is_immune(entity)
            {
                /* cheap semi-vis check */
                if self.entity_is_spotted(entity) { skip = true; }
                if skip && !self.entity_is_spotted(entity) { continue; }
                let f32vec = self.entity_get_pos(entity);
                let local_pos = self.entity_get_pos(self.local_player);
                let distsq = (f32vec.x - local_pos.x).powf(2.0) + (f32vec.y - local_pos.y).powf(2.0) + (f32vec.z - local_pos.z).powf(2.0);
                if low_dist == 0.0 || distsq < low_dist {
                    low_dist = distsq;
                    player_hd = entity;
                }
            }
        }

        return player_hd
    }

    pub fn aimbot(&mut self){
        let player = self.find_player();
        self.set_local_player();
        if unsafe { GetAsyncKeyState(VK_LBUTTON) != 0 } && player != NULL as DWORD {
            let player_head = self.entity_get_bone_pos(player, 8);
            let punch_angles = self.local_get_punch_angles();
            let view_offset = self.local_get_view_offset();
            let origin = self.entity_get_pos(self.local_player);
            let my_pos = D3DVECTOR { x: origin.x + view_offset.x, y: origin.y + view_offset.y, z: origin.z + view_offset.z };

            let del = D3DVECTOR { x: player_head.x - my_pos.x, y: player_head.y - my_pos.y, z: player_head.z - my_pos.z };
            let vec_l = (del.x.powf(2.0) + del.y.powf(2.0) + del.z.powf(2.0)).sqrt();
            let mut aim_angle = D3DVECTOR { x: -(del.z / vec_l).asin() * (180.0 / std::f32::consts::PI), y: (del.y).atan2(del.x) * (180.0 / std::f32::consts::PI), z: 0.0};
            aim_angle = D3DVECTOR { x: aim_angle.x - punch_angles.x*2.0, y: aim_angle.y - punch_angles.y*2.0, z: aim_angle.z - punch_angles.z*2.0};
            self.local_set_view_angle(&mut aim_angle);
        }
    }

    pub fn bhop(&self){
        if unsafe { GetAsyncKeyState(VK_SPACE) != 0 } && self.local_get_flags() == 257 {
            let mut val = 6;
            self.process_mem.write_mem::<i32>(self.game_module + OFFSETS_SIG.dwForceJump, &mut val);
        }
    }
}
