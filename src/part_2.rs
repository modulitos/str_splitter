struct StringSplitter<'a> {
    delimeter: &'a str,
    remaining: Option<&'a str>,
}

impl<'a> StringSplitter<'a> {
    fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        Self {
            delimeter,
            remaining: Some(haystack),
        }
    }
}

impl<'a> Iterator for StringSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.as_mut()?;
        if let Some(position) = remaining.find(self.delimeter) {
            let new_match = &remaining[..position];
            *remaining = &remaining[position + self.delimeter.len()..];
            Some(new_match)
        } else {
            self.remaining.take()
        }
    }
}

#[test]
fn char_split() {
    let splitter = StringSplitter::new("a b c d", " ");
    let res = splitter.into_iter().collect::<Vec<&str>>();
    assert_eq!(res, vec!["a", "b", "c", "d"]);
}

#[test]
fn word_split() {
    let splitter = StringSplitter::new("lazy brown fox", " ");
    let res = splitter.into_iter().collect::<Vec<&str>>();
    assert_eq!(res, vec!["lazy", "brown", "fox"]);
}

#[test]
fn tail_split() {
    let splitter = StringSplitter::new("a b c d ", " ");
    let res = splitter.into_iter().collect::<Vec<&str>>();
    assert_eq!(res, vec!["a", "b", "c", "d", ""]);
}
