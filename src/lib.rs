pub mod app;
pub use self::app::App;

pub extern crate nalgebra_glm;
pub mod gl_helper;

pub mod utils;

pub extern crate glow;

#[cfg(not(target_arch = "wasm32"))]
pub extern crate egui;

#[cfg(target_arch = "wasm32")]
pub use self::app::egui;

#[cfg(not(target_arch = "wasm32"))]
pub extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
pub use glutin::event as event;