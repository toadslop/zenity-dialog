pub trait ZenityApplication: Clone + Default + ToArgVector {
    type Return: Clone;

    fn parse(&self, stdout: &str) -> Result<Self::Return, crate::Error>;
}

pub trait ToArgVector {
    fn to_argv(&self) -> Vec<String>;
}
