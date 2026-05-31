use resvg::usvg;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::path::PathBuf;

const CAR_BODY_SVG: &str = include_str!("../assets/car_body.svg");
const CAR_DETAILS_SVG: &str = include_str!("../assets/car_details.svg");
const MOTORCYCLE_BODY_SVG: &str = include_str!("../assets/motorcycle_body.svg");
const MOTORCYCLE_DETAILS_SVG: &str = include_str!("../assets/motorcycle_details.svg");
const SIGNAL_RED_SVG: &str = include_str!("../assets/traffic_light_red.svg");
const SIGNAL_GREEN_SVG: &str = include_str!("../assets/traffic_light_green.svg");

/// Textures outlive the temporary `TextureCreator` (valid until the window canvas is dropped).
pub struct SpriteAtlas {
    pub car_body: Texture<'static>,
    pub car_details: Texture<'static>,
    pub motorcycle_body: Texture<'static>,
    pub motorcycle_details: Texture<'static>,
    pub signal_red: Texture<'static>,
    pub signal_green: Texture<'static>,
    pub car_w: u32,
    pub car_h: u32,
    pub motorcycle_w: u32,
    pub motorcycle_h: u32,
    pub signal_w: u32,
    pub signal_h: u32,
}

impl SpriteAtlas {
    pub fn load(creator: &TextureCreator<WindowContext>) -> Result<Self, String> {
        let car_dims = (56u32, 112u32);
        let bike_dims = (32u32, 72u32);
        let signal_dims = (32u32, 48u32);

        let car_body = texture_from_svg(creator, CAR_BODY_SVG, car_dims.0, car_dims.1)?;
        let car_details = texture_from_svg(creator, CAR_DETAILS_SVG, car_dims.0, car_dims.1)?;
        let motorcycle_body =
            texture_from_svg(creator, MOTORCYCLE_BODY_SVG, bike_dims.0, bike_dims.1)?;
        let motorcycle_details =
            texture_from_svg(creator, MOTORCYCLE_DETAILS_SVG, bike_dims.0, bike_dims.1)?;
        let signal_red = texture_from_svg(creator, SIGNAL_RED_SVG, signal_dims.0, signal_dims.1)?;
        let signal_green =
            texture_from_svg(creator, SIGNAL_GREEN_SVG, signal_dims.0, signal_dims.1)?;

        Ok(Self {
            car_body: extend_texture_lifetime(car_body),
            car_details: extend_texture_lifetime(car_details),
            motorcycle_body: extend_texture_lifetime(motorcycle_body),
            motorcycle_details: extend_texture_lifetime(motorcycle_details),
            signal_red: extend_texture_lifetime(signal_red),
            signal_green: extend_texture_lifetime(signal_green),
            car_w: car_dims.0,
            car_h: car_dims.1,
            motorcycle_w: bike_dims.0,
            motorcycle_h: bike_dims.1,
            signal_w: signal_dims.0,
            signal_h: signal_dims.1,
        })
    }
}

/// SDL2 keeps GPU textures valid after `TextureCreator` is dropped; the borrow checker does not.
fn extend_texture_lifetime(texture: Texture<'_>) -> Texture<'static> {
    unsafe { std::mem::transmute(texture) }
}

fn assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}

fn rasterize_svg(svg_data: &str, width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut opt = usvg::Options::default();
    opt.resources_dir = Some(assets_dir());

    let tree = usvg::Tree::from_str(svg_data, &opt).map_err(|e| e.to_string())?;
    let size = tree.size();
    if size.width() <= 0.0 || size.height() <= 0.0 {
        return Err("invalid SVG size".into());
    }

    let scale = (width as f32 / size.width()).min(height as f32 / size.height());
    let transform = tiny_skia::Transform::from_scale(scale, scale);

    let mut pixmap =
        tiny_skia::Pixmap::new(width, height).ok_or_else(|| "pixmap alloc failed".to_string())?;
    pixmap.fill(tiny_skia::Color::TRANSPARENT);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(pixmap.data().to_vec())
}

fn texture_from_svg<'a>(
    creator: &'a TextureCreator<WindowContext>,
    svg_data: &str,
    width: u32,
    height: u32,
) -> Result<Texture<'a>, String> {
    let mut rgba = rasterize_svg(svg_data, width, height)?;
    texture_from_rgba(creator, &mut rgba, width, height)
}

fn texture_from_rgba<'a>(
    creator: &'a TextureCreator<WindowContext>,
    pixels: &mut [u8],
    width: u32,
    height: u32,
) -> Result<Texture<'a>, String> {
    let pitch = width as usize * 4;
    let surface = Surface::from_data(
        pixels,
        width,
        height,
        pitch.try_into().map_err(|_| "pitch overflow")?,
        PixelFormatEnum::RGBA32,
    )
    .map_err(|e| e.to_string())?;

    let mut texture = creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    Ok(texture)
}
