use std::collections::HashMap;

use crate::Env;
use druid_shell::{Counter, HotKey, Shortcuts};

/// Uniquely identifies a menu item.
///
/// On the druid-shell side, the id is represented as a u32.
/// We reserve '0' as a placeholder value; on the Rust side
/// we represent this as an `Option<NonZerou32>`, which better
/// represents the semantics of our program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortcutId(u32);

impl ShortcutId {
    pub(crate) fn new(id: u32) -> ShortcutId {
        ShortcutId(id)
    }

    pub(crate) fn next() -> ShortcutId {
        static COUNTER: Counter = Counter::new();
        Self::new(COUNTER.next() as u32)
    }
}

type ShortcutCallback<T> = Box<dyn FnMut(&mut T, &Env)>;

///
pub struct ShortcutManager<T> {
    shortcuts: HashMap<ShortcutId, (HotKey, ShortcutCallback<T>)>,
}

impl<T> ShortcutManager<T> {
    ///
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
        }
    }

    ///
    pub fn register(&mut self, hotkey: HotKey, callback: impl FnMut(&mut T, &Env) + 'static) {
        let id = ShortcutId::next();
        self.shortcuts.insert(id, (hotkey, Box::new(callback)));
    }

    ///
    pub fn platform_shortcuts(&self) -> Shortcuts {
        let mut s = Shortcuts::new();
        for (shortcut, (hotkey, _)) in &self.shortcuts {
            s.register(shortcut.0 as i32, hotkey.clone());
        }
        s
    }

    pub(crate) fn event(&mut self, shortcut_id: ShortcutId, data: &mut T, env: &Env) {
        println!(
            "shortcut id: {:?}, register shortcut: {:?}",
            shortcut_id,
            self.shortcuts.keys()
        );
        if let Some((hotkey, cb)) = self.shortcuts.get_mut(&shortcut_id) {
            println!("<{:?}> shortcut callback", hotkey);
            cb(data, env);
        }
    }
}
