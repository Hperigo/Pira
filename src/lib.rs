pub mod app;
pub use self::app::App;

pub extern crate nalgebra_glm;
pub mod gl_helper;

pub mod utils;

pub extern crate glow;
pub extern crate image;
pub extern crate egui;

#[cfg(not(target_arch = "wasm32"))]
pub extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
pub use glutin::event as event;

#[cfg(target_arch = "wasm32")]
pub use winit::event as event;