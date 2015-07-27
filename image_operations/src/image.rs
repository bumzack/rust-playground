use image_operation::ImageOperation;

pub struct Image {
    pub image_operations: Vec<Box<ImageOperation>>
}

impl Image {
    pub fn new() -> Image {
        Image { image_operations: vec![] }
    }

    pub fn add_op(&mut self, image_ops: Box<ImageOperation>) {
           self.image_operations.push(image_ops);
    }
}
