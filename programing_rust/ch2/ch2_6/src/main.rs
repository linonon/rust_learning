use std::{env, fs::File, str::FromStr};

use anyhow::{Context, Result};
use num::{complex::ComplexFloat, Complex};

use image::{codecs::png::PngEncoder, ImageEncoder};

/// 這個是曼德博函數的雛形
#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };

    loop {
        z = z * z + c
    }
}

/// 嘗試測定 c 是否在符合曼德博, 會測試 limit 來判定
///
/// ```
/// match escape_time(c, limit) {
///     Some(i) =>  // i 表示 c 離開原點半徑為2的圓時所需要的迭代次數
///     None => // 表示 c 可能符合曼德博
/// }
/// ```
///
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        let sqr = z.norm_sqr();
        if sqr > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// 將輸入字串 `s` 解析成一個座標對
///
/// 如果正確, 返回 Some<(x,y)>, 無法解析, 返回 None
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        // 這邊可以看出, 如果我希望能調用 parse(), 那這個 T 就要實現 std::str::FromStr
        Some(i) => match (s[..i].parse::<T>(), s[i + 1..].parse::<T>()) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// 解析字串變成複數形式
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// 給定輸出圖像的行和列, 返回復平面中對應的座標
///
/// - `bounds` 是一個 `pair`, 給出了圖像的像素寬度和像素高度.
/// - `pixel` 是表示圖像特定像素的 (column, row) 二元組(座標?)
/// - `upper_left` 和 `lower_right` 是復平面中表示執行圖像覆蓋範圍的點
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re() - upper_left.re(),
        upper_left.im() - lower_right.im(),
    );

    Complex {
        re: upper_left.re() + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im() - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75,
        }
    )
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                Some(count) => 255 - count as u8,
                None => 0,
            }
        }
    }
}

fn render_v2(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<_> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
    // scope 保證裡面所有的 threads 執行完以後才會返回 Ok(())
    crossbeam::scope(|s| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            s.spawn(move |_| render(band, band_bounds, band_upper_left, band_lower_right));
        }
    })
    .unwrap();
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<()> {
    let output = File::create(filename).context("failed to create file")?;
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(
            pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            image::ExtendedColorType::L8,
        )
        .context("failed to write image")?;
    Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(Some((1, 2)), parse_pair("1,2", ','));
    assert_eq!(Some((1.0, 2.1)), parse_pair("1,2.1", ','));
}

/// time cargo run -p ch2_6_mandelbrot --release -- mandel.png 8000x6000 -1.20,0.35 -1,0.20
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0.20",
            args[0]
        );
        std::process::exit(1)
    }

    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    render_v2(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
