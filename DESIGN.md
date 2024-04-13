# Design Specifications

## Frontmatter
The start of each document can have frontmatter with the following fields:
```
[_metadata_:title]:- "The Best Blogpost Ever!"
[_metadata_:permalink]:- "index.html"
```

## HTML Snippets
HTML snippets from the html folder can be included with jinja template syntax:

```
{% calendar.html %}
```