fn main()->Result<(), String> {
    let window_title:&str = "RUST GAME";
    let window_width:u32 = 960;
    let window_height:u32 = 540;
    
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window(window_title, window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    let clear_color = sdl2::pixels::Color::RGB(80, 96, 96);
    canvas.set_draw_color(clear_color);

    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut running = true;
    while running {
        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                },
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape), ..} => {
                    running = false;
                },
                sdl2::event::Event::MouseMotion {x, y, ..} => {
                    println!("Mouse x: {}, y: {}", x, y);
                },
                _ => {}
            }
        }
        
        canvas.clear();
        canvas.present();
    }
    
    Ok(())
}
