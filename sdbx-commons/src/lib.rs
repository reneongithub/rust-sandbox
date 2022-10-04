mod traits;

pub use traits::Draw;
pub use traits::PerformOnSdbx;

// pub trait Draw {
//     fn draw(&self);
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drew button to screen with width of {}", self.width);
    }
}