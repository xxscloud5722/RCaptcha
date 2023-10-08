use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba, RgbaImage};
use rand::Rng;
use rust_embed::RustEmbed;

use crate::captcha_error::CaptchaError;
use crate::captcha_error::CaptchaError::OptionError;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 280;
const SLIDE_WIDTH: u32 = 100;
const SLIDE_HEIGHT: u32 = 100;

const SCALE: u32 = 2;

#[derive(RustEmbed)]
#[folder = "source/template"]
struct Asset;


/// 生成滑块验证码
pub fn generate(buffer: &[u8]) -> Result<(RgbaImage, RgbaImage, (u32, u32)), CaptchaError> {
    // 随机坐标
    let mut position = position();
    // 原始图片
    let mut original_image = original_image(buffer)?;
    // 滑块图片
    let cutout = cutout(&mut original_image, position)?
        .resize(SLIDE_WIDTH / SCALE, SLIDE_HEIGHT / SCALE, image::imageops::Triangle).to_rgba8();
    block(position, &mut original_image)?;
    let original_rgba_image = original_image
        .resize(WIDTH / SCALE, HEIGHT / SCALE, image::imageops::Triangle).to_rgba8();
    position.0 = position.0 / SCALE;
    position.1 = position.1 / SCALE;
    return Ok((original_rgba_image, cutout, position));
}


/// 获取原始图片
fn original_image(buffer: &[u8]) -> Result<DynamicImage, CaptchaError> {
    let mut image = image::load_from_memory(buffer)?;
    if image.width() != WIDTH || image.height() != HEIGHT {
        image = image.resize(WIDTH, HEIGHT, image::imageops::Triangle);
    }
    Ok(DynamicImage::ImageRgba8(image.to_rgba8()))
}

/// 随机坐标
fn position() -> (u32, u32) {
    // 最小X轴 必须大于一个滑块的宽度
    let x = SLIDE_WIDTH + random(WIDTH - SLIDE_WIDTH - SLIDE_WIDTH - random(10));
    let y = random(HEIGHT - SLIDE_HEIGHT - random(10));
    return (x, y);
}

/// 剪切图片
fn cutout(image: &mut DynamicImage, position: (u32, u32)) -> Result<DynamicImage, CaptchaError> {
    let asset_template = Asset::get("template.png").ok_or(OptionError)?;
    let template = image::load_from_memory(&asset_template.data)?.to_rgba8();

    // 滑块
    let mut cutout = image.crop(position.0, position.1, SLIDE_WIDTH, SLIDE_HEIGHT);

    // 添加变宽的滑块
    border(&mut cutout)?;

    // 跟布局形状切图
    for x in 0..SLIDE_WIDTH {
        for y in 0..SLIDE_HEIGHT {
            let alpha = template.get_pixel(x, y)[3];
            let pixel = cutout.get_pixel(x, y);
            cutout.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], alpha]));
        }
    }
    Ok(cutout)
}

/// 遮挡层
fn block(position: (u32, u32), image: &mut DynamicImage) -> Result<(), CaptchaError> {
    let asset_template = Asset::get("template.png").ok_or(OptionError)?;
    let template = image::load_from_memory(&asset_template.data)?.to_rgba8();

    for x in position.0..(position.0 + SLIDE_WIDTH) {
        for y in position.1..(position.1 + SLIDE_HEIGHT) {
            let alpha = template.get_pixel(x - position.0, y - position.1)[3];
            if alpha > 0 {
                let pixel = image.get_pixel(x, y);
                let merge_pixel = merge_colors(Rgba::from([0, 0, 0, alpha / 2]), pixel.to_rgba());
                image.put_pixel(x, y, merge_pixel);
            }
        }
    }
    Ok(())
}


/// 随机数 0-value 之间.
fn random(value: u32) -> u32 {
    rand::thread_rng().gen_range(0, value)
}

/// 给图片添加边框
fn border(image: &mut DynamicImage) -> Result<(), CaptchaError> {
    let asset_border = Asset::get("border.png").ok_or(OptionError)?;
    let border = image::load_from_memory(&asset_border.data)?.to_rgba8();
    for x in 0..SLIDE_WIDTH {
        for y in 0..SLIDE_HEIGHT {
            let pixel = border.get_pixel(x, y);
            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 && pixel[3] == 0 {
                continue;
            }
            image.put_pixel(x, y, pixel.to_rgba());
        }
    }
    Ok(())
}

/// 合并颜色
fn merge_colors(color_a: Rgba<u8>, color_b: Rgba<u8>) -> Rgba<u8> {
    let r_a = color_a[0] as u16;
    let g_a = color_a[1] as u16;
    let b_a = color_a[2] as u16;
    let a_a = color_a[3] as u16;

    let r_b = color_b[0] as u16;
    let g_b = color_b[1] as u16;
    let b_b = color_b[2] as u16;
    let a_b = color_b[3] as u16;


    // Calculate the resulting RGBA components by merging the two colors
    let r_result = ((r_a * a_a + r_b * (255 - a_a)) / 255) as u8;
    let g_result = ((g_a * a_a + g_b * (255 - a_a)) / 255) as u8;
    let b_result = ((b_a * a_a + b_b * (255 - a_a)) / 255) as u8;
    let a_result = (a_a + a_b * (255 - a_a) / 255) as u8;

    // Create the merged color
    Rgba([r_result, g_result, b_result, a_result])
}