require 'pry'

lines = ARGF.each_line.to_a.map(&:strip)
segments = lines.map{|line| line.split(',')}

def trace_wire(spec)
	wire = [[0, 0]]

	# "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"
	spec.each do |identifier|
		scan = identifier.scan(/[A-Za-z]+|\d+/)
		# => ["R", "98"], ...

		xv, yv = 0, 0

		case scan[0]
		when "R"
			xv = 1
		when "U"
			yv = 1
		when "L"
			xv = -1
		when "D"
			yv = -1
		end

		len = scan[1].to_i

		# Might need to be ...
		(0...len).each do |n|
			wire << [wire.last[0] + xv, wire.last[1] + yv]
		end
	end

	wire
end

first_wire = trace_wire(segments[0])
second_wire = trace_wire(segments[1])

intersections = (first_wire & second_wire) - [[0, 0]]

def manhattan_distance_from_origin(point)
	point[0].abs + point[1].abs
end

puts "Part 1: #{(intersections.map{ |point| manhattan_distance_from_origin(point) } - [0]).min}"

def steps_to_intersection(wire, intersection)
	wire.index(intersection)
end

steps_to_all_intersections = intersections.map do |intersection|
	first_steps = steps_to_intersection(first_wire, intersection)
	second_steps = steps_to_intersection(second_wire, intersection)

	first_steps + second_steps
end

puts "Part 2: #{steps_to_all_intersections.min}"
