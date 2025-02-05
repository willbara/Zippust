mod gui;
mod encryption;
mod utils;

fn main() {
    gui::run();  // This will now work since `run()` is defined in `gui.rs`
}