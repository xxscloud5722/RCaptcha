package com.xxscloud.beer.captcha;

import lombok.Getter;
import lombok.ToString;

/**
 * @author 橘猫.
 */
@Getter
@ToString
public class SliderCaptcha {
    private final Integer x;
    private final Integer y;
    private final byte[] background;
    private final byte[] cutout;

    public SliderCaptcha(Integer x, Integer y, byte[] background, byte[] cutout) {
        this.x = x;
        this.y = y;
        this.background = background;
        this.cutout = cutout;
    }
}
