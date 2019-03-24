pub type UserInput = Vec<String>;

pub struct Command {
  pub name: String,
  pub args: UserInput,
}
