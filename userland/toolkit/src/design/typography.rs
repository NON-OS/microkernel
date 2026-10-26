#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontWeight {
    Regular,
    Medium,
    Bold,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextStyle {
    pub px: u8,
    pub weight: FontWeight,
    pub letter_spacing: i8,
    pub line_height: u8,
}

/*
 * The NØNOS type roles (HANDOFF/ui/identity.md): the brand's ramp of 1.2 a step
 * from a 17 px body, in logical pixels at 1x, each with the line it sits on.
 * Labels are the only role under 14, and they are drawn in mono capitals.
 */
impl TextStyle {
    pub const fn label() -> Self {
        Self { px: 12, weight: FontWeight::Regular, letter_spacing: 1, line_height: 16 }
    }
    pub const fn caption() -> Self {
        Self { px: 14, weight: FontWeight::Regular, letter_spacing: 0, line_height: 20 }
    }
    pub const fn body() -> Self {
        Self { px: 17, weight: FontWeight::Regular, letter_spacing: 0, line_height: 24 }
    }
    pub const fn heading() -> Self {
        Self { px: 20, weight: FontWeight::Medium, letter_spacing: 0, line_height: 28 }
    }
    pub const fn lead() -> Self {
        Self { px: 24, weight: FontWeight::Medium, letter_spacing: 0, line_height: 32 }
    }
    pub const fn title() -> Self {
        Self { px: 35, weight: FontWeight::Medium, letter_spacing: 0, line_height: 40 }
    }
}
