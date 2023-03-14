use repaint::base::transform::Transform2d;

fn main() {
    println!("Hello from an example!");

    let exe_path = std::env::current_exe().unwrap_or("\\\\?\\./target".into());
    let exe_dir = exe_path.parent().unwrap();

    let out_file = exe_dir.join("hello.txt");
    let out_file_str = out_file.to_str();
    let out_file_str = out_file_str
        .unwrap()
        .strip_prefix("\\\\?\\")
        .unwrap()
        .replace("\\", "/");

    let out_file2 = exe_dir.join("hello.svg");
    let out_file2_str = out_file2
        .to_str()
        .unwrap()
        .strip_prefix("\\\\?\\")
        .unwrap()
        .replace("\\", "/");

    std::fs::write(out_file.clone(), "Hello from an example!").unwrap();

    use pp::PathCommand as pc;
    use repaint::base::shapes::path as pp;
    use repaint::base::shapes::polyline as pl;
    type Vec2f = nalgebra::Vector2<f32>;

    let path: Vec<pp::PathCommand> = vec![
        pc::MoveTo(Vec2f::new(0.0, 0.0)),
        pc::LineTo(Vec2f::new(100.0, 0.0)),
        pc::CubicBezierCurveTo {
            control_pt_1: Vec2f::new(100.0, 100.0),
            control_pt_2: Vec2f::new(0.0, 100.0),
            end_pt: Vec2f::new(0.0, 0.0),
        },
        pc::LineTo(Vec2f::new(20.0, 0.0)),
        pc::LineTo(Vec2f::new(20.0, 20.0)),
        pc::LineTo(Vec2f::new(23.0, 20.0)),
        pc::EllipticalArcToOffset {
            radii: Vec2f::new(20.0, 20.0),
            x_axis_rotation: 0.25,
            large_arc_flag: false,
            sweep_flag: false,
            end_pt_offset: Vec2f::new(20.0, 0.0),
        },
        pc::ClosePath,
        pc::MoveTo(Vec2f::new(50.0, 50.0)),
        pc::EllipticalArcToOffset {
            radii: Vec2f::new(10.0, 10.0),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: false,
            end_pt_offset: Vec2f::new(20.0, 0.0),
        },
        pc::EllipticalArcToOffset {
            radii: Vec2f::new(10.0, 10.0),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: false,
            end_pt_offset: Vec2f::new(-20.0, 0.0),
        },
        pc::MoveTo(Vec2f::new(50.0, 20.0)),
        pc::QuadraticBezierCurveToOffset {
            control_pt_offset: Vec2f::new(0.0, 10.0),
            end_pt_offset: Vec2f::new(20.0, 0.0),
        },
        pc::SmoothCubicBezierCurveToOffset {
            control_pt_2_offset: Vec2f::new(0.0, 10.0),
            end_pt_offset: Vec2f::new(20.0, 0.0),
        },
    ];
    let mut params = pp::discretization::DiscretizationParams::default();
    params.tolerance = 1.5;
    params.max_angle = 1.1;
    let discretizer = pp::discretization::PathDiscretizer::new(
        params,
        Transform2d::Identity,
    );
    let mut it = path.iter();
    let discr_commands_iter = discretizer.discretize(&mut it);

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">"#
    ));
    {
        let possible_colors = [
            "red", "green", "blue", "yellow", "orange", "purple", "pink", "brown", "black", "white",
        ];
        let mut color_counter = 0;
        let mut points: Vec<Vec2f> = Vec::new();
        let mut count = 0;
        let mut flush = |points: &mut Vec<Vec2f>| {
            if points.len() > 1 {
                svg.push_str(&format!(r#"<polyline points=""#));
                for i in 0..points.len() {
                    let pt = points[i];
                    svg.push_str(&format!(r#"{} {} "#, pt.x, pt.y));
                }
                svg.push_str(&format!(
                    r#"" stroke="{}" fill="none" />"#,
                    possible_colors[color_counter]
                ));
                color_counter = (color_counter + 1) % possible_colors.len();
            }
            points.clear();
        };
        for command in discr_commands_iter {
            match command {
                pl::BrokenPolylineCommand::MoveTo(pt) => {
                    flush(&mut points);
                    points.push(pt);
                    count += 1;
                }
                pl::BrokenPolylineCommand::LineTo(pt) => {
                    points.push(pt);
                    count += 1;
                }
            }
        }
        flush(&mut points);

        println!("count: {}", count);
    }
    svg.push_str(&format!(r#"</svg>"#));
    std::fs::write(out_file2.clone(), svg).unwrap();

    println!("see output: {}", out_file_str);
    println!("see output: {}", out_file2_str);
}
