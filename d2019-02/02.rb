require 'pry'

lines = ARGF.each_line.to_a
program = lines[0].split(',').map(&:to_i)

# Patch
program[1] = 12
program[2] = 2

def run(program)
	idx = 0

	loop do
		case program[idx]
		when 1
			# puts "i#{idx}: Got #{program[idx]} (ADD) #{program[idx+1]} (#{program[program[idx+1]]}) + #{program[idx+2]} (#{program[program[idx+2]]}) -> #{program[idx+3]} (#{program[program[idx+3]]} -> #{program[program[idx+1]]+program[program[idx+2]]})"
			program[program[idx+3]] = program[program[idx+1]] + program[program[idx+2]]
			idx += 4
		when 2
			# puts "i#{idx}: Got #{program[idx]} (MUL) #{program[idx+1]} (#{program[program[idx+1]]}) * #{program[idx+2]} (#{program[program[idx+2]]}) -> #{program[idx+3]} (#{program[program[idx+3]]} -> #{program[program[idx+1]]*program[program[idx+2]]})"
			program[program[idx+3]] = program[program[idx+1]] * program[program[idx+2]]
			idx += 4
		when 99
			# puts "i#{idx}: Got #{program[idx]} (HALT)"
			break
		else
			puts "Bad opcode: #{program[idx]} at #{idx}"
			break
		end

		break if idx > program.count
	end

	program[0]
end

prog = program.clone

puts "Part 1: #{run(prog)}"

def solve(program, value)
	(1..100).step(1).product((1..100).step(1)).collect do |noun, verb|
		prog = program.clone
		prog[1] = noun
		prog[2] = verb

		result = run(prog)

		return [noun, verb] if result == value
	end
end

noun, verb = solve(program, 19690720)

puts "Part 2: #{100 * noun + verb}"
