use minifb::Key;
use super::Emulator;
use std::collections::hash_map::HashMap;

impl Emulator{
    // Get the key mapping
    fn get_keymap() -> HashMap<Key, u8>{
        HashMap::from([
            (Key::Key1,1),
            (Key::Key2,2),
            (Key::Key3,3),
            (Key::Key4,12),
            (Key::Q,4),
            (Key::W,5),
            (Key::E,6),
            (Key::R,13),
            (Key::A,7),
            (Key::S,8),
            (Key::D,9),
            (Key::F,14),
            (Key::Z,10),
            (Key::X,0),
            (Key::C,11),
            (Key::V,15),
        ])
    }

    // Check If a key is being pressed
    pub fn scan_key(&self, key:u8) -> bool{
        let key_map = Emulator::get_keymap();
        let keycode = key_map.iter().find_map(|(k, &v)| if v == key {Some(k)} else {None});

        match keycode{
            None => return false,
            Some(a) => {
                return self.display.window.is_key_down(*a)
            }
        }
    }

    // Check what key is being pressed
    pub fn scan_any(&self) -> Option<u8>{
        let key_map = Emulator::get_keymap();
        let pressed = self.display.window.get_keys();

        for key in pressed{
            let value = key_map.get(&key);
            match value{
                Some(&v) => return Some(v),
                None => continue
            }
        }
        None
    }
}

