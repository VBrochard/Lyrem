use serde::Serialize;

/// Dynamic Section tag understood by Lyrem.
///
/// Dynamic tags describe entries used by the dynamic linker. Lyrem currently
/// parses a focused subset of tags needed for linking metadata and RELRO
/// analysis, while preserving unknown values for diagnostics.
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

/// Entry from the ELF Dynamic Section.
///
/// Each entry contains a tag and an associated value or pointer. Lyrem uses
/// these entries to detect properties such as immediate symbol binding, which is
/// required to distinguish Partial RELRO from Full RELRO.
#[derive(Debug, PartialEq, Serialize)]
pub struct DynamicEntry {
    pub tag: DynamicTag,
    pub value: u64,
}
