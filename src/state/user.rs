
pub enum PermissionLevel {
    Read,
    Write,
    Delete
}

#[derive(Default)]
pub struct User {
    pub uuid : i32,
    pub room : i32,
    pub apikey : String,

    //TODO
    //pub permission_level : PermissionLevel,
}
