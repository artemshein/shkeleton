pub mod sync {
    #[cfg(all(feature = "concurrency", not(debug_assertions)))]
    pub use parking_lot::{RwLock as RwLockImpl, RwLockReadGuard, RwLockWriteGuard};
    #[cfg(any(not(feature = "concurrency"), debug_assertions))]
    pub use std::sync::{RwLock as RwLockImpl, RwLockReadGuard, RwLockWriteGuard};

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
}
