pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self(rails as usize)
    }

    fn indices(&self, len: usize) -> impl Iterator<Item = usize> {
        let mut idxs: Vec<(usize, usize)> = (0..self.0)
            .chain((1..self.0 - 1).rev())
            .cycle()
            .zip(0..len)
            .collect();
        idxs.sort_unstable();
        idxs.into_iter().map(|x| x.1)
    }

    pub fn encode(&self, text: &str) -> String {
        let text: Vec<char> = text.chars().collect();
        self.indices(text.len()).map(|n| text[n]).collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut chunks: Vec<(usize, char)> =
            self.indices(cipher.len()).zip(cipher.chars()).collect();
        chunks.sort_unstable();
        chunks.into_iter().map(|x| x.1).collect()
    }
}
