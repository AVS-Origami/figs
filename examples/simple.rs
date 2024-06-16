use figs::prelude::*;

fn main() -> FigResult<()> {
    // Initialize the canvas
    let mut canvas = Canvas::new("simple example", (320, 320), 60)?;

    // Load assets from assets.tgz
    let mut assets = AssetLoader::from_tar("examples/assets.tgz");
    // You can also load assets from the assets directory
    // let mut assets = AssetLoader::from_dir("examples/assets");

    // Create some entities on the canvas, using assets from the tarball
    let bop = Entity::new((200.0, 200.0), vec![assets.load_png("bop.png")?], (8, 8), 20);
    let blob = Entity::new((100.0, 100.0), assets.load_png_dir("blob")?, (8, 8), 20);

    // Add the entities to the canvas
    canvas.add_entity(bop);
    canvas.add_entity(blob);

    // Main event loop (do something more interesting here)
    while canvas.is_open() {
        canvas.update()?;
    }

    Ok(())
}