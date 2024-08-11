# discord_query_language

Very basic discord-based query language using nom

# Syntax

- Guild filter: `guild(123)`, `guild("name")`, `guild(123 || "name")`
- User filter: `from(123)`, `from("name")`, `from(123 || "name")`
- full query: `from(123) guild(321 || "test server #2")`
    - at least one filter has to be present

# Extra

You can implement your own `select` using this syntax by implementing `FromQuery` trait.
Example can be found in [src/backend/json](src/backend/json.rs)

# Credits

- Language idea by [toonlink](https://github.com/twnlink)