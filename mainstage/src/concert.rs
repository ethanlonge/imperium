use plist::Value;
use crate::{MsError, Patch};
use crate::MsError::{MissingPlistValue, PlistError};

#[derive(Clone, Debug)]
pub struct Concert {
    pub name: String,
    pub banks: Vec<Patch>
}

impl Concert {
    pub fn from_file(path: &str) -> Result<Self, MsError> {
        let ctx = format!("{}/Concert.patch", &path);

        let concert = Value::from_file(format!("{}/data.plist", &ctx)).map_err(|_e| MsError::FileLoadError(format!("{}/data.plist", &ctx)))?
            .into_dictionary().ok_or(PlistError("Concert file".to_string()))?;

        let engine_node = concert.get("patch").ok_or(MissingPlistValue("patch".to_string(), "concert file".to_string()))?
            .as_dictionary().ok_or(PlistError("concert patch dictionary".to_string()))?
            .get("engineNode").ok_or(MissingPlistValue("patch.engineNode".to_string(), "concert file".to_string()))?
            .as_dictionary().ok_or(PlistError("concert patch.engineNode dictionary".to_string()))?;

        let name = engine_node
            .get("name").ok_or(MissingPlistValue("patch.engineNode.name".to_string(), "concert file".to_string()))?
            .as_string().ok_or(PlistError("concert patch.engineNode.name dictionary".to_string()))?
            .to_string();

        let banks = concert.get("nodes").ok_or(MissingPlistValue("nodes".to_string(), "concert file".to_string()))?;

        let banks = banks.as_array().ok_or(PlistError("Banks Array".to_string()))?;

        let banks = banks.iter()
            .map(|val| {
                let name = val.as_string().ok_or(PlistError("concert Node name".to_string()))?;

                Patch::load(name.to_owned(), &ctx)
            }).collect::<Result<Vec<Patch>, MsError>>()?;

        Ok(Concert {
            name,
            banks
        })
    }
}