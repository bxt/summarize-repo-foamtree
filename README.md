# summarize-repo-foamtree

Tool to summarize what is in a git repo.

This uses some makeshift Rust code calling into libgit2 to get the file tree of a repository, and some information about the authors. I then puts the data into a tool called Foamtree to create a voronoi diagram from it. It's more of a proof-of-concept right now, so you have to dig into the code to get anything useful out of this.

## Example

<img width="100%" alt="Result of running summarize-repo-foamtree on the remix repo, some colorful poloygons" src="https://github.com/bxt/summarize-repo-foamtree/assets/639509/abe51c92-1188-4120-aae3-6e7bb0b5d40c">

## Running locally

Have rust / cargo and node / npx installed. Download the `carrotsearch.foamtree.js` file from:

https://get.carrotsearch.com/foamtree/latest/download/

Install:

    npm install -g browser-sync

Run:

    browser-sync start --server --files "index.html,*.js"

For gathering data:

    cargo run -- ../path/to/repo > data.js

If you do not want to run rust and gather actual repo data you can also do:

    cp data.js.example data.js

More info about the used libs:

* https://get.carrotsearch.com/foamtree/latest/api/#groupColorDecorator
* https://get.carrotsearch.com/foamtree/latest/demos/settings.html
* https://docs.rs/git2/latest/git2/
* https://github.com/rust-lang/git2-rs/tree/master/examples

Have fun!
