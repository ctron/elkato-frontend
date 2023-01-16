pub mod index;
pub mod login;

use yew_nested_router::Target;

#[derive(Target, Clone, Debug, PartialEq, Eq)]
pub enum Pages {
    #[target(index)]
    Index,
}
