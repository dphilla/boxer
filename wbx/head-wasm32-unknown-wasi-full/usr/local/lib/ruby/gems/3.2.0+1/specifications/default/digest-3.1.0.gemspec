# -*- encoding: utf-8 -*-
# stub: digest 3.1.0 ruby lib

Gem::Specification.new do |s|
  s.name = "digest".freeze
  s.version = "3.1.0"

  s.required_rubygems_version = Gem::Requirement.new(">= 0".freeze) if s.respond_to? :required_rubygems_version=
  s.metadata = { "msys2_mingw_dependencies" => "openssl" } if s.respond_to? :metadata=
  s.require_paths = ["lib".freeze]
  s.authors = ["Akinori MUSHA".freeze]
  s.bindir = "exe".freeze
  s.date = "2022-06-01"
  s.description = "Provides a framework for message digest libraries.".freeze
  s.email = ["knu@idaemons.org".freeze]
  s.files = ["digest.rb".freeze, "digest/loader.rb".freeze, "digest/sha2.rb".freeze, "digest/sha2/loader.rb".freeze, "digest/version.rb".freeze]
  s.homepage = "https://github.com/ruby/digest".freeze
  s.licenses = ["Ruby".freeze, "BSD-2-Clause".freeze]
  s.required_ruby_version = Gem::Requirement.new(">= 2.5.0".freeze)
  s.rubygems_version = "3.4.0.dev".freeze
  s.summary = "Provides a framework for message digest libraries.".freeze
end
