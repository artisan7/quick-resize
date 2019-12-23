## quick-resize
A CLI program to quickly resize images built using the Rust programming language.<br><br>

### Intallation
1. Make sure you have Cargo installed. (If you haven't installed it yet, see <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html"> Rust and Cargo Installation Instructions </a>)
2. Go to the <i>quick-resize</i> directory using the command line.
3. Run ```cargo build --release``` in the command line.
4. After building, you can find the executable file in <i>quick-resize/target/release/</i>

<b>NOTE</b> : If you want to easily use the program from any directory, make sure you add it to PATH.

### Usage
<b>```quick-resize [COMMAND] [OPTIONS]```</b>

#### COMMANDS
1. <b>```help``` -</b> Display helpful instructions
2. <b>```resize [FLAG] [SIZE] [IMAGE NAME]``` -</b> Resize an image

##### ```resize``` FLAGs
<b>```-s```</b> indicates that [IMAGE NAME] should be resized to [SIZE] bytes.<br>
<b>```-p```</b> indicates that the dimensions of <IMAGE NAME> should be resized by [SIZE]%.<br>
<b>```-h```</b> indicates that the height of [IMAGE NAME] should be resized to [SIZE] pixels.<br>
<b>```-w```</b> indicates that the width of [IMAGE NAME] should be resized to [SIZE] pixels.<br>
			
<b>NOTE</b> : All <b> resize </b> commands preserve the aspect ratio of the image.<br><br>
<b>CAUTION</b> : The program will save the resized version of the image as <i>resize_[IMAGE NAME]</i> and will overwrite any file that has the exact name and file extension.

### CREDITS
Author : Junrick Bation <a href="https://twitter.com/masterbation">@masterbation</a>
