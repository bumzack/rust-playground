use num::complex::Complex64;
use std::rc::Rc;

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

        let mut complex_bitmap: Vec<Complex64> = vec![];
        let mut pixel = RGBA8 {r:0, g: 0, b: 0, a: 0};

        for x in 0..self.input_bitmapdata.width {
            for y in 0..self.input_bitmapdata.height {
                pixel  = self.input_bitmapdata.get_pixel(x, y);
                println!("pixel fft input  {}/{} = r: {}, g: {}, b: {}, a: {}", x, y, pixel.r, pixel.g,  pixel.b,  pixel.a);

                complex_bitmap.push(Complex64{ re: pixel.r as f64, im: 0.0});
            }
        }

        let fft_res = fft2D(&mut complex_bitmap, self.input_bitmapdata.width, self.input_bitmapdata.height, 1);
        println!("ImageOperationFFT::execute_op2   ->  res: {}", fft_res);
        println!("ImageOperationFFT::complex_bitmap   ->  res: {:?}", complex_bitmap);
        println!("ImageOperationFFT::complex_bitmap   ->  self.input_bitmapdata.width: {}, self.input_bitmapdata.height: {}", self.input_bitmapdata.width, self.input_bitmapdata.height);
        // println!("ImageOperationFFT::input_bitmapdata   ->  self.input_bitmapdata.pixels: {:?}",  self.input_bitmapdata.pixels);

        let mut idx = 0;

        for x in 0..self.output_bitmapdata.width {
            for y in 0..self.output_bitmapdata.height {
                idx = y * self.output_bitmapdata.width + x;
                let val = complex_bitmap[idx as usize].re as u8;
                //println!("fft result {}/{} =   {} ", x, y, complex_bitmap[idx as usize]);
                let pixel = RGBA8 {r:val, g: val, b: val, a: 255};
                self.output_bitmapdata.set_pixel(x, y, pixel);
            }
        }


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












//-------------------------------------------------------------------------
//   Calculate the closest but lower power of two of a number
//   twopm = 2**m <= n
//   Return TRUE if 2**m == n
//
fn power_of_2(n: i32) -> (i32, i32, bool) {

    let mut m: i32 = 0;
    let mut twopm: i32 = 0;
    if n <= 1 {
        return (0, 1, false);
    }

    m = 1;
    twopm = 2;
    loop {
        m += 1;
        twopm *= 2;
        if (2*twopm > n) { break } ;
    }

    if (twopm != n) {
        (m, twopm, false)
    } else {
        (m, twopm, true)
    }
}

