pub trait Util {
    fn run(&mut self);
}

pub trait Save {
    fn save(&mut self);
    fn save_as(&mut self, path: &str);
}

pub trait NesUtil: Util + Save { }
