# TermUI-graf
A simple interface for displaying terminal graphs with the 'textplots' crate.
The terminal will prompt the user for the following input: 
- selecting a single graph type,
- the X and Y plot display dimensions (measured in dpi, I think),
- the option to define a single custom RGB color (or use default),
- the number of data points to display on the graph,
- the data point values to display 

To run: 
A. create a new bin 'cargo new TermUI-graf --bin'   B. Replace the generated main.rs (Hello, World!) and cargo.toml with the ones in this repo.   C. cargo run

THIS IS WIP! I have many ideas for enhanced functionality before delving into the textplot crate (and its dependent 'drawille'), such as file parsing (xml seems popular so some kind of formatting for it..), multicolor and multiplot schemes. These are noted in the Comments of the main.rs itself. The current implementation is very basic, stay tuned!
