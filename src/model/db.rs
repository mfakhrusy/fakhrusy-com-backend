use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
