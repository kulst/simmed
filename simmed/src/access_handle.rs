use crate::marker::{Init, Uninit};
use crate::raw_handle::RawHandle;
use core::marker::PhantomData;

pub struct AccessHandle<T, Sh, L, I, M, C, S> {
    inner: RawHandle<T, Sh, L>,
    phantom: PhantomData<(I, M, C, S)>,
}
