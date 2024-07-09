pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
    pub fn new() -> Self {
        Self { components: vec![] }
    }
    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component)
    }
    pub fn delete_component(&mut self) -> Option<Box<dyn Draw>> {
        match self.components.pop() {
            Some(c) => Some(c),
            None => None,
        }
    }
}
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Self {
        Self {
            width: width,
            height: height,
            label: label,
        }
    }
}
impl Draw for Button {
    fn draw(&self) {
        println!("drawing: {self:?}");
    }
}
