require "boxcars/ffi"
require "boxcars/version"
require "json"

module Boxcars
  class Error < StandardError; end

  def self.run(filename)
    JSON.parse(FFI.run(filename))
  end
end
