#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyCode {
    Backspace = 8,
    Tab = 9,
    Enter = 12,
    Shift = 16,
    Control = 17,
    Alt = 18,
    Pause = 19,
    CapsLock = 20,
    Escape = 27,
    Space = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    ArrowLeft = 37,
    ArrowUp = 38,
    ArrowRight = 39,
    ArrowDown = 40,
    PrintScreen = 44,
    Insert = 45,
    Delete = 46,
    Digit0 = 48,
    Digit1 = 49,
    Digit2 = 50,
    Digit3 = 51,
    Digit4 = 52,
    Digit5 = 53,
    Digit6 = 54,
    Digit7 = 55,
    Digit8 = 56,
    Digit9 = 57,
    SemiColon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    Meta = 91,
    ContextMenu = 93,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    NumpadMul = 106,
    NumpadPlus = 107,
    NumpadMinus = 109,
    NumpadPeriod = 110,
    NumpadSlash = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    NumLock = 144,
    ScrollLock = 145,
    Minus = 173,
    Comma = 188,
    Period = 190,
    Slash = 191,
    Tilde = 192,
    BracketLeft = 219,
    BackSlash = 220,
    BracketRight = 221,
    Quote = 222,
}

impl From<KeyCode> for u8 {
    fn from(value: KeyCode) -> Self {
        value as u8
    }
}

#[derive(Clone, Debug)]
pub struct InvalidKeyCode(u8);

impl TryFrom<u8> for KeyCode {
    type Error = InvalidKeyCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(KeyCode::Backspace),
            9 => Ok(KeyCode::Tab),
            12 => Ok(KeyCode::Enter),
            16 => Ok(KeyCode::Shift),
            17 => Ok(KeyCode::Control),
            18 => Ok(KeyCode::Alt),
            19 => Ok(KeyCode::Pause),
            20 => Ok(KeyCode::CapsLock),
            27 => Ok(KeyCode::Escape),
            32 => Ok(KeyCode::Space),
            33 => Ok(KeyCode::PageUp),
            34 => Ok(KeyCode::PageDown),
            35 => Ok(KeyCode::End),
            36 => Ok(KeyCode::Home),
            37 => Ok(KeyCode::ArrowLeft),
            38 => Ok(KeyCode::ArrowUp),
            39 => Ok(KeyCode::ArrowRight),
            40 => Ok(KeyCode::ArrowDown),
            44 => Ok(KeyCode::PrintScreen),
            45 => Ok(KeyCode::Insert),
            46 => Ok(KeyCode::Delete),
            48 => Ok(KeyCode::Digit0),
            49 => Ok(KeyCode::Digit1),
            50 => Ok(KeyCode::Digit2),
            51 => Ok(KeyCode::Digit3),
            52 => Ok(KeyCode::Digit4),
            53 => Ok(KeyCode::Digit5),
            54 => Ok(KeyCode::Digit6),
            55 => Ok(KeyCode::Digit7),
            56 => Ok(KeyCode::Digit8),
            57 => Ok(KeyCode::Digit9),
            59 => Ok(KeyCode::SemiColon),
            61 => Ok(KeyCode::Equal),
            65 => Ok(KeyCode::A),
            66 => Ok(KeyCode::B),
            67 => Ok(KeyCode::C),
            68 => Ok(KeyCode::D),
            69 => Ok(KeyCode::E),
            70 => Ok(KeyCode::F),
            71 => Ok(KeyCode::G),
            72 => Ok(KeyCode::H),
            73 => Ok(KeyCode::I),
            74 => Ok(KeyCode::J),
            75 => Ok(KeyCode::K),
            76 => Ok(KeyCode::L),
            77 => Ok(KeyCode::M),
            78 => Ok(KeyCode::N),
            79 => Ok(KeyCode::O),
            80 => Ok(KeyCode::P),
            81 => Ok(KeyCode::Q),
            82 => Ok(KeyCode::R),
            83 => Ok(KeyCode::S),
            84 => Ok(KeyCode::T),
            85 => Ok(KeyCode::U),
            86 => Ok(KeyCode::V),
            87 => Ok(KeyCode::W),
            88 => Ok(KeyCode::X),
            89 => Ok(KeyCode::Y),
            90 => Ok(KeyCode::Z),
            91 => Ok(KeyCode::Meta),
            93 => Ok(KeyCode::ContextMenu),
            96 => Ok(KeyCode::Numpad0),
            97 => Ok(KeyCode::Numpad1),
            98 => Ok(KeyCode::Numpad2),
            99 => Ok(KeyCode::Numpad3),
            100 => Ok(KeyCode::Numpad4),
            101 => Ok(KeyCode::Numpad5),
            102 => Ok(KeyCode::Numpad6),
            103 => Ok(KeyCode::Numpad7),
            104 => Ok(KeyCode::Numpad8),
            105 => Ok(KeyCode::Numpad9),
            106 => Ok(KeyCode::NumpadMul),
            107 => Ok(KeyCode::NumpadPlus),
            109 => Ok(KeyCode::NumpadMinus),
            110 => Ok(KeyCode::NumpadPeriod),
            111 => Ok(KeyCode::NumpadSlash),
            112 => Ok(KeyCode::F1),
            113 => Ok(KeyCode::F2),
            114 => Ok(KeyCode::F3),
            115 => Ok(KeyCode::F4),
            116 => Ok(KeyCode::F5),
            117 => Ok(KeyCode::F6),
            118 => Ok(KeyCode::F7),
            119 => Ok(KeyCode::F8),
            120 => Ok(KeyCode::F9),
            121 => Ok(KeyCode::F10),
            122 => Ok(KeyCode::F11),
            123 => Ok(KeyCode::F12),
            144 => Ok(KeyCode::NumLock),
            145 => Ok(KeyCode::ScrollLock),
            173 => Ok(KeyCode::Minus),
            188 => Ok(KeyCode::Comma),
            190 => Ok(KeyCode::Period),
            191 => Ok(KeyCode::Slash),
            192 => Ok(KeyCode::Tilde),
            219 => Ok(KeyCode::BracketLeft),
            220 => Ok(KeyCode::BackSlash),
            221 => Ok(KeyCode::BracketRight),
            222 => Ok(KeyCode::Quote),
            n => Err(InvalidKeyCode(n)),
        }
    }
}