use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x_range = -1f32..1f32;
    let y_range = -0.1f32..1f32;

    let root = BitMapBackend::new("curve.png", (640, 480)).into_drawing_area();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range, y_range)?;

    chart.configure_mesh().draw()?;
    let xs = (0..=100).map(|x| x as f32 / 100.0);
    let bezier_curve = BezierCurve {
        points: (
            Point::new(-0.5, 0.0),
            Point::new(-0.5, 0.5),
            Point::new(0.5, 0.5),
            Point::new(0.5, 0.0),
        )
    };
    let f = |x: f32| bezier_curve.use_parameter(x).to_primitive();

    chart
        .draw_series(LineSeries::new(
            xs.map(f),
            &RED,
        ))?;

    root.present()?;

    Ok(())
}

#[derive(Clone, Copy)]
struct BezierCurve {
    points: (Point, Point, Point, Point),
}

impl BezierCurve {
    fn use_parameter(&self, parameter: f32) -> Point {
        let a_lerp = lerp(&self.points.0, &self.points.1, parameter); // Lerped point between P0 and P1
        let b_lerp = lerp(&self.points.1, &self.points.2, parameter); // Lerped point between P1 and P2
        let c_lerp = lerp(&self.points.2, &self.points.3, parameter); // Lerped point between P2 and P3

        let ab_lerp = lerp(&a_lerp, &b_lerp, parameter); // Lerped point between a_lerp and b_lerp
        let bc_lerp = lerp(&b_lerp, &c_lerp, parameter); // Lerped point between b_lerp and c_lerp

        lerp(&ab_lerp, &bc_lerp, parameter)
    }
}

fn lerp(point_a: &Point, point_b: &Point, parameter: f32) -> Point {
    Point::new(
        point_a.x * (1.0 - parameter) + point_b.x * parameter,
        point_a.y * (1.0 - parameter) + point_b.y * parameter,
    )
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }

    pub fn to_primitive(self) -> (f32, f32) {
        (self.x, self.y)
    }
}
