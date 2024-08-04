pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut output = Vec::<U>::new();
    input.into_iter().for_each(|x| output.push(function(x)));
    output
}
