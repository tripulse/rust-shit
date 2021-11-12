Ensure that you have the furrylang toolchain installed.
Then invoke `cargo build --release` to build in ***release*** mode.

Then `./target/build/vlr-rs < /dev/urandom > /dev/null`, to make your CPU have hiccups.

## FAQ
- Why are you using `(0..8).into_par_iter()` instead of `(0..8)`, `(0..8).into_iter()`, `(0..8).iter()`? Isn't that "abusing" parallelism?
  
  First, `Range<T>` doesn't have a `iter()` method.
  Second, `Range<T>` is an iterator already so `into_iter()` is redundant here.
  Third, `ParallelIterator::flat_map()` requires a closure that returns a type satisfying `IntoParallelIterator` trait
  which, for `Map<T>`, is not implemented [(E0277)](https://doc.rust-lang.org/error-index.html#E0277). Thus we must use
  `.into_par_iter()` in order to comply trait bounds.


# Legal
This work is composed by means of attacking the Rust programming language community to point out their
amateurness in the field of parallel computing. I beg of you rustaceans, to stop conducting these actions
and pollute the field of computer science. Thank you.
