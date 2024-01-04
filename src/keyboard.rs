use sdl2::keyboard::Keycode;

pub fn get_hex_val(kc: Keycode) -> Option<u8> {
    match kc {
        Keycode::Num0 => Some(0x0),
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0x4),
        Keycode::Num5 => Some(0x5),
        Keycode::Num6 => Some(0x6),
        Keycode::Num7 => Some(0x7),
        Keycode::Num8 => Some(0x8),
        Keycode::Num9 => Some(0x9),
        Keycode::A => Some(0xa),
        Keycode::B => Some(0xb),
        Keycode::C => Some(0xc),
        Keycode::D => Some(0xd),
        Keycode::E => Some(0xe),
        Keycode::F => Some(0xf),
        _ => return None,
    }
}
