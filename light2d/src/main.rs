#![allow(dead_code)]

use clap::{App, Arg};
use libm::{cos, fmin, sin, sqrt};
use rand::prelude::*;
use std::env;
use rustc_apfloat::ieee::{Quad};

fn main() {
    let default_num = "64";
    let default_max_step = "10";
    let default_max_distance = "2.0";
    let default_method = "uniform";
    let default_light_x = "0.5";
    let default_light_y = "0.5";
    let default_light_r = "0.1";
    let default_light_s = "2.0";
    let default_width = "128";
    let default_height = "128";
    let default_output_path;

    if "windows" == env::consts::OS {
        default_output_path = env::var("USERPROFILE").unwrap() + "\\";
    } else {
        default_output_path = env::var("HOME").unwrap() + "/";
    }

    let args = App::new("basic")
        .version("0.0.1")
        .arg(
            Arg::with_name("num")
                .short("n")
                .required(false)
                .default_value(default_num).help("取样次数"),
        )
        .arg(
            Arg::with_name("max_step")
                .short("t")
                .required(false)
                .default_value(default_max_step),
        )
        .arg(
            Arg::with_name("max_distance")
                .short("d")
                .required(false)
                .default_value(default_max_distance),
        )
        .arg(
            Arg::with_name("method")
                .short("m")
                .required(false)
                .default_value(default_method)
                .possible_values(&["uniform", "stratified", "jittered", "halton5"]).help("取样方法:\nuniform=均匀(随机)取样\nstratified=分层取样\njittered=抖动采样\nhalton5=低差异序列\n"),
        )
        .arg(
            Arg::with_name("lightx")
                .short("x")
                .required(false)
                .default_value(default_light_x).help("光源X坐标, 取值范围: [0, 1)"),
        )
        .arg(
            Arg::with_name("lighty")
                .short("y")
                .required(false)
                .default_value(default_light_y).help("光源Y坐标, 取值范围: [0, 1)"),
        )
        .arg(
            Arg::with_name("lightr")
                .short("r")
                .required(false)
                .default_value(default_light_r).help("光源半径"),
        )
        .arg(
            Arg::with_name("lights")
                .short("s")
                .required(false)
                .default_value(default_light_s).help("光源强度"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .required(false)
                .default_value(default_width).help("图像宽度"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .required(false)
                .default_value(default_height).help("图像高度"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .required(false)
                .default_value(default_output_path.as_str())
                .help("输出路径"),
        )
        .get_matches();

    let num: u32 = args
        .value_of("num")
        .unwrap_or(default_num)
        .parse()
        .expect("error");
    let max_step: u32 = args
        .value_of("max_step")
        .unwrap_or(default_max_step)
        .parse()
        .expect("error");
    let max_distance: f64 = args
        .value_of("max_distance")
        .unwrap_or(default_max_distance)
        .parse()
        .expect("error");
    let method: &str = args.value_of("method").unwrap_or(default_method);
    let lightx: f64 = args
        .value_of("lightx")
        .unwrap_or(default_light_x)
        .parse()
        .expect("error");
    let lighty: f64 = args
        .value_of("lighty")
        .unwrap_or(default_light_y)
        .parse()
        .expect("error");
    let lightr: f64 = args
        .value_of("lightr")
        .unwrap_or(default_light_r)
        .parse()
        .expect("error");
    let lights: f64 = args
        .value_of("lights")
        .unwrap_or(default_light_r)
        .parse()
        .expect("error");
    let width: usize = args
        .value_of("width")
        .unwrap_or(default_width)
        .parse()
        .expect("error");
    let height: usize = args
        .value_of("height")
        .unwrap_or(default_height)
        .parse()
        .expect("error");
    let output_path = String::from(
        args.value_of("output")
            .unwrap_or(default_output_path.as_str()),
    );

    let b = Basic::new(
        num,
        max_step,
        max_distance,
        method,
        lightx,
        lighty,
        lightr,
        lights,
    );
    let mut c = Canvas::new(width, height);

    for y in 0..c.height {
        for x in 0..c.width {
            let p = &mut c.pixels[x + y * c.width];
            let c = fmin(
                b.sample(x as f64 / c.width as f64, y as f64 / c.height as f64) * 255f64,
                255f64,
            ) as u8;
            p.0 = c;
            p.1 = c;
            p.2 = c;
        }
    }

    let mut output_file = [
        (width.to_string() + "x" + height.to_string().as_str()).as_str(),
        (String::from("n") + num.to_string().as_str()).as_str(),
        (String::from("t") + max_step.to_string().as_str()).as_str(),
        (String::from("d") + max_distance.to_string().as_str()).as_str(),
        (String::from("x") + lightx.to_string().as_str()).as_str(),
        (String::from("y") + lighty.to_string().as_str()).as_str(),
        (String::from("r") + lightr.to_string().as_str()).as_str(),
        (String::from("s") + lights.to_string().as_str()).as_str(),
        method,
    ]
    .join("_");
    output_file = output_path + output_file.as_str() + ".png";

    println!("Saving to {}...", output_file.as_str());
    c.save(output_file.as_str());
}

struct Basic {
    num_sample: u32,
    max_step: u32,
    max_distance: f64,
    lightx: f64,
    lighty: f64,
    lightr: f64,
    lights: f64,
    index: usize,
    sample_method: fn(u32, u32, usize) -> f64,
}

impl Basic {
    const TWO_PI: f64 = 6.28318530718f64;
    const EPSILON: f64 = 1e-6f64;

    fn new(
        num_sample: u32,
        max_step: u32,
        max_distance: f64,
        method: &str,
        lightx: f64,
        lighty: f64,
        lightr: f64,
        lights: f64,
    ) -> Self {
        let sample_method = match method {
            "stratified" => Basic::sample_method_stratified,
            "jittered" => Basic::sample_method_jittered,
            "halton5" => Basic::sample_method_halton5,
            _ => Basic::sample_method_uniform,
        };

        Basic {
            num_sample,
            max_step,
            max_distance,
            lightx,
            lighty,
            lightr,
            lights,
            index: 0,
            sample_method,
        }
    }

    fn sample(&self, x: f64, y: f64) -> f64 {
        let mut sum = 0f64;
        for i in 0..self.num_sample {
            let a: f64 = (self.sample_method)(i, self.num_sample, self.index);
            sum += self.trace(x, y, cos(a), sin(a))
        }

        sum / self.num_sample as f64
    }

    fn trace(&self, ox: f64, oy: f64, dx: f64, dy: f64) -> f64 {
        let mut t = 0f64;
        let mut i = 0u32;

        while self.max_step > i && self.max_distance > t {
            let sd = self.sdf_circle(
                ox + dx * t,
                oy + dy * t,
                self.lightx,
                self.lighty,
                self.lightr,
            );
            if Basic::EPSILON > sd {
                return self.lights;
            }
            t += sd;
            i += 1;
        }

        return 0f64;
    }

    fn sdf_circle(&self, x: f64, y: f64, cx: f64, cy: f64, r: f64) -> f64 {
        let dx = x - cx;
        let dy = y - cy;
        sqrt(dx * dx + dy * dy) - r
    }

    fn sample_method_uniform(_i: u32, _n: u32, _index: usize) -> f64 {
        Basic::TWO_PI * random::<f64>()
    }

    fn sample_method_stratified(i: u32, n: u32, _index: usize) -> f64 {
        Basic::TWO_PI * i as f64 / n as f64
    }

    fn sample_method_jittered(i: u32, n: u32, _index: usize) -> f64 {
        Basic::TWO_PI * (i as f64 + random::<f64>()) / n as f64
    }

    /// https://zhuanlan.zhihu.com/p/20374706
    fn sample_method_halton5(_i: u32, _n: u32, index: usize) -> f64 {
        let faure_permutation = [
            0, 75, 50, 25, 100, 15, 90, 65, 40, 115, 10, 85, 60, 35, 110, 5, 80, 55, 30, 105, 20,
            95, 70, 45, 120, 3, 78, 53, 28, 103, 18, 93, 68, 43, 118, 13, 88, 63, 38, 113, 8, 83,
            58, 33, 108, 23, 98, 73, 48, 123, 2, 77, 52, 27, 102, 17, 92, 67, 42, 117, 12, 87, 62,
            37, 112, 7, 82, 57, 32, 107, 22, 97, 72, 47, 122, 1, 76, 51, 26, 101, 16, 91, 66, 41,
            116, 11, 86, 61, 36, 111, 6, 81, 56, 31, 106, 21, 96, 71, 46, 121, 4, 79, 54, 29, 104,
            19, 94, 69, 44, 119, 14, 89, 64, 39, 114, 9, 84, 59, 34, 109, 24, 99, 74, 49, 124,
        ];

        let a:f64 = "0x1.fffffep-1".parse::<Quad>().expect("error").to_string().parse().expect("error");

        (faure_permutation[index % 125usize] * 1953125i32
            + faure_permutation[(index / 125usize) % 125usize] * 15625i32
            + faure_permutation[(index / 15625usize) % 125usize] * 125i32
            + faure_permutation[(index / 1953125usize) % 125usize]) as f64
            * (a / 244140625f64)
    }
}

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<(u8, u8, u8)>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![(255u8, 255u8, 255u8); width * height],
        }
    }

    fn clear(&mut self) {
        for (r, g, b) in self.pixels.iter_mut() {
            *r = 255u8;
            *g = 255u8;
            *b = 255u8;
        }
    }

    fn save(&self, path: &str) {
        let mut img = image::RgbImage::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx = x as usize + (self.height - 1 - y as usize) * self.width;
            *pixel = image::Rgb([self.pixels[idx].0, self.pixels[idx].1, self.pixels[idx].2])
        }

        img.save(path).unwrap();
    }
}
