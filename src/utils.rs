/// Partial compares sets.
macro_rules! partial_cmp_sets {
    ($a:ident, $b:ident) => {
        if $a.len() == $b.len() && $a.eq(&$b) {
            Some(Ordering::Equal)
        } else if $a.len() < $b.len() && $a.is_subset(&$b) {
            Some(Ordering::Less)
        } else if $a.len() > $b.len() && $a.is_superset(&$b) {
            Some(Ordering::Greater)
        } else {
            None
        }
    };
}

pub(crate) use partial_cmp_sets;
