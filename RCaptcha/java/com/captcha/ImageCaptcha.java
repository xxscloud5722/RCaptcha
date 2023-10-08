package com.captcha;

import com.billbear.tools.common.LoadLibs;
import lombok.extern.log4j.Log4j2;

import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

/**
 * 橘猫.
 */
@Log4j2
public class ImageCaptcha {

    /**
     * 堆外内容的缓存对象.
     */
    public static final List<ByteBuffer> ARRAY = new ArrayList<>();

    static {
        log.info("JNI Captcha(ImageCaptcha) Loaded ...");

        final List<String> libs;
        if (LoadLibs.isWindows()) {
            libs = List.of("captcha/rcaptcha.dll");
        } else if (LoadLibs.isMac()) {
            libs = Collections.emptyList();
        } else {
            libs = List.of("captcha/librcaptcha.so");
        }
        final String rootPath = LoadLibs.getRootPath();
        LoadLibs.load(rootPath, libs, libPath -> log.info("System Load .so(dll) <== {}", libPath));

        // 加载图片
        final List<String> files = Arrays.asList("201.jpg", "202.jpg", "203.jpg", "102.jpg");
        for (String fileName : files) {
            try (InputStream inputStream = ImageCaptcha.class.getResourceAsStream("/captcha_slider/" + fileName)) {
                if (Objects.isNull(inputStream)) {
                    continue;
                }
                loadSliderImage(inputStream.readAllBytes());
                log.info("System Load ImageCaptcha  <== {}", fileName);
            } catch (IOException e) {
                throw new RuntimeException(e.getMessage());
            }
        }
    }


    /**
     * 图形验证.
     *
     * @param text 内容.
     * @return 二进制文件.
     */
    public static native byte[] codeCaptcha(String text);

    /**
     * 滑块验证.
     *
     * @param bytes 原始内容.
     * @return 二进制文件.
     */
    public static native byte[] sliderCaptcha(ByteBuffer bytes);

    /**
     * 加载滑块验证码到堆外
     *
     * @param image 图片二进制.
     */
    public static void loadSliderImage(byte[] image) {
        final ByteBuffer byteBuffer = ByteBuffer.allocateDirect(image.length);
        byteBuffer.put(image);
        byteBuffer.rewind();
        ARRAY.add(byteBuffer);
    }

    /**
     * 生成滑块验证码.
     *
     * @return 滑块验证码结果.
     */
    public static SliderCaptcha sliderCaptcha() {
        if (ARRAY.isEmpty()) {
            return null;
        }
        final ByteBuffer byteBuffer = ARRAY.get(ThreadLocalRandom.current().nextInt(ARRAY.size()));
        final byte[] result = ImageCaptcha.sliderCaptcha(byteBuffer);

        // 返回处理
        final int x = ByteBuffer.wrap(new byte[]{result[3], result[2], result[1], result[0]}).getInt();
        final int y = ByteBuffer.wrap(new byte[]{result[7], result[6], result[5], result[4]}).getInt();
        final int size = ByteBuffer.wrap(new byte[]{result[11], result[10], result[9], result[8]}).getInt();

        final byte[] cutout = Arrays.copyOfRange(result, 12, size + 12);
        final byte[] background = Arrays.copyOfRange(result, size + 12, result.length);
        return new SliderCaptcha(x, y, background, cutout);
    }
}
