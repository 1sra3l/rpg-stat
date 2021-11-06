pub struct Part<T> {
    pub size:T,
}
pub struct Body<T> {
    pub head:Part<T>,
    pub torso:Part<T>,
    pub arms:Vec<Part<T>>,
    pub legs:Vec<Part<T>>,
    pub color:T,
}