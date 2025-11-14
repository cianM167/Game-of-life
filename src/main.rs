//#![windows_subsystem = "windows"]
extern crate piston_window;
extern crate image as im;
use piston_window::{types::Width, *};



fn draw_to_buffer(matrix: [[u8; 96]; 96], width: u32, height: u32) -> im::ImageBuffer<im::Rgba<u8>, Vec<u8>> {
    let mut canvas = im::ImageBuffer::new(width, height);
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

    return canvas;
}

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
    let mut numatrix = [[0u8; 96]; 96];
    let mut adj = 0;
    let mut last_pos: Option<[f64; 2]> = None;

    //println!("{}", (948.6/10.0) as u8);

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
            //println!("{}:{}",x, y);
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                let arx = (x/10.0) as usize;
                let ary = (y/10.0) as usize;
                if arx < 96 && ary < 96 {
                    if matrix[arx][ary] == 0 {
                        matrix[arx][ary] = 1;
                    } else {
                        matrix[arx][ary] = 0;
                    }
                }
                //println!("adding at index {}:{}", (x/10.0) as usize, (y/10.0) as usize);
            }
        };
        
        canvas = draw_to_buffer(matrix, width, height);

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
        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::Space) {
                break;
            }
        }
    }
    
    let mut frame = 0;
    while let Some(e) = window.next() {
        //println!("{}", frame);
        if frame == 120 {
            frame = 0;
            //println!("newframe");
            
            canvas = draw_to_buffer(matrix, width, height);
            numatrix = matrix;

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

                    if i != 0 && j != 95 {
                        adj += matrix[i-1][j+1];
                    }

                    if i != 95 && j != 0 {
                        adj += matrix[i+1][j-1];
                    }

                    if adj != 0 {
                        println!("adl:{}", adj)
                    }

                    if adj < 2 && matrix[i][j] == 1 {
                        println!("killing cell due to under");
                        numatrix[i][j] = 0;
                    } 
                    else if adj == 3 && matrix[i][j] == 0 {
                        numatrix[i][j] = 1;
                    } else if adj > 3 && matrix[i][j] == 1 {
                        println!("killing cell due to over");
                        numatrix[i][j] = 0;
                    }

                }

                
            }
            matrix = numatrix;

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
        frame += 1;

    }
    
}
