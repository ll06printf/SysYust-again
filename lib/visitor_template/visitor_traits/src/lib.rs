pub trait Transformable {
    type Transformer;
    fn transform(self, tf: &mut Self::Transformer) -> Self;
}