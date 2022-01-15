function range(start: number, end: number): number[] {
    const ans = [];
    for (let i = start; i < end; i++) {
        ans.push(i);
    }
    return ans;
}

export {
    range,
}