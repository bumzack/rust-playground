use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;

pub trait ImageOperation {
    fn execute_op(&self);
    fn execute_op2(&self, input: &ImageOperationParam) -> ImageOperationParam;
    fn prepare_op(&self) -> Vec<ImageOperationParam>;
    fn merge_results(&self, partial_results: Vec<ImageOperationParam>) -> PixelImageSimple;
}
