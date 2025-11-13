extern crate piston_window;
extern crate image as im;
use piston_window::*;


fn main() {
    let opengl = OpenGL::V3_2;
    let (width,height) = (960 , 960);
    let mut window: PistonWindow = 
        WindowSettings::new("Game of loss", [width, height])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut matrix = [[0u8; 96]; 96];
    let mut adj = 0;
    matrix[2][2] = 1;
    matrix[2][3] = 1;
    matrix[2][4] = 1;
    let mut last_pos: Option<[f64; 2]> = None;

    println!("{}", (948.6/10.0) as u8);

    /* 
    println!("New frame:");
        for i in 0..95 {
            println!("{:?}", matrix[i]);
        }
        */
    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut x = 0.0;
    let mut y = 0.0;
    while let Some(e) = window.next() {
        let mut draw = false;
        if let Some(pos) = e.mouse_cursor_args() {
            (x, y) = (pos[0] as f32, pos[1] as f32);
            println!("{}:{}",x, y);
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                matrix[(x/10.0) as usize][(y/10.0) as usize] = 1;
                println!("adding at index {}:{}", (x/10.0) as usize, (y/10.0) as usize);
            }
        };
        
        for i in 0..95 {
            for j in 0..95 {
                if matrix[i][j] == 1 {
                    let boundx = i*10;
                    let boundy = j*10;
                    for k in boundx..boundx+10 {
                        for l in boundy..boundy+10 {
                            canvas.put_pixel(k.try_into().unwrap(), l.try_into().unwrap(), im::Rgba([0, 0, 0, 255]));
                        }
                    }
                }
            }
        }

        if e.render_args().is_some() {
            //println!("rendering");
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        
        //break;
    }
    /*
    while let Some(e) = window.next() {
        //println!("newframe");
        
        for i in 0..95 {
            for j in 0..95 {
                if matrix[i][j] == 1 {
                    let boundx = i*10;
                    let boundy = j*10;
                    for k in boundx..boundx+10 {
                        for l in boundy..boundy+10 {
                            canvas.put_pixel(k.try_into().unwrap(), l.try_into().unwrap(), im::Rgba([0, 0, 0, 255]));
                        }
                    }
                }
            }
        }

        for i in 0..95 {
            for j in 0..95 {
                //println!("Finding adjacent cells");
                adj = 0;
                //checking if its on the edge
                if j != 0 && i != 0 {
                    adj += matrix[i-1][j];
                    adj += matrix[i][j-1];
                    adj += matrix[i-1][j-1];
                } else if j != 0 {
                    adj += matrix[i][j-1];
                } else if i != 0 {
                    adj += matrix[i-1][j];
                }
                
                
                if j != 95 && i != 95 {
                    adj += matrix[i+1][j];
                    adj += matrix[i][j+1];
                    adj += matrix[i+1][j+1];
                } else if j != 95 {
                    adj += matrix[i][j+1];
                } else if i != 95 {
                    adj += matrix[i+1][j];
                }

                if adj < 2 {
                    //println!("killing cell");
                    matrix[i][j] = 0;
                } else if adj == 3 {
                    matrix[i][j] = 1;
                } else if adj > 3 {
                    //println!("killing cell");
                    matrix[i][j] = 0;
                }

            }

            
        }

        if e.render_args().is_some() {
            //println!("rendering");
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
    */
}
