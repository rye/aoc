require 'pry'

data = ARGF.each_line.to_a.map(&:strip).first.chars

width = 25
height = 6

slices = data.each_slice(width * height)

slice_statistics = slices.map do |slice|
	{ slice: slice.join, counts: { zeroes: slice.count("0"), ones: slice.count("1"), twos: slice.count("2") } }
end


sorted_by_zeroes = slice_statistics.sort do |a, b|
	a[:counts][:zeroes] <=> b[:counts][:zeroes]
end

fewest_zeroes = sorted_by_zeroes.first

puts "Part 1: #{fewest_zeroes[:counts][:ones] * fewest_zeroes[:counts][:twos]}"

image = "2" * (width * height)

slices.each do |slice|
	slice.each_with_index do |elt, index|
		if image[index] == "2"
			image[index] = elt
		end
	end
end

final_image = image.chars.each_slice(width).map {|slice| slice.map{|c| unless c == "1"; ' '; else c; end}.join('')}.join("\n")

puts "Part 2: \n#{final_image}"
