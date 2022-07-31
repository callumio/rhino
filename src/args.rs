use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// todo
pub struct RhinoArgs {
 
    /// invert the program - run if already running
    #[argh(switch, short = 'i')]
    pub invert: bool,

    /// search for the process, case sensitively - by default it uses smart case
    #[argh(switch, short = 'c')]
    pub case_sensitive: bool,

    /// allow partial matches in the process search - by default it only allows exact matches 
    #[argh(switch, short = 'p')]
    pub partial: bool,

    /// require all processes to match, to continue - by default at least one must be present 
    #[argh(switch, short = 'a')]
    pub all: bool,

    /// which program to check if it is running, delimited by a space - by default the command to
    /// be ran will be checked
    #[argh(option, short = 'n')]
    pub name: Option<String>,

    /// the command to be ran
    #[argh(positional)]
    pub cmd: String,

    /// the options to be passed to the command to be ran
    #[argh(positional)]
    pub cmd_options: Vec<String>

}
