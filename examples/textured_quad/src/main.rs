extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use piralib::glm;

use image;

fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        title: "🔥".to_string()
    });


    let img = image::open("../../assets/bellargus.jpg").unwrap().to_rgba();
    println!("Image width: {:?} height: {:?}", img.width(), img.height());
    let texture = glh::Texture::new_from_image(&img);

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

    // build vertex data ----
    let mut vertices : Vec<f32> = Vec::new();
    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![1104.0, 736.0, 0.0] );
    vertices.append( &mut vec![0.0,   736.0, 0.0,] );

    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![1104.0,736.0, 0.0,] );        
    vertices.append( &mut vec![1104.0, 0.0, 0.0] );

    let mut colors : Vec<f32> = Vec::new();
    let mut texure_vertices : Vec<f32> = Vec::new();
    {   
        let num_of_vertices = vertices.len();
        let mut i = 0;
        while i < num_of_vertices {

            colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
            texure_vertices.append( &mut vec![ vertices[i] / 1104.0, vertices[i+1]/736.0 ] );
            
            i = i + 3;
        }
    }   

    pos_attrib.data = vertices;
    color_attrib.data = colors;
    texture_attrib.data = texure_vertices;

    let shader = glh::StockShader::new().color().texture().build();
    let attribs = vec![pos_attrib, color_attrib, texture_attrib];
    let vao = glh::Vao::new_from_attrib(&attribs, &shader);

    unsafe{ 
       gl::Enable(gl::BLEND);
       gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
    }

    while app.run() {

        glh::clear(0.2, 0.1, 0.1, 1.0);
 
        shader.bind();
        texture.bind();
        shader.set_uniform_mat4("perspectiveMatrix",
                        &glm::ortho(0.0,
                                    app.get_framebuffer_size().0 as f32 * 0.5,
                                    app.get_framebuffer_size().1 as f32 * 0.5,
                                    0.0, -1.0,
                                    1.0));

        shader.set_uniform_mat4("viewMatrix", &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        shader.set_uniform_mat4("modelMatrix", &model_view );
        
        shader.set_uniform_4f("uColor", &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw( gl::TRIANGLES );

        texture.unbind();
        shader.unbind();
    }
}
