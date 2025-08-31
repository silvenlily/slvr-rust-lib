
## lily's library
This is the common library lily uses for her projects.
Everything in this library is behind a feature flag.
If you aren't lily, you probably shouldn't use this library as it is not well documented,
and contains exactly the set of features that I found personally useful.
If you want to use something from this crate you are probably best off just stealing a code snippet.
If you find a bug open a github issue and I might fix it.

## features

| feature(s)                                          | description                                                                                                                                                                                                                                                                                                                                           |
|-----------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| tauri_wasm  tauri_wasm_backend  tauri_wasm_frontend | a set of tools for dealing with the connection between your frontend and backend code in tauri where your frontend is written in rust (ie: leptos)                                                                                                                                                                                                    |
| encapsulate                                         | A macro and trait that makes encapsulating structs easy and allows the retrieval of data from those structs in a generic way. Particularly useful for enabling caching without having to manually implement some kind of cache trait for every single struct. It is possible to implement inheritance using this, if you do this I will be very sad.  |

## Crates
If you are a consumer of this library, you want `slvr_rust_lib` or `slvr_rust_lib_build` as a build-dependency.  
The slvr_rust_lib crate contains items exported from the other crates in this library.  
The slvr_rust_lib_build crate contains functions intended to be used in build files.  
The slvr_rust_lib_common crate contains most of the actual logic, trait defs, etc.  
The slvr_rust_lib_macro crate contains (surprisingly) all the macros.  
The slvr_rust_lib_dev is not commited to the repo. Used locally for development.

## FAQ
When are you going to publish this crate to crates.io?  
Probably never, feel free to fork this repo and do that yourself.

Why did you put everything in one big crate instead of breaking it up into smaller crates?  
It's more convenient for me, feel free to fork this repo and do that yourself.

Dear god \<feature> is hacky, please fix \<feature>  
Yes, most things in this repo are hacks to get around problems in unintended ways, feel free to fork this repo and do that yourself though.

What is with the `.aiignore` `.noai` `.aiexclude` `.codeiumignore` `.cursorignore` files?  
See: https://www.jetbrains.com/help/ai-assistant/disable-ai-assistant.html