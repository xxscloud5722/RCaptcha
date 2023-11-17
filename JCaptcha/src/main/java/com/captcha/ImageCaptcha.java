package com.captcha;

import lombok.extern.log4j.Log4j2;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
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
