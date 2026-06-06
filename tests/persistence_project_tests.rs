/* standard library */

/* external crates */

/* ferrumcad crates */
use ferrumcad::persistence::{Project, create_test_project};

#[test]
fn serialize_project() {
    let project = create_test_project();

    let json = serde_json::to_string_pretty(&project).unwrap();

    assert!(json.contains("version"));
    assert!(json.contains("units"));
}

#[test]
fn project_roadtrip() {
    let project = create_test_project();

    let json = serde_json::to_string_pretty(&project).unwrap();

    let loaded: Project = serde_json::from_str(&json).unwrap();

    assert_eq!(
        loaded.model.shapes.len(),
        project.model.shapes.len()
    );
}