use libc::{c_char, c_int, c_long, c_void};
use std::str::Utf8Error;

#[allow(non_camel_case_types)]
type Xdo_c = *mut c_void;
#[allow(non_camel_case_types)]
type c_unsigned_int = u32;

extern "C" {
    fn xdo_new(display: *const c_char) -> Xdo_c;
    fn xdo_version() -> *const c_char;
    fn xdo_free(xdo: Xdo_c);
    fn xdo_move_mouse(xdo: *const c_void, x: c_int, y: c_int, screen: c_int) -> c_int;
    // fn xdo_move_mouse_relative_to_window(xdo: *const c_void, window: Window, x: c_int, y: c_int) -> c_int;
    fn xdo_move_mouse_relative(xdo: *const c_void, x: c_int, y: c_int) -> c_int;
    // fn xdo_mouse_down(xdo: *const c_void, window: Window, button: c_int) -> c_int;
    // fn xdo_mouse_up(xdo: *const c_void, window: Window, button: c_int) -> c_int;
    fn xdo_get_mouse_location(
        xdo: *const c_void,
        x: *mut c_int,
        y: *mut c_int,
        screen_num: *mut c_int,
    ) -> c_int;
    // fn xdo_get_window_at_mouse(xdo: *const c_void, window: Window) -> c_int;
    // fn xdo_get_mouse_location2(
    //     xdo: *const c_void,
    //     x: *mut c_int,
    //     y: *mut c_int,
    //     screen_num: *mut c_int,
    //     window: *mut Window
    // ) -> c_int;
    fn xdo_wait_for_mouse_move_from(xdo: *const c_void, x: c_int, y: c_int) -> c_int;
    fn xdo_wait_for_mouse_move_to(xdo: *const c_void, x: c_int, y: c_int) -> c_int;
    // fn xdo_click_window(xdo: *const c_void, window: Window, button: c_int) -> c_int;
    // fn xdo_click_window(
    //     xdo: *const c_void,
    //     window: Window,
    //     button: c_int,
    //     repeat: c_int,
    //     delay: ?useconds_t,
    // ) -> c_int;
    // fn xdo_enter_text_window(
    //     xdo: *const c_void,
    //     window: Window,
    //     string: *const c_char,
    //     delay: ?useconds_t,
    // ) -> c_int;
    // fn xdo_send_keysequence_window(
    //     xdo: *const c_void,
    //     window: Window,
    //     keysequence: *const c_char,
    //     delay: ?useconds_t,
    // ) -> c_int;
    // fn xdo_send_keysequence_window_up(
    //     xdo: *const c_void,
    //     window: Window,
    //     keysequence: *const c_char,
    //     delay: ?useconds_t,
    // ) -> c_int;
    // fn xdo_send_keysequence_window_down(
    //     xdo: *const c_void,
    //     window: Window,
    //     keysequence: *const c_char,
    //     delay: ?useconds_t,
    // ) -> c_int;
    // fn xdo_send_keysequence_window_list_do(
    //     xdo: *const c_void,
    //     window: Window,
    //     keysequence: *mut ?charcodemap_t,
    //     nkeys: c_int,
    //     pressed: c_int,
    //     modifier: *mut c_int,
    //     delay: ?useconds_t,
    // ) -> c_int;
    // fn xdo_wait_for_window_map_state(
    //     xdo: *const c_void,
    //     window: Window,
    //     map_state: c_int
    // ) -> c_int;
    // fn xdo_wait_for_window_size(
    //     xdo: *const c_void,
    //     window: Window,
    //     width: c_unsigned_int,
    //     height: c_unsigned_int,
    //     flags: c_int,
    //     to_or_from: to_or_from
    // ) -> c_int;
    // fn xdo_move_window(
    //     xdo: *const c_void,
    //     window: Window,
    //     x: c_int,
    //     y: c_int
    // ) -> c_int;

    // fn xdo_translate_window_with_sizehint(
    //     xdo: *const c_void,
    //     window: Window,
    //     width: c_unsigned_int,
    //     height: c_unsigned_int,
    //     width: *mut c_unsigned_int,
    //     height: *mut c_unsigned_int,
    // ) -> c_int;

    // fn xdo_set_window_size(
    //     xdo: *const c_void,
    //     window: Window,
    //     x: c_int,
    //     y: c_int,
    //     flags: c_int
    // ) -> c_int;

    // fn xdo_set_window_property(
    //     xdo: *const c_void,
    //     window: Window,
    //     property: *const c_char,
    //     value: *const c_char
    // ) -> c_int;

    // fn xdo_set_window_class(
    //     xdo: *const c_void,
    //     window: Window,
    //     name: *const c_char,
    //     _class: *const c_char
    // ) -> c_int;

    // fn xdo_set_window_urgency(
    //     xdo: *const c_void,
    //     window: Window,
    //     urgency: c_int
    // ) -> c_int;

    // fn xdo_set_window_override_redirect(
    //     xdo: *const c_void,
    //     window: Window,
    //     override_redirect: c_int
    // ) -> c_int;

    // fn xdo_focus_window(
    //     xdo: *const c_void,
    //     window: Window
    // ) -> c_int;

    // fn xdo_raise_window(
    //     xdo: *const c_void,
    //     window: Window
    // ) -> c_int;

    // fn xdo_get_focused_window(
    //     xdo: *const c_void,
    //     window: *mut Window
    // ) -> c_int;

    fn xdo_set_number_of_desktops(xdo: *const c_void, ndesktops: c_long) -> c_int;
    fn xdo_get_number_of_desktops(xdo: *const c_void, ndesktops: *mut c_long) -> c_int;
    fn xdo_set_current_desktop(xdo: *const c_void, ndesktops: c_long) -> c_int;
    fn xdo_get_current_desktop(xdo: *const c_void, ndesktops: *mut c_long) -> c_int;
    // fn xdo_set_desktop_for_window(xdo: *const c_void, window: Window, ndesktops: c_long) -> c_int;
    // fn xdo_get_desktop_for_window(xdo: *const c_void, window: Window, ndesktops: *mut c_long) -> c_int;
    fn xdo_get_input_state(xdo: *const c_void) -> c_unsigned_int;
    fn xdo_get_symbol_map() -> *mut *const c_char;
}

