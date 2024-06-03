/// Allows a struct or enum to be provided as a Zenity application.
pub trait ZenityApplication: Clone + Default + ToArgVector {
    /// The type that Zenity returns. Usually it should be a string,
    /// but in some cases you might want to parse the string into another
    /// type.
    type Return: Clone;

    /// Parse the raw output from Zenity into another type
    fn parse(&self, stdout: &str) -> Result<Self::Return, crate::Error>;
}

pub trait ToArgVector {
    fn to_argv(&self) -> Vec<String>;
}
