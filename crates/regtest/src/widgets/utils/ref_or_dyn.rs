
/// Allows saving a heterogenus list of `Box<dyn Trait>` and `&dyn Trait`.
pub enum RefOrDyn<'a, T: ?Sized + 'a> {
    Ref(&'a T),
    Owned(Box<T>)
}

impl <'a, T: ?Sized> AsRef<T> for RefOrDyn<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            RefOrDyn::Ref(t) => t,
            RefOrDyn::Owned(t) => t.as_ref(),
        }
    }
}
