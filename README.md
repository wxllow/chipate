# ChipAte

A rusty Chip-8 emulator that ate (but left a lot of crumbs).

## Roadmap

- [x] Chip-8 CPU
- [x] Chip-8 Keypad
- [x] Chip-8 Sound
- [x] Chip-8 Display
- [x] CLI arguments for configuration
- [ ] Load ROM while running
- [ ] Save/Load state
- [ ] Overlay (Show FPS, Speed Change, etc.)
- [ ] Better logging
- [ ] Super Chip support

## Keybinds

### Functions

Esc - Quit
F3 - Reset
F4 - Load ROM (Coming Soon)
F5 - Toggle UI (Coming Soon)
F8 - Pause
F12 - Debug mode
-/_ - Speed down 1 (-60Hz)
+/= - Speed up 1 (+60Hz)

### Keypad

| 1   | 2   | 3   | 4   |
| --- | --- | --- | --- |
| Q   | W   | E   | R   |
| A   | S   | D   | F   |
| Z   | X   | C   | V   |

(Keyboard)

corresponds to

| 1   | 2   | 3   | C   |
| --- | --- | --- | --- |
| 4   | 5   | 6   | D   |
| 7   | 8   | 9   | E   |
| A   | 0   | B   | F   |

(Chip-8 Keypad)
