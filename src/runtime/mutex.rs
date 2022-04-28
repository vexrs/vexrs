// Super basic mutex implementation.
// This article was used as reference:
// https://mnwa.medium.com/building-a-stupid-mutex-in-the-rust-d55886538889

use core::{cell::UnsafeCell, ops::{Deref, DerefMut}};
use alloc::collections::VecDeque;
use super::{util::get_runtime, task::WakeSignal};


/// A basic mutex implementation
pub struct Mutex<T> {
    /// A boolean that determines if the lock is currently taken
    lock: bool,
    /// A queue of tasks waiting on the lock
    queue: VecDeque<usize>,
    /// The data the mutex is storing
    data: UnsafeCell<T>
}

impl<T> Mutex<T> {
    /// Creates a new mutex
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: false,
            queue: VecDeque::new(),
            data: UnsafeCell::new(data),
        }
    }

    /// Returns true if the lock is taken
    pub fn is_taken(&self) -> bool {
        self.lock
    }

    /// Acquires the lock on the mutex
    #[allow(clippy::while_immutable_condition)]
    pub fn acquire(&mut self) {
        // If the lock is not acquired and no-one is waiting on the queue, then take the lock
        if !self.lock && self.queue.is_empty() {
            self.lock = true;
            return;
        }

        // Add ourselves to the queue
        self.queue.push_front(get_runtime().current_task());

        // And go to sleep until we recieve the mutex unlocked signal, repeating for 
        // as long as the lock is taken
        while self.lock {
            get_runtime().await_wake(WakeSignal::MutexRelease);
        }

        // Once we are woken and the lock is not taken, set the lock and return
        self.lock = true;
    }

    /// Releases the lock on the mutex
    pub fn release(&mut self) {
        // Release the lock
        self.lock = false;

        // If the queue is empty, just return
        if self.queue.is_empty() {
            return;
        }

        // If not, get the next id and wake it
        let next = self.queue.pop_back().unwrap_or_else(|| get_runtime().current_task());
        get_runtime().wake(next, WakeSignal::MutexRelease);
    }

}


/// A guard smart pointer for the mutex
pub struct MutexGuard<'a, T> {
    mutex: &'a mut Mutex<T>
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    /// Derefs the contents of the mutex
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.mutex.data.get()
        }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {

    // Mutably dereferences the contents of the mutex
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut * self.mutex.data.get()
        }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    
    /// Releases the contained mutex when the guard is dropped
    fn drop(&mut self) {
        self.mutex.release()
    }
}


// Force send ans sync on the mutex.

unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}unsafe impl<T> Send for MutexGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for MutexGuard<'_, T> where T: Send + Sync {}