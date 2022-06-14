// Lifetimes are for pointers
// how long does the compiler need to keep it around?
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

// need lifetimes here, compiler will keep strsplit around for as long as
// its inputs are around
impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    // need to specify lifetime in Iterator because next returns a pointer
    // how long does it hold onto it?
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + 1))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &*delim)
        .next()
        .expect("StrSplit always returns at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}

#[test]
fn empty_tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
