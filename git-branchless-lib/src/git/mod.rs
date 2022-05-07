//! Tools for interfacing with the Git repository.

mod config;
mod index;
mod oid;
mod repo;
mod run;
mod snapshot;
mod status;
mod tree;

pub use config::{Config, ConfigRead, ConfigValue, ConfigWrite};
pub use index::{update_index, Index, IndexEntry, Stage, UpdateIndexCommand};
pub use oid::{MaybeZeroOid, NonZeroOid};
pub use repo::{
    message_prettify, AmendFastOptions, Branch, BranchType, CategorizedReferenceName,
    CherryPickFastError, CherryPickFastOptions, Commit, Diff, GitVersion, PatchId, Reference,
    ReferenceTarget, Repo, ResolvedReferenceInfo, Time,
};
pub use run::{GitRunInfo, GitRunOpts, GitRunResult};
pub use snapshot::WorkingCopySnapshot;
pub use status::{FileMode, FileStatus, StatusEntry};
pub use tree::Tree;
