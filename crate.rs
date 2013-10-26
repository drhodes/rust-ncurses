#[link(
    name = "ncurses",
    vers = "0.01",
    uuid = "49265224-c7dd-11e1-88f6-b3e67a766193"
)];

#[crate_type = "lib"];

pub mod ncurses;

#[path = "ncurses.rs"] 
pub mod c;

pub mod window;
pub mod types;
