use figs::prelude::*;

fn main() -> FigResult<()> {
    // Load assets from assets.tgz
    let mut assets = AssetLoader::from_tar("examples/assets.tgz");
    // You can also load assets from the assets directory
    // let mut assets = AssetLoader::from_dir("examples/assets");

    // Create some entities, using assets from the tarball
    let bop = Entity::new((200.0, 200.0), vec![assets.load_png("bop.png")?], (8, 8), 20);
    let blob = Entity::new((100.0, 100.0), assets.load_png_dir("blob")?, (8, 8), 20);

    // Initialize the canvas and input, after creating entities
    let mut canvas = Canvas::new("simple example", (320, 320), 60)?;

    // Control blob with WASD
    canvas.input.add(Key::W, Box::new(||{ blob.clone().borrow_mut().mov(0.0, -1.0) }));
    canvas.input.add(Key::A, Box::new(||{ blob.clone().borrow_mut().mov(-1.0, 0.0) }));
    canvas.input.add(Key::S, Box::new(||{ blob.clone().borrow_mut().mov(0.0, 1.0) }));
    canvas.input.add(Key::D, Box::new(||{ blob.clone().borrow_mut().mov(1.0, 0.0) }));

    // Add the entities to the canvas
    canvas.add_entity("bop", bop.clone());
    canvas.add_entity("blob", blob.clone());

    // Main event loop (do something more interesting here)
    while canvas.is_open() {
        canvas.update()?;
    }

    Ok(())
}