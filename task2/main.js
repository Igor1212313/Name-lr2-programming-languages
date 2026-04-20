const fs = require('fs');

function minRefuels(n, v, r) {
    for (const dist of r) {
        if (dist > v) return 0;
    }

    let fuel = v;
    let refuels = 0;

    for (let i = 0; i < n - 1; i++) {
        if (fuel < r[i]) return 0;

        fuel -= r[i];

        if (i < n - 2 && fuel < r[i + 1]) {
            refuels++;
            fuel = v;
        }
    }

    return refuels;
}

let n, v, r;

if (fs.existsSync('input.txt')) {
    const data = fs.readFileSync('input.txt', 'utf8').trim().split(/\s+/).map(Number);
    n = data[0];
    v = data[1];
    r = data.slice(2);
} else {
    const readline = require('readline-sync');
    n = Number(readline.question('Введите количество планет N: '));
    v = Number(readline.question('Введите емкость бака V: '));
    r = readline.question('Введите расстояния между соседними планетами: ').split(/\s+/).map(Number);
}

const result = minRefuels(n, v, r);

console.log('Результат:', result);
fs.writeFileSync('output.txt', result + '\n');
