use cairo::Context;

pub const SHAPE_WIDTH: f64 = 84.0;
pub const SHAPE_HEIGHT: f64 = 132.0;
pub const MARGIN: f64 = 30.0;
pub const VIEWBOX_SIZE: f64 = SHAPE_HEIGHT + 2.0 * MARGIN;

pub fn build_path(cr: &Context, width: f64, height: f64) {
    cr.save().unwrap();
    cr.scale(width / SHAPE_WIDTH, height / SHAPE_HEIGHT);

    cr.move_to(21.0, 0.0);
    cr.curve_to(42.0, 0.0, 42.0, 0.0, 63.0, 21.0);
    cr.curve_to(84.0, 42.0, 84.0, 42.0, 84.0, 75.0);
    cr.curve_to(84.0, 108.0, 84.0, 108.0, 72.0, 120.0);
    cr.curve_to(60.0, 132.0, 60.0, 132.0, 39.0, 132.0);
    cr.curve_to(24.0, 132.0, 18.0, 129.0, 18.0, 120.0);
    cr.curve_to(18.0, 108.0, 18.0, 108.0, 30.0, 96.0);
    cr.curve_to(42.0, 84.0, 42.0, 84.0, 21.0, 63.0);
    cr.curve_to(0.0, 42.0, 0.0, 42.0, 0.0, 21.0);
    cr.curve_to(0.0, 6.0, 6.0, 0.0, 21.0, 0.0);
    cr.close_path();

    cr.restore().unwrap();
}
