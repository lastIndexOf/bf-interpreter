const path = require("path");
const cwd = process.cwd();

require("./interpreter")(path.resolve(cwd, process.argv[2]));
