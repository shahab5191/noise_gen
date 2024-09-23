#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Color {r, g, b}
    }
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn as_array(&self) -> [u8; 3]{
        [self.r, self.g, self.b]
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        let color = Color {
            r: self.r,
            g: self.g,
            b: self.b,
        };
        return color;
    }
}
