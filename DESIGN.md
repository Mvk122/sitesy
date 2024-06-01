# Design Specifications

# Ideas
## Frontmatter
The start of each document can have frontmatter with arbitrary variables that can be accessed with jinja templates:
```
[_metadata_:title]:- "The Best Blogpost Ever!"
[_metadata_:permalink]:- "index.html"
```

## HTML Snippets
HTML snippets from the html folder can be included with jinja template syntax:

```
{% calendar.html %}
```

# Simple Implementation 
* Reads from the markdown folder.
* Uses the templates from `html/templates` for each item, falling back to the default from `pulldown-cmark::html` if there isn't one.
* Outputs in the same folder structure as `md`