pub struct Xdo {
    handle: Xdo_c,
}

/**
 * Safety requirements:
 *  pointer must be non-null
 *  pointed to sequence of characters should end up with null-byte
 */
unsafe fn c_char_const_ptr_to_str_slice<'a>(c_string: *const c_char) -> Result<&'a str, Utf8Error> {
    let mut len = 0;
    while *((c_string as usize + len) as *const i8) != 0 {
        len += std::mem::size_of::<c_char>();
    }

    std::str::from_utf8(std::slice::from_raw_parts::<'a>(c_string as *const u8, len))
}

const XDO_ERROR: c_int = 1;
const XDO_SUCCESS: c_int = 0;

fn xdo_check_error(ret: c_int) -> Result<(), ()> {
    match ret {
        XDO_ERROR => Err(()),
        XDO_SUCCESS => Ok(()),
        _ => unreachable!(),
    }
}

impl Xdo {
    pub fn new(screen: &str) -> Result<Self, ()> {
        let xdo = unsafe { xdo_new(screen.as_bytes().as_ptr() as *const i8) };
        if xdo.is_null() {
            return Err(());
        }
        Ok(Self { handle: xdo })
    }
    pub fn xdo_version() -> Result<&'static str, Utf8Error> {
        unsafe { c_char_const_ptr_to_str_slice(xdo_version()) }
    }

    pub fn move_mouse(&mut self, x: i32, y: i32, screen: i32) -> Result<(), ()> {
        xdo_check_error(unsafe { xdo_move_mouse(self.handle, x, y, screen) })
    }

    pub fn move_mouse_relative(&mut self, x: i32, y: i32) -> Result<(), ()> {
        xdo_check_error(unsafe { xdo_move_mouse_relative(self.handle, x, y) })
    }

    pub fn get_mouse_location(&mut self) -> Result<(i32, i32, i32), ()> {
        let mut x: c_int = 0;
        let mut y: c_int = 0;
        let mut screen_num: c_int = 0;
        xdo_check_error(unsafe {
            xdo_get_mouse_location(self.handle, &mut x, &mut y, &mut screen_num)
        })?;
        Ok((x, y, screen_num))
    }

    pub fn wait_for_mouse_move_to(&mut self, x: i32, y: i32) -> Result<(), ()> {
        xdo_check_error(unsafe { xdo_wait_for_mouse_move_to(self.handle, x, y) })
    }

    pub fn wait_for_mouse_move_from(&mut self, x: i32, y: i32) -> Result<(), ()> {
        xdo_check_error(unsafe { xdo_wait_for_mouse_move_from(self.handle, x, y) })
    }

    pub fn set_number_of_desktops(&mut self, ndesktops: c_long) -> Result<(), ()> {
        xdo_check_error(unsafe { xdo_set_number_of_desktops(self.handle, ndesktops) })
    }
    pub fn get_number_of_desktops(&mut self) -> Result<i64, ()> {
        let mut ndesktops: c_long = 0;
        xdo_check_error(unsafe { xdo_get_number_of_desktops(self.handle, &mut ndesktops) })?;
        Ok(ndesktops)
    }
    pub fn set_current_desktop(&mut self, ndesktops: c_long) -> Result<(), ()> {
        xdo_check_error(unsafe { xdo_set_current_desktop(self.handle, ndesktops) })
    }
    pub fn get_current_desktop(&mut self) -> Result<i64, ()> {
        let mut ndesktops: c_long = 0;
        xdo_check_error(unsafe { xdo_get_current_desktop(self.handle, &mut ndesktops) })?;
        Ok(ndesktops)
    }

    pub fn get_input_state(&mut self) -> c_unsigned_int {
        unsafe { xdo_get_input_state(self.handle) }
    }
    pub fn get_symbol_map() -> Vec<&'static str> {
        let begin = unsafe { xdo_get_symbol_map() } as *const *const c_char;
        let mut cur = begin;
        let mut res = Vec::new();
        while !cur.is_null() {
            if let Ok(string) = unsafe { c_char_const_ptr_to_str_slice(*cur) } {
                res.push(string);
            }
            cur = (cur as usize + std::mem::size_of::<*const c_char>()) as *const *const c_char;
        }
        res
    }
}

impl Drop for Xdo {
    fn drop(&mut self) {
        unsafe { xdo_free(self.handle) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
