use std::ops::Rem;

pub struct Matcher<'a, T> {
    func: Box<dyn Fn(T) -> bool + 'a>,
    subs: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F: Fn(T) -> bool + 'a, S: ToString>(matcher: F, subs: S) -> Matcher<'a, T> {
        Self {
            func: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<'a, T>(Vec<Matcher<'a, T>>);

impl<'a, T: ToString + Clone> Fizzy<'a, T> {
    pub fn new() -> Self {
        Fizzy::<T>(Vec::<Matcher<T>>::new())
    }

    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.0.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = T> + 'a,
        T: 'a,
    {
        iter.map(move |n| {
            match self
                .0
                .iter()
                .filter_map(|matcher| ((matcher.func)(n.clone())).then_some(matcher.subs.clone()))
                .collect::<String>()
            {
                s if s.is_empty() => n.to_string(),
                s => s,
            }
        })
    }
}

pub fn fizz_buzz<'a, T: Rem<Output = T> + ToString + Clone + PartialEq + From<u8>>() -> Fizzy<'a, T>
{
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(
            |n| n % T::from(3) == T::from(0),
            "fizz".to_string(),
        ))
        .add_matcher(Matcher::new(
            |n| n % T::from(5) == T::from(0),
            "buzz".to_string(),
        ))
}
