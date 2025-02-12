// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use typespec_client_core::{create_enum, create_extensible_enum};

create_extensible_enum!(
    #[doc = r#"/// Reflects the deletion recovery level currently in effect for secrets in the current vault. If it contains 'Purgeable',
/// the secret can be permanently deleted by a privileged user; otherwise, only the system can purge the secret, at the end
/// of the retention interval."#]
    DeletionRecoveryLevel,
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable without the possibility for immediate and permanent deletion (i.e.
/// purge when 7 <= SoftDeleteRetentionInDays < 90).This level guarantees the recoverability of the deleted entity during
/// the retention interval and while the subscription is still available."#]
    (CustomizedRecoverable, "CustomizedRecoverable"),
    #[doc = r#"/// Denotes a vault and subscription state in which deletion is recoverable, immediate and permanent deletion (i.e. purge)
/// is not permitted, and in which the subscription itself cannot be permanently canceled when 7 <= SoftDeleteRetentionInDays
/// < 90. This level guarantees the recoverability of the deleted entity during the retention interval, and also reflects
/// the fact that the subscription itself cannot be cancelled."#]
    (
        CustomizedRecoverableProtectedSubscription,
        "CustomizedRecoverable+ProtectedSubscription"
    ),
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable, and which also permits immediate and permanent deletion (i.e.
/// purge when 7 <= SoftDeleteRetentionInDays < 90). This level guarantees the recoverability of the deleted entity during
/// the retention interval, unless a Purge operation is requested, or the subscription is cancelled."#]
    (
        CustomizedRecoverablePurgeable,
        "CustomizedRecoverable+Purgeable"
    ),
    #[doc = r#"/// Denotes a vault state in which deletion is an irreversible operation, without the possibility for recovery. This level
/// corresponds to no protection being available against a Delete operation; the data is irretrievably lost upon accepting
/// a Delete operation at the entity level or higher (vault, resource group, subscription etc.)"#]
    (Purgeable, "Purgeable"),
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable without the possibility for immediate and permanent deletion (i.e.
/// purge). This level guarantees the recoverability of the deleted entity during the retention interval (90 days) and while
/// the subscription is still available. System wil permanently delete it after 90 days, if not recovered"#]
    (Recoverable, "Recoverable"),
    #[doc = r#"/// Denotes a vault and subscription state in which deletion is recoverable within retention interval (90 days), immediate
/// and permanent deletion (i.e. purge) is not permitted, and in which the subscription itself cannot be permanently canceled.
/// System wil permanently delete it after 90 days, if not recovered"#]
    (
        RecoverableProtectedSubscription,
        "Recoverable+ProtectedSubscription"
    ),
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable, and which also permits immediate and permanent deletion (i.e.
/// purge). This level guarantees the recoverability of the deleted entity during the retention interval (90 days), unless
/// a Purge operation is requested, or the subscription is cancelled. System wil permanently delete it after 90 days, if not
/// recovered"#]
    (RecoverablePurgeable, "Recoverable+Purgeable")
);
