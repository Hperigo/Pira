//extern crate nalgebra_glm as glm;
extern  crate glam;
use crate::gl_helper as glh;
use crate::gl_helper::Bindable;
use glow::{self, HasContext};

#[derive(Clone, Copy)]
pub struct GlslProg {
    handle: Option<glow::Program>,
}

impl GlslProg {
    pub fn new(gl: &glow::Context, vertex_source: &str, frag_source: &str) -> GlslProg {
        let vertex_handle = compile_shader(gl, vertex_source, glow::VERTEX_SHADER);
        let frag_handle = compile_shader(gl, frag_source, glow::FRAGMENT_SHADER);

        let program_id = unsafe { gl.create_program().unwrap() };

        unsafe {
            gl.attach_shader(program_id, vertex_handle);
            gl.attach_shader(program_id, frag_handle);
            gl.link_program(program_id);
            let success = gl.get_program_link_status(program_id);

            if !success {
                println!("Could not LINK shader: {}\n\n", gl.get_program_info_log(program_id));
                return Self { handle: None };
            }

            gl.detach_shader(program_id, vertex_handle);
            gl.detach_shader(program_id, frag_handle);
        }

        GlslProg {
            handle: Some(program_id),
        }
    }

    pub fn get_handle(&self) -> Option<glow::Program> {
        self.handle
    }

    pub fn get_uniform_location(&self, gl: &glow::Context, name: &str) -> glow::UniformLocation {
        let loc = unsafe {
            gl.get_uniform_location(self.handle.unwrap(), name)
                .expect(format!("\n\n\tno uniform named: {}\n\n", name).as_str())
        };
        loc
    }

    pub fn set_orthographic_matrix(&self, gl: &glow::Context, size: &[f32; 2]) {

        let mat = glam::Mat4::orthographic_rh_gl(0.0, size[0], size[1], 0.0, -1.0, 1.0 );
        let mut slice :[f32; 16] = [0.0; 16];
        mat.write_cols_to_slice(&mut slice);

        self.set_uniform_mat4_slice(
            gl,
            glh::StockShader::uniform_name_perspective_matrix(),
            &slice,
        );
    }

    pub fn set_perspective_matrix(&self, gl: &glow::Context, mat: &glam::Mat4) {
        self.set_uniform_mat4(gl, glh::StockShader::uniform_name_perspective_matrix(), mat);
    }

    pub fn set_view_matrix(&self, gl: &glow::Context, mat: &glam::Mat4) {
        self.set_uniform_mat4(gl, glh::StockShader::uniform_name_view_matrix(), mat);
    }

    pub fn set_model_matrix(&self, gl: &glow::Context, mat: &glam::Mat4) {
        self.set_uniform_mat4(gl, glh::StockShader::uniform_name_model_matrix(), mat);
    }

    pub fn set_transform(
        &self,
        gl: &glow::Context,
        pos: glam::Vec3,
        rot: glam::Quat,
        scale: glam::Vec3
    ) {
        let t = glam::Affine3A::from_scale_rotation_translation(scale, rot, pos);
        let mat4 = glam::Mat4::from(t);

        self.set_uniform_mat4(
            gl,
            glh::StockShader::uniform_name_model_matrix(),
            &mat4,
        );
    }

    pub fn set_color(&self, gl: &glow::Context, color: &[f32; 4]) {
        self.set_uniform_4f(gl, glh::StockShader::uniform_name_color(), color);
    }

    
    pub fn set_uniform_mat4(&self, gl: &glow::Context, name: &str, value: &glam::Mat4) {
        let mut slice : [f32; 16] = [0.0; 16];
        value.write_cols_to_slice(&mut slice);
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_matrix_4_f32_slice(Some(&loc), false, &slice);
        };
    }

    pub fn set_uniform_mat4_slice(&self, gl: &glow::Context, name: &str, value: &[f32; 16]) {
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_matrix_4_f32_slice(Some(&loc), false, value);
        };
    }

    pub fn set_uniform_1i(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_1_i32(Some(&loc), value);
        };
    }

    pub fn set_uniform_1f(&self, gl: &glow::Context, name: &str, value: f32) {
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_1_f32(Some(&loc), value);
        };
    }

    pub fn set_uniform_2f(&self, gl: &glow::Context, name: &str, value: &[f32; 2]) {
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_2_f32(Some(&loc), value[0], value[1]);
        };
    }

    pub fn set_uniform_3f(&self, gl: &glow::Context, name: &str, value: &[f32; 3]) {
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_3_f32(Some(&loc), value[0], value[1], value[2]);
        };
    }

    pub fn set_uniform_4f(&self, gl: &glow::Context, name: &str, value: &[f32; 4]) {
        unsafe {
            let loc = self.get_uniform_location(gl, name);
            gl.uniform_4_f32(Some(&loc), value[0], value[1], value[2], value[3]);
        };
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            assert!(self.handle != None);
            gl.use_program(self.handle);
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(None);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.handle.unwrap());
        }
    }
}

fn compile_shader(gl: &glow::Context, src: &str, shader_type: u32) -> glow::Shader {
    let shader_id = unsafe { gl.create_shader(shader_type).unwrap() };

    unsafe {
        gl.shader_source(shader_id, src);
        gl.compile_shader(shader_id);
    }

    let success = unsafe { gl.get_shader_compile_status(shader_id) };
    if !success {
        let shader_type_string: &str;
        match shader_type {
            glow::VERTEX_SHADER => shader_type_string = "VERTEX_SHADER",
            glow::FRAGMENT_SHADER => shader_type_string = "FRAGMENT",
            glow::COMPUTE_SHADER => shader_type_string = "COMPUTE",
            _ => shader_type_string = "unkwon shader type",
        };
        unsafe {
            let log = gl.get_shader_info_log(shader_id);
            println!("Failed to compile {} :: error {}", shader_type_string, log);
        }
    }
    shader_id
}

impl Bindable for GlslProg {
    fn bind(&self, gl: &glow::Context) {
        unsafe {
            assert!(
                self.handle.is_some(),
                "You are trying to bind a NONE Shader"
            );
            gl.use_program(self.handle);
        }
    }

    fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(None);
        }
    }
}
