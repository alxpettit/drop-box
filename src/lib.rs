use std::ops::{Deref, DerefMut};

pub struct DropBox<T, F>
where
    F: Fn(&mut T),
{
    item: T,
    f: F,
}

impl<T, F> DropBox<T, F>
where
    F: Fn(&mut T),
{
    pub fn new(item: T, f: F) -> Self
where {
        Self { item, f }
    }
}

impl<T, F> Deref for DropBox<T, F>
where
    F: Fn(&mut T),
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T, F> DerefMut for DropBox<T, F>
where
    F: Fn(&mut T),
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<T, F> Drop for DropBox<T, F>
where
    F: Fn(&mut T),
{
    fn drop(&mut self) {
        (self.f)(&mut self.item);
    }
}
