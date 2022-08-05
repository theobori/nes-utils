pub trait Header {
    fn parse(&mut self) -> &mut Self;
    fn dump(&self) -> &Self;
    fn check_magic(&self);
}
