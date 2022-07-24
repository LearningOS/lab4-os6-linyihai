//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.


use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use lazy_static::*;
pub use crate::timer::get_time;

pub struct TaskManager {
    ready_queue: alloc::vec::Vec<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: alloc::vec::Vec::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let task = None;
        if self.ready_queue.is_empty() {
            return task;
        }
        let mut i: usize  = get_time() % self.ready_queue.len();
        for _i in 0..self.ready_queue.len() {
            let task = &self.ready_queue[_i];
            let inner = task.inner_exclusive_access();
            let _pass = inner.pass;
            drop(task);
            drop(inner);
            let pass = self.ready_queue[i].inner_exclusive_access().pass;
            if (pass - _pass) as i8 > 0 {
                i = _i;
            }           
        }
        let task = self.ready_queue.remove(i);
        let mut stride = u8::MAX / task.inner_exclusive_access().prio;
        if stride == 0 {
            stride = 1;
        }
        task.inner_exclusive_access().pass += stride as usize;
        Some(task)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
