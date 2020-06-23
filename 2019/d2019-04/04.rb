# You arrive at the Venus fuel depot only to discover it's protected by a
# password. The Elves had written the password on a sticky note, but someone
# threw it out.
#
# However, they do remember a few key facts about the password:
#
# - It is a six-digit number.
# - The value is within the range given in your puzzle input.
# - Two adjacent digits are the same (like 22 in 122345).
# - Going from left to right, the digits never decrease; they only ever
#   increase or stay the same (like 111123 or 135679).
#
# Other than the range rule, the following are true:
#
# - 111111 meets these criteria (double 11, never decreases).
# - 223450 does not meet these criteria (decreasing pair of digits 50).
# - 123789 does not meet these criteria (no double).
#
# How many different passwords within the range given in your puzzle input meet
# these criteria?

bounds = ARGF.each_line.to_a.map(&:strip).first.split('-').map(&:to_i)
range = Range.new(bounds.first, bounds.last)

class Integer
	def six_digit?
		digits.count == 6
	end

	def two_adjacent_digits?
		digits = self.digits
		digits.each_with_index.any? do |digit, idx|
			if idx-1 >= 0
				digits[idx-1] == digit
			else
				false
			end
		end
	end

	def exactly_two_adjacent_digits?
		digits = self.digits
		counts = digits.map{|digit| digits.count(digit)}
		counts.any? {|ct| ct == 2}
	end

	def monotonic?
		digits = self.digits.reverse
		digits.each_with_index.all? do |digit, idx|
			if idx-1 >= 0
				digits[idx-1] <= digit
			else
				true
			end
		end
	end
end

actual_range = range.filter do |x|
	x.monotonic?
end.filter do |x|
	x.two_adjacent_digits?
end

puts "Part 1: #{actual_range.count}"

stricter = actual_range.filter do |x|
	x.exactly_two_adjacent_digits?
end

puts "Part 2: #{stricter.count}"
