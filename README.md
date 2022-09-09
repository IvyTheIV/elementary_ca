
# Elementary Cellular automata
a simple cli application to play around with 1-D cellular automata.

### Dependencies
`ctrlc, termion`

## Installation
<b>clone the repository, build & run</b>\
`git clone https://github.com/IvyTheIV/elementary_ca.git`\
`cd elementary_ca`\
`cargo run`

## Usage
`elementary-ca [<command> [<argument>]]`

`-h | --help` - help message\
`-r | --rule` - set the ruleset. Default ruleset is 90\
`-s | --speed` - set the speed of the simulation\
`-w | --width` - set the width. If unspecified, takes the width of the terminal window\
`-wr | --wrap-around` specifies whether cellular automata should wrap around the edges.
