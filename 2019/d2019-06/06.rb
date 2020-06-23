require 'pry'


data = ARGF.each_line.to_a.map(&:strip)

bodies = {}

data.each do |line|
	body, orbiter = line.split(")")

	bodies[body] = [] unless bodies.key?(body)
	bodies[orbiter] = [] unless bodies.key?(orbiter)

	bodies[orbiter] << body
end

def count(registry, body)
	sum = registry[body].map do |body|
		1 + count(registry, body)
	end.inject(:+)

	if !sum
		sum = 0
	end

	sum
end

count = bodies.map do |body, _|
	count(bodies, body)
end.inject(:+)

puts "Part 1: #{count}"
