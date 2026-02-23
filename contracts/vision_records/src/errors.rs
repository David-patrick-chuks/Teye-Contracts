#![allow(clippy::arithmetic_side_effects)]
use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Symbol, Vec};

pub const ERROR_LOG_KEY: Symbol = symbol_short!("ERR_LOG");
pub const ERROR_COUNT_KEY: Symbol = symbol_short!("ERR_CNT");
pub const MAX_ERROR_LOG_SIZE: u32 = 100;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ErrorCategory {
    Validation = 1,
    Authorization = 2,
    NotFound = 3,
    StateConflict = 4,
    Storage = 5,
    Transient = 6,
    System = 7,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ErrorSeverity {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ErrorContext {
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub message: String,
    pub user: Option<Address>,
    pub resource_id: Option<String>,
    pub timestamp: u64,
    pub retryable: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ErrorLogEntry {
    pub error_code: u32,
    pub context: ErrorContext,
    pub stack_trace: Option<String>,
}

#[soroban_sdk::contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    UserNotFound = 4,
    RecordNotFound = 5,
    InvalidInput = 6,
    AccessDenied = 7,
    Paused = 8,
    ProviderNotFound = 9,
    ProviderAlreadyRegistered = 10,
    InvalidVerificationStatus = 11,
    InvalidAddress = 12,
    InvalidTimestamp = 13,
    StorageError = 14,
    RateLimitExceeded = 15,
    ExpiredAccess = 16,
    InvalidRole = 17,
    InvalidPermission = 18,
    DelegationExpired = 19,
    InvalidDataHash = 20,
    DuplicateRecord = 21,
    InvalidRecordType = 22,
    ContractPaused = 23,
    InsufficientPermissions = 24,
    TransientFailure = 25,
}

impl ContractError {
    pub fn category(&self) -> ErrorCategory {
        match self {
            ContractError::NotInitialized
            | ContractError::AlreadyInitialized
            | ContractError::InvalidInput
            | ContractError::InvalidAddress
            | ContractError::InvalidTimestamp
            | ContractError::InvalidRole
            | ContractError::InvalidPermission
            | ContractError::InvalidDataHash
            | ContractError::InvalidRecordType
            | ContractError::InvalidVerificationStatus => ErrorCategory::Validation,
            ContractError::Unauthorized
            | ContractError::AccessDenied
            | ContractError::InsufficientPermissions
            | ContractError::ExpiredAccess => ErrorCategory::Authorization,
            ContractError::UserNotFound
            | ContractError::RecordNotFound
            | ContractError::ProviderNotFound => ErrorCategory::NotFound,
            ContractError::ProviderAlreadyRegistered
            | ContractError::DuplicateRecord
            | ContractError::DelegationExpired => ErrorCategory::StateConflict,
            ContractError::StorageError => ErrorCategory::Storage,
            ContractError::TransientFailure | ContractError::RateLimitExceeded => {
                ErrorCategory::Transient
            }
            ContractError::Paused | ContractError::ContractPaused => ErrorCategory::System,
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ContractError::NotInitialized
            | ContractError::AlreadyInitialized
            | ContractError::InvalidInput
            | ContractError::InvalidAddress
            | ContractError::InvalidTimestamp
            | ContractError::InvalidRole
            | ContractError::InvalidPermission
            | ContractError::InvalidDataHash
            | ContractError::InvalidRecordType
            | ContractError::InvalidVerificationStatus
            | ContractError::UserNotFound
            | ContractError::RecordNotFound
            | ContractError::ProviderNotFound
            | ContractError::DuplicateRecord => ErrorSeverity::Low,
            ContractError::Unauthorized
            | ContractError::AccessDenied
            | ContractError::InsufficientPermissions
            | ContractError::ExpiredAccess
            | ContractError::ProviderAlreadyRegistered
            | ContractError::DelegationExpired
            | ContractError::RateLimitExceeded => ErrorSeverity::Medium,
            ContractError::StorageError | ContractError::TransientFailure => ErrorSeverity::High,
            ContractError::Paused | ContractError::ContractPaused => ErrorSeverity::Critical,
        }
    }

    pub fn retryable(&self) -> bool {
        matches!(
            self,
            ContractError::TransientFailure
                | ContractError::RateLimitExceeded
                | ContractError::StorageError
        )
    }

    pub fn message(&self) -> &'static str {
        match self {
            ContractError::NotInitialized => "Contract has not been initialized",
            ContractError::AlreadyInitialized => "Contract is already initialized",
            ContractError::Unauthorized => "Caller is not authorized for this operation",
            ContractError::UserNotFound => "User not found in the system",
            ContractError::RecordNotFound => "Record not found",
            ContractError::InvalidInput => "Invalid input parameters provided",
            ContractError::AccessDenied => "Access denied to the requested resource",
            ContractError::Paused => "Contract operations are currently paused",
            ContractError::ProviderNotFound => "Provider not found in the system",
            ContractError::ProviderAlreadyRegistered => "Provider is already registered",
            ContractError::InvalidVerificationStatus => "Invalid verification status provided",
            ContractError::InvalidAddress => "Invalid address format",
            ContractError::InvalidTimestamp => "Invalid timestamp value",
            ContractError::StorageError => "Storage operation failed",
            ContractError::RateLimitExceeded => "Rate limit exceeded, please retry later",
            ContractError::ExpiredAccess => "Access grant has expired",
            ContractError::InvalidRole => "Invalid role specified",
            ContractError::InvalidPermission => "Invalid permission specified",
            ContractError::DelegationExpired => "Role delegation has expired",
            ContractError::InvalidDataHash => "Invalid data hash format",
            ContractError::DuplicateRecord => "Record with this ID already exists",
            ContractError::InvalidRecordType => "Invalid record type specified",
            ContractError::ContractPaused => "Contract is paused",
            ContractError::InsufficientPermissions => "Insufficient permissions for operation",
            ContractError::TransientFailure => "Transient failure, operation may succeed on retry",
        }
    }
}

pub fn log_error(
    env: &Env,
    error: ContractError,
    user: Option<Address>,
    resource_id: Option<String>,
    stack_trace: Option<String>,
) {
    let context = ErrorContext {
        category: error.category(),
        severity: error.severity(),
        message: String::from_str(env, error.message()),
        user,
        resource_id,
        timestamp: env.ledger().timestamp(),
        retryable: error.retryable(),
    };

    let log_entry = ErrorLogEntry {
        error_code: error as u32,
        context,
        stack_trace,
    };

    let mut error_log: Vec<ErrorLogEntry> = env
        .storage()
        .instance()
        .get(&ERROR_LOG_KEY)
        .unwrap_or(Vec::new(env));

    error_log.push_back(log_entry);

    if error_log.len() > MAX_ERROR_LOG_SIZE {
        let mut new_log = Vec::new(env);
        for i in 1..error_log.len() {
            if let Some(entry) = error_log.get(i) {
                new_log.push_back(entry);
            }
        }
        error_log = new_log;
    }

    env.storage().instance().set(&ERROR_LOG_KEY, &error_log);

    let error_count: u64 = env.storage().instance().get(&ERROR_COUNT_KEY).unwrap_or(0);
    env.storage()
        .instance()
        .set(&ERROR_COUNT_KEY, &(error_count + 1));
}

pub fn get_error_log(env: &Env) -> Vec<ErrorLogEntry> {
    env.storage()
        .instance()
        .get(&ERROR_LOG_KEY)
        .unwrap_or(Vec::new(env))
}

pub fn get_error_count(env: &Env) -> u64 {
    env.storage().instance().get(&ERROR_COUNT_KEY).unwrap_or(0)
}

pub fn clear_error_log(env: &Env) {
    env.storage().instance().remove(&ERROR_LOG_KEY);
    env.storage().instance().set(&ERROR_COUNT_KEY, &0u64);
}

pub fn create_error_context(
    env: &Env,
    error: ContractError,
    user: Option<Address>,
    resource_id: Option<String>,
) -> ErrorContext {
    ErrorContext {
        category: error.category(),
        severity: error.severity(),
        message: String::from_str(env, error.message()),
        user,
        resource_id,
        timestamp: env.ledger().timestamp(),
        retryable: error.retryable(),
    }
}

const RETRY_COUNT_KEY: Symbol = symbol_short!("RETRY_CNT");

pub fn retry_operation(env: &Env, caller: &Address, operation: &String, max_retries: u32) -> bool {
    let key = (RETRY_COUNT_KEY, caller.clone(), operation.clone());
    let count: u32 = env.storage().instance().get(&key).unwrap_or(0);

    if count >= max_retries {
        return false;
    }

    env.storage().instance().set(&key, &(count + 1));
    true
}

pub fn reset_retry_count(env: &Env, caller: &Address, operation: &String) {
    let key = (RETRY_COUNT_KEY, caller.clone(), operation.clone());
    env.storage().instance().remove(&key);
}
