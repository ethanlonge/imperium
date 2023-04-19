#[derive(Clone, Debug)]
pub struct PatchControl {
    pub name: String,
    pub label: Option<String>
}

#[derive(Clone, Debug)]
pub struct PatchControls {
    pub knobs: [PatchControl; 8],
    pub faders: [PatchControl; 8]
}