extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use piralib::glm;

use rand::*;

extern crate image;

pub struct Particle {
    position : glm::Vec3,
    speed : glm::Vec3,
    scale : f32,
    rotation : f32,
    lifetime : f32,
}

fn main() {
    let a  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 300,
        window_height: 400,
        title: "pira!!!".to_string()
    });
    let mut app  = a.try_borrow_mut().unwrap();

    let mut attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

    {
        let mut vertices : Vec<f32> = Vec::new();
        vertices.append( &mut vec![-250.0, -250.0, 0.0] );
        vertices.append( &mut vec![250.0, -250.0, 0.0] );
        vertices.append( &mut vec![0.0,  250.0, 0.0,] );
        attrib.data = vertices;
 

        let mut colors : Vec<f32> = Vec::new();
        colors.append( &mut vec![1.0, 1.0, 1.0, 1.0] );
        colors.append( &mut vec![1.0, 1.0, 1.0, 1.0] );
        colors.append( &mut vec![1.0, 1.0, 1.0, 1.0] );
        color_attrib.data = colors;


        let mut texure_vertices : Vec<f32> = Vec::new();

        texure_vertices.append( &mut vec![0.5, 0.5, 0.0] );
        texure_vertices.append( &mut vec![0.5, 0.5, 1.0] );
        texure_vertices.append( &mut vec![1.0, 1.0, 1.0] );

        texture_attrib.data = texure_vertices;
    }

    let shader = glh::StockShader::new().color().build();

    let attribs = vec![attrib, color_attrib, texture_attrib];
    let vao = glh::Vao::new_from_attrib(&attribs, &shader);

    // Particles -------

    let mut mParticles : Vec<Particle> = Vec::new();

    while app.run() {

        glh::clear(0.2, 0.1, 0.1, 1.0);

//        tex.bind();

        shader.bind();
        shader.set_uniform_mat4("perspectiveMatrix",
                                &glm::ortho(0.0,
                                            app.window.drawable_size().0 as f32,
                                            app.window.drawable_size().1 as f32,
                                            0.0, -1.0,
                                            1.0));



        shader.set_uniform_mat4("viewMatrix", &glm::Mat4::identity() );



        for p in &mut mParticles{

            p.lifetime += 1.0;
            p.scale  -= 0.001;

            p.position += p.speed;
            p.rotation += 0.01;


            let mut mat = glm::Mat4::identity();
            mat =  glm::translate(&mat, &p.position );
            mat =  glm::rotate(&mat, p.rotation, &glm::vec3(0.0, 0.0, 1.0));
            mat =   glm::scale(&mat,  &glm::vec3(p.scale, p.scale, 1.0));


            shader.set_uniform_mat4("modelMatrix", &mat);


            let green : f32 = 0.5 * (p.scale * 10.0) ;
            let red : f32 = 0.8 * (p.scale * 10.0);

            shader.set_uniform_4f("uColor", &glm::vec4(red, green, 0.2, 1.0));
            vao.draw( gl::TRIANGLES );

        }

        let mut rng = thread_rng();
        let sx : f32 = rng.gen_range(-1.0, 1.0);
        let sy : f32 = rng.gen_range(-5.0, -1.0);

        let r : f32 = rng.gen_range(-3.14, 3.14);

         mParticles.push(Particle{
            position: glm::vec3(300.0, 600.0, 0.0),
            speed : glm::vec3(sx, sy, 0.0),
            scale : 0.1,
            rotation : r,
            lifetime : 0.0,
        });

         // leave particles that are visible
         mParticles.retain(|p| {
             p.scale > 0.0
         });

        shader.unbind();
//        tex.unbind();

    }
}
