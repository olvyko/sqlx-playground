pub use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}
