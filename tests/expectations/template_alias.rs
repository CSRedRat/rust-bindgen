/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub type Wrapped<T> = T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted<T> {
    pub ptr: Wrapped<T>,
}
