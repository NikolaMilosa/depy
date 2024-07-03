#[derive(Debug)]
pub struct Target {
    pub name: String,
    pub kind: TargetKind,
}

#[derive(Debug)]
pub enum TargetKind {
    Binary,
    Library,
}
