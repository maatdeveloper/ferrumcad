/* standard library */

/* external crates */
use serde::{Deserialize, Serialize};

/* ferrumcad crates */
use crate::cad::{Model};

pub const PROJECT_EXTENSION: &str = "fcad";

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Units {
    #[default] Millimeter,
    Centimeter,
    Meter,
    Inch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub author: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub version: String,
    pub units: Units,
    pub metadata: ProjectMetadata,
    pub model: Model,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            author: String::new(),
            created_at: String::new(),
            modified_at: String::new(),
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            units: Units::default(),
            metadata: ProjectMetadata::default(),
            model: Model::default(),
        }
    }
}

pub fn create_test_project() -> Project {
    use crate::geometry::{Line, Point, Circle};
    use crate::cad::Shape;

    let mut project = Project::default();

    project.metadata.name = "Test Project".into();

    project.model.shapes.push(
        Shape::Line(
            Line {
                start: Point { x: 0.0, y: 0.0 },
                end: Point { x: 100.0 , y: 100.0 },
            }
        )
    );

    project.model.shapes.push(
        Shape::Circle(
            Circle {
                center: Point { x: 50.0, y: 50.0 },
                radius: 25.0,
                diameter: 50.0,
            }
        )
    );

    project
}