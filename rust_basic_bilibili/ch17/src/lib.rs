pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

pub struct Bottom {
    pub width: u32,
    pub height: u32,
    pub lable: String,
}

impl Draw for Bottom {
    fn draw(&self) {
        println!(
            "drawing bottom..., width: {}, height: {}",
            self.width, self.height
        );
    }
}

pub struct Circle {
    pub radius: u32,
    pub lable: String,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("drawing circle..., radius: {}", self.radius);
    }
}
