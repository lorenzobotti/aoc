import { range } from './utils.ts'

type BinaryDigit = number[]

function parseDigit(line: string): BinaryDigit {
    return range(0, line.length)
        .map(i => line[i])
        .map(n => n == '0' ? 0 : 1)
}

function mostPopular(within: BinaryDigit[], at: number): number {
    let ones = 0
    let zeroes = 0

    for (const digit of within) {
        digit[at] == 0 ? zeroes++ : ones++
    }

    if (ones == zeroes) {
        return 1
    }
    return ones > zeroes ? 1 : 0
}

function leastPopular(within: BinaryDigit[], at: number): number {
    let ones = 0
    let zeroes = 0

    for (const digit of within) {
        digit[at] == 0 ? zeroes++ : ones++
    }

    if (ones == zeroes) {
        return 0
    }
    return zeroes > ones ? 1 : 0
}


function keepMostPopular(within: BinaryDigit[], at: number): BinaryDigit[] {
    const keep = mostPopular(within, at);
    return within.filter(digit => digit[at] == keep)
}

function keepLeastPopular(within: BinaryDigit[], at: number): BinaryDigit[] {
    const keep = leastPopular(within, at);
    return within.filter(digit => digit[at] == keep)
}

function toDecimal(digit: BinaryDigit): number {
    let out = 0;
    
    for (let i = digit.length - 1; i >= 0; i--) {
        const bit = digit[i];
        const spot = digit.length - 1- i;


        if (bit == 1) {
            out += Math.pow(2, spot)
        }
    }

    return out
}

export type {
    BinaryDigit,
}

export {
    parseDigit,
    keepMostPopular,
    keepLeastPopular,
    toDecimal,
}