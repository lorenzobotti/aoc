import sys



class Sequence:
    start: str = ""
    end: str = ""
    pairs: dict[(str, str), int] = {}
    recipes: dict[str, str]

    def __init__(self, input: str, recipes: str):
        p: list[(str, str)] = pairs(input)

        for pair in p:
            if self.pairs.get(pair) == None:
                self.pairs[pair] = 1
            else:
                self.pairs[pair] += 1


        self.start = input[0]
        self.end = input[len(input) - 1]
        self.recipes = parse_recipes(recipes)

    def iteration(self):
        next_gen: dict[(str, str), int] = {}
        for pair, amount in self.pairs.items():
            one, two = multiply_pair(pair, self.recipes)

            if next_gen.get(one) == None:
                next_gen[one] = 0
            if next_gen.get(two) == None:
                next_gen[two] = 0
            
            
            next_gen[one] += amount
            next_gen[two] += amount

        self.pairs = next_gen


    def count(self) -> dict[str, int]:
        out: dict[str, int] = {}
        for pair, amount in self.pairs.items():
            one = pair[0]
            two = pair[1]

            if out.get(one) == None:
                out[one] = 0
            if out.get(two) == None:
                out[two] = 0
            
            out[one] += amount
            out[two] += amount

        out[self.start] += 1
        out[self.end] += 1

        for letter, amount in out.items():
            out[letter] = amount // 2

        return out

    def most_and_least_common(self) -> (int, int):
        least = sys.maxsize
        most = 0

        for letter, amount in self.count().items():
            if amount > most:
                most = amount
            if amount < least:
                least = amount
            
        return (most, least)





def main():
    file = open("input.txt")
    contents = file.read()

    parts: list[str] = contents.split("\n\n")
    start: str = parts[0]
    synthesis: list[str] = parts[1].split("\n")

    sequence = Sequence(start, synthesis)
    
    for i in range(40):
        sequence.iteration()

    most, least = sequence.most_and_least_common()
    print(most - least)





def parse_recipes(input: str) -> dict[str, str]:
    recipes: dict[str, str] = {}

    for synth in input:
        recipe_part: list[str] = synth.split(" -> ")

        parents: str = recipe_part[0]
        child: str = recipe_part[1]
        recipes[parents] = child

    return recipes



def count_pairs(input: str) -> dict[(str, str), int]:
    out: dict[(str, str), int] = {}
    
    for pair in pairs(input):
        if out.get(pair) == None:
            out[pair] = 1
        else:
            out[pair] += 1

    return out



def pairs(input: str) -> list[(str, str)]:
    out: list[(str, str)] = []

    for i, letter in enumerate(input[1:]):
        previous = input[i]

        pair: str = previous + letter
        out.append(pair)

    return out



def multiply_pair(pair: str, recipes: dict[str, str]) -> (str, str):
    child = recipes[pair]

    one = pair[0] + child
    two = child + pair[1]

    return (one, two)


def interpolate(start: str, recipes: dict[str, str]) -> str:
    out: str = start[0]
    for i, letter in enumerate(start[1:]):
        previous = start[i]

        pair = previous + letter
        child = recipes[pair]

        # print("i:", i)
        # print("pair:", pair)
        # print("child:", child)
        
        out += child + letter
    
    return out

def most_and_least_common(input: str) -> (int, int):
    letters: dict[str, str] = {}
    for letter in input:
        if letters.get(letter) == None:
            letters[letter] = 1
        else:
            letters[letter] += 1    


    most_common: int = 0
    least_common: int = sys.maxsize

    for letter, amount in letters.items():
        if amount > most_common:
            most_common = amount
        if amount < least_common:
            least_common = amount
        
    return (most_common, least_common)


if __name__ == "__main__":
    main()