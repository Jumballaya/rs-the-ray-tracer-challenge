const color = (hex) => {
  let to_float = s => parseInt(s, 16) / 255;
  const clean_hex = h => h.slice(0, 1) === '#' ? h.slice(1) : h.trim();
  const chunk_hex_string = s => [s.slice(0, 2), s.slice(2, 4), s.slice(4,6)];
  const hex_to_array = s => chunk_hex_string(clean_hex(s)).map(to_float);
  const num_to_float = n => n % 1 == 0 ? `${n}.0` : n.toString();
  const parse_hex_array = arr => arr.map(num_to_float)
  const hex_aray_str = arr => `(${arr.reduce((acc, cur) => acc + ', ' + cur)})`;
  const hex_to_color = hex => `Color::new${hex_aray_str(parse_hex_array(hex_to_array(hex)))};`;
  return hex_to_color(hex);
}


const usage = `
Usage: node hex_to_color.js HEX_STRING

Hex String to Color::new call

Examples:

    $ node hex_to_color.js 1a2b3c

    Or with a leading # sign

    $ node hex_to_color.js \\#1a2b3c
`;


const input = process.argv;

if (input.length < 3) {
  console.log(usage);
  process.exit(0);
}
const hex = input[2];
if (hex.length < 6 || hex.length > 7) {
  throw new Error('Must provide correct hex string format: FFFFFF or \\#FFFFFF');
}

console.log('\n', color(hex), '\n');