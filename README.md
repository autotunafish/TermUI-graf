# TermUI-graf
A simple interface for displaying terminal graphs with the 'textplots' crate.
Use a Mono based font.
The terminal will prompt the user for the following input: 
- selecting a single graph type,
- the X and Y plot display dimensions (measured in dpi, I think),
- the option to define a single custom RGB color (or use default),
- the number of data points to display on the graph,
- the data point values to display 

To run: 
A. create a new bin 'cargo new TermUI-graf --bin'   B. Replace the generated main.rs (Hello, World!) and cargo.toml with the ones in this repo.   C. cargo run

THIS IS WIP! I have many ideas for enhanced functionality before delving into the textplot crate (and its dependent 'drawille'), such as file parsing (xml seems popular so some kind of formatting for it..), multicolor and multiplot schemes. These are noted in the Comments of the main.rs itself. 
However, the limitations of he terminals ability to display colors hinders this prospect. For example, if the top of a bar and another line are printed in close-enough proximity, then they will share the same place on a single line and will both be given the last color assigned in the function chain for drawing the image. This is because that FOREGROUND placeholder can only be one color. The color of the BACKGROUND can be of a single different color but this will require researching the textplot crate functionality and its dependents (seems more like highlighting but maybe..). Will update.
