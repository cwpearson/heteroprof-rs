#[derive(Deserialize, Serialize)]
pub struct Dim3<T> {
    x: T,
    y: T,
    z: T,
}
