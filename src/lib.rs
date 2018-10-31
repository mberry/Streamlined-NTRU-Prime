
pub mod r3;
pub mod rq;


pub use rq::encoding;
pub use r3::mod3;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
