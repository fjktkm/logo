use std::{fs::File, path::Path};

use anyhow::Result;
use cairo::{Context, Format, ImageSurface, Operator, PdfSurface, RadialGradient, SvgSurface};

use crate::{
    noise,
    shape::{self, SHAPE_HEIGHT, SHAPE_WIDTH, VIEWBOX_SIZE},
    theme::Theme,
};

const SHAPE_NOISE_SEED_SALT: u64 = 0x5a5a5a5a_5a5a5a5a;

fn shape_bounds(output_size: f64) -> (f64, f64, f64, f64) {
    let scale = output_size / VIEWBOX_SIZE;
    (
        scale * (VIEWBOX_SIZE - SHAPE_WIDTH) * 0.5,
        scale * shape::MARGIN,
        scale * SHAPE_WIDTH,
        scale * SHAPE_HEIGHT,
    )
}

fn make_image_surface(size: u32) -> Result<ImageSurface> {
    ImageSurface::create(Format::ARgb32, size as i32, size as i32).map_err(Into::into)
}

fn apply_surface_noise(
    surface: &mut ImageSurface,
    size: u32,
    seed: u64,
    intensity: f64,
) -> Result<()> {
    if intensity <= 0.0 {
        return Ok(());
    }

    let stride = surface.stride() as usize;
    let mut data = surface.data()?;
    noise::apply_noise_argb32(&mut data, size, seed, intensity, stride);
    Ok(())
}

fn make_gradient_surface(size: u32, theme: &Theme) -> Result<ImageSurface> {
    let mut surface = make_image_surface(size)?;
    let (_, shape_y, _, _) = shape_bounds(size as f64);

    {
        let cr = Context::new(&surface)?;
        let gradient = RadialGradient::new(
            size as f64 * 0.5,
            size as f64 - shape_y,
            0.0,
            size as f64 * 0.5,
            size as f64 - shape_y,
            size as f64 - 2.0 * shape_y,
        );
        let start = theme.gradient_start;
        let end = theme.gradient_end;
        gradient.add_color_stop_rgb(0.0, start.r, start.g, start.b);
        gradient.add_color_stop_rgb(1.0, end.r, end.g, end.b);
        cr.set_source(&gradient)?;
        cr.paint()?;
    }

    apply_surface_noise(
        &mut surface,
        size,
        theme.noise_seed ^ SHAPE_NOISE_SEED_SALT,
        theme.shape_noise_intensity,
    )?;

    Ok(surface)
}

fn paint_gradient_shape(cr: &Context, output_size: f64, gradient: &ImageSurface) -> Result<()> {
    let (shape_x, shape_y, shape_width, shape_height) = shape_bounds(output_size);

    cr.save()?;
    cr.translate(shape_x, shape_y);
    shape::build_path(cr, shape_width, shape_height);
    cr.clip();
    cr.identity_matrix();
    cr.set_source_surface(gradient, 0.0, 0.0)?;
    cr.set_operator(Operator::Over);
    cr.paint()?;
    cr.restore()?;
    Ok(())
}

fn draw_logo(cr: &Context, output_size: f64, theme: &Theme) -> Result<()> {
    let raster_size = output_size as u32;
    let mut background = make_image_surface(raster_size)?;
    {
        let cr = Context::new(&background)?;
        let color = theme.background;
        cr.set_source_rgb(color.r, color.g, color.b);
        cr.paint()?;
    }
    apply_surface_noise(
        &mut background,
        raster_size,
        theme.noise_seed,
        theme.bg_noise_intensity,
    )?;

    let gradient = make_gradient_surface(raster_size, theme)?;

    cr.set_source_surface(&background, 0.0, 0.0)?;
    cr.set_operator(Operator::Source);
    cr.paint()?;
    paint_gradient_shape(cr, output_size, &gradient)
}

pub fn render_png(path: &Path, size: u32, theme: &Theme) -> Result<()> {
    let surface = make_image_surface(size)?;
    let cr = Context::new(&surface)?;
    draw_logo(&cr, size as f64, theme)?;
    drop(cr);

    let mut file = File::create(path)?;
    surface.write_to_png(&mut file)?;
    Ok(())
}

pub fn render_svg(path: &Path, size: f64, theme: &Theme) -> Result<()> {
    let surface = SvgSurface::new(size, size, Some(path))?;
    let cr = Context::new(&surface)?;
    draw_logo(&cr, size, theme)?;
    drop(cr);
    surface.finish();
    Ok(())
}

pub fn render_pdf(path: &Path, size: f64, theme: &Theme) -> Result<()> {
    let surface = PdfSurface::new(size, size, path)?;
    let cr = Context::new(&surface)?;
    draw_logo(&cr, size, theme)?;
    drop(cr);
    surface.finish();
    Ok(())
}
