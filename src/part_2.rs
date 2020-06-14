struct StringSplitter<'a> {
    delimeter: &'a str,
    remaining: &'a str,
}

impl<'a> StringSplitter<'a> {
    fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        Self {
            delimeter,
            remaining: haystack,
        }
    }
}

impl<'a> Iterator for StringSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.remaining.find(self.delimeter) {
            let new_match = &self.remaining[..position];
            self.remaining = &self.remaining[position + self.delimeter.len()..];
            Some(new_match)
        } else if self.remaining.is_empty() {
            None
        } else {
            let remaining = self.remaining;
            self.remaining = "";
            Some(remaining)
        }
    }
}

#[test]
fn simple_split() {
    let splitter = StringSplitter::new("a b c d", " ");
    let res = splitter.into_iter().collect::<Vec<&str>>();
    assert_eq!(res, vec!["a", "b", "c", "d"]);
}

#[test]
fn tail_split() {
    let splitter = StringSplitter::new("a b c d ", " ");
    let res = splitter.into_iter().collect::<Vec<&str>>();
    assert_eq!(res, vec!["a", "b", "c", "d", ""]);
}
