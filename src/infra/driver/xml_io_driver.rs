use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "EmEditorProject")]
pub struct XmlProject {
    #[serde(rename = "$value", default)]
    pub items: Vec<XmlItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum XmlItem {
    #[serde(rename = "File")]
    File {
        #[serde(rename = "@Name")]
        name: String,
        #[serde(rename = "@RelativePath")]
        relative_path: String,
    },
    #[serde(rename = "Folder")]
    Folder {
        #[serde(rename = "@Name")]
        name: String,
        #[serde(rename = "$value", default)]
        children: Vec<XmlItem>,
    },
}

pub struct XmlIoDriver;

impl XmlIoDriver {
    pub fn new() -> Self {
        Self
    }

    pub fn write_project(&self, path: &PathBuf, project: &XmlProject) -> Result<(), String> {
        let xml = to_string(project).map_err(|e| e.to_string())?;
        fs::write(path, xml).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_project(&self, path: &PathBuf) -> Result<XmlProject, String> {
        let xml = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let project: XmlProject = from_str(&xml).map_err(|e| e.to_string())?;
        Ok(project)
    }

    pub fn scan_directory(&self, root_path: &PathBuf) -> Result<Vec<(PathBuf, bool)>, String> {
        let mut results = Vec::new();
        self.scan_recursive(root_path, &mut results)?;
        Ok(results)
    }

    fn scan_recursive(
        &self,
        current_path: &PathBuf,
        results: &mut Vec<(PathBuf, bool)>,
    ) -> Result<(), String> {
        let entries = fs::read_dir(current_path).map_err(|e| e.to_string())?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let is_dir = path.is_dir();
            results.push((path.clone(), is_dir));
            if is_dir {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if file_name != ".git" && file_name != "target" && file_name != "node_modules" {
                    self.scan_recursive(&path, results)?;
                }
            }
        }
        Ok(())
    }
}
