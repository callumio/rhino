use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// todo
pub struct RhinoArgs {
    /// which program to check if it is running
    #[argh(option, short = 'n')]
    pub name: Option<String>,

    /// invert the program - run if already running
    #[argh(switch, short = 'i')]
    pub invert: bool,
}
