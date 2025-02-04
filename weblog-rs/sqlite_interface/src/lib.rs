mod people;
mod ratelimits;
mod roles;
mod roles_to_people;
mod sessions;

use crate::people::People;

pub struct AuthDb {}

impl AuthDb {
    pub fn new() -> AuthDb {
        AuthDb {}
    }
}

pub struct DomainDb {}

impl DomainDb {
    pub fn new() -> DomainDb {
        DomainDb {}
    }
}
