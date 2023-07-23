pub trait Cmd: clap::Parser {
    fn run(self) -> eyre::Result<()>;
}
