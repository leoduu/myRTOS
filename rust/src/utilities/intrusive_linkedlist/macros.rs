
#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(1 as *const $ty)).$field as *const _ as usize - 1
    }
}

#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $ty:ty, $field:ident) => ({
        let offset = &(*(1 as *const $ty)).$field as *const _ as usize - 1;
        &*((($ptr as *const _ as usize) - offset) as *const $ty)
    })
}

#[macro_export]
macro_rules! container_of_mut {
    ($ptr:expr, $ty:ty, $field:ident) => ({
        let offset = &(*(1 as *const $ty)).$field as *const _ as usize - 1;
        &mut *((($ptr as *const _ as usize) - offset) as *mut $ty)
    })
}