/*-------------------------------------------------------------------------
   This computes an in-place complex-to-complex FFT
   x and y are the real and imaginary arrays of 2^m points.
   dir =  1 gives forward transform
   dir = -1 gives reverse transform

     Formula: forward
                  N-1
                  ---
              1   \          - j k 2 pi n / N
      X(n) = ---   >   x(k) e                    = forward transform
              N   /                                n=0..N-1
                  ---
                  k=0

      Formula: reverse
                  N-1
                  ---
                  \          j k 2 pi n / N
      X(n) =       >   x(k) e                    = forward transform
                  /                                n=0..N-1
                  ---
                  k=0
*/
fn fft(dir: i32, m: i32,  x: &mut Vec<f64>, y: &mut Vec<f64>) -> bool {
    let mut nn: usize = 0;
    let mut i : usize = 0;
    let mut i1 : usize = 0;
    let mut j : usize = 0;
    let mut k : usize = 0;
    let mut i2 : usize = 0;
    let mut l : usize = 0;
    let mut l1 : usize = 0;
    let mut l2 : usize = 0;

    let mut c1 : f64 = 0.0;
    let mut c2 : f64 = 0.0;
    let mut tx : f64 = 0.0;
    let mut ty : f64 = 0.0;
    let mut t1 : f64 = 0.0;
    let mut t2 : f64 = 0.0;
    let mut u1 : f64 = 0.0;
    let mut u2 : f64 = 0.0;
    let mut z : f64 = 0.0;

    // Calculate the number of points
    nn = 1;
    for i in 0..m {
        nn *= 2;
    }

    i2 = nn >> 1;
    j = 0;

    for i in 0..(nn-1) {
        if i < j {
            tx = x[i];
            ty = y[i];
            x[i] = x[j];
            y[i] = y[j];
            x[j] = tx;
            y[j] = ty;
        }
        k = i2;
        while k <= j {
           j -= k;
           k = k >> 1;
        }
        j += k;
    }

    /* Compute the FFT */
    c1 = -1.0;
    c2 = 0.0;
    l2 = 1;
    for l in 0..m {
      l1 = l2;
      l2 <<=  1;
      u1 = 1.0;
      u2 = 0.0;
      for j in 0..l1 {
        for i in (j..nn).step_by(l2) {
            i1 = i + l1;
            t1 = u1 * x[i1] - u2 * y[i1];
            t2 = u1 * y[i1] + u2 * x[i1];
            x[i1] = x[i] - t1;
            y[i1] = y[i] - t2;
            x[i] += t1;
            y[i] += t2;
         }
         z =  u1 * c1 - u2 * c2;
         u2 = u1 * c2 + u2 * c1;
         u1 = z;
      }
      c2 = (1.0 - c1).sqrt() / 2.0;
      if dir == 1 {
            c2 = -c2;
      }

      c1 = (1.0 + c1).sqrt() / 2.0;
    }

    /* Scaling for forward transform */
    if dir == 1 {
        for i in 0..nn {
            x[i] = x[i] / nn as f64;
            y[i] = y[i] / nn as f64;
        }
    }
    true
}

// -------------------------------------------------------------------------
//   Perform a 2D FFT inplace given a complex 2D array
//   The direction dir, 1 for forward, -1 for reverse
//   The size of the array (nx,ny)
//   Return false if there are memory problems or
//   the dimensions are not powers of 2

fn fft2D(c: &mut Vec<Complex64>, nx: i32, ny: i32, dir: i32) -> bool {
    let mut m: i32 = 0;
    let mut twopm: i32 = 0;

    let mut real: Vec<f64> = vec![0.0; nx as usize];
    let mut imag: Vec<f64> = vec![0.0; nx as usize];

    /* Transform the rows */
    let (m, twopm, res) = power_of_2(nx);
    // println!("nx     m: {}, twopm: {}, res: {}", m, twopm, res);
    if !res || twopm != nx {
        return false;
    }
    let mut idx: usize = 0;
    for j in 0..ny {
        for i in 0..nx {
            idx = (j*nx + i) as usize;
            real[i as usize] = c[idx].re;
            imag[i as usize] = c[idx].im;
        //    println!("nx - first for  i: {}, idx: {} ", i, idx);
        }
        let a = fft(dir, m, &mut real, &mut imag);
        for i in 0..nx {
            idx = (j*nx + i) as usize;
            c[idx].re = real[i as usize];
            c[idx].im = imag[i as usize];
            // println!("nx - second for  i: {}, idx: {} ", i, idx);
        }
    }

    real = vec![0.0; ny as usize];
    imag = vec![0.0; ny as usize];

    /* Transform the columns */
    let (m, twopm, res) = power_of_2(ny);
    println!("ny     m: {}, twopm: {}, res: {}", m, twopm, res);
    if !res || twopm != ny {
        return false;
    }
    for i in 0..nx {
        for j in 0..ny {
            idx = (j*nx + i) as usize;
            // println!("ny - first for  i: {}, idx: {} ", i, idx);
            real[j as usize] = c[idx].re;
            imag[j as usize ] = c[idx].im;
        }
        let res = fft(dir, m, &mut real, &mut imag);
        for j in 0..ny {
            idx = (j*nx + i) as usize;
            c[idx].re = real[j as usize];
            c[idx].im = imag[j as usize];
            // println!("ny - second for  i: {}, idx: {} ", i, idx);
        }
    }
    true
}
