use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct DbtColumn {
    pub name: String,
    pub data_type: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbtNode {
    pub name: String,
    pub resource_type: String,
    #[serde(default)]
    pub columns: HashMap<String, DbtColumn>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbtManifest {
    pub nodes: HashMap<String, DbtNode>,
    #[serde(default)]
    pub sources: HashMap<String, DbtNode>,
}

impl DbtManifest {
    pub fn load_from_file(path: &str) -> Result<Self, anyhow::Error> {
        let content = fs::read_to_string(path)?;
        let manifest: DbtManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dbt_manifest() {
        let json = r#"{
            "nodes": {
                "model.my_project.users": {
                    "name": "users",
                    "resource_type": "model",
                    "columns": {
                        "id": {
                            "name": "id",
                            "data_type": "integer",
                            "meta": { "pii": true }
                        },
                        "email": {
                            "name": "email",
                            "data_type": "varchar",
                            "meta": {}
                        }
                    },
                    "meta": { "partition_column": "created_at" }
                }
            },
            "sources": {}
        }"#;

        let manifest: DbtManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.nodes.len(), 1);
        let node = &manifest.nodes["model.my_project.users"];
        assert_eq!(node.name, "users");
        assert_eq!(node.resource_type, "model");
        assert_eq!(node.columns.len(), 2);
        assert_eq!(node.columns["id"].meta["pii"], true);
    }
}
