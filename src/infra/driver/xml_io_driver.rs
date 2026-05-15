use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Project")]
pub struct XmlProject {
    #[serde(rename = "Files")]
    pub files: XmlFiles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XmlFiles {
    #[serde(rename = "$value", default)]
    pub items: Vec<XmlItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum XmlItem {
    #[serde(rename = "File")]
    File {
        #[serde(rename = "@RelativePath")]
        relative_path: String,
    },
    #[serde(rename = "Filter")]
    Filter {
        #[serde(rename = "@Name")]
        name: String,
        #[serde(rename = "$value", default)]
        children: Vec<XmlItem>,
    },
}

pub struct XmlIoDriver;

impl Default for XmlIoDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl XmlIoDriver {
    pub fn new() -> Self {
        Self
    }

    pub fn write_project(&self, path: &PathBuf, project: &XmlProject) -> Result<(), String> {
        let xml_body = to_string(project).map_err(|e| e.to_string())?;
        let xml = format!(
            "<?xml version=\"1.0\"?>\r\n<!--EmEditor Project file-->\r\n{}",
            xml_body
        );
        fs::write(path, xml).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_project(&self, path: &PathBuf) -> Result<XmlProject, String> {
        let xml = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let project: XmlProject = from_str(&xml).map_err(|e| e.to_string())?;
        Ok(project)
    }
}
