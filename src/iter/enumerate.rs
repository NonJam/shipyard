use super::{CurrentId, IntoIterator, Shiperator};

/// Shiperator yielding iteration count as well.
#[derive(Clone, Copy)]
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}

impl<I> Enumerate<I> {
    pub(super) fn new(iter: I) -> Self {
        Enumerate { iter, count: 0 }
    }
}

impl<I: Shiperator> Shiperator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn first_pass(&mut self) -> Option<Self::Item> {
        let item = self.iter.first_pass()?;
        let current = self.count;
        self.count += 1;
        Some((current, item))
    }
    fn post_process(&mut self) {
        self.iter.post_process()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I: CurrentId> CurrentId for Enumerate<I> {
    type Id = I::Id;

    unsafe fn current_id(&self) -> Self::Id {
        self.iter.current_id()
    }
}

impl<I: Shiperator> core::iter::IntoIterator for Enumerate<I> {
    type IntoIter = IntoIterator<Self>;
    type Item = <Self as Shiperator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator(self)
    }
}
