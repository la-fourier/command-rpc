use clap::{
    Args,
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    // /// First arg
    // pub first_arg: String,
    // /// Sec arg
    // pub sec_arg: String,
    // /// 3rd arg
    // pub third_arg: String,

    #[clap(subcommand)]
    pub entity_type: Entity_type,
}

#[derive(Debug, Subcommand)]
pub enum Entity_type {
    User(UserCommand),
    Video(VideoCommand),
    View(ViewCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteUser),
    Read(ReadUser),
}

#[derive(Debug, Args)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct ReadUser {
    pub name: String,
    pub email: String,
}