function total_distance(left: number[], right: number[]): number {
    let total = 0;
    for (let i = 0; i < left.length; i++) {
        total += Math.abs(left[i] - right[i]);
    }

    return total;
}

function score(left: number[], right: number[]): number {
    let occurences = {};
    for (let num of right) {
        occurences[num] = occurences[num] ? occurences[num] + 1 : 1;
    }

    let score = 0;
    for (let num of left) {
        score += occurences[num] ? num * occurences[num] : num * 0;
    }

    return score;
}

async function main() {
    // part 1
    let input: string = await Deno.readTextFile("./input.txt");
    const lines = input.split("\n");

    let left: number[] = [];
    let right: number[] = [];
    for (let line of lines) {
        const parts = line.split("   ");
        const ln = parseInt(parts[0]);
        const rn = parseInt(parts[1]);

        if (isNaN(ln)) {
            continue;
        }
        left.push(ln);
        right.push(rn);
    }

    left = left.sort();
    right = right.sort();
    console.log(total_distance(left, right));

    // part 2
    console.log(score(left, right));
}

await main();
