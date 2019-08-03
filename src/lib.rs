extern crate libc;
//use libc::uint32_t;

#[no_mangle]
pub extern fn add(a: u32, b: u32) -> u32 {
    a + b + 10
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(1,2), 13);
    }
}