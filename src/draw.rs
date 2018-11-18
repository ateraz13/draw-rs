use shapes::Rect;
use shapes::Circle;
// use shapes::Vec2;

#[allow(dead_code)]
pub enum ColorSpace
{
    RGB, CYMK, RGBA
}

pub struct Color
{
    pub r : u8,
    pub g : u8,
    pub b : u8
}

#[allow(dead_code)]
pub enum Style
{
    FillOnly( Color ),
    OutlineOnly( Color/*outline_color*/, u32/*outline_thikness*/ ),
    FillAndOutline( Color /*fill_color*/, Color/*outline_color*/, u32 /*outline_thickness*/)
}


pub trait Drawable
{
    fn draw_rect( &mut self,  rect : &Rect<u32>, style : &Style );
    fn draw_circle( &mut self, circle : &Circle<u32>, style : &Style );
}


#[allow(dead_code)]
pub struct PixelBuffer
{
    width : usize,
    height : usize,
    color_space : ColorSpace,
    bytes_per_pixel : usize,
    buffer : Vec<u8>
}

#[allow(dead_code)]
impl PixelBuffer
{
    pub fn new( color_space : ColorSpace ) -> PixelBuffer
    {
        PixelBuffer
        {
            width : 0,
            height : 0,
            bytes_per_pixel : match color_space {
                ColorSpace::RGB => 3,
                _ => 4
            },
            color_space : color_space,
            buffer : vec![]
        }
    }

    pub fn resize( &mut self, width : usize, height : usize ) 
    {
        let pixel_size = match self.color_space {
            ColorSpace::RGB => 3,
            ColorSpace::RGBA => 4,
            ColorSpace::CYMK => 4
        };

        self.buffer.resize( pixel_size*width*height, 0 );
    }

    pub fn get_size( &self ) -> (usize, usize)
    {
        (self.width, self.height)
    }

    pub fn index_at_point( &self, x : u32, y : u32 ) -> usize
    {
        return (self.width * y as usize + x as usize)* self.bytes_per_pixel;
    }

    pub fn with_size( width : usize, height : usize, color_space : ColorSpace ) -> PixelBuffer
    {
        let pixel_size = match color_space {
            ColorSpace::RGB => 3,
            ColorSpace::RGBA => 4,
            ColorSpace::CYMK => 4
        };

        PixelBuffer
        {
            width : width,
            height : height,
            bytes_per_pixel : pixel_size,
            color_space : color_space,
            buffer : vec![0; pixel_size*width*height]
        }
    }


    pub fn access<F>( &self, func : F ) 
    where F: FnOnce( &[u8] )
    {
        func( &self.buffer[..] );
    }
}

impl Drawable for PixelBuffer
{
    fn draw_rect( &mut self, rect : &Rect<u32>, style : &Style ) 
    {
        let mut _fill_rect = |r:&Rect<u32>,color:&Color| {
            let rp = r.get_pos();
            let rs = r.get_size();

            //FIXME: Please check for "self.buffer"'s bounds overflow 
            // Top left corner of rect inside buffer

            for ly in 0 .. rs.y {
                let y = ly + rp.y;
                for lx in 0 .. rs.x {
                    let x = lx + rp.x;
                    let i = self.index_at_point(x,y);
                    self.buffer[i+0] = color.r;
                    self.buffer[i+1] = color.g;
                    self.buffer[i+2] = color.b;
                }
            }
        };
        
        match style {
            Style::FillOnly( fill_color ) => _fill_rect(&rect,fill_color),
            Style::OutlineOnly( stroke_color, thickness )=> {
                let rp = rect.get_pos();
                let rs = rect.get_size();

                let (_top, _right, _bottom, _left) = 
                    (
                        Rect::new( rp.x, rp.y, rs.x, *thickness ),
                        Rect::new( rp.x+rs.x-thickness,rp.y, *thickness, rs.y-thickness ),
                        Rect::new( rp.x+thickness, rp.y+rs.y-(*thickness), rs.x-thickness, *thickness),
                        Rect::new( rp.x,rp.y+thickness, *thickness, rs.y-thickness ),
                    );
                _fill_rect(&_top, stroke_color);
                _fill_rect(&_right, stroke_color);
                _fill_rect(&_left, stroke_color);
                _fill_rect(&_bottom, stroke_color);
            },
            _ => {}
        }
    }

    fn draw_circle( &mut self, circle : &Circle<u32>, style : &Style )
    {
        match style {
            Style::FillOnly( fill_color ) => {

                let r = circle.get_radius() as u32;
                let center = circle.get_pos();

                // ly = local y
                // lx = local x
                for ly in 0 .. r {
                    // let lx = ((r*r-ly*ly) as f32).sqrt();
                    let lx = ((r*r-ly*ly) as f32).sqrt();
                    let y1 = ly + center.y; // bottom half of circle
                    let y2 = center.y - ly; // top half of circle
                    for x in (center.x - lx.round() as u32) .. center.x+lx.round() as u32 {
                        let mut i = self.index_at_point(x,y1);
                        self.buffer[i+0] = fill_color.r;
                        self.buffer[i+1] = fill_color.g;
                        self.buffer[i+2] = fill_color.b;
                        i = self.index_at_point(x, y2);
                        self.buffer[i+0] = fill_color.r;
                        self.buffer[i+1] = fill_color.g;
                        self.buffer[i+2] = fill_color.b;
                    }
                }
            },
            _ => {}
        }

    }
}
