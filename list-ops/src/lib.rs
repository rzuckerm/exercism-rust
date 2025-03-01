/// Yields each item of a and then each item of b
pub fn append<T, I>(a: I, b: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
{
    struct Append<T, I: Iterator<Item = T>> {
        a: I,
        b: I,
    }

    impl<T, I: Iterator<Item = T>> Iterator for Append<T, I> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.a.next().or_else(|| self.b.next())
        }
    }

    Append { a, b }
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I, NI, T>(nested_iter: I) -> impl Iterator<Item = T>
where
    NI: Iterator<Item = T>,
    I: Iterator<Item = NI>,
{
    struct Concat<I: Iterator<Item = NI>, NI: Iterator<Item = T>, T> {
        nested_iter: I,
        curr: Option<NI>,
    }

    impl<I: Iterator<Item = NI>, NI: Iterator<Item = T>, T> Iterator for Concat<I, NI, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(nested_iter) = self.curr.as_mut() {
                if let Some(val) = nested_iter.next() {
                    return Some(val);
                }
            }

            if let Some(next_nested_iter) = self.nested_iter.next() {
                self.curr = Some(next_nested_iter);
                self.next()
            } else {
                None
            }
        }
    }

    Concat {
        nested_iter,
        curr: None,
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, T, F>(iter: I, predicate: F) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    struct Filter<I: Iterator<Item = T>, T, F: Fn(&T) -> bool> {
        iter: I,
        predicate: F,
    }

    impl<I: Iterator<Item = T>, T, F: Fn(&T) -> bool> Iterator for Filter<I, T, F> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|val| (self.predicate)(val))
        }
    }

    Filter { iter, predicate }
}

pub fn length<I: Iterator<Item = T>, T>(iter: I) -> usize {
    foldl(iter, 0usize, |acc, _| acc + 1)
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, T, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
{
    struct Map<I: Iterator<Item = T>, T, F: Fn(T) -> U, U> {
        iter: I,
        function: F,
    }

    impl<I: Iterator<Item = T>, T, F: Fn(T) -> U, U> Iterator for Map<I, T, F, U> {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(&self.function)
        }
    }

    Map { iter, function }
}

pub fn foldl<I, F, T, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator<Item = T>,
    F: Fn(U, T) -> U,
{
    let mut acc = initial;
    for val in iter {
        acc = (function)(acc, val);
    }

    acc
}

pub fn foldr<I, F, T, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator<Item = T>,
    F: Fn(U, T) -> U,
{
    foldl(reverse(iter), initial, function)
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator<Item = T>, T>(iter: I) -> impl Iterator<Item = T> {
    struct Reverse<I: DoubleEndedIterator<Item = T>, T> {
        iter: I,
    }

    impl<I: DoubleEndedIterator<Item = T>, T> Iterator for Reverse<I, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }

    Reverse { iter }
}
