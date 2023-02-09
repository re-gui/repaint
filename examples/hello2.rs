use std::path::PathBuf;



fn main() {
    println!("Hello from an example!");

    let exe_path = std::env::current_exe().unwrap_or("\\\\?\\./target".into());
    let exe_dir = exe_path.parent().unwrap();
    //let exe_dir: PathBuf = "\\\\?\\C:/Users/lucac/Documents/GitHub/repaint/examples".into();

    let out_file = exe_dir.join("hello.txt");
    let out_file_str = out_file.to_str();
    let out_file_str = out_file_str.unwrap().strip_prefix("\\\\?\\").unwrap().replace("\\", "/"); 

    let out_file2 = exe_dir.join("hello.svg");
    let out_file2_str = out_file2.to_str().unwrap().strip_prefix("\\\\?\\").unwrap().replace("\\", "/"); 

    std::fs::write(out_file.clone(), "Hello from an example!").unwrap();

    use repaint::base::shapes::path as pp;
    use repaint::base::shapes::polyline as pl;
    use pp::PathCommand as pc;
    type Vec2f = nalgebra::Vector2<f32>;

    let path: Vec<pp::PathCommand> = vec![
        pc::MoveTo(Vec2f::new(0.0, 0.0)),
        pc::LineTo(Vec2f::new(100.0, 0.0)),
        pc::CubicBezierCurveTo{
            control_pt_1: Vec2f::new(100.0, 100.0),
            control_pt_2: Vec2f::new(0.0, 100.0),
            end_pt: Vec2f::new(0.0, 0.0),
        },
        pc::LineTo(Vec2f::new(20.0, 0.0)),
        pc::LineTo(Vec2f::new(20.0, 20.0)),
        pc::LineTo(Vec2f::new(23.0, 20.0)),
        pc::EllipticalArcToOffset{
            radii: Vec2f::new(20.0, 50.0),
            x_axis_rotation: 0.25,
            large_arc_flag: false,
            sweep_flag: false,
            end_pt_offset: Vec2f::new(30.0, 0.0),
        },
    ];
    let params = pp::DiscretizationParams {
        tolerance: 1.25,
        max_angle: 1.1,
        aoi: None,
        transform: pp::DiscretizationTransform::Identity,
    };
    let discr_commands = pp::discretize(&path, &params);

    let mut svg = String::new();
    svg.push_str(&format!(r#"<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">"#));
    {
        let mut points: Vec<Vec2f> = Vec::new();
        let mut flush = |points: &mut Vec<Vec2f>| {
            if points.len() > 1 {
                svg.push_str(&format!(r#"<polyline points=""#));
                for i in 0..points.len() {
                    let pt = points[i];
                    svg.push_str(&format!(r#"{} {} "#, pt.x, pt.y));
                }
                svg.push_str(&format!(r#"" stroke="black" fill="none" />"#));
            }
            points.clear();
        };
        for i in 0..discr_commands.len() {
            let command = &discr_commands[i];
            match command {
                pl::BrokenPolylineCommand::MoveTo(pt) => {
                    flush(&mut points);
                    points.push(*pt);
                },
                pl::BrokenPolylineCommand::LineTo(pt) => {
                    points.push(*pt);
                },
            }
        }
        flush(&mut points);
    }
    svg.push_str(&format!(r#"</svg>"#));
    std::fs::write(out_file2.clone(), svg).unwrap();

    println!("see output: {}", out_file_str);
    println!("see output: {}", out_file2_str);
}

