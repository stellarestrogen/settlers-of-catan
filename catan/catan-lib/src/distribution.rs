#[derive(Clone)]
pub struct Distribution<T, const A: usize> {
    distribution: [(T, u32); A],
}

impl<T: PartialEq, const A: usize> Distribution<T, A> {
    pub fn new(distribution: [(T, u32); A]) -> Self {
        Distribution { distribution }
    }

    pub fn for_obj(&self, obj: T) -> u32 {
        let val = self.distribution.iter().find(|(rsrc, _)| *rsrc == obj);
        let default = (obj, 0);
        let (_, d) = val.unwrap_or(&default);
        *d
    }
}
