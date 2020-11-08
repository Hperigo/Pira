extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use piralib::glm;


fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        samples : 4,
        title: "#️⃣".to_string()
    });

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();

    // build vertex data ----
    let mut vertices : Vec<f32> = Vec::new();
    vertices.append( &mut vec![0.0, 0.0, 0.0] );
    vertices.append( &mut vec![0.0, 736.0, 0.0,] );
    vertices.append( &mut vec![1104.0, 736.0, 0.0] );
    vertices.append( &mut vec![1104.0, 0.0, 0.0] );

    let mut colors : Vec<f32> = Vec::new();
    colors.append( &mut vec![1.0, 0.0, 0.0, 1.0] );
    colors.append( &mut vec![0.0, 1.0, 0.0, 1.0] );
    colors.append( &mut vec![0.0, 0.0, 1.0, 1.0] );
    colors.append( &mut vec![0.0, 0.4, 0.4, 1.0] );

    let mut indices : Vec<u32> = Vec::new(); 
    indices.append( &mut vec![0,2,3] );
    indices.append( &mut vec![0,1,2] );

    pos_attrib.data = vertices;
    color_attrib.data = colors;

    let stock_shader = glh::StockShader::new().color();
    let shader = stock_shader.build();
    let attribs = vec![pos_attrib, color_attrib];
    let vao = glh::Vao::new_from_attrib_indexed(&attribs, &indices, &shader).unwrap();

    unsafe{ 
       gl::Enable(gl::BLEND);
       gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
    }

    while app.run() {

        glh::clear(0.2, 0.1, 0.1, 1.0);

        shader.bind();
        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    app.get_framebuffer_size().0 as f32 * 0.5,
                                    app.get_framebuffer_size().1 as f32 * 0.5,
                                    0.0, -1.0,
                                    1.0));

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        
        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw( gl::TRIANGLES );

        shader.unbind();


        if cfg!(test){

            if app.get_frame_number() > 10 {

                let width : usize = (app.get_window_size().0 * 2) as usize;
                let height : usize = (app.get_window_size().1 * 2) as usize;
                let array_size : usize = width * height * 3;

                // let  pixel_data : [u8; array_size] = [0; array_size];    
                let mut pixel_data : Vec<u8> = Vec::with_capacity(array_size);
                pixel_data.resize(array_size, 0);
                unsafe{
                    gl::ReadPixels(0, 0, width as i32, height as i32, gl::RGB, gl::UNSIGNED_BYTE, pixel_data.as_ptr() as *mut gl::types::GLvoid );
                }
                let img : image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::from_raw(width as u32, height as u32, pixel_data).unwrap();            
                let img = image::imageops::flip_vertical(&img);
                img.save("../../test_images/indexed.png").unwrap();
                return;
            }
        }
    }
}


#[test]
fn save_frame_test() {
    main();
}