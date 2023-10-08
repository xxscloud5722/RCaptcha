use std::io::Cursor;
use std::ptr::null_mut;

use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JString};
use jni::sys::jbyteArray;

use crate::captcha_error::CaptchaError;

mod slider_captcha;
mod code_captcha;
mod captcha_error;


#[no_mangle]
pub extern "system" fn Java_com_captcha_ImageCaptcha_sliderCaptcha(mut env: JNIEnv, _class: JClass,
                                                                   buffer: JByteBuffer) -> jbyteArray {
    match slider_captcha(&mut env, buffer) {
        Ok(value) => value,
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("throw exception");
            null_mut()
        }
    }
}

fn slider_captcha(env: &mut JNIEnv, buffer: JByteBuffer) -> Result<jbyteArray, CaptchaError> {
    let buffer_address = env.get_direct_buffer_address(&buffer)?;
    let buffer_length = env.get_direct_buffer_capacity(&buffer)?;
    let buffer_data = unsafe { std::slice::from_raw_parts(buffer_address as *const u8, buffer_length) };
    let generate_result = slider_captcha::generate(buffer_data)?;

    // 写出背景
    let mut background = Cursor::new(Vec::new());
    generate_result.0.write_to(&mut background, image::ImageOutputFormat::Png)?;

    // 写出滑块
    let mut cutout = Cursor::new(Vec::new());
    generate_result.1.write_to(&mut cutout, image::ImageOutputFormat::Png)?;


    let mut result_buffer: Vec<u8> = Vec::new();
    // 4 字节 X轴
    result_buffer.extend((generate_result.2.0 as i32).to_le_bytes());
    // 4 字节 Y轴
    result_buffer.extend((generate_result.2.1 as i32).to_le_bytes());
    // 4 字节 滑块下标
    result_buffer.extend((cutout.get_ref().len() as i32).to_le_bytes());
    result_buffer.extend(cutout.get_ref());
    result_buffer.extend(background.get_ref());

    // 返回结果
    let output_array = env.byte_array_from_slice(&result_buffer)?;
    Ok(output_array.into_raw())
}


#[no_mangle]
pub extern "system" fn Java_com_captcha_ImageCaptcha_codeCaptcha(mut env: JNIEnv, _class: JClass, text: JString) -> jbyteArray {
    match code_captcha(&mut env, text) {
        Ok(value) => value,
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("throw exception");
            null_mut()
        }
    }
}

fn code_captcha(env: &mut JNIEnv, text: JString) -> Result<jbyteArray, CaptchaError> {
    // 读取参数
    let text: String = env.get_string(&text)?.into();
    // 生成验证码
    let image = code_captcha::generate(text)?;
    // 创建缓存区
    let mut buffer = Cursor::new(Vec::new());
    // 输出JPEG 格式, 并指定图片质量
    image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(75))?;
    // 返回结果
    let output_array = env.byte_array_from_slice(buffer.get_ref())?;
    Ok(output_array.into_raw())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{code_captcha, slider_captcha};

    #[test]
    fn captcha() {
        let generate_result = code_captcha::generate("AP09".to_string()).unwrap();
        generate_result.save("captcha.jpg").unwrap();
    }

    #[test]
    fn slider() {
        let buffer_data = fs::read("./example/01.jpg").unwrap();
        let generate_result = slider_captcha::generate(&buffer_data).unwrap();
        generate_result.0.save("slider.png").unwrap();
        generate_result.1.save("slider_background.png").unwrap();
        println!("{:?}", generate_result.2);
    }
}