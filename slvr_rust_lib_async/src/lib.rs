pub trait SendSync: Send + Sync {}
impl<T: Send + Sync> SendSync for T {}
pub trait RequireSendSync: SendSync {}
