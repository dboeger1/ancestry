use crate::Ancestry;
use std::path::{
    Ancestors,
    Path,
};


impl<'a> Ancestry<'a> for Path {
    type AncestorsIterator = Ancestors<'a>;

    fn ancestors(&'a self) -> Self::AncestorsIterator {
        self.ancestors()
    }
}
