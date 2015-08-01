// http://homepages.inf.ed.ac.uk/rbf/HIPR2/pixlog.htm

use num::complex::Complex64;
use std::rc::Rc;

use std::f64::consts;

use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;
use pixel_image_simple::RGBA8;

use image_operation::ImageOperation;

pub struct ImageOperationFFT {
    pub input_bitmapdata: Rc<PixelImageSimple>,
    pub output_bitmapdata: PixelImageSimple,
}

impl ImageOperation for ImageOperationFFT {
    fn execute_op(&self ) {
        println!("ImageOperationRotate - width = {}, height = {}",
             &self.input_bitmapdata.width, &self.input_bitmapdata.height );
    }

    fn execute_op2(&mut self, input: &ImageOperationParam) -> ImageOperationParam {
        let res_bitmap: Vec<RGBA8> = vec![];
        let mut res = ImageOperationParam::new2(0, 0, res_bitmap);

        let mut fft_input: Vec<f64> = vec![];
        let mut amp: Vec<f64> = vec![];
        // let mut phase: Vec<f64> = vec![];

        let mut pixel = RGBA8 {r: 0, g: 0, b: 0, a: 0};

        for y in 0..self.input_bitmapdata.height {
            for x in 0..self.input_bitmapdata.width {
                pixel = self.input_bitmapdata.get_pixel(x, y);
                // println!("pixel fft input  {}/{} = r: {}, g: {}, b: {}, a: {}", x, y, pixel.r, pixel.g,  pixel.b,  pixel.a);

                fft_input.push( pixel.b as f64);
            }
        }
        let fft_res = FFT::new().execute_real(fft_input);

        let mut idx = 0;
        let mut amp_max = 0.0;

        for y in 0..self.output_bitmapdata.height {
            for x in 0..self.output_bitmapdata.width {
                idx = y * self.output_bitmapdata.width + x;
                let val = (fft_res[idx as usize].re * fft_res[idx as usize].re  +
                             fft_res[idx as usize].im * fft_res[idx as usize].im).sqrt().abs();
                amp.push(val);

                println!("ImageOperationFFT::input_bitmapdata   {}/{}: val =  {}", x, y, val);
                if val > amp_max {
                    amp_max = val;
                    println!("ImageOperationFFT::input_bitmapdata  setting amp_max to  {} @ {}/{}", val, x, y);
                }
            }
        }

        let c = 255.0 / (1.0 + amp_max).ln();

        println!("ImageOperationFFT::input_bitmapdata   max amplitude: {}", amp_max);
        println!("ImageOperationFFT::input_bitmapdata   c: {}", c);
        // println!("ImageOperationFFT::input_bitmapdata   phase: {:?}", phase);

        let mut pixel = RGBA8 {r: 0, g: 0, b: 0, a: 255};

        for y in 0..self.output_bitmapdata.height {
            for x in 0..self.output_bitmapdata.width {
                idx = y * self.output_bitmapdata.width + x;
                //let val = (fft_res[idx as usize].re * fft_res[idx as usize].re  +
                //             fft_res[idx as usize].im * fft_res[idx as usize].im).sqrt().abs();
                let q = c * (1.0 + amp[idx as usize]).ln();
                let q8 = q as u8;
                if amp[idx as usize] > 0.0 {
                    //println!("ImageOperationFFT::input_bitmapdata   val: {}, q: {}, q8: {}    @ x= {}, y= {}", amp[idx], q, q8, x,y);
                }

                //if q8 > 0 {
                pixel = RGBA8 {r:q8, g: q8, b: q8, a: 255};
                //} else {
                //    pixel = RGBA8 {r:255, g: 255, b: 255, a: 255};
                //}
                self.output_bitmapdata.set_pixel(x, y, pixel);
            }
        }
        println!("ImageOperationFFT::input_bitmapdata   max amplitude: {}", amp_max);
        println!("ImageOperationFFT::input_bitmapdata   c: {}", c);

        res.startx = 0;
        res.starty = 0;
        res.endx = 0;
        res.endy = 0;

        res
    }

    // TODO: move to trait "ImageOperation" - they can be used indepent of the actual image operation
    fn prepare_op(&self) -> Vec<ImageOperationParam> {
        panic!("ImageOperationFFT - not implemented");
    }

    // TODO: move to trait "ImageOperation"
    fn merge_results(&self, partial_results: Vec<ImageOperationParam>) -> PixelImageSimple {
        panic!("ImageOperationFFT - not implemented");
    }

    fn set_input_bitmap(&mut self, input_bitmap: PixelImageSimple) {
        self.input_bitmapdata = Rc::new(input_bitmap);
    }

    fn get_output_bitmap(&self) -> &PixelImageSimple {
        &self.output_bitmapdata
    }
}





#[macro_export]
macro_rules! complex_vec(
    ($($real:expr, $imaginary:expr),*) => ({
        let mut _temp = ::std::vec::Vec::new();
        $(
            let c = Complex64::new($real, $imaginary);
            _temp.push(c);
         )*
        _temp
    });
    ($($e:expr),+,) => (vec!($($e),+))
);

#[macro_export]
macro_rules! assert_complex_vec_eq(
    ($expected:expr, $actual:expr, $tolerance:expr) => (
        for (i, a) in $actual.iter().enumerate() {
            let e = $expected[i];
            let diff = e - *a;
            assert!(Float::abs(diff.re) < $tolerance && Float::abs(diff.im) < $tolerance);
        }
    );
);

pub struct FFT;

impl FFT {
    pub fn new() -> FFT {
        let fft = FFT;
        fft
    }

    pub fn execute_real(&self, vec: Vec<f64>) -> Vec<Complex64> {
        let length = vec.len();
        if length == 1 {
            return complex_vec!(vec[0],0f64);
        }
        let mut i = 0i32;
        let (even, odd): (Vec<f64>,Vec<f64>) = vec.into_iter().partition(|_: &f64| {
            let ret = i % 2 == 0;
            i +=  1;
            return ret;
        });

        let mut fft_even = self.execute_real(even);
        let mut fft_odd = self.execute_real(odd);

        for e in fft_even.clone().iter() {
            fft_even.push(*e);
        }
        for o in fft_odd.clone().iter() {
            fft_odd.push(*o);
        }

        let mut output = Vec::new();

        for i in 0..length {
            let c = fft_even[i] + self.omega(-(i as f64), length as f64) * fft_odd[i];
            output.push(c);
        }

        return output;
    }

    pub fn omega(&self, k: f64, n: f64) -> Complex64 {
        // e^(i2πk/n) = cos(2πk/n) + sin(2πk/n)i
        let theta = (2.0 * 3.141592653589793238460 * k) / n;
        Complex64::new(theta.cos(), theta.sin())
    }
}
