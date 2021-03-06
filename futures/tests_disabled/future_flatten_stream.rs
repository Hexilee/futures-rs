
use futures::executor::block_on_stream;
use futures::future::{ok, err};
use futures::stream;
use core::marker;

#[test]
fn successful_future() {
    let stream_items = vec![17, 19];
    let future_of_a_stream = ok::<_, bool>(stream::iter_ok(stream_items));

    let stream = future_of_a_stream.flatten_stream();

    let mut iter = block_on_stream(stream);
    assert_eq!(Ok(17), iter.next().unwrap());
    assert_eq!(Ok(19), iter.next().unwrap());
    assert_eq!(None, iter.next());
}

struct PanickingStream<T, E> {
    _marker: marker::PhantomData<(T, E)>
}

impl<T, E> Stream for PanickingStream<T, E> {
    type Item = T;
    type Error = E;

    fn poll_next(&mut self, _: &LocalWaker) -> Poll<Option<Self::Item>, Self::Error> {
        panic!()
    }
}

#[test]
fn failed_future() {
    let future_of_a_stream = err::<PanickingStream<bool, u32>, _>(10);
    let stream = future_of_a_stream.flatten_stream();
    let mut iter = block_on_stream(stream);
    assert_eq!(Err(10), iter.next().unwrap());
    assert_eq!(None, iter.next());
}
