/*

* * * * * * * "TermUI-graf mk1 '24" * * * * * * * *
A simple interface for displaying terminal graphs with the 'textplots' crate.
by John Bruhling
License: CC-0
https://docs.rs/textplots/0.8.6/textplots/index.html

⡁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⡏⠉⠉⠉⡇⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⡁ 78.0
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂ 68.2
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡁
⠄⠀⠀⠀⢠⠤⠤⠤⢤⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄ 58.5
⠂⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂
⡁⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡁ 48.8
⠄⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄
⠂⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂ 39.0
⡁⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡁
⠄⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄ 29.2
⡏⠉⠉⠉⢹⠀⠀⠀⢸⠤⠤⠤⠤⡇⠀⠀⠀⡇⠀⠀⠀⢸⠉⠉⠉⢹⠀⠀⠀⠀⠂
⡇⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡧⠤⠤⠤⢼⠀⠀⠀⢸⠀⠀⠀⠀⡁ 19.5
⡇⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⢸⠤⠤⠤⠤⡄
⡇⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇ 9.8
⡇⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠀⢸⠀⠀⠀⢸⠀⠀⠀⠀⡇
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁ 0.0
0.0                        7.0


All displaced function definitions are included at the bottom.

The Program will prompt the user to either manually enter graph parameters,
or TODO select a properly formatted xml file. EXACT FORMAT TBD and select a
single custom color for one of four graph types.

Commented out println statements and certain dependancies are left in for potential
future use, reuse and in troubleshooting, plz ignore XP.

*/
//use std::fmt::format;
//use chrono::{Duration, NaiveDate};
use rgb::RGB8;

//MAY NEED  use textplots::{utils, LabelBuilder, LabelFormat}
use textplots::{Chart, ColorPlot, Plot, Shape, TickDisplay, TickDisplayBuilder};

//TODO ADD FILE READ FOR PARAM LOADING.
//use std::fs::File;
//use std::process;

// MAY NEED std::{thread, time}? idk
use std::io;

