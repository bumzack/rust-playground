struct PixelImageSimple {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

struct ImageOperationResult  {
    image: PixelImageSimple,
}

struct ImageOperationInput {
    image: PixelImageSimple
}

struct ImageOperationOutput {
    image: PixelImageSimple,
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
    fn before_execute_op(&self) -> Vec<ImageOperationInput>;
    fn execute_op(&self, Vec<ImageOperationInput>) -> Vec<ImageOperationOutput>;
    fn after_execute_op(&self, Vec<ImageOperationOutput>) -> ImageOperationResult;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

impl ImageOperation for ImageOperationReplaceColor {
    // Replace `Self` with the implementor type: `ImageOperationReplaceColor`

    fn before_execute_op(&self) -> Vec<ImageOperationInput> {
        let one_res = ImageOperationInput {
            image : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
        };
        let mut res: Vec<ImageOperationInput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationReplaceColor -> before_execute_op");
         // return res as function result
        res
    }

    fn execute_op(&self, input: Vec<ImageOperationInput>) -> Vec<ImageOperationOutput> {
        let one_res = ImageOperationOutput {
            image : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
        };
        let mut res: Vec<ImageOperationOutput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationReplaceColor -> executeOp");
         // return res as function result
        res
    }

    fn after_execute_op(&self, output: Vec<ImageOperationOutput>) -> ImageOperationResult {
        let res = ImageOperationResult {
           image : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
       };
       println!("impl ImageOperation for ImageOperationReplaceColor -> after_execute_op");
        // return res as function result
       res
    }

    // TODO: move this to the "ImageOperation" trait or struct ?
    fn description(&self) -> &'static str {
        self.description
    }

    // TODO: move this to the "ImageOperation" trait or struct ?
    fn name(&self) -> &'static str {
        self.name
    }
}

impl ImageOperation for ImageOperationSharpenColor {
    // Replace `Self` with the implementor type: `ImageOperationSharpenColor`
    fn before_execute_op(&self) -> Vec<ImageOperationInput> {
        let one_res = ImageOperationInput {
            image : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
        };
        let mut res: Vec<ImageOperationInput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationSharpenColor -> before_execute_op");
         // return res as function result
        res
    }

    fn execute_op(&self, input: Vec<ImageOperationInput>) -> Vec<ImageOperationOutput> {
        let one_res = ImageOperationOutput {
            image : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
        };
        let mut res: Vec<ImageOperationOutput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationSharpenColor -> executeOp");
         // return res as function result
        res
    }

    fn after_execute_op(&self, output: Vec<ImageOperationOutput>) -> ImageOperationResult {
        let res = ImageOperationResult {
           image : PixelImageSimple { width: 0, height: 0, pixels: Vec::new() }
       };
       println!("impl ImageOperation for ImageOperationSharpenColor -> after_execute_op");
        // return res as function result
       res
    }

    // TODO: move this to the "ImageOperation" trait or struct ?
    fn name(&self) -> &'static str {
        self.name
    }

    // TODO: move this to the "ImageOperation" trait or struct ?
    fn description(&self) -> &'static str {
        self.description
    }
}

// thx to http://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
struct Image {
    image_operations: Vec<Box<ImageOperation>>,
}

impl Image {
    // add an image operation to the array of image operation
    fn add_op(&mut self, image_op: Box<ImageOperation>) {
        println!("impl Image -> add_op() - adding the following image_op:  {}", image_op.name());
        println!("impl Image -> add_op() - adding the following image_op (description):  {}", image_op.description());
        &self.image_operations.push(image_op);
    }

    fn execute_image_ops(&mut self) {
        println!("impl Image -> execute_image_ops() - iterate over vec/array and execute all image operations");
        // &self.image_operations
        // for image_op in &self.image_operations.iter_mut() {
        for image_op in self.image_operations.iter_mut() {
            println!("iterating over image_ops - execute {}", image_op.name());
            image_op.execute_op();
        }
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

    let mut image = Image { image_operations: Vec::new() };

    println!("MAIN: add 2 image filter\n\n");
    image.add_op(sharpen_filter_op);
    image.add_op(replace_color_op);
    println!("\n\n");

    println!("MAIN: execute all image operations\n\n");
    image.execute_image_ops();
 }
