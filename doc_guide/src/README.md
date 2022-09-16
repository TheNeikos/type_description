# Introduction

The `type_description` crate is the basic block to allow for discovery of
configuration options in your Rust projects.

It allows for self-describing types in your project, and an easy way to export
them. 
This description can then be manipulated like any other data, allow for
processing into for example Markdown or a questionnaire.

An example would be the following:

```rust
{{#rustdoc_include example_config.rs:definition}}
```

This would then output a datastructure (if rendered as JSON) like this:

```rust
{{#rustdoc_include example_config.rs:test}}
```

As you can see, not only is the struct itself described, but also all of its
constituents as well as the documentation you have provided.

If you want to, you could use the binary provided by the `type_description` crate to transform it into markdown!

This would output something akin to:

> # MyConfiguration
> 
> 
> **Fields:**
> 
> - `name` (String): The name of this configuration
> - `action` (Array of 'String's): List of actions this config can take
> 
> # String
> 
> An UTF-8 string
> 
> # Array of 'String's
> 
> 
> _Array Elements of String_


If this sounds intruiging, read on and find out how it can be used!

