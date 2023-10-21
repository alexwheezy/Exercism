#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Clone + PartialEq + Ord,
{
    pub fn new(_input: &[T]) -> Self {
        let mut data = Vec::from(_input);
        data.sort_unstable();
        data.dedup();

        Self { data }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.data.push(_element);
            self.data.sort_unstable();
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.data.iter().all(|v| _other.contains(v))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !self.data.iter().any(|v| _other.contains(v))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|v| _other.contains(*v))
                .cloned()
                .collect::<Vec<_>>(),
        }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|v| !_other.contains(*v))
                .cloned()
                .collect::<Vec<_>>(),
        }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut data = self.data.clone();
        if !_other.is_empty() || !self.is_subset(_other) {
            data.extend_from_slice(&_other.data);
            data.sort_unstable();
            data.dedup();
        }

        Self { data }
    }
}
