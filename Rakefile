require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

task :rust_build do
  `cargo rustc --release`
  `cp -f ./target/release/libboxcars.dylib ./lib/boxcars/boxcars.dylib`
end

task :build => :rust_build
task :default => :build
