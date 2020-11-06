# fuckin-about-with-rs
Let's fuck about with Rust

## Do what
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

The answer is basically actix-web for now.