use std::cmp::Ordering;

#[deriving(PartialEq, Eq)]
pub struct InvOrd<T: Ord>(pub T);

impl<T: Ord> InvOrd<T> {
    pub fn value(&self) -> &T {
        let InvOrd(ref value) = *self;
        value
    }

    pub fn value_mut(&mut self) -> &mut T {
        let InvOrd(ref mut value) = *self;
        value
    }

    pub fn get(self) -> T {
        let InvOrd(value) = self;
        value
    }
}

impl<T: Ord> PartialOrd for InvOrd<T> {
    fn partial_cmp(&self, other: &InvOrd<T>) -> Option<Ordering> {
        self.value().partial_cmp(other.value()).map(|value| match value {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        })
    }
}

impl<T: Ord> Ord for InvOrd<T> {
    fn cmp(&self, other: &InvOrd<T>) -> Ordering {
        match self.value().cmp(other.value()) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl<T: Ord> Deref<T> for InvOrd<T> {
    fn deref(&self) -> &T {
        self.value()
    }
}

impl<T: Ord> DerefMut<T> for InvOrd<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value_mut()
    }
}
