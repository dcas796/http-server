use iced::Font;
use iced::font::Weight;
use iced::widget::Text;

pub trait TextExt {
    fn bold(self) -> Self;
}

impl TextExt for Text<'_> {
    fn bold(self) -> Self {
        let mut font = Font::default();
        font.weight = Weight::Bold;
        self.font(font)
    }
}