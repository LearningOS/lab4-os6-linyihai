mod stdio;
mod inode;

use crate::mm::UserBuffer;

/// The common abstraction of all IO resources
pub trait File : Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn fstat(&self) -> (u64, StatMode, usize) { panic!("not implemented") }
}

/// The stat of a inode
#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    /// ID of device containing file
    pub dev: u64,
    /// inode number
    pub ino: u64,
    /// file type and mode
    pub mode: StatMode,
    /// number of hard links
    pub nlink: u32,
    /// unused pad
    pad: [u64; 7],
}

bitflags! {
    /// The mode of a inode
    /// whether a directory or a file
    pub struct StatMode: u32 {
        const NULL  = 0;
        /// directory
        const DIR   = 0o040000;
        /// ordinary regular file
        const FILE  = 0o100000;
    }
}

pub use stdio::{Stdin, Stdout};
pub use inode::{OSInode, open_file, OpenFlags, list_apps};


pub fn linkat(old_path: &str, new_path: &str) -> isize {
    if let Some(inode) = inode::ROOT_INODE.find(old_path) {
        let inode_id = inode.find_inode_id_by_inode() as u32;
        inode::ROOT_INODE.create_link(inode_id, new_path)
    } else {
        -1
    }
}

pub fn unlinkat(name: &str) -> isize {
    inode::ROOT_INODE.remove(name)
}