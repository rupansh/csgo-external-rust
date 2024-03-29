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
use phf::phf_map;

#[allow(dead_code)]
pub static OFFSETS_NET: phf::Map<&'static str, u32> = phf_map!{
        "cs_gamerules_data" => 0x0,
        "m_ArmorValue" => 0x113CC,
        "m_Collision" => 0x320,
        "m_CollisionGroup" => 0x474,
        "m_Local" => 0x2FCC,
        "m_MoveType" => 0x25C,
        "m_OriginalOwnerXuidHigh" => 0x31D4,
        "m_OriginalOwnerXuidLow" => 0x31D0,
        "m_SurvivalGameRuleDecisionTypes" => 0x1328,
        "m_SurvivalRules" => 0xD00,
        "m_aimPunchAngle" => 0x303C,
        "m_aimPunchAngleVel" => 0x3048,
        "m_angEyeAnglesX" => 0x113D0,
        "m_angEyeAnglesY" => 0x113D4,
        "m_bBombDefused" => 0x29C0,
        "m_bBombPlanted" => 0x9A5,
        "m_bBombTicking" => 0x2990,
        "m_bFreezePeriod" => 0x20,
        "m_bGunGameImmunity" => 0x9990,
        "m_bHasDefuser" => 0x113DC,
        "m_bHasHelmet" => 0x113C0,
        "m_bInReload" => 0x32B5,
        "m_bIsDefusing" => 0x997C,
        "m_bIsQueuedMatchmaking" => 0x74,
        "m_bIsScoped" => 0x9974,
        "m_bIsValveDS" => 0x7C,
        "m_bSpotted" => 0x93D,
        "m_bSpottedByMask" => 0x980,
        "m_bStartedArming" => 0x3400,
        "m_bUseCustomAutoExposureMax" => 0x9D9,
        "m_bUseCustomAutoExposureMin" => 0x9D8,
        "m_bUseCustomBloomScale" => 0x9DA,
        "m_clrRender" => 0x70,
        "m_dwBoneMatrix" => 0x26A8,
        "m_fAccuracyPenalty" => 0x3340,
        "m_fFlags" => 0x104,
        "m_flC4Blow" => 0x29A0,
        "m_flCustomAutoExposureMax" => 0x9E0,
        "m_flCustomAutoExposureMin" => 0x9DC,
        "m_flCustomBloomScale" => 0x9E4,
        "m_flDefuseCountDown" => 0x29BC,
        "m_flDefuseLength" => 0x29B8,
        "m_flFallbackWear" => 0x31E0,
        "m_flFlashDuration" => 0x10470,
        "m_flFlashMaxAlpha" => 0x1046C,
        "m_flLastBoneSetupTime" => 0x2928,
        "m_flLowerBodyYawTarget" => 0x9ADC,
        "m_flNextAttack" => 0x2D80,
        "m_flNextPrimaryAttack" => 0x3248,
        "m_flSimulationTime" => 0x268,
        "m_flTimerLength" => 0x29A4,
        "m_hActiveWeapon" => 0x2F08,
        "m_hBombDefuser" => 0x29C4,
        "m_hMyWeapons" => 0x2E08,
        "m_hObserverTarget" => 0x339C,
        "m_hOwner" => 0x29DC,
        "m_hOwnerEntity" => 0x14C,
        "m_hViewModel" => 0x3308,
        "m_iAccountID" => 0x2FD8,
        "m_iClip1" => 0x3274,
        "m_iCompetitiveRanking" => 0x1A84,
        "m_iCompetitiveWins" => 0x1B88,
        "m_iCrosshairId" => 0x11438,
        "m_iDefaultFOV" => 0x333C,
        "m_iEntityQuality" => 0x2FBC,
        "m_iFOV" => 0x31F4,
        "m_iFOVStart" => 0x31F8,
        "m_iGlowIndex" => 0x10488,
        "m_iHealth" => 0x100,
        "m_iItemDefinitionIndex" => 0x2FBA,
        "m_iItemIDHigh" => 0x2FD0,
        "m_iMostRecentModelBoneCounter" => 0x2690,
        "m_iObserverMode" => 0x3388,
        "m_iShotsFired" => 0x103E0,
        "m_iState" => 0x3268,
        "m_iTeamNum" => 0xF4,
        "m_lifeState" => 0x25F,
        "m_nBombSite" => 0x2994,
        "m_nFallbackPaintKit" => 0x31D8,
        "m_nFallbackSeed" => 0x31DC,
        "m_nFallbackStatTrak" => 0x31E4,
        "m_nForceBone" => 0x268C,
        "m_nTickBase" => 0x3440,
        "m_nViewModelIndex" => 0x29D0,
        "m_rgflCoordinateFrame" => 0x444,
        "m_szCustomName" => 0x304C,
        "m_szLastPlaceName" => 0x35C4,
        "m_thirdPersonViewAngles" => 0x31E8,
        "m_vecOrigin" => 0x138,
        "m_vecVelocity" => 0x114,
        "m_vecViewOffset" => 0x108,
        "m_viewPunchAngle" => 0x3030,
        "m_zoomLevel" => 0x33E0,
};


