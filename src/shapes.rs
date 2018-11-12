#[allow(dead_code)]

use std::ops::{Add,Mul,Div};


pub struct Vec2<T>
{
    pub x : T,
    pub y : T
}


#[derive(Copy,Clone)]
pub struct Rect<T: From<u32> + Add + Div + Mul>
{
    pub x : T,
    pub y : T,
    pub width : T,
    pub height : T,
}

pub struct Circle<T>
{
    x : T,
    y : T,
    radius : T
}

pub struct Line<T>
{
    from : Vec2<T>,
    to   : Vec2<T>
}


impl<T> Vec2<T>
{
    pub fn new( x : T, y : T ) -> Self
    {
        Self{ x : x, y : y }
    }
}


impl Vec2<f32>
{
    #[allow(dead_code)]
    pub fn rotated( &self, _ang : f32 ) -> Self
    {
        return Vec2{ x : self.x.cos() - self.x.sin(), y : self.y.cos() + self.y.sin() };
    }
}


#[allow(dead_code)]
impl<T: From<u32> + Add + Div + Mul + Copy > Rect<T>
where
    T: From<<T as Add>::Output>
    + From<<T as Div>::Output>
    + From<<T as Mul>::Output>

{
    pub fn null() -> Self
    {
        Rect{ x: T::from(0u32), y: T::from(0u32), width: T::from(0u32), height: T::from(0u32) }
    }

    pub fn new ( x : T,  y : T, width : T, height : T ) -> Self
    {
        Rect{ x: x, y: y, width: width as T, height: height }
    }

    pub fn resize( &mut self, width : T, height : T )
    {
        self.width = width;
        self.height = height;
    }

    pub fn get_pos( &self ) -> Vec2<T>
    {
        Vec2::new( self.x, self.y )
    }

    pub fn get_size( &self ) -> Vec2<T>
    {
        Vec2::new( self.width, self.height )
    }

    pub fn set_pos( &mut self, x : T, y : T )
    {
        self.x = x;
        self.y = y;
    }

    pub fn set_xy( &mut self, x : T, y : T )
    {
        self.x = T::from(self.x + x); 
        self.y = T::from(self.y + y);
    }

    pub fn move_by( &mut self, x : T, y : T )
    {
        self.x = T::from(self.x + x);
        self.y = T::from(self.x + y);
    }

    pub fn scale_height( &mut self, enumer : T, denom : T )
    {
        self.height = T::from(self.height / denom);
        self.height = T::from(self.height * enumer);
    }

    pub fn scale_width( &mut self, enumer : T, denom : T )
    {
        self.width = T::from(self.width / denom);
        self.width = T::from(self.width * enumer);
    }

    pub fn scale_by( &mut self, enumer : T, denom : T )
    {
        self.width = T::from(self.width / denom);
        self.width = T::from(self.width * enumer);
        self.height = T::from(self.height / denom);
        self.height = T::from(self.height * enumer);
    }

    pub fn set_height( &mut self, height : T )
    {
        self.height = height;
    }

    pub fn set_width( &mut self, width : T )
    {
        self.width = width;
    }

}

#[allow(dead_code)]
impl<T: Copy> Circle<T>
{
    pub fn new( x : T, y : T, radius : T ) -> Self
    {
        Circle{ x : x, y : y, radius : radius }
    }

    pub fn get_radius( &self ) -> T
    {
        return self.radius;
    }

    pub fn get_pos( &self ) -> Vec2<T>
    {
        Vec2::new( self.x , self.y )
    }

    pub fn set_radius( &mut self, radius : T ) 
    {
        self.radius = radius;
    }

    pub fn set_pos( &mut self, x : T, y : T )
    {
        self.x = x;
        self.y = y;
    }
}

impl<T: Copy> Line<T>
{
    fn between( p1 : Vec2<T>, p2 : Vec2<T> ) -> Self
    {
        Self { from: p1, to: p2 }
    }
}

