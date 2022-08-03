pub trait Header {
    fn parse(&mut self) -> &mut Self;
    fn dump(&self) -> &Self;
    fn init_header(&mut self, mem: &Vec<u8>) -> &mut Self;
    fn check_magic(&self);
}
