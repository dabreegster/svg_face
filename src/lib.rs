use rand::Rng;
use std::io::{Error, Write};

// TODO More descriptive variable names
// TODO Better colors
// TODO Fix the viewport size / don't cut off the top and left

/// Generates a random face and renders it as an SVG string.
///
/// ```
/// use rand::SeedableRng;
///
/// fn main() -> std::io::Result<()> {
///     let mut rng = rand_xorshift::XorShiftRng::from_entropy();
///     let mut file = std::fs::File::create("face.svg")?;
///     svg_face::generate_face(&mut file, &mut rng)
/// }
/// ```
pub fn generate_face<R: Rng>(svg: &mut dyn Write, rng: &mut R) -> Result<(), Error> {
    let hsx = rand(rng, 145.0, 400.0);
    let hsy = 0.0;
    let hcp1x = rand(rng, 0.0, 400.0);
    let hcp1y = rand(rng, 190.0, 400.0);

    let bun_size = rand(rng, 6.0, 15.0);
    let buny = rand(rng, -hcp1y / 2.0, 120.0);
    let bunx = rand(rng, -150.0, bun_size * 2.0);

    let hairk = rand(rng, 5.0, 15.0);
    let hairstr = rand(rng, 1.5, 5.0);
    let hairl = rand(rng, -22.0, 33.0);
    let hairln = 14.0;

    let espac = rand(rng, 50.0 / 2.0, hsx - 30.0);
    let eypos = rand(rng, 0.0, hcp1y / 3.5);
    let ew = rand(rng, 10.0, 100.0);
    let eh = ew - rand(rng, 0.0, 80.0);

    let p = rand(rng, eh / 4.0, eh / 1.5);

    let ch_spacing = rand(rng, 40.0, 60.0);
    let ch_ypos = rand(rng, hcp1y / 4.0, hcp1y / 2.0);
    let ch = rand(rng, 0.0, 65.0);

    let mouth_x = rand(rng, 15.0, hcp1x / 3.0);
    let mouth_y = rand(rng, 125.0, 150.0);
    let mouth_cx = rand(rng, 250.0 / 6.0, 250.0 / 4.0);
    let mouth_cy = rand(rng, 125.0, 160.0);

    let nose_x = rand(rng, 10.0, 25.0);
    let nose_y = rand(rng, 90.0, 140.0);
    let nose_cx = rand(rng, 5.0, 120.0);
    let nose_cy = rand(rng, 0.0, 125.0);

    // ???
    let width = 800.0;
    let height = 600.0;

    writeln!(
        svg,
        r##"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"##,
        width * 2.0,
        height * 2.0
    )?;

    // Palette
    let black = (0, 0, 0);
    let white = (255, 255, 255);

    let hair_color = white;
    let skin_color = white;
    let eye_color = white;
    let cheeks_color = black;

    let mut style = Style {
        fill: None,
        stroke: Some(black),
        stroke_width: 1.0,
    };

    {
        style.fill = Some(hair_color);

        let mut i = bun_size;
        while i > 0.0 {
            ellipse(
                svg,
                &style,
                width / 2.0 - hsx - bunx,
                height / 2.0 - buny,
                i * i,
                i * i,
            )?;
            ellipse(
                svg,
                &style,
                width / 2.0 + hsx + bunx,
                height / 2.0 - buny,
                i * i,
                i * i,
            )?;
            i -= hairstr;
        }
    }

    style.fill = Some(skin_color);
    bezier(
        svg,
        &style,
        width / 2.0 - hsx,
        height / 2.0 + hsy,
        width / 2.0 - hcp1x / 10.0,
        height / 2.0 - height / 7.0,
        width / 2.0 + hcp1x / 10.0,
        height / 2.0 - height / 7.0,
        width / 2.0 + hsx,
        height / 2.0 + hsy,
    )?;
    bezier(
        svg,
        &style,
        width / 2.0 - hsx,
        height / 2.0 + hsy,
        width / 2.0 - hcp1x,
        height / 2.0 + hcp1y,
        width / 2.0 + hcp1x,
        height / 2.0 + hcp1y,
        width / 2.0 + hsx,
        height / 2.0 + hsy,
    )?;

    style.fill = Some(cheeks_color);
    ellipse(
        svg,
        &style,
        width / 2.0 - hsx + ch_spacing,
        height / 2.0 + ch_ypos,
        ch,
        ch,
    )?;
    ellipse(
        svg,
        &style,
        width / 2.0 + hsx - ch_spacing,
        height / 2.0 + ch_ypos,
        ch,
        ch,
    )?;

    style.fill = Some(white);
    ellipse(
        svg,
        &style,
        width / 2.0 - espac,
        height / 2.0 + eypos,
        ew,
        eh,
    )?;
    ellipse(
        svg,
        &style,
        width / 2.0 + espac,
        height / 2.0 + eypos,
        ew,
        eh,
    )?;

    style.fill = Some(eye_color);
    ellipse(svg, &style, width / 2.0 - espac, height / 2.0 + eypos, p, p)?;
    ellipse(svg, &style, width / 2.0 + espac, height / 2.0 + eypos, p, p)?;

    // Hair
    {
        let mut i = 0.0;
        while i <= hairk {
            style.fill = if i >= hairk - 1.0 {
                None
            } else {
                Some(hair_color)
            };
            bezier(
                svg,
                &style,
                width / 2.0 - hsx,
                height / 2.0 + i * hairl,
                width / 2.0 - hsx,
                height / 4.0 + i * i,
                width / 2.0,
                height / 2.5,
                width / 2.0,
                height / 2.0 - height / 8.0 + hairln,
            )?;
            bezier(
                svg,
                &style,
                width / 2.0 + hsx,
                height / 2.0 + i * hairl,
                width / 2.0 + hsx,
                height / 4.0 + i * i,
                width / 2.0,
                height / 2.5,
                width / 2.0,
                height / 2.0 - height / 8.0 + hairln,
            )?;
            i += hairstr;
        }
    }

    style.fill = None;
    style.stroke_width = 5.0;
    style.stroke = Some((
        if skin_color.0 > 45 {
            skin_color.0 - 45
        } else {
            skin_color.0
        },
        if skin_color.1 > 45 {
            skin_color.1 - 45
        } else {
            skin_color.1
        },
        if skin_color.2 > 45 {
            skin_color.2 - 45
        } else {
            skin_color.2
        },
    ));
    bezier(
        svg,
        &style,
        width / 2.0 - mouth_x,
        height / 2.0 + mouth_y,
        width / 2.0 - mouth_cx,
        height / 2.0 + mouth_cy,
        width / 2.0 + mouth_cx,
        height / 2.0 + mouth_cy,
        width / 2.0 + mouth_x,
        height / 2.0 + mouth_y,
    )?;

    style.fill = Some(skin_color);
    bezier(
        svg,
        &style,
        width / 2.0 - nose_x,
        height / 2.0 + nose_y,
        width / 2.0 - nose_cx,
        height / 2.0 + nose_cy,
        width / 2.0 + nose_cx,
        height / 2.0 + nose_cy,
        width / 2.0 + nose_x,
        height / 2.0 + nose_y,
    )?;

    style.fill = None;
    style.stroke = Some(black);
    style.stroke_width = 1.0;
    bezier(
        svg,
        &style,
        width / 2.0 - mouth_x,
        height / 2.0 + mouth_y,
        width / 2.0 - mouth_cx,
        height / 2.0 + mouth_cy,
        width / 2.0 + mouth_cx,
        height / 2.0 + mouth_cy,
        width / 2.0 + mouth_x,
        height / 2.0 + mouth_y,
    )?;

    bezier(
        svg,
        &style,
        width / 2.0 - nose_x,
        height / 2.0 + nose_y,
        width / 2.0 - nose_cx,
        height / 2.0 + nose_cy,
        width / 2.0 + nose_cx,
        height / 2.0 + nose_cy,
        width / 2.0 + nose_x,
        height / 2.0 + nose_y,
    )?;

    writeln!(svg, "</svg>")
}

