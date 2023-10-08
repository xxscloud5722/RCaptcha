use image::{DynamicImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_cubic_bezier_curve_mut, draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut};
use rand::Rng;
use rust_embed::RustEmbed;
use rusttype::{Font, Scale};

use crate::captcha_error::CaptchaError;

const HEIGHT: u32 = 96;
const WIDTH: u32 = 260;
const FONT_SIZE: f32 = 72f32;
const FONT_WIDTH: usize = 48;
const FONT_INTERVAL: usize = 15;
const COLORS: [Rgb<u8>; 12] = [
    Rgb([0, 135, 255]),
    Rgb([51, 135, 51]),
    Rgb([255, 102, 102]),
    Rgb([255, 153, 0]),
    Rgb([153, 102, 0]),
    Rgb([153, 102, 153]),
    Rgb([51, 153, 153]),
    Rgb([102, 102, 255]),
    Rgb([0, 102, 104]),
    Rgb([204, 51, 51]),
    Rgb([0, 153, 204]),
    Rgb([0, 51, 102]),
];


#[derive(RustEmbed)]
#[folder = "source/font"]
struct Asset;

pub fn generate(text: String) -> Result<DynamicImage, CaptchaError> {
    let mut image = ImageBuffer::new(WIDTH, HEIGHT);
    // 背景颜色
    set_background_white(&mut image);

    // 圆圈
    draw_hollow_circle(&mut image);

    // 干扰线
    draw_random_lines(&mut image);

    // 贝塞尔曲线
    draw_bezier_curve(&mut image);

    // 字体
    add_text_to_image(&mut image, text)?;

    // 转成JPG 图片
    let image_rgb = DynamicImage::ImageRgb8(image.into());
    Ok(image_rgb)
}

/// 设置图片背景为白色
fn set_background_white(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let white_pixel = Rgb([255, 255, 255]);
    for pixel in image.pixels_mut() {
        *pixel = white_pixel;
    }
}

/// 画空心圆
fn draw_hollow_circle(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for _ in 0..random_range(5, 10) {
        let size = random_range(4, 8) as i32;
        let point = (random_range(size as u32, WIDTH - size as u32) as i32,
                     random_range(size as u32, HEIGHT - size as u32) as i32);
        draw_hollow_circle_mut(image, point, size, color());
    }
}

/// 干扰线
fn draw_random_lines(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for _ in 0..random_range(3, 6) {
        let x1 = random(WIDTH) as f32;
        let y1 = random(HEIGHT) as f32;
        let x2 = random(WIDTH) as f32;
        let y2 = random(HEIGHT) as f32;

        draw_line_segment_mut(img, (x1, y1), (x2, y2), color());
    }
}

/// 画贝塞尔曲线
fn draw_bezier_curve(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for _ in 0..random_range(3, 5) {
        let x1 = 5f32;
        let mut y1 = random_range(5, HEIGHT / 2) as f32;

        let x2 = (WIDTH - 5) as f32;
        let mut y2 = random_range(HEIGHT / 2, HEIGHT - 5) as f32;

        let lx = random_range(WIDTH / 4, WIDTH / 4 * 3) as f32;
        let ly = random_range(5, HEIGHT - 5) as f32;

        let lx1 = random_range(WIDTH / 4, HEIGHT / 4 * 3) as f32;
        let ly1 = random_range(5, HEIGHT - 5) as f32;

        if random(2) == 0 {
            let ty = y1;
            y1 = y2;
            y2 = ty;
        }
        draw_cubic_bezier_curve_mut(img, (x1, y1), (x2, y2),
                                    (lx, ly), (lx1, ly1),
                                    color());
    }
}

/// 图片添加文字
fn add_text_to_image(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, text: String) -> Result<(), CaptchaError> {
    let font_data = Asset::get("actionj.ttf").ok_or(CaptchaError::OptionError)?.data;
    let font = Font::try_from_bytes(&*font_data).ok_or(CaptchaError::OptionError)?;

    let chars: Vec<char> = text.chars().collect();
    // 计算字体所占宽度
    let mut font_width = FONT_WIDTH as i32 * chars.len() as i32;
    if chars.len() > 2 {
        font_width += FONT_INTERVAL as i32 * (chars.len() - 2) as i32;
    }
    // 计算居中位置
    let x = (WIDTH as i32 - font_width) / 2 - 5;
    let y = (HEIGHT as i32 - FONT_SIZE as i32) / 2 + 5;
    for i in 0..chars.len() {
        draw_text_mut(img, color(),
                      x + (i * (FONT_WIDTH + FONT_INTERVAL)) as i32,
                      y, Scale::uniform(FONT_SIZE), &font,
                      &chars.get(i).ok_or(CaptchaError::OptionError)?.to_string());
    }
    Ok(())
}

/// 随机数 0-value 之间.
fn random(value: u32) -> u32 {
    rand::thread_rng().gen_range(0, value)
}

/// 随机数 min-max 之间.
fn random_range(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min, max)
}

/// 随机生成颜色.
fn color() -> Rgb<u8> {
    COLORS[random(COLORS.len() as u32) as usize]
}
