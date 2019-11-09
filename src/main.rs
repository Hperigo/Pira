extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use piralib::glm;

use rand::*;
extern crate image;

// create a simple particle object
#[derive(Copy, Clone)]
pub struct Particle {
    position : glm::Vec3,
    speed : glm::Vec3,
    scale : f32,
    rotation : f32,
    lifetime : f32,
}

fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 300,
        window_height: 400,
        title: "!!!pira!!!".to_string()
    });

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    let mut texture_attrib = glh::VertexAttrib::new_texture_attr();


    // build vertex data ----
    {
        let mut vertices : Vec<f32> = Vec::new();
        vertices.append( &mut vec![-250.0, -250.0, 0.0] );
        vertices.append( &mut vec![250.0, -250.0, 0.0] );
        vertices.append( &mut vec![0.0,  350.0, 0.0,] );
        pos_attrib.data = vertices;
 

        let mut colors : Vec<f32> = Vec::new();
        colors.append( &mut vec![1.0, 1.0, 1.0, 1.0] );
        colors.append( &mut vec![0.9, 0.8, 0.9, 1.0] );
        colors.append( &mut vec![1.0, 1.0, 1.0, 1.0] );
        color_attrib.data = colors;

        let mut texure_vertices : Vec<f32> = Vec::new();
        texure_vertices.append( &mut vec![0.5, 0.5, 0.0] );
        texure_vertices.append( &mut vec![0.5, 0.5, 1.0] );
        texure_vertices.append( &mut vec![1.0, 1.0, 1.0] );

        texture_attrib.data = texure_vertices;
    }

    let shader = glh::StockShader::new().color().build();

    let attribs = vec![pos_attrib, color_attrib];
    let vao = glh::Vao::new_from_attrib(&attribs, &shader);

    // Particles -------
    let mut particles : Vec<Particle> = Vec::new();

    unsafe{ 
       gl::Enable(gl::BLEND);
       gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
      //   gl::BlendFunc( gl::ONE, gl::ONE); 
    }

    while app.run() {

        glh::clear(0.2, 0.1, 0.1, 1.0);

        shader.bind();
        shader.set_uniform_mat4("perspectiveMatrix",
                                &glm::ortho(0.0,
                                            app.window.drawable_size().0 as f32,
                                            app.window.drawable_size().1 as f32,
                                            0.0, -1.0,
                                            1.0));

        shader.set_uniform_mat4("viewMatrix", &glm::Mat4::identity() );

        // update particles ----
        for p in &mut particles{

            p.lifetime += 1.0;
            p.scale  -= 0.001;

            p.position += p.speed;
            p.rotation += 0.01;


            let mut mat = glm::Mat4::identity();
            mat =  glm::translate(&mat, &p.position );
            mat =  glm::rotate(&mat, p.rotation, &glm::vec3(0.0, 0.0, 1.0));
            mat =  glm::scale(&mat,  &glm::vec3(p.scale, p.scale, 1.0));

            shader.set_uniform_mat4("modelMatrix", &mat);

            let green : f32 = 0.5 * (p.scale * 10.0) ;
            let red : f32 = 0.8 * (p.scale * 10.0);

            shader.set_uniform_4f("uColor", &glm::vec4(red, green, 0.2, 1.0));
            vao.draw( gl::TRIANGLES );

        }

        shader.unbind();
        
        let mut rng = thread_rng();
        let sx : f32 = rng.gen_range(-1.0, 1.0);
        let sy : f32 = rng.gen_range(-5.0, -1.0);
        let r : f32 = rng.gen_range(-std::f32::consts::PI, std::f32::consts::PI);

        // let rv = rng.gen::<glm::Vec3>();

        let p = app.mouse_pos;
        particles.push(Particle{
            
            position: glm::vec3(p.x * 2.0, p.y * 2.0, 0.0),
            speed : glm::vec3(sx, sy, 0.0),
            scale : 0.1,
            rotation : r,
            lifetime : 0.0,
            
        });
        
        particles.retain(|&p|{
            p.scale > 0.0
        });
    }
}
