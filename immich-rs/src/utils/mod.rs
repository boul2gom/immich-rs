pub mod checksum;
pub(crate) mod mime;

pub(crate) trait IsTrue: Copy {
    fn is_true(self) -> bool;
}

impl IsTrue for bool {
    #[inline]
    fn is_true(self) -> bool {
        self
    }
}

impl IsTrue for Option<bool> {
    #[inline]
    fn is_true(self) -> bool {
        self.unwrap_or(false)
    }
}