fn main() {
    //These values will be used for the graph parameters taken from stdin.
    //TODO ADD FUNCTIONALITY TO GRAB A FILE AND LOAD A GRAPH AND ALL OPTIONS!
    //TODO COSMETICS
    //////////////////////////////
    //////////////////
    /////////
    /////
    //
    //_goodxval is the Graph X value/Horizontal length.
    let mut _goodxval = String::new();

    //_goodyval is the Graph Y value/Vertical Height.
    let mut _goodyval = String::new();

    //goodcval is a call for a color scheme selection, "Y" or "N".
    let mut goodcval = String::new();

    //_gooddpoint is the number of data points to display entered by the user.
    let mut _gooddpoint = String::new();

    //_upuserinput is the user input graph selection of a single char e.g. 'L', 'B'.
    let mut _upuserinput = String::new();

    //goodrender is a user input for borders. UNUSED, Currently default Yes.
    //This would determine .draw() or .nice() as the render method.
    //let mut goodrender = String::new();

    //_goodxvalnum is the checked and parsed u32 version of the String input for goodxval.
    //TODO THIS VALUE TO CHECK AGAINST peakpoint AND GET A GOOD TICK MEASUREMENT
    let mut _goodxvalnum: u32 = 0;

    //_goodyvalnum is the checked and parsed u32 version of the String input for _goodyval.
    let mut _goodyvalnum: u32 = 0;

    //_gooddpointnum is the parsed u32 version of the String input for _gooddpoint.
    let mut _gooddpointnum: u32 = 0;

    //_datacounter is a sequential count of the total datapointbits found while checking before parse.
    //It is used to set the graph total point index as the default X range.
    //TODO CONSIDER MAKING CUSTOMIZABLE TO DISPLAY SUBSETS OF LARGER GRAPHS.
    let mut _datacounter: f32 = 1.0;

    //peakpoint is a copy of the largest datapointbit value accquired.
    //It is used for setting the y_tick_display to a desired value or otherwise relative to the maximum.
    //TODO FOR MULTIPLOTS, CONSIDER THE PEAKPOINTS OF EVERY DATA POINT INDEX FOR COLOR OPTIMIZATIONS
    //(MAY NOT BE NECESSARY EXCEPT FOR LARGE GRAPHS, IDK)
    let mut peakpoint: f32 = 0.0;

    //_goodcolornum holds the values of the custom rgb values 0-255 of a u8 type.
    let mut _goodcolornum: u8 = 0;

    //_goodcpoint holds the potential color value to be parsed after (double) type checking.
    let mut _goodcpoint = String::new();

    //Forgot what this was for, probably un-needed but will leave for now.
    //let mut indexofpoint: f32 = 0.0;

    //arrayofpoints is the user provided datapoint set.
    //This creates an array for (_datacounter and _datacluster)'s, the zeroeth at 0.0 is req'd for proper rendering.
    //REQUIRES A CONST SO MAKE AN ARRAY LARGE ENOUGH... MAYBE 1000? 10,000?
    //TODO CHANGE? NOT IDEAL BUT SEEMS NOT TOO UNCOMMON OF A GENERAL ISSUE, THINK IT OUT.
    //TODO WILL NEED TO CLONE, REUSE etc FOR MULTIPLOTS
    let mut arrayofpoints = vec![(0.0, 0.0); 10000];
    //Use split_at to remove the unused part of the array, if any.
    //THE SPLIT WILL AFFECT THE ABILITY TO USE DATASETS LARGER THAN THE PREVIOUS! THINK IT OUT.

    //arrgeebee will u8 hold color definitions of R, G, B.
    //TODO DEFINE METHODS AND PRE DETERMINED SCHEMES FOR COLORATIONS.
    //CUSTOMIZABLE IS COOL BUT PALLETTE SETS ARE NICE TOO.
    let mut arrgeebee = [0 as u8; 3];

    //The outer loop spans the user's input of the graph type and paramters.
    //The inner loops span those individual processes.
    //This is done so that if a wrong input occurs, then it also doesn't nuke any/all prior and correct input.
    //TODO LABEL LOOPS BETTER. SHADOWING YEAH BUT I MEAN CMON
    'outer: loop {
        //Prompt the user to select the graph type.
        'inner: loop {
            println!(
            "\x1b[48;5;232m\x1b[38;5;191m* * * * * * * \"TermUI-graf mk1 '24\" * * * * * * * *\x1b[0m"
        );
            println!("\x1b[48;5;232m\x1b[38;5;154m* Select the graph type  *\x1b[0m");
            println!("\x1b[48;5;232m\x1b[38;5;155m* [B]ar [L]ine [P]oints [S]tep *\x1b[0m");

            //Create a variable for the input.
            _upuserinput = getcapstring();

            //Check if the user entered anything AND entered a single char. If not, reprompt.
            if _upuserinput.len() != 1 {
                println!("\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m");
                continue 'inner;
            }

            //If the input is properly formatted, match to a recognized char. If no match, reprompt.
            //println!("2 {}", &_upuserinput);
            match _upuserinput.as_str() {
                "P" => println!("\x1b[48;5;232m\x1b[38;5;154m* You selected Points *\x1b[0m"),
                "B" => println!("\x1b[48;5;232m\x1b[38;5;154m* You selected Bar *\x1b[0m"),
                "L" => println!("\x1b[48;5;232m\x1b[38;5;154m* You selected Line *\x1b[0m"),
                "S" => println!("\x1b[48;5;232m\x1b[38;5;154m* You selected Step *\x1b[0m"),
                _ => {
                    println!(
                        "\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m"
                    );
                    continue 'inner;
                }
            }

            break 'inner;
        }

        //Prompt for the X value.
        //TODO PROMPT FOR X AND Y RANGES, Y IS MORE USEFUL BUT BOTH SHOULD BE ACCESSIBLE
        //(I THINK THIS IS IN DPI)
        'inner: loop {
            println!("\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
            println!("\x1b[48;5;232m\x1b[38;5;154m* Select the X length, min 32, (line below is 120) *\x1b[0m");
            println!("\x1b[48;5;232m\x1b[38;5;191m⡁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⡁\x1b[0m");

            //User input for the X value.
            let xval = getcapstring();

            //If the user enters nothing, retry.
            if xval.len() == 0 {
                continue 'inner;
            }

            //If parsecheck is true then this value will be parsed from a string to a u32.
            _goodxval = xval.clone();

            //Check if the input is valid.
            if parsecheck(xval) == true {
                //Parse the correct value to an integer.
                _goodxvalnum = _goodxval.parse::<u32>().unwrap();

                println!("X {}", _goodxval);
                if _goodxvalnum <= 31 {
                    println!("\x1b[48;5;232m\x1b[38;5;191m* Too Small! minimum 32, please try again *\x1b[0m");

                    continue 'inner;
                }

                break 'inner;
            } else {
                //If the input is invalid, retry.
                println!("\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m");
                continue 'inner;
            }
        }

        'inner: loop {
            //Prompt for the Y value.
            //The REAL minimum is actually 3, but the TICK intervals go by 8's as 3+1(1),12(2),20(3),28(4),36(5),44(6)..
            //3 makes it weird, 4 is better. That is to say that the intervals are the spaces BETWEEN TickDisplay values.
            //This really only for me for making better TICK calculations for the user so you're welcome.
            println!("\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
            println!("\x1b[48;5;232m\x1b[38;5;154m* Select the Y height (min 8) *\x1b[0m");
            //User input for the Y value.
            let yval = getcapstring();

            //If the user enters nothing, retry.
            if yval.len() == 0 {
                continue 'inner;
            }

            //If parsecheck is true then this value will be parsed from a string to a u32.
            _goodyval = yval.clone();

            //Check if the input is valid.
            if parsecheck(yval) == true {
                //Parse the correct value to an integer.
                _goodyvalnum = _goodyval.parse::<u32>().unwrap();
                println!("Y {}", _goodyval);
                if _goodyvalnum <= 8 {
                    println!("\x1b[48;5;232m\x1b[38;5;191m* Too Small! minimum 8, please try again *\x1b[0m");
                    continue 'inner;
                }

                break 'inner;
            } else {
                println!("\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m");
                continue 'inner;
            }
        }

        'inner: loop {
            //Prompt for the Color value.
            println!(
            "\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m"
        );
            println!("\x1b[48;5;232m\x1b[38;5;154m* To select a custom color, Enter Y  *\x1b[0m");
            println!(
                "\x1b[48;5;232m\x1b[38;5;154m* Enter any other input to use the Default  *\x1b[0m"
            );
            //User input for the "Y" String.
            let cval = getcapstring();
            //Check if the input is valid.
            if cval == "Y" {
                println!(
                    "\x1b[48;5;232m\x1b[38;5;191m* You selected a custom color option! *\x1b[0m"
                );

                goodcval.push('Y');

                //TODO PRINT COLOR OPTIONS AND A NUMBER TO ENTER
                //SELECT COLOR SCHEMES PERHAPS
                //OTHERWISE ONLY SUPPORTS ONE COLOR PER PLOT SO MULTIPLOTS ARE NEEDED

                'subinner: loop {
                    println!("\x1b[48;5;232m\x1b[38;5;154m* Select the RGB values  *\x1b[0m");
                    println!(
                    "\x1b[48;5;232m\x1b[38;5;154m* Enter whole values between 0 and 255 *\x1b[0m"
                );
                    println!("\x1b[48;5;232m\x1b[38;5;191m* Use the format 'R value, G value, B value' *\x1b[0m");
                    println!("\x1b[48;5;232m\x1b[38;5;154m* Comma seperated, whitespaces are optional, decimals ignored *\x1b[0m");
                    println!("\x1b[48;5;232m\x1b[38;5;191m* <Reference> black:0,0,0  red:255,0,0  green:0,255,0  yellow:255,255,0 *\n* blue:0,0,255  magenta:255,0,255  cyan:0,255,255  white:255,255,255 *\x1b[0m");

                    //Do the same check as below on commas, ignore decimals and whitespaces.
                    //colorpoint counts the number of good values collected.
                    let mut colorpoint = 0;

                    //This breaks the loop after the final, good value is collected.
                    'rgbloop: loop {
                        if colorpoint == 3 {
                            break 'rgbloop;
                        }

                        //Letter placeholders for the prompt given by colorpoint.
                        let letters = ['R', 'G', 'B', '?'];
                        println!("{}:", letters[colorpoint]);

                        //input for the color value.
                        let mut cpoint = String::new();

                        //Get the User input.
                        io::stdin().read_line(&mut cpoint).unwrap();

                        //Remove newline
                        cpoint.pop();
                        //println!("initialpoints {:?}", cpoint);

                        //If the user entered nothing, retry.
                        if cpoint.len() == 0 {
                            continue 'rgbloop;
                        }

                        //If all passes check out, then this value will be parsed from a string to a u8
                        _goodcpoint = cpoint.clone();

                        //String is not an iterator so we have to remove one-by-one
                        if parsecheck(cpoint) == true {
                            //maybecnum is a placeholder to check for proper type before parsing to a u8 which could more easily panic.
                            let maybecnum = _goodcpoint.parse::<u32>().unwrap();

                            //_arrayindex = _gooddpointnum.clone();
                            //println!("points {}", _goodcpoint);
                            if maybecnum > 255 {
                                println!("\x1b[48;5;232m\x1b[38;5;191m* Too Big! maximum 255, please try again *\x1b[0m");
                                continue 'rgbloop;
                            }
                            //Parse the correct value to an integer.
                            _goodcolornum = _goodcpoint.parse::<u8>().unwrap();

                            //Push the value into the index given by colorpoint.
                            arrgeebee[colorpoint] = _goodcolornum;

                            //Increment the count and continue.
                            colorpoint = colorpoint + 1;

                            continue 'rgbloop;
                        } else {
                            //If the user entered a non ascii_digit, retry.
                            println!("\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m");
                            continue 'rgbloop;
                        }
                    }

                    //println!("{:?}", arrgeebee);
                    //Put values into arrgeebee
                    //Use in linecolorplot fn as values

                    //TODO BEFORE BREAK, DISPLAY COLOR AND ASK TO RETRY IF NOT GOOD. IF USER SELECTS AGAIN, CONTINUE 'subinner
                    break 'subinner;
                }

                //And then eventually break

                break 'inner;
            } else {
                //If the user entered anything other than 'Y', no color.
                println!("\x1b[48;5;232m\x1b[38;5;191m* You selected Default color *\x1b[0m");
                goodcval.push('N');
                break 'inner;
            }
        }

        //TODO ALLOW OPTION FOR BORDERS, CURRENTLY DEFAULT YES
        /*
                //Prompt for the Borders.
                println!(
                    "\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m"
                );
                println!("\x1b[48;5;232m\x1b[38;5;154m* To use borders, Enter Y  *\x1b[0m");
                println!(
                    "\x1b[48;5;232m\x1b[38;5;154m* Enter any other input to use the Default  *\x1b[0m"
                );

                'inner: loop {
                    //User input for the "Y" String.
                    let mut rval = getcapstring();
                    //Check if the input is valid.
                    if rval == "Y" {
                        println!("You selected borders!");
                        goodrender.push_str(".nice()");

                        //DO STUFF!
                        //'goodcval' will need to carry 3 values somehow..

                        //And then eventually
                        break 'inner;
                    } else {
                        println!("You selected NO borders");
                        goodrender.push_str(".display()");
                        break 'inner;
                    }
                }
        */

        //All graph parameters are captured.
        break 'outer;
    }

    //TODO MAKE THIS MANUAL OR READ FROM A FILE, SELECTION!
    //CMD ARG FLAGS POINTING TO A FILE WOULD BE SLICK!
    //
    /*
        //PROMPT AN OPTION FOR MANUAL OR FILE ENTRY

        println!("\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m");
        println!("\x1b[48;5;232m\x1b[38;5;154m* Enter the data points file to parse COMING SOON! currently no-use *\x1b[0m");


        let mut dfile = String::new();

        //Get the User input.
        io::stdin().read_line(&mut dfile).unwrap();

        //Remove newline
        dfile.pop();
        println!("filename: {:?}", dfile);

        //let mut dtext = fs::read_to_string(&dfile).unwrap();
        //println!("{}", dtext);
    */

    //CANT REMEMEBER WHY I MADE THIS, WILL LEAVE IN FOR NOW
    //let _arrayindex;

    //Prompt for the data points to be entered manually.

    'inner: loop {
        println!(
            "\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m"
        );
        println!(
            "\x1b[48;5;232m\x1b[38;5;154m* Enter the number of data points to diplay *\x1b[0m"
        );

        //User input string
        let dpoint = getcapstring();
        if dpoint.len() == 0 {
            continue 'inner;
        }

        //If all passes check out, then this value will be parsed from a string to a u32
        _gooddpoint = dpoint.clone();

        //String is not an iterator so we have to remove one-by-one
        if parsecheck(dpoint) == true {
            //Parse the correct value to an integer.
            _gooddpointnum = _gooddpoint.parse::<u32>().unwrap();
            //_arrayindex = _gooddpointnum.clone();
            //println!("points {}", _gooddpoint);

            break 'inner;
        } else {
            //If the user entered a non ascii_digit, retry.
            println!("\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m");
            continue 'inner;
        }
    }

    'outer: loop {
        println!(
            "\x1b[48;5;232m\x1b[38;5;191m* * * * * * * * * * * * * * * * * * * * * * * *\x1b[0m"
        );
        println!("\x1b[48;5;232m\x1b[38;5;154m* Enter the data point values *\x1b[0m");
        println!(
            "\x1b[48;5;232m\x1b[38;5;191m* Use the format 'value 1, value 2, value 3...' *\x1b[0m"
        );
        println!("\x1b[48;5;232m\x1b[38;5;154m* Comma seperated, whitespaces are optional, decimals ok *\x1b[0m");

        //This is the raw user input to be checked.
        let mut initialpoints = String::new();

        //Get the User input.
        io::stdin().read_line(&mut initialpoints).unwrap();

        //Remove newline
        initialpoints.pop();
        //println!("initialpoints {:?}", initialpoints);

        //If the user entered nothing, retry.
        if initialpoints.len() == 0 {
            continue 'outer;
        }

        //pointcounter governs the maximun number of data points collected during the search.
        let mut pointcounter = _gooddpointnum;

        //datachunk is the concatenation of the found ascii_digit (this includes decimals).
        let mut datachunk = String::new();

        //_datacluster is the checked and parsed f32 version of datachunk.
        let mut _datacluster: f32 = 0.0;

        //_datacounter is set to 1.0 if the following data entry below fails and returns to here.
        //This is used as the datapoint index in arrayofpoints as (_datacounter, _datacluster)
        _datacounter = 1.0;

        //decimalcount is a check on excessive decimals which themselves pass the is_ascii_digit, but cause parse to fail.
        let mut decimalcount = 0;

        //datapointbitstring is the corresponding datapoint index val in the arrayofpoints, as a String, before parsing.
        //This will become _datacluster in (_datacounter, _datacluster)
        let mut datapointbitstring = String::new();

        //Like with parsecheck(), we need to grab each value one at a time and validate as an integer.
        'inner: loop {
            //If the string is exhausted before the pointcounter, break.
            if initialpoints.len() == 0 {
                break 'inner;
            }
            //if the pointcounter is exhausted, break.
            if pointcounter == 0 {
                break 'inner;
            }

            //Grab the leading char in the string.
            let datapointbit = initialpoints.remove(0);
            //println!("{:?}  -  {:?}", pointcounter, datapointbit);

            //Match on what char was removed.
            //Whitespaces are ignored, all ascii_digits are pushed.
            //Commas indicate the end of the number collecting, everything else is evaluated.
            match datapointbit {
                ' ' => continue 'inner,
                '.' => {
                    //Push the decimal into datapointbit and increment the decimalcount. If 2, retry.
                    datachunk.push(datapointbit);
                    decimalcount = decimalcount + 1;
                    if decimalcount == 2 {
                        println!("\x1b[48;5;232m\x1b[38;5;191m* Found a second decimal: {} try again! *\x1b[0m", datachunk);

                        continue 'outer;
                    }
                    continue 'inner;
                }

                ',' => {
                    //A data point is finished being collected so decrement the pointcounter by 1.
                    pointcounter = pointcounter - 1;

                    //The value is collected and used as _datacluster.
                    datapointbitstring.push_str(&datachunk);
                    //datapointbitstring.pop();

                    //If the datachunk was empty, skip and continue.
                    if datapointbitstring.len() == 0 {
                        continue 'inner;
                    }

                    //Convert the String to an integer.
                    _datacluster = datapointbitstring.parse::<f32>().unwrap();

                    //Sets the Highest value captured as a reference to the chart size, CURRENTLY UNUSED.
                    if peakpoint < _datacluster {
                        peakpoint = _datacluster;
                    }

                    //println!("_datacluster {:?} --  datapointbitstring {:?}\n -- _datacounter {:?}", _datacluster, datapointbitstring, _datacounter);

                    //Sets the tuple of the index and data point values in the arrayofpoints
                    arrayofpoints[_datacounter as usize] = (_datacounter, _datacluster);

                    //_datacounter is incremented for the next value.
                    _datacounter = _datacounter + 1.0;

                    //Relative values are reset for reuse.
                    datachunk.clear();
                    datapointbitstring.clear();
                    decimalcount = 0;
                    continue 'inner;
                }
                _ => {
                    //All ascii_digits found are pushed to datachunk.
                    //Found a letter or symbol? Exit the function with false and retry.
                    if datapointbit.is_ascii_digit() == false {
                        println!(
                            "\x1b[48;5;232m\x1b[38;5;191m* Unrecognized char, try again! *\x1b[0m"
                        );
                        continue 'outer;
                    }

                    //Here, get the digit char and collect to somewhere to string to parse later
                    datachunk.push(datapointbit);
                    //println!("found a decimal {:?}", datachunk);

                    //If the String is exhausted, take what was found and attempt usage.
                    if initialpoints.len() == 0 {
                        datapointbitstring.push_str(&datachunk);
                        _datacluster = datapointbitstring.parse::<f32>().unwrap();

                        if peakpoint < _datacluster {
                            peakpoint = _datacluster;
                        }

                        //println!("final _datacluster {:?} --  datapointbitstring {:?}",_datacluster, datapointbitstring);

                        //Sets the index and data point values in the arrayofpoints
                        arrayofpoints[_datacounter as usize] = (_datacounter, _datacluster);

                        //_datacounter is incremented for the next value.
                        //Relative values are reset for reuse.
                        _datacounter = _datacounter + 1.0;
                        datachunk.clear();
                        datapointbitstring.clear();
                    }
                }
            }

            continue 'inner;
        }

        //Split the arrayofpoints at the index of the set amount of datapoints or whatever N happened to be there.
        //THIS MAY NOT BE NECESSARY AS ANY 0.0 INDEX VALUES WILL BE IGNORED
        //THE FUNCTIONS ARE USING arrayofpoints INSTEAD OF lhalfarray!
        let datasplit: usize = _datacounter as usize;
        let (lhalfarray, _) = arrayofpoints.split_at(datasplit);

        //FOR TESTING!
        //Print it all.
        println!("goodxval {} \n_goodyval {} \ngoodcval {} \n_gooddpoint{}\narrgeebee {:?}\nlhalfarray {:?} ", _goodxval, _goodyval, goodcval, _gooddpoint, arrgeebee, lhalfarray);

        break 'outer;
    }

    //Lets build a graph!

    //TODO COLOR SCHEMES

    //TODO!! CREATE NEW impl ON Chart::new() THAT RECURSES lineplot AND linecolorplot
    //AGAINST SOME COUNTER AND RETURNS SERIES OF lineplot OR linecolorplot fn's WITH APPROPRIATE TYPES.

    //UNTIL THEN..
    //TODO BUILD FUNCTIONS OF VERBOSE .linecolorplot().linecolorplot().linecolorplot()....
    //HOW WONDERFUL IS THAT?! THE NUMBER OF SUCCESSIONS SHOULD CORRELATE TO SOME COLOR SCHEME
    //THIS IMPLIES, FOR EXAMPLE, A 7-BAR 7-COLOR, SINGLE datapoint GRAPH WOULD HAVE 7 SEPERATELY COLORED BARS.
    //HOWEVER A SINGLE-COLORED, MULTI-ARRAY OVERLAY WOULDN'T BE TOO BAD, THINK IT OUT!
    //THAT IMPLIES FOR A 7-BAR, 7-COLOR, 7-datapoint GRAPH WOULD HAVE 7 SAME-COLOR BARS FOR 7 DIFFERENT COLOR SETS OF BARS.
    //(THIS SEEMS EASIER)
    //A PRE DETERMINED COLOR SCHEME WILL AT LEAST LIMIT THE EXPONENTIAL NESTING, SOMEWHAT. PERHAPS,
    //FIRSTLY THE BACKGROUND GRID THEN A X2, X3, X4... TO X7!? THATS A LOT BUT MIGHT BE NICE FOR A WEEKLY-GRAGH,
    //THIS BASIC VERSION YOUR LOOKING AT IS A 1X COLOR VERSION W/ NO BACKGROUND ;( BABY STEPS! ITS A START, I GUESS

    //scalemod is evaluated to determine whether another interval should be added.
    let scalemod = _goodyvalnum % 8;

    //intervals is the number of intervals that should appear.
    //This function adjusts it depending on the scalemod (This is that 3+1 stuff from above).
    let mut intervals = _goodyvalnum / 8;
    if scalemod >= 4 {
        intervals = intervals + 1;
        //println!("I added!");
    }

    //This the size of each interval that should appear.
    let goodintervalue = peakpoint / intervals as f32;

    println!(
        "scaleval: {}  \nintervals: {} \ngoodintervalue {}",
        scalemod, intervals, goodintervalue
    );
    /*
    //FOR TESTING, USE A SEPERATE ARRAY FOR EACH COLOR RENDER
    //CONSIDER LIMITING, SCHEMES MIGHT BE USEFUL.

    /////////////////////////////////////////////
    //HERE!
    //TODO ALLOW USER TO SELECT BACKGROUND. IF YES,
    //GENERATE AN ARRAY OF THE SIZE THE datapoints * THE intervals SET WITH THE VALUES OF goodintervalue
    //THIS WILL CREATE HORIZONZAL LINES ACROSS THE LENGTH AT THE goodintervalue IF USING Steps
    //THIS WILL CREATE VERT AND HORZ LINES IF USING Bars
    //MAY NEED TO REMOVE THE SUBSEQUENT (0.0, 0.0)'s AFTER THE INITIAL AT i=ZERO, I CANT REMEMBER..
    let testarray = [
        (0.0, 0.0),
        (1.0, 3.0),
        (2.0, 5.0),
        (3.0, 4.0),
        (4.0, 8.0),
        (5.0, 2.0),
        (6.0, 9.0),
        (7.0, 7.0),
        (8.0, 1.0),
        (9.0, 9.0),
        (10.0, 3.0),
        (0.0, 0.0),
        (1.0, 5.0),
        (2.0, 4.0),
        (3.0, 2.0),
        (4.0, 10.0),
        (5.0, 12.0),
        (6.0, 19.0),
        (7.0, 17.0),
        (8.0, 11.0),
        (9.0, 19.0),
        (10.0, 13.0),
    ];
    */
    if goodcval == "Y" {
        match _upuserinput.as_str() {
            "B" => {
                Chart::new_with_y_range(
                    _goodxvalnum,
                    _goodyvalnum,
                    0.0,
                    _datacounter - 1.0,
                    0.0,
                    peakpoint as f32,
                )
                .linecolorplot(
                    &Shape::Bars(&arrayofpoints),
                    RGB8 {
                        r: arrgeebee[0],
                        g: arrgeebee[1],
                        b: arrgeebee[2],
                    },
                )
                .y_tick_display(TickDisplay::Dense)
                .nice();
            }

            "L" => {
                //TESTING METHOD CHAINING VARIATIONS
                //NOT ALL OF THE METHODS ARE FULLY SUPPORTED, LINESTYLE IN X_STYLE NO, SET AS STATIC
                //SEE https://docs.rs/textplots/0.8.6/src/textplots/lib.rs.html#321
                let mut tryit = Chart::new(_goodxvalnum, _goodyvalnum, 0.0, _datacounter - 1.0);

                //TODO SEPERATION LIKE THIS MIGHT FACILITATE MULTIPLOTS, THINK IT OUT.
                tryit
                    .linecolorplot(
                        &Shape::Lines(&arrayofpoints),
                        RGB8 {
                            r: arrgeebee[0],
                            g: arrgeebee[1],
                            b: arrgeebee[2],
                        },
                    )
                    .y_tick_display(TickDisplay::Dense)
                    .nice();
            }

            "P" => {
                Chart::new(_goodxvalnum, _goodyvalnum, 0.0, _datacounter - 1.0)
                    .linecolorplot(
                        &Shape::Points(&arrayofpoints),
                        RGB8 {
                            r: arrgeebee[0],
                            g: arrgeebee[1],
                            b: arrgeebee[2],
                        },
                    )
                    .y_tick_display(TickDisplay::Dense)
                    .nice();
            }

            "S" => {
                Chart::new(_goodxvalnum, _goodyvalnum, 0.0, _datacounter - 1.0)
                    .linecolorplot(
                        &Shape::Steps(&arrayofpoints),
                        RGB8 {
                            r: arrgeebee[0],
                            g: arrgeebee[1],
                            b: arrgeebee[2],
                        },
                    )
                    .y_tick_display(TickDisplay::Dense)
                    .nice();
            }

            _ => (),
        }
    } else {
        match _upuserinput.as_str() {
            "B" => {
                Chart::new_with_y_range(
                    _goodxvalnum,
                    _goodyvalnum,
                    0.0,
                    _datacounter - 1.0,
                    0.0,
                    peakpoint as f32,
                )
                .lineplot(&Shape::Bars(&arrayofpoints))
                .y_tick_display(TickDisplay::Dense)
                .nice();
            }

            "L" => {
                let mut tryit = Chart::new(_goodxvalnum, _goodyvalnum, 0.0, _datacounter - 1.0);

                tryit
                    .lineplot(&Shape::Lines(&arrayofpoints))
                    .y_tick_display(TickDisplay::Dense)
                    .nice();
            }

            "P" => {
                Chart::new(_goodxvalnum, _goodyvalnum, 0.0, _datacounter - 1.0)
                    .lineplot(&Shape::Points(&arrayofpoints))
                    .y_tick_display(TickDisplay::Dense)
                    .nice();
            }

            "S" => {
                Chart::new(_goodxvalnum, _goodyvalnum, 0.0, _datacounter - 1.0)
                    .lineplot(&Shape::Steps(&arrayofpoints))
                    .y_tick_display(TickDisplay::Dense)
                    .nice();
            }

            _ => (),
        }
    }
}

