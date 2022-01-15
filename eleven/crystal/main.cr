require "./floor.cr"

input = File.new("input.txt").gets_to_end
floor = Seafloor.new input

# puts(floor.get 0, 2)
# floor.neighbours(0, 2).each do |coord|
#     puts coord
#     puts floor.get(coord[0], coord[1])
# end

i = 0
while true
    floor.iteration

    if floor.all_flashed
        puts i + 1
        break
    end

    i += 1
end

# floor.print
# puts floor.flashes