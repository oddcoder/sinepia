use salsa::Update;
use std::fmt::Display;

#[derive(Update, Clone, Debug)]
pub struct Punctuated<T: Update, P: Update> {
    pub inner: Vec<(T, P)>,
    pub last: Option<Box<T>>,
}

impl<T, P> Default for Punctuated<T, P>
where
    T: Update,
    P: Update,
{
    fn default() -> Self {
        Punctuated {
            inner: Vec::new(),
            last: None,
        }
    }
}

impl<T, P> Display for Punctuated<T, P>
where
    T: Update + Display,
    P: Update + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Punctuated")?;
        if let Some((_, punct)) = self.inner.first() {
            write!(f, "({punct})[")?;
        } else {
            write!(f, "()[")?;
        }
        let list = self
            .inner
            .iter()
            .map(|(item, _)| format!("{item}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{list}")?;
        if let Some(item) = &self.last {
            write!(f, ", {item}]")
        } else {
            write!(f, "]")
        }
    }
}

impl<T, P> Punctuated<T, P>
where
    T: Update,
    P: Update,
{
    /// Determines whether this punctuated sequence is empty, meaning it
    /// contains no syntax tree nodes or punctuation.
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0 && self.last.is_none()
    }

    /// Returns the number of syntax tree nodes in this punctuated sequence.
    ///
    /// This is the number of nodes of type `T`, not counting the punctuation of
    /// type `P`.
    pub fn len(&self) -> usize {
        self.inner.len() + if self.last.is_some() { 1 } else { 0 }
    }
    pub fn clear(&mut self) {
        self.inner.clear();
        self.last = None;
    }
    /// Appends a syntax tree node onto the end of this punctuated sequence.
    ///
    /// If there is not a trailing punctuation in this sequence when this method
    /// is called, the default value of punctuation type `P` is inserted before
    /// the given value of type `T`.
    pub fn push(&mut self, value: T)
    where
        P: Default,
    {
        if !self.empty_or_trailing() {
            self.push_punct(Default::default());
        }
        self.push_value(value);
    }
    /// Appends a syntax tree node onto the end of this punctuated sequence. The
    /// sequence must already have a trailing punctuation, or be empty.
    ///
    /// Use [`push`] instead if the punctuated sequence may or may not already
    /// have trailing punctuation.
    ///
    /// [`push`]: Punctuated::push
    ///
    /// # Panics
    ///
    /// Panics if the sequence is nonempty and does not already have a trailing
    /// punctuation.
    pub fn push_value(&mut self, value: T) {
        assert!(
            self.empty_or_trailing(),
            "Punctuated::push_value: cannot push value if Punctuated is missing trailing punctuation",
        );

        self.last = Some(Box::new(value));
    }
    /// Appends a trailing punctuation onto the end of this punctuated sequence.
    /// The sequence must be non-empty and must not already have trailing
    /// punctuation.
    ///
    /// # Panics
    ///
    /// Panics if the sequence is empty or already has a trailing punctuation.
    pub fn push_punct(&mut self, punctuation: P) {
        assert!(
            self.last.is_some(),
            "Punctuated::push_punct: cannot push punctuation if Punctuated is empty or already has trailing punctuation",
        );

        let last = self.last.take().unwrap();
        self.inner.push((*last, punctuation));
    }

    pub fn empty_or_trailing(&self) -> bool {
        self.last.is_none()
    }
    pub fn trailing(&self) -> bool {
        !self.inner.is_empty() && self.last.is_none()
    }
}

impl<T, P> FromIterator<T> for Punctuated<T, P>
where
    T: Update,
    P: Update + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self {
        let mut ret = Punctuated::default();
        ret.extend(i);
        ret
    }
}

impl<T, P> Extend<T> for Punctuated<T, P>
where
    T: Update,
    P: Update + Default,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, i: I) {
        for value in i {
            self.push(value);
        }
    }
}
