use crate::{ChessPiece, Color};

use fltk::image::{SharedImage, SvgImage};

pub struct TupleWrapper<T, U>(T, U);

impl<T, U> TupleWrapper<T, U>
where
    T: Into<ChessPiece>,
    U: Into<Color>,
{
    pub fn into_shared_image(self) -> Option<SharedImage> {
        self.into()
    }
}
impl<T, U> Into<Option<SvgImage>> for TupleWrapper<T, U>
where
    T: Into<ChessPiece>,
    U: Into<Color>,
{
    fn into(self) -> Option<SvgImage> {
        let svg_string = match (self.0.into(), self.1.into()) {
            (ChessPiece::King, Color::Black) => include_str!("resources/king_black.svg"),
            (ChessPiece::King, Color::White) => include_str!("resources/king_white.svg"),
            (ChessPiece::Queen, Color::Black) => include_str!("resources/queen_black.svg"),
            (ChessPiece::Queen, Color::White) => include_str!("resources/queen_white.svg"),
            (ChessPiece::Bishoph, Color::Black) => include_str!("resources/bishoph_black.svg"),
            (ChessPiece::Bishoph, Color::White) => include_str!("resources/bishoph_white.svg"),
            (ChessPiece::Pawn, Color::Black) => include_str!("resources/pawn_black.svg"),
            (ChessPiece::Pawn, Color::White) => include_str!("resources/pawn_white.svg"),
            (ChessPiece::Rook, Color::Black) => include_str!("resources/rook_black.svg"),
            (ChessPiece::Rook, Color::White) => include_str!("resources/rook_white.svg"),
            (ChessPiece::Knight, Color::Black) => include_str!("resources/knight_black.svg"),
            (ChessPiece::Knight, Color::White) => include_str!("resources/knight_white.svg"),
        };

        SvgImage::from_data(svg_string).ok()
    }
}

impl<T, U> Into<Option<SharedImage>> for TupleWrapper<T, U>
where
    T: Into<ChessPiece>,
    U: Into<Color>,
{
    fn into(self) -> Option<SharedImage> {
        let img: Option<SvgImage> = self.into();
        img.and_then(|svg| SharedImage::from_image(svg).ok())
    }
}

impl From<(ChessPiece, Color)> for TupleWrapper<ChessPiece, Color> {
    fn from(tuple: (ChessPiece, Color)) -> Self {
        TupleWrapper(tuple.0, tuple.1)
    }
}

#[cfg(test)]
mod test {
    use super::TupleWrapper;
    use crate::{ChessPiece, Color};
    use fltk::image::SvgImage;
    #[test]
    fn creates_svg() {
        let img: Option<SvgImage> = TupleWrapper::from((ChessPiece::King, Color::White)).into();
        assert!(img.is_some());
    }
}
