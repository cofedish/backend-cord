#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

/// Result type with custom Error
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error information
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Error {
    /// Type of error and additional information
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub error_type: ErrorType,

    /// Where this error occurred
    pub location: String,
}

/// Possible error types
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[derive(Debug, Clone)]
pub enum ErrorType {
    /// This error was not labeled :(
    LabelMe,

    // ? Onboarding related errors
    AlreadyOnboarded,

    // ? User related errors
    UsernameTaken,
    InvalidUsername,
    UnknownUser,
    AlreadyFriends,
    AlreadySentRequest,
    Blocked,
    BlockedByOther,
    NotFriends,

    // ? Channel related errors
    UnknownChannel,
    UnknownAttachment,
    UnknownMessage,
    CannotEditMessage,
    CannotJoinCall,
    TooManyAttachments {
        max: usize,
    },
    TooManyReplies {
        max: usize,
    },
    TooManyChannels {
        max: usize,
    },
    EmptyMessage,
    PayloadTooLarge,
    CannotRemoveYourself,
    GroupTooLarge {
        max: usize,
    },
    AlreadyInGroup,
    NotInGroup,

    // ? Server related errors
    UnknownServer,
    InvalidRole,
    Banned,
    TooManyServers {
        max: usize,
    },
    TooManyEmoji {
        max: usize,
    },
    TooManyRoles {
        max: usize,
    },

    // ? Bot related errors
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,

    // ? User safety related errors
    CannotReportYourself,

    // ? Permission errors
    MissingPermission {
        permission: String,
    },
    MissingUserPermission {
        permission: String,
    },
    NotElevated,
    NotPrivileged,
    CannotGiveMissingPermissions,
    NotOwner,

    // ? General errors
    DatabaseError {
        operation: String,
        with: String,
    },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    InvalidProperty,
    InvalidSession,
    DuplicateNonce,
    NotFound,
    NoEffect,
    FailedValidation {
        error: String,
    },

    // ? Legacy errors
    VosoUnavailable,
}

#[macro_export]
macro_rules! create_error {
    ( $error:ident ) => {
        $crate::Error {
            error_type: $crate::ErrorType::$error,
            location: format!("{}:{}:{}", file!(), line!(), column!()),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::ErrorType;

    #[test]
    fn use_macro_to_construct_error() {
        let error = create_error!(LabelMe);
        assert!(matches!(error.error_type, ErrorType::LabelMe));
    }
}
