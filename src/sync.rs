#[cfg(all(feature = "concurrency", not(debug_assertions)))]
pub use parking_lot::{RwLock as RwLockImpl, RwLockReadGuard, RwLockWriteGuard};
#[cfg(any(not(feature = "concurrency"), debug_assertions))]
pub use std::sync::{RwLock as RwLockImpl, RwLockReadGuard, RwLockWriteGuard};
use std::sync::{atomic::{AtomicPtr, Ordering}, Condvar, Mutex};
use std::{ptr, thread};

pub struct RwLock<T>(RwLockImpl<T>);

impl<T: Default> Default for RwLock<T> {
    fn default() -> Self {
        RwLock::new(Default::default())
    }
}

#[cfg(all(feature = "concurrency", not(debug_assertions)))]
impl<T> RwLock<T> {
    pub fn new(v: T) -> Self {
        RwLock(RwLockImpl::new(v))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.0.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.0.write()
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
        self.0.try_read()
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
        self.0.try_write()
    }
}

#[cfg(any(not(feature = "concurrency"), debug_assertions))]
impl<T> RwLock<T> {
    pub fn new(v: T) -> Self {
        RwLock(RwLockImpl::new(v))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner().ok().unwrap()
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.0.write().unwrap()
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
        self.0.try_read().ok()
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
        self.0.try_write().ok()
    }
}

pub trait AtomicHolder<T> {
    fn take(&self) -> Box<T>;

    fn set(&self, v: Box<T>);

    fn with<R>(&self, f: impl Fn(&mut T) -> R) -> R {
        let mut v = self.take();
        let res = f(v.as_mut());
        self.set(v);
        res
    }

    fn with_mut<R>(&self, mut f: impl FnMut(&mut T) -> R) -> R {
        let mut v = self.take();
        let res = f(v.as_mut());
        self.set(v);
        res
    }

    fn with_move<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut v = self.take();
        let res = f(v.as_mut());
        self.set(v);
        res
    }
}

pub struct AtomicPtrHolder<T>(AtomicPtr<T>, Condvar, Mutex<()>);

impl<T> AtomicPtrHolder<T> {
    pub fn empty() -> Self {
        AtomicPtrHolder(
            AtomicPtr::new(ptr::null_mut()),
            Condvar::new(),
            Mutex::new(()),
        )
    }

    pub fn is_null(&self) -> bool {
        self.0.load(Ordering::Acquire).is_null()
    }
}

impl<T> From<Box<T>> for AtomicPtrHolder<T> {
    fn from(v: Box<T>) -> Self {
        AtomicPtrHolder(
            AtomicPtr::new(Box::into_raw(v)),
            Condvar::new(),
            Mutex::new(()),
        )
    }
}

impl<T: Default> Default for AtomicPtrHolder<T> {
    fn default() -> Self {
        AtomicPtrHolder(
            AtomicPtr::new(Box::into_raw(Default::default())),
            Default::default(),
            Default::default(),
        )
    }
}

impl<T> AtomicHolder<T> for AtomicPtrHolder<T> {
    #[allow(unused_must_use)]
    fn take(&self) -> Box<T> {
        let mut counter = 0;
        loop {
            let v = self.0.swap(ptr::null_mut(), Ordering::Acquire);
            if !v.is_null() {
                return unsafe { Box::from_raw(v) };
            }
            if counter > 20 {
                let lock = self.2.lock().unwrap();
                self.1.wait(lock).unwrap();
                counter = 0;
            } else {
                counter += 1;
                thread::yield_now();
            }
        }
    }

    fn set(&self, v: Box<T>) {
        self.0.store(Box::into_raw(v), Ordering::Release);
        self.1.notify_one();
    }
}
