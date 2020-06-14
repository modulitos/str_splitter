// Usually, we do not need multiple lifetimes. There are only some cases where we do, and this is
// one of them. It comes up when we need to store multiple references, and it is important that they
// are not the same, because we want to return one without tying it to the other.

struct StringSplitter<'delimeter, 'remaining> {
    delimeter: &'delimeter str,
    remaining: Option<&'remaining str>,
}

impl<'delimeter, 'haystack> StringSplitter<'delimeter, 'haystack> {
    fn new(haystack: &'haystack str, delimeter: &'delimeter str) -> Self {
        Self {
            delimeter,
            remaining: Some(haystack),
        }
    }
}

// Note how were using lifetime elision, aka "anonymous lifetime", because the 'delimeter lifetime
// lifetime is not being used. IOW, this block does not care what the 'delimeter lifetime is. Any
// lifetime will do, as long as it's unique from all the other lifetimes.

impl<'delimeter, 'remaining> Iterator for StringSplitter<'_, 'remaining> {
    type Item = &'remaining str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.as_mut()?;

        // Note that this doesn't work, because we'd copy, instead of move.
        // let ref mut remaining = self.remaining?;
        // (which is the same thing as:)
        // let remaining = &mut self.remaining?;

        if let Some(position) = remaining.find(self.delimeter) {
            let new_match = &remaining[..position];
            *remaining = &remaining[position + self.delimeter.len()..];
            Some(new_match)
        } else {
            self.remaining.take()
        }
    }
}

// The anonymous lifetime isn't needed here, but it can be nice to give it, just to be clear that
// it's auto-inferred.

fn until_char(haystack: &str) -> &'_ str {
    let delim = &String::from("x");

    // Defining a &str literal below would give it a static lifetime!
    // let delim = "x";

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
    assert_eq!(until_char("asdfxqwer"), "asdf");
}
