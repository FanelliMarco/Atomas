struct CIELAB {
    L: f32,
    a: f32,
    b: f32,
}

fn gamma_correct(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn nonlinear_to_linear(v: f32) -> f32 {
    if v > 0.008856 {
        v.cbrt()
    } else {
        (7.787 * v) + (16.0 / 116.0)
    }
}

fn rgb_to_cielab(r: u8, g: u8, b: u8) -> CIELAB {
    let red = gamma_correct(r as f32 / 255.0);
    let green = gamma_correct(g as f32 / 255.0);
    let blue = gamma_correct(b as f32 / 255.0);

    let xr = nonlinear_to_linear((red * 0.4124564 + green * 0.3575761 + blue * 0.1804375) / 95.047);
    let yr = nonlinear_to_linear((red * 0.2126729 + green * 0.7151522 + blue * 0.0721750) / 100.0);
    let zr =
        nonlinear_to_linear((red * 0.0193339 + green * 0.1191920 + blue * 0.9503041) / 108.883);

    CIELAB {
        L: 116.0 * yr - 16.0,
        a: 500.0 * (xr - yr),
        b: 200.0 * (yr - zr),
    }
}

fn similarity(r0: u8, g0: u8, b0: u8, r1: u8, g1: u8, b1: u8) -> f32 {
    let lab0 = rgb_to_cielab(r0, g0, b0);
    let lab1 = rgb_to_cielab(r1, g1, b1);

    let dl = lab0.L - lab1.L;
    let da = lab0.a - lab1.a;
    let db = lab0.b - lab1.b;

    dl * dl + da * da + db * db
}
