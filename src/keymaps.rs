use std::{fs::File, io::Read, path::Path};

/**
 * Keymap parsing
 */
#[cfg(feature = "keymaps")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "keymaps")]
#[derive(Serialize, Deserialize, Clone)]
pub struct Keymap {
    pub scancode: i32,
    pub key: u8,
    pub comments: Option<String>,
}

#[cfg(not(feature = "keymaps"))]
#[derive(Clone)]
pub struct Keymap {
    pub scancode: i32,
    pub key: u8,
    pub comments: Option<String>,
}

#[cfg(feature = "keymaps")]
pub fn parse_keymap_file(keymap_file: &Path) -> Vec<Keymap> {
    let mut file = File::open(keymap_file).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).expect(
        format!(
            "Failed to parse keymap file: {}",
            keymap_file.to_str().unwrap()
        )
        .as_str(),
    )
}
