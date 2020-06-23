require 'pry'

lines = ARGF.each_line.to_a

components = lines.map(&:to_f)

fuel_requirements = components.map do |component|
	(component / 3).floor - 2.0
end

fuel_sum = fuel_requirements.inject(:+)

puts "Part 1: #{fuel_sum.to_i}"

new_fuel_requirements = components.map do |component|
	component_fuel_rqmt = (component / 3.0).floor - 2.0

	chunk = [component_fuel_rqmt]

	until (n = (chunk.last / 3.0).floor - 2.0) < 0
		chunk << n
	end

	chunk.inject(:+)
end

total = new_fuel_requirements.inject(:+)

puts "Part 2: #{total.to_i}"