//////
//////////
/////////////
////////////////
////////////////////
//Func Basement ////////
////////////////////////////

//This function validates the user input string for the prompted graph parameters.
//If the passed-in string contains a non-ascii_digit, return false. Else, true.
fn parsecheck(mut queued: String) -> bool {
    //Iterate over the entire string.
    loop {
        //Check if anything exists.
        //The newline was removed in 'getcapstring()' so any remaining values should be a char type.
        if queued.len() != 0 {
            //Popping is cheaper than remove(0) but requires unwrap for the option type.
            let dpointbyte = queued.pop().unwrap();

            //Check if the char is not an integer.
            if dpointbyte.is_ascii_digit() == false {
                //Found a letter or symbol? Exit the function with false.
                return false;
            }

            //If the previous value was good, then proceed to the next value.
            continue;
        }
        //If NO non-ascii_digit was found, then exit the function with 'true'.
        return true;
    }
}

//This function gets user input from stdin. It passes 'to_uppercase' for match formatting (h != H).
fn getcapstring() -> String {
    //Define the input variable.
    let mut aval = String::new();

    //Get the User input.
    io::stdin().read_line(&mut aval).unwrap();

    //Convert to uppercase for matching.
    aval = aval.to_uppercase();

    //Remove newline
    aval.pop();

    //Return the User input.
    return aval;
}
