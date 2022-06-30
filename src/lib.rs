#![allow(non_camel_case_types)]
#[macro_use]
extern crate const_cstr;

pub mod abilities_list;
pub mod camera;
pub mod context;
pub mod file;
pub mod filesys;
pub mod library;
pub mod list;
pub mod port;
pub mod port_info_list;
pub mod result;
pub mod setting;
pub mod version;
pub mod widget;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_context() {
        let ptr = unsafe { crate::context::gp_context_new() };

        assert_eq!(ptr.is_null(), false);

        unsafe {
            crate::context::gp_context_unref(ptr);
        }
    }
}
