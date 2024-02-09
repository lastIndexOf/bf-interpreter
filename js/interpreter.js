function run(filename, input = "") {
  const code = require("fs").readFileSync(filename, "utf8");

  // @1
  const inputChars = input.split(""); // @2

  const codes = code.split(""); // @3
  let codeIdx = 0;

  const arr = []; // @4
  let arrIdx = 0;
  let outputStr = ""; // @5
  let output = [];

  while (codeIdx < code.length) {
    // @6
    const ops = codes[codeIdx];

    const handleLeftBracket = () => {
      // @7
      if (~~arr[arrIdx] === 0) {
        let cnt = 1;

        while (cnt) {
          codeIdx++;
          if (codes[codeIdx] === "[") {
            cnt += 1;
          }
          if (codes[codeIdx] === "]") {
            cnt -= 1;
          }
        }
      }
    };

    const handleRightBracket = () => {
      // @8
      if (~~arr[arrIdx] !== 0) {
        let cnt = 1;

        while (cnt) {
          codeIdx--;
          if (codes[codeIdx] === "]") {
            cnt += 1;
          }
          if (codes[codeIdx] === "[") {
            cnt -= 1;
          }
        }
      }
    };

    switch (
      ops // @9
    ) {
      case ">":
        arrIdx += 1;
        break;
      case "<":
        arrIdx -= 1;
        break;
      case "+":
        arr[arrIdx] = (~~arr[arrIdx] + 1) % 256;
        break;
      case "-":
        arr[arrIdx] = (~~arr[arrIdx] || 256) - 1;
        break;
      case ",":
        const iptChar = inputChars.shift();
        arr[arrIdx] = iptChar ? iptChar.charCodeAt(0) : arr[arrIdx];
        break;
      case ".":
        output.push(arr[arrIdx]);
        if (arr[arrIdx] === 10) {
          process.stdout.write(Buffer.from(output));
          output = [];
        }
        break;
      case "[":
        handleLeftBracket();
        break;
      case "]":
        handleRightBracket();
        break;
    }

    codeIdx++; // @10
  }

  return outputStr; // @11
}

module.exports = run;
