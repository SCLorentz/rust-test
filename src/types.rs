/*pub enum Option<T>
{
    None,
    Some(T)
}*/

pub struct String<'a, const N: usize>
{
    pub(crate) context: &'a [u8; N]
}

impl<'a, const N: usize> String<'a, N>
{
    pub fn new(context: &'a [u8; N]) -> String<'a, N>
    {
        String { context }
    }

    pub fn len(&self) -> usize
    {
        self.as_bytes().len()
    }

    pub fn as_bytes(&self) -> &'a [u8; N]
    {
        self.context
    }
}