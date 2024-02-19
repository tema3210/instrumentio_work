use std::{fmt, future::Future, pin::Pin, rc::Rc};

fn main() {
    let arr = [255u8];
    SayHi::say_hi(Pin::new(&&arr[..]));
}

// `Box<T>`, `Rc<T>`, `Vec<T>`, `String`, `&[u8]`, `T`.
trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T> SayHi for Box<T>
where
    T: fmt::Debug,
{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T> SayHi for Rc<T>
where
    T: fmt::Debug,
{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T> SayHi for Vec<T>
where
    T: fmt::Debug,
{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for &[u8] {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

/// COmmented out so it doesn't mess with specialization

// impl<T> SayHi for T where T: fmt::Debug {
//     fn say_hi(self: Pin<&Self>) {
//         println!("Hi from {:?}", self)
//     }
// }

trait MutMeSomehow {
    /// Implementation must be meaningful, and
    /// obviously call something requiring `&mut self`.
    /// The point here is to practice dealing with
    /// `Pin<&mut Self>` -> `&mut self` conversion
    /// in different contexts, without introducing
    /// any `Unpin` trait bounds.
    fn mut_me_somehow(self: Pin<&mut Self>);
}

/// shoot in leg
/// DO NOT USE EVER
fn get_a_menaingful_T<T>() -> T {
    unsafe { std::mem::zeroed() }
}

/// Same thing

// impl<T> MutMeSomehow for T {
//     fn mut_me_somehow(mut self: Pin<&mut Self>) {

//         self.set(get_a_menaingful_T)
//     }
// }

impl<T> MutMeSomehow for Box<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(get_a_menaingful_T())
    }
}

impl<T> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        unimplemented!("It's hardly possible to mutate a refcounted value in a sane way");
    }
}

impl<T> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(vec![get_a_menaingful_T()]);
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        let lc = self.to_ascii_lowercase(); // mb could have done that in place
        self.set(lc);
    }
}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(&self[1..]) //cut the leftmost slice element
    }
}

//###################################################

struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<std::time::Instant>,
}

impl<F> MeasurableFuture<F> {
    fn project<'s>(
        self: Pin<&'s mut Self>,
    ) -> (Pin<&'s mut F>, &'s mut Option<std::time::Instant>) {
        // Safety: we don't move anything here
        let this = unsafe { self.get_unchecked_mut() };

        // safety: and here also
        let inner = unsafe { Pin::new_unchecked(&mut this.inner_future) };
        let time_ = &mut this.started_at;

        //no intersection between pointers, and pin is preserved
        (inner, time_)
    }
}

impl<Fut> Future for MeasurableFuture<Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let (inner, started_at) = self.project();

        match Future::poll(inner, cx) {
            std::task::Poll::Ready(out) => {
                println!("time elapsed: {:?}", started_at.unwrap().elapsed());
                std::task::Poll::Ready(out)
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
