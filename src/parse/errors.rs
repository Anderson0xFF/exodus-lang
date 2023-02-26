#[derive(Debug)]
#[allow(non_camel_case_types)]

pub enum ErrorCode {
    STATUS_SEGMENT_FAILED,
    STATUS_FAILED_TYPING,
    STATUS_TYPE_UNKNOWN,
    STATUS_HEAP_CORRUPTION,
    STATUS_CONSTRUCTOR_NOT_FOUND,
    STATUS_SYNTAX_ERROR,
    STATUS_FUNCTION_PROTO_ERROR,
    STATUS_VARIABLE_ERROR,
    STATUS_PARAMETRE_TYPE,
    STATUS_NOT_DECLARED_VARIABLE,
    STATUS_MISMATCHED_TYPES,
    STATUS_MISSING_VALUE,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        match self {
            ErrorCode::STATUS_SEGMENT_FAILED => 1000,
            ErrorCode::STATUS_FAILED_TYPING => 1001,
            ErrorCode::STATUS_TYPE_UNKNOWN => 1002,
            ErrorCode::STATUS_HEAP_CORRUPTION => 1003,
            ErrorCode::STATUS_CONSTRUCTOR_NOT_FOUND => 1004,
            ErrorCode::STATUS_SYNTAX_ERROR => 1005,
            ErrorCode::STATUS_FUNCTION_PROTO_ERROR => 1006,
            ErrorCode::STATUS_VARIABLE_ERROR => 1007,
            ErrorCode::STATUS_PARAMETRE_TYPE => 1008,
            ErrorCode::STATUS_NOT_DECLARED_VARIABLE => 1009,
            ErrorCode::STATUS_MISMATCHED_TYPES => 1010,
            ErrorCode::STATUS_MISSING_VALUE => 1011,
        }
    }
}
