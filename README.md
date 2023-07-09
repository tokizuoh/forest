# forest
Tool for hashtag aggregation in Scrapbox.

## Usage

1. Access `https://scrapbox.io/projects/{PROJECT_NAME}/settings/page-data` and push "Export Pages".
2. `cargo run ./{PROJECT_NAME}.json`
3. Display hashtags and number of articles.

```sh
$cargo run ./tokizuoh-public.json
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/forest ./tokizuoh-public.json`
#Swift, 18
#Rust, 3
#SwiftUI, 3
#WIP, 3
#Xcode, 2
#Foundation, 1
#keyboard, 1
#Combine, 1
#apollo-ios, 1
#INS, 1
#Warp, 1
#xed, 1
#Git, 1
#Makefile, 1
```
