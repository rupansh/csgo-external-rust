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
use winapi::shared::minwindef::{DWORD, FALSE, LPVOID, HMODULE};
use winapi::shared::basetsd::DWORD_PTR;
use winapi::shared::ntdef::{HANDLE, NULL};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::{CloseHandle};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::psapi::{EnumProcesses, EnumProcessModulesEx, GetProcessImageFileNameW, GetModuleFileNameExW, LIST_MODULES_32BIT, LIST_MODULES_64BIT};

use std::mem::size_of;
use std::process::exit;
use std::string::String;


#[allow(non_snake_case)]
#[derive(Clone)]
pub struct ProcMem{
    pub h_process: HANDLE,
    pub dw_pid: DWORD,
    pub dw_protection: DWORD,
    pub dw_cave_addr: DWORD,
    pub b_pOn: bool,
    pub b_iOn: bool,
    pub b_prot: bool,
}

impl Drop for ProcMem {
    fn drop(&mut self){
        unsafe { CloseHandle(self.h_process) };
    }
}

impl ProcMem{
    pub fn process(&mut self, proc_name: String){
        let mut proc_list: [DWORD; 1024] = [0; 1024];
        let mut cur_proc: [u16; 260] = [0; 260];
        let mut proc_cnt: DWORD = 0;

        if unsafe { EnumProcesses(proc_list.as_mut_ptr(), size_of::<[DWORD; 1024]>() as u32, &mut proc_cnt) == 0 } {
            println!("ERROR!");
            exit(-1);
        }

        for i in 0..proc_list.len() {
            let proc_h: HANDLE = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, proc_list[i]) };
            unsafe { GetProcessImageFileNameW(proc_h, cur_proc.as_mut_ptr(), size_of::<[DWORD; 260]>() as u32); };
            let proc_str = String::from_utf16(&cur_proc).unwrap();
            if proc_str.contains(&proc_name) {
                self.dw_pid = proc_list[i as usize];
                unsafe { CloseHandle(proc_h) };
                self.h_process = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, self.dw_pid) };
                return
            }
            unsafe { CloseHandle(proc_h) };
        }

        println!("Process not found!");
        exit(0);
    }

    pub fn module(&self, mod_name: String) -> DWORD {
        let mut mod_list: [HMODULE; 1024] = [NULL as HMODULE; 1024];
        let mut cur_mod: [u16; 260] = [0; 260];
        let mut mod_cnt: DWORD = 0;

        if unsafe { EnumProcessModulesEx(self.h_process, mod_list.as_mut_ptr(), size_of::<[DWORD; 1024]>() as u32, &mut mod_cnt, LIST_MODULES_32BIT | LIST_MODULES_64BIT) == 0 } {
            println!("ERROR!");
            exit(-1);
        }

        for i in 0..mod_list.len() {
            unsafe { GetModuleFileNameExW(self.h_process, mod_list[i], cur_mod.as_mut_ptr(), 260); };
            let mod_str = String::from_utf16(&cur_mod).unwrap();
            if mod_str.contains(&mod_name) {
                return mod_list[i] as DWORD_PTR as u32
            }
        }

        println!("Module not found!");
        exit(0);
    }

    pub fn read_mem<T>(&self, dw_addr: DWORD, res: &mut T) {
        unsafe { ReadProcessMemory(self.h_process, dw_addr as usize as *mut _, res as *mut _ as LPVOID, size_of::<T>(), NULL as *mut usize); };
    }

    pub fn write_mem<T>(&self, dw_addr: DWORD, value: &mut T) {
        unsafe { WriteProcessMemory(self.h_process, dw_addr as usize as *mut _, value as *mut _ as LPVOID, size_of::<T>(), NULL as *mut usize); };
    }
}