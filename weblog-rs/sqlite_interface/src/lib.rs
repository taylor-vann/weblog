mod ip_rate_limits;
mod people;
mod roles;
mod roles_to_people;
mod session_rate_limits;
mod sessions;

use crate::ip_rate_limits::IpRateLimits;
use crate::people::People;
use crate::roles::Roles;
use crate::roles_to_people::RolesToPeople;
use crate::session_rate_limits::SessionRateLimits;
use crate::sessions::Sessions;

pub struct AuthDb {
    people: People,
    roles: Roles,
    roles_to_people: RolesToPeople,
    ip_rate_limits: IpRateLimits,
    sessions: Sessions,
    session_rate_limits: SessionRateLimits,
}

impl AuthDb {
    pub fn new() -> AuthDb {
        AuthDb {
            people: People::new(),
            roles: Roles::new(),
            roles_to_people: RolesToPeople::new(),
            ip_rate_limits: IpRateLimits::new(),
            sessions: Sessions::new(),
            session_rate_limits: SessionRateLimits::new(),
        }
    }
}

pub struct DomainDb {}

impl DomainDb {
    pub fn new() -> DomainDb {
        DomainDb {}
    }
}
