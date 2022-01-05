use crate::user;

pub struct Member {
    id: user::Id,
    role: Role,
}

pub enum Role {
    Owner,
    Maintainer,
    Worker,
    Viewer,
} 

pub struct Members(Vec<Member>);
