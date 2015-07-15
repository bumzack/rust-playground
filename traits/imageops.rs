struct PixelImageSimple {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

struct ImageOperationResult  {
    subimage: PixelImageSimple,
    width:  i32,
    height: i32,
    startx: i32,
    starty: i32,
}

struct ImageOperationReplaceColor  {
    color_old: f64,
    color_new: f64,

    name: &'static str,
    description: &'static str,
}

trait ImageOperation {
    fn execute_op(&self) -> ImageOperationResult;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

impl ImageOperation for ImageOperationReplaceColor {
    // Replace `Self` with the implementor type: `Dog`
    fn execute_op(&self) -> ImageOperationResult {

         let res = ImageOperationResult {
            // Static methods are called using double colons
            width: 0,
            height: 0,
            startx: 0,
            starty: 0,
            subimage : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
        };
        println!("impl ImageOperation for ImageOperationReplaceColor -> executeOp");

        // return res as function result
        res
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn name(&self) -> &'static str {
        self.name
    }
}


// thx to http://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
struct Image {
    // image_operations: &'static  Vec<ImageOperation>,
    image_operations: Vec<Box<ImageOperation>>,
}

impl Image {
    fn new() {
        println!("impl Image -> new() - initialize the image_operations array");
        //self.image_operations = Vec::new();
    }

}

fn main () {
    let imagesimple = PixelImageSimple {
        // Static methods are called using double colons
        width: 0,
        height: 0,
        pixels: Vec::new()
    };

    let image = Image { image_operations: Vec::new() };

}
