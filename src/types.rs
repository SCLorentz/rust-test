pub struct String<'a>
{
    pub(crate) context: &'a [u8]
}

impl<'a> String<'a>
{
    pub fn new(context: &'a [u8]) -> String<'a>
    {
        String { context }
    }

    pub fn len(&self) -> usize
    {
        self.as_bytes().len()
    }

    pub fn as_bytes(&self) -> &'a [u8]
    {
        self.context
    }
}