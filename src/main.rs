use sdl2::Sdl;

fn main() -> Result<(), String>{
    let _context:Sdl = sdl2::init()?;
    println!("FUKKYOU");

    Ok(())
}
