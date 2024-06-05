# Sitesy

A static site generator utilising a pulldown parsing strategy for fast and efficent static site generation.

# List of features
* Runs a simple, single binary.
* Shareable themes.
* Full `jinja2` templating support including: 
    * Referencing other files. 
    * Random variable interpolation. 
* Include arbitrary HTML snippets in your markdown.
* CSS compilation.
* Prebuilt components for easy blogging.

# Usage
```bash
Commands:
  generate  Run static site generation
  new       Create a new sitesy project
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Usage Specifics
* Markdown files stored in the generated `md` folder will get output as html files. 
* The HTML can be customised by creating custom tags in `templates` i.e `h1.jinja`, `p1.jinja`, the contents will be filled in at `{{ contents }}`
* Place reusable components in `components`, these can be referenced by other `jinja` files or in the markdown with the following syntax: 
```
---
{% include "header.jinja" %}
---
```