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

struct ImageOperationSharpenColor  {
    sharpen_factor: f64,

    name: &'static str,
    description: &'static str,
}

trait ImageOperation {
    fn execute_op(&self) -> ImageOperationResult;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

impl ImageOperation for ImageOperationReplaceColor {
    // Replace `Self` with the implementor type: `ImageOperationReplaceColor`
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

//impl ImageOperationSharpenColor {
//    fn new (&self) {
        //self.name = "Image Sharpen Filter";
        //self.description = "Image Sharpen Filter - i am the descirptipn of the Image Sharpen Filter";
//    }
//}

impl ImageOperation for ImageOperationSharpenColor {
    // Replace `Self` with the implementor type: `ImageOperationSharpenColor`
    fn execute_op(&self) -> ImageOperationResult {

         let res = ImageOperationResult {
            // Static methods are called using double colons
            width: 0,
            height: 0,
            startx: 0,
            starty: 0,
            subimage : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
        };
        println!("impl ImageOperation for ImageOperationSharpenColor -> executeOp");

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

    // add an image operation to the array of image operation
    fn add_op(&self, image_op: Box<ImageOperation>) {
        println!("impl Image -> add_op() - adding the following image_op:  {}", image_op.name());

     }
 }

fn main () {
    let imagesimple = PixelImageSimple {
        // Static methods are called using double colons
        width: 0,
        height: 0,
        pixels: Vec::new()
    };

    let sharpen_filter_op = Box::new(ImageOperationSharpenColor {
            name : "Sharpen Filter",
            description: "Sharpen Filter - description",
            sharpen_factor: 0.67
    });

    let replace_color_op = Box::new(ImageOperationReplaceColor {
            name : "Replace Color Filter",
            description: "Replace Color  Filter - description",
            color_old: 0.12,
            color_new: 0.44
    });

    let image = Image { image_operations: Vec::new() };

    image.add_op(sharpen_filter_op);
    image.add_op(replace_color_op);
 }
