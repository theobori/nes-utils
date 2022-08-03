pub trait Util {
    fn run(&mut self) -> &mut Self;
}

pub trait Save {
    fn save(&mut self) -> &mut Self;
    fn save_as(&mut self, path: &str) -> &mut Self;
}
