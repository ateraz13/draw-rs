extern crate xcb;

pub struct Session
{
    connection : xcb::Connection,
    screen_number : usize 
}

pub struct Window<'a>
{
    id: xcb::Drawable ,
    session: &'a Session
}


macro_rules! res_unwrap_or_err {
    ($maybe:expr, $error:expr) => {
        match $maybe {
            Ok(c) => c,
            Err(e) => return Err(format!("{} : {}", $error.to_string(), e))
        }
    }
}

macro_rules! opt_unwrap_or_err {
    ($maybe:expr, $error:expr) => {
        match $maybe {
            Some(c) => c,
            None => return Err( $error.to_string())
        }
    }
}


impl Session
{
    pub fn prepare() -> Result<Session, String>
    {

        let (conn, scr_num)= res_unwrap_or_err!(
            xcb::Connection::connect(None),
            "Could not establish connection to X server!"
        );

        {
            let setup = conn.get_setup();
            
            let screen = opt_unwrap_or_err!(
            setup.roots().nth(scr_num as usize),
                "Could not acquire screen handle from set of \"roots\" from X server!"
        );

            let foreground = conn.generate_id();
            xcb::create_gc(&conn, foreground, screen.root(), &[
                (xcb::GC_FOREGROUND,  screen.black_pixel()),
                (xcb::GC_GRAPHICS_EXPOSURES, 0)
            ]);
        }

        return Ok(Session{
            connection: conn,
            screen_number : scr_num as usize
        });
    }

    pub fn create_window(&self, x : i16, y : i16,  w : u16, h : u16 ) -> Result<Window, String>
    {
        let setup = self.connection.get_setup();
        let screen = opt_unwrap_or_err!(
            setup.roots().nth(self.screen_number),
            "Could not acquire screen handle from X server"
        );


        let win = self.connection.generate_id();
        xcb::create_window(
            &self.connection,
            xcb::COPY_FROM_PARENT as u8,
            win, screen.root(),
            x, y, w, h, 0,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(), &[
                (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS)
            ]
        );

        xcb::map_window(&self.connection, win);
        self.connection.flush();

        return Ok(Window{ session: &self, id: win });
    }

}
