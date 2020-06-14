// TODO: try adding a where D: Delimeter clause...

struct StringSplitter<'remaining, D> {
    delimeter: D,
    remaining: Option<&'remaining str>,
}

impl<'haystack, D> StringSplitter<'haystack, D> {
    fn new(haystack: &'haystack str, delimeter: D) -> Self {
        Self {
            delimeter,
            remaining: Some(haystack),
        }
    }
}

trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(pos, _)| (pos, pos + self.len_utf8()))
    }
}

impl<'remaining, D> Iterator for StringSplitter<'remaining, D>
where
    D: Delimeter,
{
    type Item = &'remaining str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.as_mut()?;

        // Note that this doesn't work, because we'd copy, instead of move.
        // let ref mut remaining = self.remaining?;
        // (which is the same thing as:)
        // let remaining = &mut self.remaining?;

        if let Some((delim_start, delim_end)) = self.delimeter.find_next(remaining) {
            let new_match = &remaining[..delim_start];
            *remaining = &remaining[delim_end..];
            Some(new_match)
        } else {
            self.remaining.take()
        }
    }
}

// The anonymous lifetime isn't needed here, but it can be nice to give it, just to be clear that
// it's auto-inferred.

fn until_char(haystack: &str, delim: char) -> &'_ str {
    let splitter = StringSplitter::new(haystack, delim);
    splitter.into_iter().next().expect("always has a match")
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

#[test]
fn test_until_char() {
    assert_eq!(until_char("asdfxqwer", 'x'), "asdf");
}
