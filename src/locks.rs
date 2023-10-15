use core::cell::UnsafeCell;
use core::mem::ManuallyDrop;
use core::sync::atomic::{AtomicU32, Ordering, AtomicBool};
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum SpinlockState {
    Unlocked,
    Locked,
}

struct Spinlock {
    state: AtomicU32, 
}

impl Spinlock {
    const fn new() -> Self {
        Self { state: AtomicU32::new(SpinlockState::Unlocked as u32) } 
    }

    #[inline]
    fn lock(&self) {
        while !self.try_lock() {
            while self.state.load(Ordering::Relaxed) == SpinlockState::Locked as u32 { 
                // Back off
            }
        }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        self.state.swap(SpinlockState::Locked as u32, Ordering::Acquire) == SpinlockState::Unlocked as u32
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.state.store(SpinlockState::Unlocked as u32, Ordering::Release)
    }
}

pub struct Mutex<T: ?Sized> {
    inner: Spinlock,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
}

impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<'mutex, T: ?Sized> MutexGuard<'mutex, T> {
    fn new(lock: &'mutex Mutex<T>) -> MutexGuard<'mutex, T> {
        MutexGuard { lock }
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.lock.inner.unlock();
        }
    }
}

impl<T> Mutex<T> {
    pub const fn new(t: T) -> Self {
        Self {
            inner: Spinlock::new(),
            data: UnsafeCell::new(t),
        }
    }   
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock();
        MutexGuard::new(self)
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.inner.try_lock() {
            Some(MutexGuard::new(self))
        } else {
            None
        }
    }

    pub fn unlock(guard: MutexGuard<'_, T>) {
        drop(guard);
    }
}

union Data<T, F> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

pub struct Lazy<T, F = fn() -> T> {
    initialized: AtomicBool,
    data: UnsafeCell<Data<T, F>>,
}

/// Must only be used inside Mutex: Mutex<Lazy<T>>
/// Initializing function must not panic
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub const unsafe fn new(f: F) -> Self {
        Self {
            initialized: AtomicBool::new(false),
            data: UnsafeCell::new(Data { f: ManuallyDrop::new(f) }),
        }
    }

    #[cold]
    fn init(&self) {
        let data = unsafe { &mut *self.data.get() };
        let f = unsafe { ManuallyDrop::take(&mut data.f) };
        data.value = ManuallyDrop::new(f());
    }

    #[inline]
    fn get(&self) -> &T {
        if !self.initialized.load(Ordering::Relaxed) {
            self.init();
            self.initialized.store(true, Ordering::Relaxed);
        }

        unsafe { &*(*self.data.get()).value }
    }
    
    #[inline]
    fn get_mut(&mut self) -> &mut T {
        if !self.initialized.load(Ordering::Relaxed) {
            self.init();
            self.initialized.store(true, Ordering::Relaxed);
        }

        unsafe { &mut self.data.get_mut().value }
    }
}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()       
    }
}

impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T, F> Drop for Lazy<T, F> {
    fn drop(&mut self) {
        if self.initialized.load(Ordering::Relaxed) {
            unsafe { ManuallyDrop::drop(&mut self.data.get_mut().value); }
        } else {
            unsafe { ManuallyDrop::drop(&mut self.data.get_mut().f); }
        }
    }
}

