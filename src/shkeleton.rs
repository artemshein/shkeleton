pub mod sync {
    #[cfg(feature = "concurrency")]
    pub use parking_lot::{RwLock as RwLockImpl, RwLockReadGuard, RwLockWriteGuard};
    #[cfg(not(feature = "concurrency"))]
    pub use std::sync::{RwLock as RwLockImpl, RwLockReadGuard, RwLockWriteGuard};

    pub struct RwLock<T>(RwLockImpl<T>);

    impl<T: Default> Default for RwLock<T> {
        fn default() -> Self {
            RwLock::new(Default::default())
        }
    }

    impl<T> RwLock<T> {
        pub fn new(v: T) -> Self {
            RwLock(RwLockImpl::new(v))
        }

        #[cfg(feature = "concurrency")]
        pub fn read(&self) -> RwLockReadGuard<T> {
            self.0.read()
        }

        #[cfg(not(feature = "concurrency"))]
        pub fn read(&self) -> RwLockReadGuard<T> {
            self.0.read().unwrap()
        }

        #[cfg(feature = "concurrency")]
        pub fn write(&self) -> RwLockWriteGuard<T> {
            self.0.write()
        }

        #[cfg(not(feature = "concurrency"))]
        pub fn write(&self) -> RwLockWriteGuard<T> {
            self.0.write().unwrap()
        }

        #[cfg(feature = "concurrency")]
        pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
            self.0.try_read()
        }

        #[cfg(not(feature = "concurrency"))]
        pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
            self.0.try_read().ok()
        }

        #[cfg(feature = "concurrency")]
        pub fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
            self.0.try_write()
        }

        #[cfg(not(feature = "concurrency"))]
        pub fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
            self.0.try_write().ok()
        }
    }
}
