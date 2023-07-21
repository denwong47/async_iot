use super::super::ResultJson;
use super::base::ResultJsonEntry;

impl From<ResultJsonEntry> for ResultJson {
    /// If this [`ResultJsonEntry`] is a mapping, discard its key and map the children
    /// into root level.
    /// Otherwise, create a [`ResultJson`] with this [`ResultJsonEntry`] as the
    /// only entry.
    fn from(value: ResultJsonEntry) -> Self {
        Self::new().with_children(if let Some(children) = value.children {
            children.into_iter().collect()
        } else {
            vec![value]
        })
    }
}
