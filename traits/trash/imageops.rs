struct PixelImageSimple {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

impl<'a> PixelImageSimple  {
    fn new() -> PixelImageSimple {
        PixelImageSimple {
            pixels: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn set_width(& mut self , width: i32) ->  &mut PixelImageSimple  {
        self.width  = width;
        self
    }

    fn get_width(&  mut  self) -> i32 {
        self.width
    }

    fn set_height(& mut self, height: i32) ->  &mut PixelImageSimple {
        self.height  = height;
        self
    }

    fn get_height(&mut self) -> i32 {
        self.height
    }

    fn set_pixel(&mut self, x: i32, y: i32, value: i32)  {
        let idx:   usize  = (y*self.width + x) as usize ;
        if (x < self.width) && (y < self.height) {
            self.pixels[idx] = value;
        }
    }

    fn get_pixel(&mut self, x: i32, y: i32) -> Result<i32, &str>   {
        let idx:   usize  = (y*self.width + x) as usize ;
        if (x < self.width) && (y < self.height) {
            Ok(self.pixels[idx])
        } else {
            Err("x or y coordinate out of bounds")
        }
    }
}

struct PixelImageSimpleBuilder {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

impl PixelImageSimpleBuilder {
    fn new() -> PixelImageSimpleBuilder {
        PixelImageSimpleBuilder {
            pixels: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn set_width(&mut self, width: i32) ->  &mut PixelImageSimpleBuilder {
        self.width  = width;
        self
    }

    fn set_height(&mut self, height: i32) ->  &mut PixelImageSimpleBuilder {
        self.height  = height;
        self
    }

    fn init_image(&mut self) ->  &mut PixelImageSimpleBuilder {
        // self.pixels = Vec::with_capacity((self.width * self.height) as usize);
        self.pixels = vec![0; (self.width * self.height) as usize];
        self
    }

    fn finalize(&self) -> PixelImageSimple  {
        PixelImageSimple {
            pixels: vec![0; (self.width * self.height) as usize],
            width: self.width,
            height: self.height,
        }
    }
}

struct ImageOperationResult  {
    image: PixelImageSimple,
}

struct ImageOperationInput  {
    image: PixelImageSimple
}

struct ImageOperationOutput {
    image: PixelImageSimple,
}

struct ImageOperationReplaceColor<'a>  {
    color_old: f64,
    color_new: f64,
    bitmap: &'a PixelImageSimple,
    name: &'static str,
    description: &'static str,
}

struct ImageOperationSharpenColor<'a>  {
    sharpen_factor: f64,
    bitmap: &'a PixelImageSimple,
    name: &'static str,
    description: &'static str,
}

trait ImageOperation<'a> {
    // type Bitmap = PixelImageSimple;

    fn before_execute_op(&self) -> Vec<ImageOperationInput>;
    fn execute_op(&self, Vec<ImageOperationInput>) -> Vec<ImageOperationOutput>;
    fn after_execute_op(&self, Vec<ImageOperationOutput>) -> ImageOperationResult;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    // fn set_bitmap(&mut self, bitmap: &PixelImageSimple);
    fn set_bitmap(&mut self, bitmap: &'a PixelImageSimple);
}

impl<'a> ImageOperation<'a> for ImageOperationReplaceColor<'a> {
    // Replace `Self` with the implementor type: `ImageOperationReplaceColor`

    fn before_execute_op(&self) -> Vec<ImageOperationInput> {
        let one_res = ImageOperationInput {
            image : PixelImageSimple::new()
        };
        let mut res: Vec<ImageOperationInput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationReplaceColor -> before_execute_op");
         // return res as function result
        res
    }

    fn execute_op(&self, input: Vec<ImageOperationInput>) ->  Vec<ImageOperationOutput> {
        let one_res = ImageOperationOutput {
            image : PixelImageSimple::new()
        };
        // let dummy = Vec::new();
        let mut res: Vec<ImageOperationOutput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationReplaceColor -> executeOp");
         // return res as function result
        res
    }

    fn after_execute_op(&self, output: Vec<ImageOperationOutput>) -> ImageOperationResult {
        let res = ImageOperationResult {
           image : PixelImageSimple::new()
       };
       println!("impl ImageOperation for ImageOperationReplaceColor -> after_execute_op");
        // return res as function result
       res
    }

    fn set_bitmap(&mut self, bitmap: &'a PixelImageSimple) {
        self.bitmap = bitmap;
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

impl<'a> ImageOperation<'a> for ImageOperationSharpenColor <'a>  {
    // Replace `Self` with the implementor type: `ImageOperationSharpenColor`
    fn before_execute_op(&self) -> Vec<ImageOperationInput> {
        let one_res = ImageOperationInput {
            image : PixelImageSimple::new()
        };
        let mut res: Vec<ImageOperationInput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationSharpenColor -> before_execute_op");
         // return res as function result
        res
    }

    fn execute_op(&self, input: Vec<ImageOperationInput>) -> Vec<ImageOperationOutput> {
        let one_res = ImageOperationOutput {
            image : PixelImageSimple::new()
        };
        let mut res: Vec<ImageOperationOutput> = Vec::new();
        res.push(one_res);
        println!("impl ImageOperation for ImageOperationSharpenColor -> executeOp");
         // return res as function result
        res
    }

    fn after_execute_op(&self, output: Vec<ImageOperationOutput>) -> ImageOperationResult {
        let res = ImageOperationResult {
           image : PixelImageSimple::new()
       };
       println!("impl ImageOperation for ImageOperationSharpenColor -> after_execute_op");
        // return res as function result
       res
    }

    fn set_bitmap(&mut self, bitmap: &'a PixelImageSimple) {
         self.bitmap = bitmap;
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
struct Image<'a> {
    image_operations:  Vec<Box<ImageOperation <'a> >>,
}

impl<'a>  Image<'a>   {
    // add an image operation to the array of image operation
    fn add_op(&mut self, image_op: Box<ImageOperation <'a> >) {
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
            let mut input: Vec<ImageOperationInput> = Vec::new();
            image_op.execute_op(input);
        }
     }
}

fn main () {
    // let imagesimple = PixelImageSimple::new();
    let width: i32 = 100;
    let height: i32 = 200;

    let mut imagesimple: PixelImageSimple = PixelImageSimpleBuilder::new()
                .set_width(width)
                .set_height(height)
                .init_image()
                .finalize();

    // let imagesimple2 = imagesimple.set_width(width).set_height(height).init_image();
    // let imagesimple3 = imagesimple.set_width(width).set_height(height).init_image();

    println!("get_pixel  x= 33, y = 12   ... val = {} ", imagesimple.get_pixel(33, 12).unwrap());
    println!("get_pixel  x= 33, y = 12   ... val = {} ", imagesimple.get_pixel(33, 12).unwrap());


    //TODO: convert the "ImageOperationSharpenColor" constructor to the Builder Logic like  PixelImageSimple
    let sharpen_filter_op = Box::new(ImageOperationSharpenColor {
            name : "Sharpen Filter",
            description: "Sharpen Filter - description",
            sharpen_factor: 0.67,
            bitmap: &imagesimple
    });

    //TODO: convert the "ImageOperationReplaceColor" constructor to the Builder Logic like  PixelImageSimple
    let replace_color_op = Box::new(ImageOperationReplaceColor {
            name : "Replace Color Filter",
            description: "Replace Color  Filter - description",
            color_old: 0.12,
            color_new: 0.44,
            bitmap: &imagesimple
    });

    let mut image = Image { image_operations: Vec::new() };

    println!("MAIN: add 2 image filter\n\n");
    image.add_op(sharpen_filter_op);
    //image.add_op(replace_color_op);
    //println!("\n\n");

    //println!("MAIN: execute all image operations\n\n");
    //image.execute_image_ops();
}
