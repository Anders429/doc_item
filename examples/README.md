# Examples

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Anders429/doc_item/examples?label=examples)](https://github.com/Anders429/doc_item/actions/workflows/examples.yaml)

Each of the examples provided here is viewable [here](https://anders429.github.io/doc_item/examples/).
These examples are built and uploaded on a daily basis, and represent how `doc_item` looks on the
current `nightly` build.

Additionally, if you would like to run the examples yourself, run the following command:

```sh
cargo rustdoc --example <example>
```

where `<example>` is the name of the example you would like to run. If the example requires custom
styling, include the relevant HTML file as follows:

```sh
cargo rustdoc --example <example> -- --html-in-header examples/<example>.html
```
