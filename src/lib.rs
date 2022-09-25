pub mod app;
pub use self::app::App;

pub mod gl_helper;

pub mod utils;

pub extern crate glow;
pub extern crate image;
pub extern crate egui_glow;
pub extern crate glam;

pub use egui_glow::egui_winit::egui as egui;


#[cfg(not(target_arch = "wasm32"))]
pub extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
pub use glutin::event as event;

#[cfg(target_arch = "wasm32")]
pub use winit::event as event;