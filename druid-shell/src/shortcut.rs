use crate::backend::shortcut as backend;
use crate::hotkey::HotKey;

pub struct Shortcuts {
    pub(crate) inner: backend::Shortcuts,
}

impl Shortcuts {
    pub fn new() -> Self {
        Self {
            inner: backend::Shortcuts::new(),
        }
    }

    pub fn register(&mut self, shortcut: i32, hotkey: HotKey) {
        self.inner.register(shortcut, hotkey);
    }

    pub fn into_inner(self) -> backend::Shortcuts {
        self.inner
    }
}
