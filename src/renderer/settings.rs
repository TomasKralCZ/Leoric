use std::{mem::size_of, ptr};

use crate::opengl::uniform_buffer::UniformBufferElement;

pub struct Settings {
    pub do_skinning: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self { do_skinning: false }
    }
}

impl UniformBufferElement for Settings {
    fn update(&self) {
        let size = size_of::<i32>();
        let num = if self.do_skinning { 1 } else { 0 };

        unsafe {
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                size as isize,
                &num as *const i32 as _,
            );
        }
    }

    fn init_buffer(&self) {
        let size = size_of::<i32>();

        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                size as isize,
                ptr::null() as _,
                gl::STATIC_DRAW,
            );
        }
    }

    const BINDING: u32 = 3;
}
