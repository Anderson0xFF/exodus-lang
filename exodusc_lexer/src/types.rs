#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Void,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Boolean,
    Char,
    String,
    Object(String),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "Any"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::Boolean => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::String => write!(f, "string"),
            Self::Object(typedef) => write!(f, "{}", typedef),
        }
    }
}

impl Type {
    pub fn is_any(&self) -> bool {
        return self == &Type::Void;
    }
}
