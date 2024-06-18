use figs::prelude::*;

fn main() -> FigResult<()> {
    // Initialize the canvas and input
    let mut canvas = Canvas::new("simple example", (320, 320), 60)?;
    let mut input = InputManager::new();

    // Load assets from assets.tgz
    let mut assets = AssetLoader::from_tar("examples/assets.tgz");
    // You can also load assets from the assets directory
    // let mut assets = AssetLoader::from_dir("examples/assets");

    // Create some entities, using assets from the tarball
    let mut bop = Entity::new((200.0, 200.0), vec![assets.load_png("bop.png")?], (8, 8), 20);
    let mut blob = Entity::new((100.0, 100.0), assets.load_png_dir("blob")?, (8, 8), 20);

    // We want to control blob with WASD or arrow keys
    input.wasd();
    input.arrows();

    // We would also like to exit when ESC is pressed
    input.add(Key::Escape);

    // Main event loop (do something more interesting here)
    while canvas.is_open() {
        // Handle keypress events
        for key in &input.keys {
            if canvas.key_down(*key) {
                match key {
                    Key::W | Key::Up => blob.mov(0.0, -1.0),
                    Key::A | Key::Left => blob.mov(-1.0, 0.0),
                    Key::S | Key::Down => blob.mov(0.0, 1.0),
                    Key::D | Key::Right => blob.mov(1.0, 0.0),
                    Key::Escape => return Ok(()),
                    _ => (),
                }
            }
        }

        // Update entities (advance animation, etc)
        bop.update();
        blob.update();

        // Update the canvas with the entities
        canvas.clear();
        canvas.draw(&bop);
        canvas.draw(&blob);
        canvas.update()?;
    }

    Ok(())
}