#![allow(non_camel_case_types)]
#[macro_use]
extern crate const_cstr;

pub mod abilities_list;
pub mod camera;
pub mod context;
pub mod file;
pub mod version;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