fn ellipse(
    svg: &mut dyn Write,
    style: &Style,
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
) -> Result<(), Error> {
    writeln!(
        svg,
        r##"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" {}/>"##,
        cx,
        cy,
        rx,
        ry,
        style.render()
    )
}

fn bezier(
    svg: &mut dyn Write,
    style: &Style,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
) -> Result<(), Error> {
    writeln!(
        svg,
        r##"<path d="M{},{} C{},{},{},{},{},{}" {}/>"##,
        x1,
        y1,
        x2,
        y2,
        x3,
        y3,
        x4,
        y4,
        style.render()
    )
}

struct Style {
    fill: Option<(u8, u8, u8)>,
    stroke: Option<(u8, u8, u8)>,
    stroke_width: f64,
}

impl Style {
    fn render(&self) -> String {
        let mut inner = format!("stroke-width: {}", self.stroke_width);
        if let Some(rgb) = self.stroke {
            inner = format!("{}; stroke: rgb({}, {}, {})", inner, rgb.0, rgb.1, rgb.2);
        }
        if let Some(rgb) = self.fill {
            inner = format!("{}; fill: rgb({}, {}, {})", inner, rgb.0, rgb.1, rgb.2);
        }
        format!(r##"style="{}""##, inner)
    }
}

fn rand<R: Rng>(rng: &mut R, low: f64, high: f64) -> f64 {
    if low <= high {
        rng.gen_range(low, high)
    } else {
        rng.gen_range(high, low)
    }
}