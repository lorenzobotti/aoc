require "./octopus"

class Seafloor
    @floor : Array(Array(Octopus))
    @rows : Int32
    @cols : Int32
    getter flashes : Int32

    def initialize(input : String)
        @floor = Array(Array(Octopus)).new
        @flashes = 0

        input.each_line do |line|
            row = Array(Octopus).new

            line.each_char do |ch|
                val = ch - '0'
                row << Octopus.new(val)
                # row << val
            end

            @floor << row
        end
        

        @rows = @floor.size
        @cols = @floor[0].size
    end

    def get(row, col)
        @floor[row][col]
    end

    def set(row, col, val)
        @floor[row][col] = val
    end

    def neighbours(row, col)
        res = Array({Int32, Int32}).new

        (-1..1).each do |r|
            (-1..1).each do |c|
                if row + r >= 0 && row + r < @rows && col + c >= 0 && col + c < @cols && (r != 0 || c != 0)

                    res << {row + r, col + c}
                end
            end
        end

        res
    end

    def iteration
        @floor.each_with_index do |row, row_i|
            row.each_with_index do |col, col_i|
               tick_octopus(row_i, col_i)
            end
        end

        @floor.each do |row|
            row.each do |oct|
               oct.reset
            end
        end
    end

    def all_flashed
        @floor.each do |row|
            row.each do |oct|
                if oct.level != 0 
                    return false
                end
            end
        end

        return true
    end

    def tick_octopus(row, col)
        oct = get row, col
        # pp oct
        
        should_flash = oct.tick()
        # pp oct
        # p! should_flash

        if should_flash
            @flashes += 1
            # puts row, col, "should flash"
            neighbours(row, col).each do |coord|
                tick_octopus coord[0], coord[1]    
            end
        end
    end

    def print
        @floor.each_with_index do |row, row_i|
            row.each_with_index do |oct, col_i|
               print oct.level
            #    print " "
            end
            puts
        end
    end
end

