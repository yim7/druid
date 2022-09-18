use std::collections::HashMap;

use winapi::shared::windef::*;
use winapi::um::winuser::*;

use crate::{HotKey, Modifiers};

use super::keyboard;

#[derive(Clone, Default)]
pub struct Shortcuts {
    hotkeys: HashMap<i32, HotKey>,
}

impl Shortcuts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, shortcut: i32, hotkey: HotKey) {
        self.hotkeys.insert(shortcut, hotkey);
    }

    pub fn initialize(&self, win: HWND) {
        for (id, hotkey) in &self.hotkeys {
            let modifiers = Modifiers::from(hotkey.mods).raw().bits();
            let vk = keyboard::key_to_vk(&hotkey.key).expect("error key");
            println!("register shortcut, modifiers {:x}, vk {:x}", modifiers, vk);
            unsafe {
                let ok = RegisterHotKey(win, *id, modifiers, vk as u32);
                tracing::info!("Register gloabl shortcut {}, ret {}", hotkey.key, ok);
            }
        }
    }
}
