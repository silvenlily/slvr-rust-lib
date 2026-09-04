
pub mod lcommon {}
pub mod prelude {
    pub use crate::AssertThreadsafe;
    pub use crate::OnceClone;
}
pub mod lib {}

use std::ops::Deref;
use std::sync::OnceLock;
use thread_local::ThreadLocal;

pub struct OnceClone<T: Clone + Send + Sync> {
    inner_ref: OnceLock<T>,
    local: ThreadLocal<T>,
}

impl<T: Clone + Send + Sync> Default for OnceClone<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + Sync> OnceClone<T> {
    pub const fn new() -> OnceClone<T> {
        Self {
            inner_ref: OnceLock::new(),
            local: ThreadLocal::new(),
        }
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        self.inner_ref.set(value)
    }

    pub fn get(&self) -> Option<&T> {
        self.inner_ref.get()
    }

    pub fn wait(&self) -> &T {
        self.inner_ref.wait()
    }

}

impl<T: Clone + Send + Sync> Deref for OnceClone<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.local.get_or(|| {
            self.inner_ref
                .get()
                .expect("Attempted to deref OnceClone before setting its value")
                .clone()
        })
    }
}

pub trait AssertThreadsafe: Send + Sync {}

impl<T: Clone + Send + Sync> AssertThreadsafe for OnceClone<T> {}
