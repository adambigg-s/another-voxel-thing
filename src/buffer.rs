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

    pub fn try_set(&mut self, value: T, indices: [usize; N]) -> Result<(), &'static str> {
        if !self.surrounds(indices) {
            return Err("out of bounds");
        }

        let idx = self.linearize(indices);
        self[idx] = value;
        Ok(())
    }

    #[inline(always)]
    pub fn linearize(&self, indices: [usize; N]) -> usize {
        debug_assert!(self.surrounds(indices));
        let mut index = 0;
        let mut stride = 1;
        (0..N).for_each(|idx| {
            index += indices[idx] * stride;
            stride *= self.size[idx];
        });
        index
    }

    #[inline(always)]
    pub fn delinearize(&self, index: usize) -> [usize; N] {
        debug_assert!(index < self.size.iter().product());
        let mut out = [0; N];
        let mut stride = 1;
        (0..N).for_each(|idx| {
            out[idx] = (index / stride) % self.size[idx];
            stride *= self.size[idx];
        });
        out
    }

    #[allow(clippy::needless_range_loop)]
    #[inline(always)]
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

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Buffer<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<const N: usize, T> Index<[usize; N]> for Buffer<T, N> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: [usize; N]) -> &Self::Output {
        &self.items[self.linearize(index)]
    }
}

impl<const N: usize, T> IndexMut<[usize; N]> for Buffer<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        &mut self.items[self.linearize(index)]
    }
}

impl<T> Index<glam::USizeVec3> for Buffer<T, 3> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: glam::USizeVec3) -> &Self::Output {
        &self[index.to_array()]
    }
}

impl<T> IndexMut<glam::USizeVec3> for Buffer<T, 3> {
    #[inline(always)]
    fn index_mut(&mut self, index: glam::USizeVec3) -> &mut Self::Output {
        &mut self[index.to_array()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_linearize() {
        let buffer = Buffer::build(0, [10, 10, 10]);
        assert!(buffer.linearize([0, 0, 1]) == 100);
        assert!(buffer.linearize([0, 1, 0]) == 10);
        assert!(buffer.linearize([1, 0, 0]) == 1);
    }

    #[test]
    fn buffer_delinearize() {
        let buffer = Buffer::build(0, [10, 10, 10]);
        assert!(buffer.delinearize(1) == [1, 0, 0]);
        assert!(buffer.delinearize(10) == [0, 1, 0]);
        assert!(buffer.delinearize(100) == [0, 0, 1]);
    }

    #[test]
    fn buffer_oob() {
        let buffer = Buffer::build(0, [10, 10]);
        assert!(!buffer.surrounds([11, 11]));
        assert!(!buffer.surrounds([11, 0]));
        assert!(!buffer.surrounds([0, 11]));
    }
}
