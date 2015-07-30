use pixel_image_simple::RGBA8;

pub trait PaletteRGBA8 {
    fn new() -> Self;
    fn get_palette(self) -> Self;
    fn get_palette_length(&self) -> usize;
}
