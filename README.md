# summarize-ts-repo-foamtree

Tool to summarize what is in a git repo.

Have rust / cargo and node / npx installed. Download the `carrotsearch.foamtree.js` file from:

https://get.carrotsearch.com/foamtree/latest/download/

Install:

    npm install -g browser-sync

Run:

    browser-sync start --server --files "index.html,*.js"

For gathering data:

    cargo run -- ../path/to/repo > data.js

Have fun!