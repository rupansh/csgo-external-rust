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
use winapi::shared::d3d9types::D3DVECTOR;
use winapi::shared::minwindef::{DWORD, BOOL, FALSE, TRUE};
use winapi::shared::ntdef::{NULL};
use winapi::um::winuser::{VK_LBUTTON, VK_SPACE, GetAsyncKeyState};

#[path = "offsets.rs"]
pub mod offsets;
use offsets::*;
#[path = "proc_mem.rs"]
pub mod proc_mem;
use proc_mem::*;
#[path = "player_struct.rs"]
pub mod player_struct;
use player_struct::*;


pub struct Entity {
    pub entity: Option<DWORD>,
    pub player: PlayerStruct,
	pub valid: bool
}

impl Default for Entity {
    fn default() -> Entity {
        return Entity { entity: None,
            player: PlayerStruct::default(),
			valid: false
        };
    }
}

impl Entity {
    fn set_valid(&mut self, process_mem: &ProcMem) {
        let mut dormant: BOOL = TRUE;
        let entity = self.entity.unwrap();
        process_mem.read_mem::<BOOL>(entity + OFFSETS_SIG.get(&"m_bDormant").cloned().unwrap(), &mut dormant);
        self.valid = self.player.m_iHealth > 0 && self.player.m_iHealth < 101 && dormant == FALSE;
    }

    fn get_bone_pos(&mut self, process_mem: &ProcMem, bone_id: u32) -> D3DVECTOR {
        let bone_matrix = self.player.m_dwBoneMatrix;
        let mut retval = D3DVECTOR { x: 0.0, y: 0.0, z: 0.0 };
        process_mem.read_mem::<f32>(bone_matrix as DWORD + 0x30 * bone_id + 0x0C, &mut retval.x);
        process_mem.read_mem::<f32>(bone_matrix as DWORD + 0x30 * bone_id + 0x1C, &mut retval.y);
        process_mem.read_mem::<f32>(bone_matrix as DWORD + 0x30 * bone_id + 0x2C, &mut retval.z);
        return retval;
    }

    pub fn set_entity(&mut self, game_module: DWORD, process_mem: &ProcMem, idx: u32) {
        let mut entity = NULL as DWORD;
		process_mem.read_mem::<DWORD>(game_module + OFFSETS_SIG.get(&"dwEntityList").cloned().unwrap() + idx*0x10, &mut entity);
        if entity != NULL as DWORD {
            self.entity = Some(entity);
            process_mem.read_mem::<PlayerStruct>(entity, &mut self.player);
        }
    }
}

pub struct Game {
    pub game_module: DWORD,
    pub client_state: DWORD,
    pub process_mem: ProcMem,
	pub local_entity: DWORD,
    pub local_player: PlayerStruct,
    pub max_players: Option<i32>,
}

impl Game {
    fn set_local_player(&mut self) {
		self.process_mem.read_mem::<DWORD>(self.game_module + OFFSETS_SIG.get(&"dwLocalPlayer").cloned().unwrap(), &mut self.local_entity);
		if self.local_entity != NULL as DWORD {
			self.process_mem.read_mem::<PlayerStruct>(self.local_entity, &mut self.local_player);
		}
    }

    fn local_set_view_angle(&self, view_angle: &mut D3DVECTOR) {
        self.process_mem.write_mem::<D3DVECTOR>(self.client_state + OFFSETS_SIG.get(&"dwClientState_ViewAngles").cloned().unwrap(), view_angle);
    }

    fn game_set_max_p(&mut self) {
        let mut num = 0;
		self.process_mem.read_mem::<i32>(self.client_state + OFFSETS_SIG.get(&"dwClientState_MaxPlayer").cloned().unwrap(), &mut num);
        self.max_players = Some(num);
    }

    pub fn wallhax(&mut self) {
        self.set_local_player();
        while self.local_entity == NULL as DWORD {
            self.set_local_player();
        }
        let mut glow_model = NULL as DWORD;
		self.process_mem.read_mem::<DWORD>(self.game_module + OFFSETS_SIG.get(&"dwGlowObjectManager").cloned().unwrap(), &mut glow_model);
        let my_team = self.local_player.m_iTeamNum;
        self.game_set_max_p();
    
        for i in 0..(self.max_players.unwrap() as u32) {
            let mut entity: Entity = Entity::default();
			entity.set_entity(self.game_module, &self.process_mem, i);
            if !entity.entity.is_none() {
				let player = entity.player;
                let glow_index = player.m_iGlowIndex;
                if player.m_iTeamNum != my_team{
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

    fn find_player(&mut self) -> Entity {
        let mut player_hd: Entity = Entity::default();
        let mut low_dist: f32 = 0.0;
        let mut skip = false;
        self.game_set_max_p();
        for i in 0..(self.max_players.unwrap() as u32) {
            let mut entity = Entity::default();
            entity.set_entity(self.game_module, &self.process_mem, i);
            if entity.entity.is_none() {
                continue
            }
			entity.set_valid(&self.process_mem);
            if entity.entity.unwrap() != self.local_entity && entity.player.m_iTeamNum != self.local_player.m_iTeamNum && entity.valid && entity.player.m_bGunGameImmunity == 0 {
                /* cheap semi-vis check */
                if entity.player.m_bSpotted == 1 { skip = true; }
                if skip && entity.player.m_bSpotted == 0 { continue; }
                let f32vec = entity.player.m_vecOrigin;
                let local_pos = self.local_player.m_vecOrigin;
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
        let mut player = self.find_player();
        self.set_local_player();
        if unsafe { GetAsyncKeyState(VK_LBUTTON) != 0 } && !player.entity.is_none() {
            let player_head = player.get_bone_pos(&self.process_mem, 8);
            let view_offset = self.local_player.m_vecViewOffset;
            let origin = self.local_player.m_vecOrigin;
            let my_pos = D3DVECTOR { x: origin.x + view_offset.x, y: origin.y + view_offset.y, z: origin.z + view_offset.z };

            let del = D3DVECTOR { x: player_head.x - my_pos.x, y: player_head.y - my_pos.y, z: player_head.z - my_pos.z };
            let vec_l = (del.x.powf(2.0) + del.y.powf(2.0) + del.z.powf(2.0)).sqrt();
            let mut aim_angle = D3DVECTOR { x: -(del.z / vec_l).asin() * (180.0 / std::f32::consts::PI), y: (del.y).atan2(del.x) * (180.0 / std::f32::consts::PI), z: 0.0};

            let _numshot = self.local_player.m_iShotsFired;
            let punch_angles = self.local_player.m_aimPunchAngle;
            aim_angle.x = aim_angle.x  - (punch_angles.x * 2.0);
            aim_angle.y = aim_angle.y - (punch_angles.y * 2.0);
    
            while aim_angle.y > 180.0 {
                aim_angle.y -= 360.0
            }
    
            while aim_angle.y < -180.0 {
                aim_angle.y += 360.0
            }
    
            if aim_angle.x > 89.0 {
                aim_angle.x = 89.0
            }
    
            if aim_angle.x < -89.0 {
                aim_angle.x = -89.0
            }

            self.local_set_view_angle(&mut aim_angle);
        }
    }

    pub fn bhop(&mut self){
        self.set_local_player();
        if unsafe { GetAsyncKeyState(VK_SPACE) != 0 } && self.local_entity != NULL as DWORD && self.local_player.m_fFlags == 257 {
            let mut val = 6;
            self.process_mem.write_mem::<i32>(self.game_module + OFFSETS_SIG.get(&"dwForceJump").cloned().unwrap(), &mut val);
        }
    }
}
