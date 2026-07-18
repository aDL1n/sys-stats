use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;

pub struct ByteString(Vec<u16>);

impl ByteString {
    pub fn from(string: String) -> ByteString {
        ByteString(string.encode_utf16().collect())
    }

    pub fn get_utf16(&self) -> &[u16] {
        self.0.as_slice()
    }
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Size {
        Size { width, height }
    }
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub fn rectangle(width: u16, height: u16, position: &Position) -> D2D_RECT_F {
    D2D_RECT_F {
        left: position.x as f32,
        top: (height + position.y) as f32,
        right: (width + position.x) as f32,
        bottom: position.y as f32,
    }
}

pub fn get_text_width(value: &str) -> u16 {
    let max_chars = value
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    ((max_chars * 9)) as u16
}
