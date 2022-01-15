class SnailNumber
    property value : Int32 | Tuple(SnailNumber, SnailNumber)

    def initialize(input : String)
        @value = 0
        
        input.chars.each_with_index do |ch, i|
            if ch == '['
                
            elsif ch.number?
                
            end
        end
    end
    
    def initialize(input : Array(Char))
        @value = 0
        
        if input[0].number?
            num = parse_number input
        end
    end

    def self.parse_numberic(input : String) : Tuple(Int32, Int32)
        last = 0
        input.chars.each_with_index do |ch, i|
            if ch.number?
                last = i
            else
                break
            end
        end

        if last == 0 
            return {0, 0}
        end
        
        num = input[..last]
        puts num
        return {num.to_i, last}
    end
end