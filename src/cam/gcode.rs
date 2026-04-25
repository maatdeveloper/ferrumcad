use crate::cad::Shape;

pub fn generate_gcode(shapes: &[Shape]) -> String {
    let mut output = String::new();

    output.push_str("G21\n"); //mm
    output.push_str("G99\n"); //absolute

    for shape in shapes {
        match shape {
            Shape::Line(line) => {
                output.push_str(&format!(
                    "G0 X{} Y{}\n",
                    line.start.x, line.start.y
                ));
                output.push_str(&format!(
                    "G0 X{} Y{}\n",
                    line.end.x, line.end.y
                ));
            },
            Shape::Circle(_) => todo!(),
        }
    }

    output
}
