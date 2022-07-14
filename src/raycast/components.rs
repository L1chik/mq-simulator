
pub trait Input {
    fn input(&mut self);
}

pub trait Physics {
    fn update(&mut self);
}

pub trait Render {
    fn render(&self);
}

pub trait Draw {
    fn draw(&self);
}
