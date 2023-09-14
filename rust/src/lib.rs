mod path;


pub trait Ancestry<'a> {
    type AncestorsIterator: Iterator<Item = &'a Self>
    where
        Self: 'a;

    fn ancestors(&'a self) -> Self::AncestorsIterator;

    fn is_descendant_of(&'a self, other: &Self) -> bool
    where Self: PartialEq,
    {
        for ancestor in self.ancestors() {
            if ancestor == other {
                return true;
            }
        }
        false
    }

    fn is_ancestor_of(&self, other: &'a Self) -> bool
    where Self: PartialEq,
    {
        other.is_descendant_of(self)
    }

    fn closest_common_ancestor(&'a self, other: &'a Self) -> Option<&Self>
    where Self: PartialEq,
    {
        for ancestor in self.ancestors() {
            if ancestor.is_ancestor_of(other) {
                return Some(ancestor);
            }
        }
        None
    }
}
