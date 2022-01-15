import { parse } from 'https://deno.land/std@0.117.0/flags/mod.ts'
import { parseDigit, keepMostPopular, keepLeastPopular, toDecimal } from './binary.ts'

async function readStdin(): Promise<string> {
    const params = parse(Deno.args);
    const inputFile = params.file ?? '../input.txt'
    const data = await Deno.readFile(inputFile)
    const decoder = new TextDecoder('utf-8')

    return decoder.decode(data)
}

const input = await readStdin();
const lines = input.split('\n')
    .map(line => parseDigit(line));

let mostPopular = [...lines]
let leastPopular = [...lines]

for (let i = 0; i < lines[0].length; i++) {
    if (mostPopular.length > 1) {
        mostPopular = keepMostPopular(mostPopular, i)
    }
    if (leastPopular.length > 1) {
        leastPopular = keepLeastPopular(leastPopular, i)
    }
}

const oxygen = toDecimal(mostPopular[0])
const co2 = toDecimal(leastPopular[0])

const solution = oxygen * co2
console.log({ solution })