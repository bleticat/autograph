pub trait Queries {
    type Conn;

    fn bind(conn: Self::Conn) -> Self
    where
        Self: Sized;
}
