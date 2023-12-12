#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

// ----needed deps above----
// ----solution below----

pub struct NestedIterator {
    nested_list: Vec<NestedInteger>,
    path: Vec<usize>,
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self { nested_list, path: vec![] }
    }

    /// This function works by alternating between walk_forward() and walk_back(), calling itself to
    /// go through it again.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        match self.walk_forward() {
            Ok(integer) => integer,
            Err(()) => match self.walk_back() {
                Ok(integer) => integer,
                Err(true) => self.next(),
                Err(false) => {
                    panic!("Tried to call next() on a finished iterator!")
                },
            },
        }
    }

    /// This function basically does what walk_forward() and walk_back() do internally, but without
    /// updating the location at the end. If next() is called after this, has_next() will have done
    /// some of the work.
    pub fn has_next(&mut self) -> bool {
        loop {
            match self.walk_path() {
                Ok(_) => return true,
                Err(true) => self.path.push(0),
                Err(false) => {
                    self.path.pop();
                    match self.path.last_mut() {
                        Some(last) => *last += 1,
                        None => return false,
                    }
                },
            }
        }
    }

    // This function will "walk" self.path. It will either return the number at the location, or a
    // bool representing whether it ended on a list.
    fn walk_path(&self) -> Result<i32, bool> {
        // Keeps track of the current nested list.
        let mut location = &self.nested_list;

        for i in self.path.iter().copied() {
            match location.get(i) {
                Some(NestedInteger::Int(integer)) => return Ok(*integer),
                Some(NestedInteger::List(list)) => location = &list,
                None => return Err(false),
            }
        }

        Err(true)
    }

    // If walk_path returns Err(true), we need to "walk forward", which means pushing a zero into
    // self.path until we get to an empty List or an Int. If walk_path() did not find an Int or
    // List, we return Err(()) to signify we need to walk_back().
    fn walk_forward(&mut self) -> Result<i32, ()> {
        loop {
            match self.walk_path() {
                Ok(integer) => {
                    *self.path.last_mut().ok_or(())? += 1;
                    return Ok(integer);
                },
                Err(true) => self.path.push(0),

                Err(false) => return Err(()),
            }
        }
    }

    // If walk_path returns Err(false), we need to "walk back", which means popping off of self.path
    // until we either get to the root (self.path is empty) or we find the next element in a List.
    // If we walk back to the root, we return Err(false). If we are not at the root, but don't have
    // an Int, we return Err(true).
    fn walk_back(&mut self) -> Result<i32, bool> {
        loop {
            self.path.pop();
            *self.path.last_mut().ok_or(false)? += 1;

            match self.walk_path() {
                Ok(integer) => {
                    *self.path.last_mut().ok_or(false)? += 1;
                    return Ok(integer);
                },
                Err(true) => {
                    self.path.push(0);
                    return Err(true);
                },
                Err(false) => {},
            }
        }
    }
}

// ----solution above----
// ----tests below----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_iterates_nested_structure_1() {
        let nested_list = vec![
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(1),
            ]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(1),
            ]),
        ];
        let mut nested_iterator = NestedIterator::new(nested_list);

        for n in [1, 1, 2, 1, 1] {
            assert!(nested_iterator.has_next());
            assert_eq!(n, nested_iterator.next());
        }
    }

    #[test]
    fn correctly_iterates_nested_structure_2() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ];
        let mut nested_iterator = NestedIterator::new(nested_list);

        for n in [1, 4, 6] {
            assert!(nested_iterator.has_next());
            assert_eq!(n, nested_iterator.next())
        }
    }
}
