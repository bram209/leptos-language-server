# leptos-language-server

A language server for the Leptos web framework.

## Status

The latest released alpha version of this language server is using a outdated formatter.
For the latest formatter, download the `leptosfmt` cli from: https://github.com/bram209/leptosfmt

At this point, I am not sure if it is worth it to build a language server for leptosfmt.
Editor support with rust analyzer inside the macro is already pretty good and I have limited time to work on this project. I will try to update the language server soon, so that the latest version of the formatter is used.

Until then, you may happenly keep using the language server if it works for you. The most notable future that the later versions of `leptosfmt` support are non-doc comments.

## Features

- `view!` macro formatting
- ...

### Formatting
![leptos-lsp-formatting](https://user-images.githubusercontent.com/9047770/228370475-729213ed-9670-4b91-8a8c-d04a87a39ee1.gif)
