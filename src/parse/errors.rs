#![allow(dead_code)]


use std::fmt::Debug;

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
    STATUS_PARAMETRE_TYPE
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        match self {
            ErrorCode::STATUS_SEGMENT_FAILED => 10000,
            ErrorCode::STATUS_FAILED_TYPING => 10001,
            ErrorCode::STATUS_TYPE_UNKNOWN => 10002,
            ErrorCode::STATUS_HEAP_CORRUPTION => 10003,
            ErrorCode::STATUS_CONSTRUCTOR_NOT_FOUND => 10004,
            ErrorCode::STATUS_SYNTAX_ERROR => 10005,
            ErrorCode::STATUS_FUNCTION_PROTO_ERROR => 10006,
            ErrorCode::STATUS_VARIABLE_ERROR => 10007,
            ErrorCode::STATUS_PARAMETRE_TYPE => 10008,
        }
    }
}
