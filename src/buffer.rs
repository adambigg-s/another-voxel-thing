use std::ops::{Index, IndexMut};

pub struct Buffer<T, const N: usize> {
    pub items: Box<[T]>,
    pub size: [usize; N],
}

impl<const N: usize, T> Buffer<T, N> {
    pub fn build(fill: T, size: [usize; N]) -> Self
    where
        T: Clone + Copy,
    {
        Self { items: vec![fill; size.iter().product()].into(), size }
    }

    pub fn fill(&mut self, fill: T)
    where
        T: Clone + Copy,
    {
        self.items.iter_mut().for_each(|item| *item = fill);
    }

    pub fn linearize(&self, indices: [usize; N]) -> usize {
        let mut index = 0;
        let mut stride = 1;
        (0..N).for_each(|idx| {
            index += indices[idx] * stride;
            stride *= self.size[idx];
        });
        index
    }

    #[allow(clippy::needless_range_loop)]
    pub fn surrounds(&self, indices: [usize; N]) -> bool {
        for i in 0..N {
            if indices[i] >= self.size[i] {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, T> Index<usize> for Buffer<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Buffer<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<const N: usize, T> Index<[usize; N]> for Buffer<T, N> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &Self::Output {
        &self.items[self.linearize(index)]
    }
}

impl<const N: usize, T> IndexMut<[usize; N]> for Buffer<T, N> {
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        &mut self.items[self.linearize(index)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_index() {
        let buffer = Buffer::build(0, [10, 10, 10]);
        assert!(buffer.linearize([0, 0, 1]) == 100);
        assert!(buffer.linearize([0, 1, 0]) == 10);
        assert!(buffer.linearize([1, 0, 0]) == 1);
    }
}
