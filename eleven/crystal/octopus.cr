require "./floor.cr"

class Octopus
    property level
    property flashed_this_turn

    def initialize(@level : Int32)
        @flashed_this_turn = false
    end

    def tick
        # puts @flashed_this_turn
        @level += 1

        if @level == 10
            if can_flash
                @flashed_this_turn = true
                return true
            end
            return false
        else 
            return false
        end
    end

    def can_flash
        !@flashed_this_turn
    end

    def reset
        if @flashed_this_turn
            @level = 0
        end

        @flashed_this_turn = false
    end
end