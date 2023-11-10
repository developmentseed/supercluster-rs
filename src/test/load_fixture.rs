use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// GeoJSON Point
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    /// Point type
    #[serde(rename = "type")]
    pub r#type: String,

    /// Array of coordinates with longitude as first value and latitude as second one
    pub coordinates: Vec<f64>,
}

/// Feature metadata
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    /// Feature's name
    pub name: Option<String>,

    /// Indicates whether the entity is a cluster
    pub cluster: Option<bool>,

    /// Cluster's unique identifier
    pub cluster_id: Option<usize>,

    // Number of points within a cluster
    pub point_count: Option<usize>,

    /// An abbreviated point count, useful for display
    pub point_count_abbreviated: Option<String>,
}

/// A GeoJSON Feature<Point>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    /// Feature type
    #[serde(rename = "type")]
    pub r#type: String,

    /// Feature ID
    pub id: Option<usize>,

    /// Feature metadata
    pub properties: Properties,

    /// Geometry of the feature
    pub geometry: Option<Geometry>,
}

pub fn load_places() -> Vec<Vec<f64>> {
    let file_path = Path::new("./fixtures/places.json");
    let json_string = fs::read_to_string(file_path).expect("places.json was not found");

    let features: Vec<Feature> =
        serde_json::from_str(&json_string).expect("places.json was not parsed");
    let coords: Vec<Vec<f64>> = features
        .iter()
        .flat_map(|feature| {
            feature
                .geometry
                .clone()
                .map(|geom| geom.coordinates.clone())
        })
        .collect();

    coords
}

#[test]
fn tmp() {
    let x = load_places();
    dbg!(&x);
}
