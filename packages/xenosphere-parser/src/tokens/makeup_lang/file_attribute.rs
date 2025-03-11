use serde::{Deserialize, Serialize};

use super::AttributeSet;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum FileAttribute {
    // package name
    Package(FilePackageAttr),

    // use / include / improt
    Import(FileImportAttr),

    // export -> export function name
    Export(AttributeSet),

    Unknown(AttributeSet),
}
// -----------
// attribute list
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FilePackageAttr {
    pub name: String,
    pub path: String,
}
impl FilePackageAttr {
    pub fn from_attribute_set(attr: &AttributeSet) -> Self {
        Self {
            name: attr.name.to_owned(),
            path: attr.value.get("path").unwrap_or(&"".to_owned()).to_string(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileImportAttr {
    pub name: String,
    pub path: String,
    pub file_type: String,
}

impl FileImportAttr {
    pub fn from_attribute_set(attr: &AttributeSet) -> Self {
        let mut name: String = "".to_string();
        // println!("from_attribute_set : {:?}", attr);

        if let Some(tmp) = attr.value.get("name") {
            name = tmp.to_owned();
        } else if attr.value.len() == 1 {
            let tmp = attr.value.get("arg:1");
            if tmp.is_some_and(|f| f.is_empty() == false) {
                name = tmp.unwrap().to_string();
            }
        } else {
            name = attr.name.to_owned();
        }

        let path = if let Some(path) = attr.value.get("path") {
            path
        } else {
            ""
        };

        let file_type = if let Some(file_type) = attr.value.get("file_type") {
            file_type
        } else {
            "xesl_cpp"
        };

        Self {
            name: name,
            path: path.to_string(),
            file_type: file_type.to_string(),
            ..Self::default()
        }
    }
}
