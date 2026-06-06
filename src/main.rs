/* standard library */

/* external crates */

/* ferrumcad crates */
use ferrumcad::cad::Model;
use ferrumcad::cam::generate_gcode;
use ferrumcad::geometry::Point;

fn main() {
    let mut model = Model::new();

    model.add_line(Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 10.0 });

    let gcode = generate_gcode(&model.shapes);

    println!("{}", gcode);
}
