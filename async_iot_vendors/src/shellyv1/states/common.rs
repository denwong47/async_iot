use enum_index::*;

#[derive(Clone, Debug, EnumIndex, PartialEq)]
#[index_type(String)]
pub enum Turn {
    #[index("on")]
    On,
    #[index("off")]
    Off,
    #[index("toggle")]
    Toggle,
}
