use glow::{self, HasContext};

pub struct Vbo {
    handle: Option<glow::Buffer>,
    gl_type: u32,
    number_of_items: usize,
}

impl Vbo {
    pub fn new<T>(gl: &glow::Context, data: &[T], gl_type: u32) -> Self {
        let data_slice: &[u8] = unsafe {
            core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>(),
            )
        };

        Vbo::new_from_raw_parts(gl, data_slice, data.len(), gl_type)
    }

    pub fn new_from_raw_parts(
        gl: &glow::Context,
        data: &[u8],
        number_of_items: usize,
        gl_type: u32,
    ) -> Self {
        let vbo = unsafe {
            let buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(gl_type, Some(buffer));
            gl.buffer_data_u8_slice(gl_type, data, glow::DYNAMIC_DRAW);
            gl.bind_buffer(gl_type, None);

            buffer
        };

        //        println!("\n data {}: {:?}\n", data.len(), data);

        Self {
            handle: Some(vbo),
            gl_type,
            number_of_items,
        }
    }

    pub fn get_handle(&self) -> Option<glow::Buffer> {
        self.handle
    }

    pub fn get_gl_type(&self) -> u32 {
        self.gl_type
    }

    pub fn len(&self) -> usize {
        self.number_of_items
    }

    pub fn is_empty(&self) -> bool {
        self.number_of_items == 0
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(self.gl_type, self.handle);
        }
    }
    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(self.gl_type, None);
        }
    }

    pub fn delete(&mut self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.handle.unwrap());
            self.handle = None;
        }
    }
}
