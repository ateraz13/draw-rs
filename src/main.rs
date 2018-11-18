extern crate image;
mod shapes;


use image::png::PNGEncoder;
use std::fs::File;
use std::{thread, time};

mod math;
mod draw;
mod x11;

fn main() {
    use draw::Drawable;

    let mut fh = File::create("hello.png").unwrap();
    let png_encoder = PNGEncoder::new(&mut fh);

    const IMG_RES : (usize, usize) = ( 512, 512 );

    let mut pb = draw::PixelBuffer::with_size( IMG_RES.0, IMG_RES.1, draw::ColorSpace::RGB );

    let circle_style = draw::Style::FillOnly( draw::Color{ r: 125, g: 0, b: 0 } );
    let rect_style = draw::Style::OutlineOnly( draw::Color{ r: 255, g: 255, b: 255}, 20 );

    let circle = shapes::Circle::new( 256u32, 256, 255);
    let rect = shapes::Rect::new( 75, 75, 510-150, 510-150); 

    pb.draw_circle( &circle, &circle_style );
    pb.draw_rect( &rect, &rect_style );

    // Gives access to raw pixels
    pb.access( |raw_pixels| { 
        png_encoder.encode( &raw_pixels, IMG_RES.0 as u32, IMG_RES.1 as u32, image::ColorType::RGB(8) ).unwrap();
    });

    let _session = x11::Session::prepare().ok().unwrap();
    let _window = _session.create_window(0,0,100,100);

    thread::sleep(time::Duration::from_millis(1000));
    println!("Wrote file ");
}
