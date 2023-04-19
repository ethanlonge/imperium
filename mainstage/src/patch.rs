use plist::Value;
use crate::MsError;
use crate::MsError::{FileLoadError, MissingPlistValue, PlistError};
use crate::patch_control::PatchControl;

#[derive(Clone, Debug)]
pub enum Patch {
    Program {
        name: String,
        controls: Vec<PatchControl>
    },
    Bank {
        name: String,
        programs: Vec<Patch>,
        controls: Vec<PatchControl>
    }
}

impl Patch {
    pub fn load(name: String, ctx: &str) -> Result<Self, MsError> {
        let ctx = format!("{}/{}", &ctx, &name);

        let patch = Value::from_file(format!("{}/data.plist", &ctx)).map_err(|_e| FileLoadError(format!("{}/data.plist", &ctx)))?
            .into_dictionary().ok_or(PlistError(format!("{} file", &name)))?;

        let engine_node = patch.get("patch").ok_or(MissingPlistValue("patch".to_string(), format!("{} file", &name)))?
            .as_dictionary().ok_or(PlistError(format!("`{}` patch dictionary", &name)))?
            .get("engineNode").ok_or(MissingPlistValue("patch.engineNode".to_string(), format!("{} file", &name)))?
            .as_dictionary().ok_or(PlistError(format!("`{}` patch.engineNode dictionary", &name)))?;

        let patch_name = engine_node
            .get("name").ok_or(MissingPlistValue("patch.engineNode.name".to_string(), format!("{} file", &name)))?
            .as_string().ok_or(PlistError(format!("`{}` patch.engineNode.name dictionary", &name)))?;

        let controls = engine_node
            .get("uiPluginDataDict").ok_or(MissingPlistValue("patch.engineNode.uiPluginDataDict".to_string(), format!("{} file", &name)))?
            .as_dictionary().ok_or(PlistError(format!("`{}` patch.engineNode.uiPluginDataDict dictionary", &name)))?
            .iter().map(|(id, e)| {
                let mut label = None;

                let store = e.as_dictionary().ok_or(PlistError(format!("{} patch.engineNode.uiPluginDataDict.*", &name)))?
                    .get("storeDict").ok_or(MissingPlistValue("patch.engineNode.uiPluginDataDict.*.storeDict".to_string(), format!("{} file", &name)))?
                    .as_dictionary().ok_or(PlistError(format!("{} patch.engineNode.uiPluginDataDict.*.storeDict", &name)))?;

                if store.get("customLabel").is_some() {
                    label = Some(
                        store.get("customLabel").unwrap()
                            .as_string().ok_or(PlistError(format!("{} patch.engineNode.uiPluginDataDict.*.storeDict.customLabel", &name)))?
                            .to_string()
                    )
                }

                Ok(PatchControl {
                    name: id.clone(),
                    label,
                })
            }).collect::<Result<Vec<_>, MsError>>()?;

        let mut programs = None;

        if let Some(nodes) = patch.get("nodes") {
            let nodes = nodes.as_array();

            if let Some(nodes) = nodes {
                programs = Some(nodes.iter()
                    .map(|val| {
                        let name = val.as_string().ok_or(PlistError(format!("{} Node name", &name)))?;

                        Patch::load(name.to_owned(), &ctx)
                    }).collect::<Result<Vec<Patch>, MsError>>()?);
            }
        }

        Ok(if programs.is_some() {
            Patch::Bank {
                name: patch_name.to_string(),
                programs: programs.unwrap(),
                controls,
            }
        } else {
            Patch::Program {
                name: patch_name.to_string(),
                controls
            }
        })
    }
}