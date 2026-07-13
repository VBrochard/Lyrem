use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum DynamicTag {
    Null,
    Needed,
    StrTab,
    StrSz,
    BindNow,
    Flags,
    Flags1,
    RPath,
    RunPath,
    Soname,
    Unknown(i64),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DynamicEntry {
    pub tag: DynamicTag,
    pub value: u64,
}
