use ancestry::*;
use lazy_static::lazy_static;
use std::path::PathBuf;


struct PathChildren {
    path: PathBuf,
    children: Vec<PathChildren>,
}

lazy_static! {
    // At the time of writing this comment, the tests in this file are dependent
    // on the following structure of sample test data. If the structure is
    // changed, the tests may need to change accordingly.
    static ref PATH_ROOT: PathChildren = PathChildren {
        path: PathBuf::from("/"),
        children: vec![
            PathChildren {
                path: PathBuf::from("/A"),
                children: vec![
                    PathChildren {
                        path: PathBuf::from("/A/1"),
                        children: Vec::new(),
                    },
                    PathChildren {
                        path: PathBuf::from("/A/2"),
                        children: Vec::new(),
                    },
                ],
            },
            PathChildren {
                path: PathBuf::from("/B"),
                children: vec![
                    PathChildren {
                        path: PathBuf::from("/B/1"),
                        children: Vec::new(),
                    },
                    PathChildren {
                        path: PathBuf::from("/B/2"),
                        children: Vec::new(),
                    },
                ],
            },
        ],
    };
}


#[test]
fn test_positive_is_descendant_of_parent() {
    let mut parents: Vec<&PathChildren> = vec![&PATH_ROOT];
    parents.extend(PATH_ROOT.children.iter());

    for parent in &parents {
        for child in &parent.children {
            assert!(child.path.is_descendant_of(&parent.path));
        }
    }
}

#[test]
fn test_negative_is_descendant_of_children() {
    let mut parents: Vec<&PathChildren> = vec![&PATH_ROOT];
    parents.extend(PATH_ROOT.children.iter());

    for parent in &parents {
        for child in &parent.children {
            assert!(!parent.path.is_descendant_of(&child.path));
        }
    }
}

#[test]
fn test_positive_is_descendant_of_grandparent() {
    for child in &PATH_ROOT.children {
        for grandchild in &child.children {
            assert!(grandchild.path.is_descendant_of(&PATH_ROOT.path));
        }
    }
}

#[test]
fn test_negative_is_descendant_of_grandchildren() {
    for child in &PATH_ROOT.children {
        for grandchild in &child.children {
            assert!(!PATH_ROOT.path.is_descendant_of(&grandchild.path));
        }
    }
}

#[test]
fn test_positive_is_ancestor_of_children() {
    let mut parents: Vec<&PathChildren> = vec![&PATH_ROOT];
    parents.extend(PATH_ROOT.children.iter());

    for parent in &parents {
        for child in &parent.children {
            assert!(parent.path.is_ancestor_of(&child.path));
        }
    }
}

#[test]
fn test_negative_is_ancestor_of_parent() {
    let mut parents: Vec<&PathChildren> = vec![&PATH_ROOT];
    parents.extend(PATH_ROOT.children.iter());

    for parent in &parents {
        for child in &parent.children {
            assert!(!child.path.is_ancestor_of(&parent.path));
        }
    }
}

#[test]
fn test_positive_is_ancestor_of_grandchildren() {
    for child in &PATH_ROOT.children {
        for grandchild in &child.children {
            assert!(PATH_ROOT.path.is_ancestor_of(&grandchild.path));
        }
    }
}

#[test]
fn test_negative_is_ancestor_of_grandparent() {
    for child in &PATH_ROOT.children {
        for grandchild in &child.children {
            assert!(!grandchild.path.is_ancestor_of(&PATH_ROOT.path));
        }
    }
}

#[test]
fn test_negative_is_descendent_of_relatives() {
    let mut families: Vec<Vec<&PathChildren>> = Vec::new();
    for parent in &PATH_ROOT.children {
        let mut family = vec![parent];
        family.extend(parent.children.iter());
        families.push(family);
    }

    for (index1, family1) in families.iter().enumerate() {
        for (index2, family2) in families.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            for relative1 in family1 {
                for relative2 in family2 {
                    assert!(!relative1.path.is_descendant_of(&relative2.path));
                }
            }
        }
    }
}

#[test]
fn test_negative_is_ancestor_of_relatives() {
    let mut families: Vec<Vec<&PathChildren>> = Vec::new();
    for parent in &PATH_ROOT.children {
        let mut family = vec![parent];
        family.extend(parent.children.iter());
        families.push(family);
    }

    for (index1, family1) in families.iter().enumerate() {
        for (index2, family2) in families.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            for relative1 in family1 {
                for relative2 in family2 {
                    assert!(!relative1.path.is_ancestor_of(&relative2.path));
                }
            }
        }
    }
}

#[test]
fn test_positive_closest_common_ancestor_children() {
    let mut parents: Vec<&PathChildren> = vec![&PATH_ROOT];
    parents.extend(PATH_ROOT.children.iter());

    for parent in &parents {
        for child in &parent.children {
            let results = (
                parent
                    .path
                    .closest_common_ancestor(&child.path),
                child
                    .path
                    .closest_common_ancestor(&parent.path),
            );
            assert_eq!(Some(parent.path.as_path()), results.0);
            assert_eq!(results.0, results.1);
        }
    }
}

#[test]
fn test_positive_closest_common_ancestor_grandchildren() {
    for child in &PATH_ROOT.children {
        for grandchild in &child.children {
            let results = (
                PATH_ROOT
                    .path
                    .closest_common_ancestor(&grandchild.path),
                grandchild
                    .path
                    .closest_common_ancestor(&PATH_ROOT.path),
            );
            assert_eq!(Some(PATH_ROOT.path.as_path()), results.0);
            assert_eq!(results.0, results.1);
        }
    }
}

#[test]
fn test_positive_closest_common_ancestor_relatives() {
    let mut families: Vec<Vec<&PathChildren>> = Vec::new();
    for parent in &PATH_ROOT.children {
        let mut family = vec![parent];
        family.extend(parent.children.iter());
        families.push(family);
    }

    for (index1, family1) in families.iter().enumerate() {
        for (index2, family2) in families.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            for relative1 in family1 {
                for relative2 in family2 {
                    assert_eq!(
                        Some(PATH_ROOT.path.as_path()),
                        relative1.path.closest_common_ancestor(&relative2.path),
                    );
                }
            }
        }
    }
}

//#[test]
//fn test_negative_closest_common_ancestor_relatives() {
//}
