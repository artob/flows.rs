require 'json'
require 'pathname'
require 'stringio'
require 'tomlrb' # https://rubygems.org/gems/tomlrb
require 'yaml'

PACKAGES = Dir['lib/**/Cargo.toml'].sort_by do |path|
  path.delete_prefix('lib/').delete_suffix('/Cargo.toml')
end.map { Pathname(it) }.freeze

task default: %w(packages.json packages.md)

task readmes: PACKAGES.map { it.parent.join('README.md').to_s }.to_a - %w[lib/flows/README.md]

PACKAGES.each do |package_toml|
  package_path = package_toml.parent
  package_meta = Tomlrb.load_file(package_toml, symbolize_keys: true)
  package_name = package_meta[:package][:name]
  next if package_name == 'flows'
  package_title = (package_meta[:package][:metadata][:readme][:title] rescue nil)
  package_description = package_meta[:package][:description]
  file package_path.join('README.md') => %[.readme/README.md.j2] do |t|
    File.open(t.name, 'w') do |out|
      IO.popen("minijinja-cli #{t.prerequisites.first} /dev/stdin -fjson", "r+") do |pipe|
        pipe.puts JSON.pretty_unparse({
          package: {
            title: package_title,
            name: package_name,
            description: package_description,
          }
        })
        pipe.close_write
        out.puts pipe.read
      end
    end
  end
end

file 'packages.json': PACKAGES do |t|
  File.open(t.name, 'w') do |out|
    out.puts generate_json(t.prerequisites)
  end
end

file 'packages.md': PACKAGES do |t|
  File.open(t.name, 'w') do |out|
    out.puts generate_markdown(t.prerequisites)
  end
end

def generate_markdown(input_paths)
  StringIO.open do |out|
    out.puts "| Package | Summary | Crate | Documentation |"
    out.puts "| :------ | :------ | :---- | :------------ |"
    load_projects(input_paths).each do |project|
      package_name = project[:package][:name]
      package_link = "[#{package_name}](https://github.com/artob/flows.rs/tree/master/lib/#{package_name})"
      package_summary = project[:package][:description].gsub("Building blocks for flow", 'Flow')
      package_links = [
        "[![Package](https://img.shields.io/crates/v/#{package_name})](https://crates.io/crates/#{package_name})",
        "[![Documentation](https://img.shields.io/docsrs/#{package_name}?label=docs.rs)](https://docs.rs/#{package_name})",
      ]
      out.puts "| " + [
        package_link,
        package_summary,
        package_links[0],
        package_links[1],
      ].join(" | ") + " |"
    end
    out.string
  end
end

def generate_json(input_paths)
  JSON.pretty_unparse(load_projects(input_paths))
end

def load_projects(input_paths)
  input_paths.map do |input_path|
    Tomlrb.load_file(input_path, symbolize_keys: true)
  end
end
