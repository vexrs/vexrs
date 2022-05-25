// Super basic mutex implementation.
// This article was used as reference:
// https://mnwa.medium.com/building-a-stupid-mutex-in-the-rust-d55886538889

use core::{cell::{UnsafeCell, RefCell}, ops::{Deref, DerefMut}};
use alloc::collections::VecDeque;
use crate::runtime::thread::WakeupSignal;


/// A basic mutex implementation
pub struct Mutex<T> {
    /// A boolean that determines if the lock is currently taken
    lock: RefCell<bool>,
    /// A queue of tasks waiting on the lock
    queue: RefCell<VecDeque<usize>>,
    /// The data the mutex is storing
    data: UnsafeCell<T>
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self {
            lock: RefCell::new(false),
            queue: RefCell::new(VecDeque::new()),
            data: UnsafeCell::new(T::default())
        }
    }
}

impl<T> Mutex<T> {
    /// Creates a new mutex
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: RefCell::new(false),
            queue: RefCell::new(VecDeque::new()),
            data: UnsafeCell::new(data),
        }
    }

    /// Returns true if the lock is taken
    pub fn is_taken(&self) -> bool {
        *self.lock.borrow()
    }

    /// Acquires the lock on the mutex
    #[allow(clippy::while_immutable_condition)]
    pub fn acquire(&self) -> MutexGuard<'_, T>{
        // If the lock is not acquired and no-one is waiting on the queue, then take the lock
        if !*self.lock.borrow() && self.queue.borrow().is_empty() {
            *self.lock.borrow_mut() = true;

            return MutexGuard { mutex: self };
        }

        // Add ourselves to the queue
        self.queue.borrow_mut().push_front(crate::RUNTIME.current_task());

        // And go to sleep until we recieve the mutex unlocked signal, repeating for 
        // as long as the lock is taken
        while *self.lock.borrow() {
            crate::RUNTIME.await_wake(WakeupSignal::MutexRelease);
        }

        // Once we are woken and the lock is not taken, set the lock
        *self.lock.borrow_mut() = true;

        // Return the mutex guard
        MutexGuard { mutex: self }
    }

    /// Releases the lock on the mutex
    pub fn release(&self) {
        // Release the lock
        *self.lock.borrow_mut() = false;

        // If the queue is empty, just return
        if self.queue.borrow().is_empty() {
            return;
        }

        // If not, get the next id and wake it
        let next = self.queue.borrow_mut().pop_back();

        // If there is another task then wake it
        if let Some(next) = next {
            crate::RUNTIME.wake(next, WakeupSignal::MutexRelease);
        }

    }

}


/// A guard smart pointer for the mutex
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>
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