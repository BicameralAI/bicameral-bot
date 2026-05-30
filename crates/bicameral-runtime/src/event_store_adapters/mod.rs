//! Event-store adapter implementations.
//!
//! Both git-backed and drive-folder substrates are in architectural scope.
//! The interface is substrate-neutral per plan section 7.1.

pub mod drive_folder;
pub mod git;
pub mod memory;

pub use drive_folder::DriveFolderAdapter;
pub use git::GitAdapter;
pub use memory::MemoryAdapter;
