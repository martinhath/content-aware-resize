extern crate content_aware_resize;
extern crate sdl2;

use content_aware_resize as car;

use sdl2::rect::Rect;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

use std::path::Path;

fn main() {
    let mut image = car::Image::load_image(Path::new("sample-image.jpeg"));
    let (mut w, h) = image.dimmensions();

    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let window = video.window("Context Aware Resize", w, h)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let update_texture = |renderer: &mut sdl2::render::Renderer, image: &car::Image| {
        let (w, h) = image.dimmensions();
        let mut tex = renderer.create_texture_static(sdl2::pixels::PixelFormatEnum::RGB24, w, h).unwrap();
        let data = image.get_image_data();
        let pitch = w * 3;
        tex.update(None, data, pitch as usize).unwrap();
        tex
    };
    let mut texture = update_texture(&mut renderer, &image);

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running },
                Event::Window {win_event: WindowEvent::Resized(new_w, _h), .. } => {
                    let x_diff = new_w as isize - w as isize;
                    if x_diff < 0 {
                        image.resize_to(car::Dimensions::Relative(x_diff, 0));
                    }
                    w = new_w as u32;
                    texture = update_texture(&mut renderer, &image);
                },
                _ => {}
            }
        }
        renderer.clear();
        renderer.copy(&texture, None, Some(Rect::new(0, 0, w, h))).unwrap();
        renderer.present();
    }
}
