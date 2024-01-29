

use crate::consts::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::surface::{Surface};


use sdl2::Sdl;

pub struct Config<'a> {
    pub fullscreen: bool,
    pub software_render: bool,
    pub fg_hex: &'a str,
    pub bg_hex: &'a str,
}

pub struct Display {
    pub canvas: sdl2::render::WindowCanvas,
    pub pixels: [[bool; WIDTH]; HEIGHT],
    pub fg: Color,
    pub bg: Color,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub info: String,
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), ()> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;

    Ok((r, g, b))
}

impl Display {
    pub fn new(sdl_context: &Sdl, config: Config) -> Result<Display, String> {
        let video_subsystem = sdl_context
            .video()
            .map_err(|_| "Failed to get video subsystem".to_string())?;

        let mut binding = video_subsystem.window(
            "Chip8",
            WIDTH as u32 * SCALE_FACTOR,
            HEIGHT as u32 * SCALE_FACTOR,
        );
        let mut window_builder = binding.position_centered();

        if config.fullscreen {
            window_builder = window_builder.fullscreen();
        }

        let window = window_builder
            .build()
            .map_err(|e| format!("Failed to create window: {}", e))?;

        let mut canvas_builder = window.into_canvas();

        if config.software_render {
            canvas_builder = canvas_builder.software();
        }

        let canvas = canvas_builder
            .build()
            .map_err(|e| format!("Failed to create software rendered canvas: {}", e))?;

        let background_color = hex_to_rgb(config.bg_hex).unwrap();
        let foreground_color = hex_to_rgb(config.fg_hex).unwrap();

        let texture_creator = canvas.texture_creator();

        let display = Display {
            canvas,
            pixels: [[false; WIDTH]; HEIGHT],
            bg: Color::RGB(background_color.0, background_color.1, background_color.2),
            fg: Color::RGB(foreground_color.0, foreground_color.1, foreground_color.2),
            texture_creator,
            info: "--".to_string(),
        };

        Ok(display)
    }

    pub fn draw(&mut self, pixels: &[[bool; WIDTH]; HEIGHT]) {
        let window_width = self.canvas.window().size().0;
        let window_height = self.canvas.window().size().1;
        let scale_factor =
            std::cmp::min(window_width / WIDTH as u32, window_height / HEIGHT as u32);

        /* Draw screen to texture */
        let mut surface = Surface::new(
            WIDTH as u32,
            HEIGHT as u32,
            sdl2::pixels::PixelFormatEnum::RGB24,
        )
        .unwrap();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if pixels[y][x] { self.fg } else { self.bg };

                surface
                    .fill_rect(Rect::new((x as u32) as i32, (y as u32) as i32, 1, 1), color)
                    .unwrap();
            }
        }

        let texture = Texture::from_surface(&surface, &self.texture_creator).unwrap();
        let src = Rect::new(0, 0, WIDTH as u32, HEIGHT as u32);
        let dst = Rect::new(
            (window_width / 2) as i32 - ((WIDTH as u32 * scale_factor) / 2) as i32,
            (window_height / 2) as i32 - ((HEIGHT as u32 * scale_factor) / 2) as i32,
            WIDTH as u32 * scale_factor,
            HEIGHT as u32 * scale_factor,
        );

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas
            .fill_rect(Rect::new(0, 0, window_width, window_height))
            .unwrap();

        /* Copy texture to window */
        self.canvas.copy(&texture, src, dst).unwrap();
        self.canvas.present();
    }
}
