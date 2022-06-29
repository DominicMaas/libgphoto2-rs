#![allow(non_camel_case_types)]
#[macro_use]
extern crate const_cstr;

pub mod abilities_list;
pub mod camera;
pub mod context;
pub mod file;
pub mod library;
pub mod list;
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
}
