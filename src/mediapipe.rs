use image::{ImageBuffer, Rgb};
use libc::{c_void, size_t};
use slint::{Rgb8Pixel, SharedPixelBuffer};

use crate::common::Point2F;

#[link(name = "mediapipe")]
extern "C" {
    fn mediapipe_new() -> *const c_void;
    fn mediapipe_delete(mediapipe: *const c_void);
    fn mediapipe_process(
        mediapipe: *const c_void,
        data: *const u8,
        width: size_t,
        height: size_t,
    ) -> *mut FFIPacket;
}

#[repr(C)]
struct FFIPacket {
    // image_len: size_t,
    // image: *const u8,
    landmarks_len: size_t,
    landmarks: *mut Point2F,
}

pub struct Packet {
    pub image_buffer: SharedPixelBuffer<Rgb8Pixel>,
    pub landmarks: Option<Vec<Point2F>>,
}

pub struct Mediapipe {
    ptr: *const c_void,
}

impl Mediapipe {
    pub fn process(&self, frame: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Packet {
        let width = frame.width();
        let height = frame.height();
        let data = frame.into_raw();
        let data = data.as_slice();

        unsafe {
            let ffi_packet = Box::from_raw(mediapipe_process(
                self.ptr,
                data.as_ptr(),
                width as size_t,
                height as size_t,
            ));
            // let image_slice = std::slice::from_raw_parts(ffi_packet.image, ffi_packet.image_len);
            let landmarks = if ffi_packet.landmarks_len > 0 {
                Some(Vec::from_raw_parts(
                    ffi_packet.landmarks,
                    ffi_packet.landmarks_len,
                    ffi_packet.landmarks_len,
                ))
            } else {
                None
            };

            Packet {
                image_buffer: SharedPixelBuffer::clone_from_slice(data, width, height),
                landmarks,
            }
        }
    }
}

impl Default for Mediapipe {
    fn default() -> Self {
        unsafe {
            Mediapipe {
                ptr: mediapipe_new(),
            }
        }
    }
}

impl Drop for Mediapipe {
    fn drop(&mut self) {
        unsafe {
            mediapipe_delete(self.ptr);
        }
    }
}
