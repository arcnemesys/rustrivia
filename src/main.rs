//#[cfg(target_arch = "wasm32")]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = RustJeopardy::new()?;
    
    main_window.run()
}
