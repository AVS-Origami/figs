# figs

A simple framework for making 2D games with Rust.

[![Crates.io version][crate-img]][crate]
[![Documentation][docs-img]][docs]
[![License][license-img]][license]

### Features

- [x] Graphics and basic input management via `minifb`
- [ ] Entity management, including animation and collisions
- [x] Asset loading from filesystem or tarball via `tar` and `regex`
- [x] Image support via `png`
- [ ] Audio playback via `cpal` and `hound`

### Usage

Add the following to your `Cargo.toml`:

```
figs = "0.0.1"
```

### Quick start

```rust
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
        blob.update();

        // Update the canvas with the entities
        canvas.clear();
        canvas.draw(&blob);
        canvas.update()?;
    }

    Ok(())
}
```

See `examples/` for more detailed examples.

### Contributing

Issues and pull requests are welcome.

[crate-img]:     https://img.shields.io/crates/v/figs.svg
[crate]:         https://crates.io/crates/figs
[license-img]:   https://img.shields.io/crates/l/figs.svg
[license]:       https://opensource.org/license/mit
[docs-img]:      https://img.shields.io/badge/docs-online-blue.svg
[docs]:          https://docs.rs/figs