pub static OFFSETS_SIG: phf::Map<&'static str, u32> = phf_map!{
        "anim_overlays" => 0x2990,
        "clientstate_choked_commands" => 0x4D30,
        "clientstate_delta_ticks" => 0x174,
        "clientstate_last_outgoing_command" => 0x4D2C,
        "clientstate_net_channel" => 0x9C,
        "convar_name_hash_table" => 0x2F0F8,
        "dwClientState" => 0x588FEC,
        "dwClientState_GetLocalPlayer" => 0x180,
        "dwClientState_IsHLTV" => 0x4D48,
        "dwClientState_Map" => 0x28C,
        "dwClientState_MapDirectory" => 0x188,
        "dwClientState_MaxPlayer" => 0x388,
        "dwClientState_PlayerInfo" => 0x52C0,
        "dwClientState_State" => 0x108,
        "dwClientState_ViewAngles" => 0x4D90,
        "dwEntityList" => 0x4DBD61C,
        "dwForceAttack" => 0x31EDB34,
        "dwForceAttack2" => 0x31EDB40,
        "dwForceBackward" => 0x31EDB7C,
        "dwForceForward" => 0x31EDB88,
        "dwForceJump" => 0x526742C,
        "dwForceLeft" => 0x31EDBA0,
        "dwForceRight" => 0x31EDB94,
        "dwGameDir" => 0x627700,
        "dwGameRulesProxy" => 0x52DA70C,
        "dwGetAllClasses" => 0xDCBA7C,
        "dwGlobalVars" => 0x588CF0,
        "dwGlowObjectManager" => 0x5305B18,
        "dwInput" => 0x520EBE0,
        "dwInterfaceLinkList" => 0x954004,
        "dwLocalPlayer" => 0xDA344C,
        "dwMouseEnable" => 0xDA8F98,
        "dwMouseEnablePtr" => 0xDA8F68,
        "dwPlayerResource" => 0x31EBEC0,
        "dwRadarBase" => 0x51F2394,
        "dwSensitivity" => 0xDA8E34,
        "dwSensitivityPtr" => 0xDA8E08,
        "dwSetClanTag" => 0x8A130,
        "dwViewMatrix" => 0x4DAEF34,
        "dwWeaponTable" => 0x520F6A4,
        "dwWeaponTableIndex" => 0x326C,
        "dwYawPtr" => 0xDA8BF8,
        "dwZoomSensitivityRatioPtr" => 0xDADE98,
        "dwbSendPackets" => 0xD7B42,
        "dwppDirect3DDevice9" => 0xA7050,
        "find_hud_element" => 0x5CC527A0,
        "force_update_spectator_glow" => 0x3B3D5A,
        "interface_engine_cvar" => 0x3E9EC,
        "is_c4_owner" => 0x3C0CA0,
        "m_bDormant" => 0xED,
        "m_flSpawnTime" => 0x103C0,
        "m_pStudioHdr" => 0x2950,
        "m_pitchClassPtr" => 0x51F2630,
        "m_yawClassPtr" => 0xDA8BF8,
        "model_ambient_min" => 0x58C00C,
        "set_abs_angles" => 0x1E37C0,
        "set_abs_origin" => 0x1E3600,
};