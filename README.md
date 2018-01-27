# fuckin-about-with-rs
Let's fuck about with Rust

## Do what
 * Stand up shitty webapp
 * Get it responding to things
 * Make a crappy quicksort
 * Bolt it together
 * Write some tests
 * Write some benchmarks?
 * Check out the doc stuff
 * Look at bolting other code-health things in

## Did what

#### Generic cargo structure for a service and a lib

I made these live alongside each other just for the purposes of keeping it in
one repo, but I don't think I'd usually be doing this for an actual project.

Cargo has some really nice functionality here -- some that I've used and some
that I've only read about so far. In no particular order:

 * You can depend on a path, but that prohibits publishing to crates.io. You can
   also depend on a path-with-a-version though!
 * Unused directives are warnings! When I misspelled "version":
 ```
 warning: unused manifest key: dependencies.fuckin_lib.versiion
 ```
 * Single dependencies can be updated with `update -p` and pinned to exact
   items with the `--precise` flag.
 * Cargo supports a `patch` section to very explicitly control how we can
   override dependencies to get the dependency structure we want. It even
   supports patching to local paths, though this seems like it might be a time
   bomb in an established project...

#### Stand up a shitty webapp

http://www.arewewebyet.org/ has a nice digest of some popular libraries or
frameworks we could use in our path to a fully-functional webapp. It also
contains a summary of some lower-level transport options. 

On the low-level:
* [Solicit](https://github.com/mlalic/solicit) looks promising as the only full
  implementation of HTTP/2, but it hasn't seen an update in over a year.
* [Hyper](https://github.com/hyperium/hyper) at least has client support for
  HTTP/2. It looks like it's liable to change under us as it works towards 1.0,
  but that's probably okay given our ability to control dependencies with Cargo.
  It's based on [Tokio](https://github.com/tokio-rs/tokio), which I've heard all
  kinds of things about, so we'll see how that goes. HTTP/2 support seems
  contingent on [Github issue 304](https://github.com/hyperium/hyper/issues/304)
  which is slated for Hyper 0.12.

When it comes to entire frameworks:
* [Gotham](https://github.com/gotham-rs/gotham) is based on Hyper and Tokio, but
  sells itself as opinionated. It looks as if it's under solid development. It's
  also an example of something in the same shape as this project, and has a
  top-level `Cargo.toml` with a `workspace` definition. This seems like
  something I should do eventually -- move the service back into its own
  subdirectory, and drop a workspace-style `Cargo.toml` for it.
* [Conduit](https://github.com/conduit-rust/conduit) is something I'm unsure
  about. It bills itself simply as a common interface for Rust, so I'm imagining
  it somewhat like Haskell's WAI or Python's WSGI. This makes sense given its
  sparse dependencies. However, fuller projects like Gotham seem to bill it as
  an alternative to their frameworks.
* [Iron](https://github.com/iron/iron) is under mild development, but it looks
  like it pulls in Hyper to do the dirty work too. From what I can tell, it is
  by far the most depended-on web framework, and probably represents a safe
  choice to get started with. However, it looks like it 
  [might not be using Hyper's Tokio-based async stuff](https://github.com/iron/iron/issues/501)?
* [Nickel](https://github.com/nickel-org/nickel.rs) looks like an express.js
  inspired framework under mild development, also based on Hyper. It doesn't
  seem to yet support Hyper 0.11.
* [Rocket](https://github.com/SergioBenitez/Rocket) is much the same.

There's also some talk about [h2](https://github.com/carllerche/h2), which I
can't see related to anything, but it seems that when Hyper 0.12 is released
[it will depend on h2](https://github.com/hyperium/hyper/issues/304#issuecomment-329264272).

In the interim, it can be enabled if we're on Rust 1.20+, Hyper 0.11+, and with
a couple of small changes documented in
[this comment](https://github.com/hyperium/hyper/issues/304#issuecomment-357338916).
This doesn't look like it can be done just yet, but I don't think we're that
far from doing it.

So I guess the way forward is Iron?
