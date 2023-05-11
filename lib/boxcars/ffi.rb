require 'ffi'

module Boxcars
  class FFI
    extend ::FFI::Library
    lib_name = "boxcars.#{::FFI::Platform::LIBSUFFIX}"
    ffi_lib File.expand_path(lib_name, __dir__)
    attach_function :run, [ :string ], :string
  end
end
