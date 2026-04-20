const fs = require('fs');

function checkPalindrome3(x) {
    if (x < 100 || x > 999) return "-";
    const a = Math.floor(x / 100);
    const c = x % 10;
    return a === c ? "Yes" : "No";
}

let n, nums;

if (fs.existsSync('input.txt')) {
    const data = fs.readFileSync('input.txt', 'utf8').trim().split(/\s+/).map(Number);
    n = data[0];
    nums = data.slice(1);
} else {
    const readline = require('readline-sync');
    n = Number(readline.question('Введите количество чисел N: '));
    nums = readline.question('Введите числа: ').split(/\s+/).map(Number);
}

const results = [];
for (let i = 0; i < n; i++) {
    results.push(checkPalindrome3(nums[i]));
}

const output = results.join(' ');
console.log(output);
fs.writeFileSync('output.txt', output + '\n');
