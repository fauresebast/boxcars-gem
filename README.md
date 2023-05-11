# Boxcars Gem

This repository is a Ruby gem to use the amazing [Boxcars](https://github.com/nickbabcock/boxcars) Rust crates in Ruby.

Thanks a lot to nickbabcock for his work !

This gem only implements one of the options of the Boxcars projet: it skips the network data to only give you the header.
More information in the [Boxcars README.md variations section](https://github.com/nickbabcock/boxcars#variations)

If you want to use this gem and you need more bindings that the one I have implement for my usage, feel free to create issues on this repository !

## Usage

```ruby
require "boxcars"

json_formatted_data = Boxcars.run("/path/to/replay.replay")

# =>
# {"header_size"=>8398,
# ...

```

## Update this gem
To create this gem from the Rust code, I followed [this tutorial](https://medium.com/@richard_23685/creating-a-ruby-gem-with-rust-49dce5b9b925) (thank you Richard Patching) and merged his ["helloworld" from Rust repository](https://github.com/patchfx/helloworld) with Boxcars projects.

If you modify the Rust code (the `src/lib.rs` for exemple), all you have to do after is to run `rake` and then you can interact with your gem in `irb` or run `bin/console` (start a irb with the gem already imported).