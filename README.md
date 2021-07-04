# Aquarium Boss
Run an aquarium! Feed sharks! What could go wrong? Rust port of a C++ command line game I had coded previously.

##### Why did this interest me?
<blockquote>
The original C++ game has sentimental value to me--I coded it for an introductory course after failing the previous assignment (hello seg fault during the demo!). It was the last term of a year where professors and teaching assistants alike seemed intent on scaring me out of engineering with comments like, "this is NOT hard" and, "well, people are either naturals at programming, or they aren't." I inferred their meanings loud and clear: if it was hard, I wasn't a natural and had no place there.
<br/>
<br/>
There was only one thing left to do: <b>make things weird</b>. This game was the first coding assignment I had ever finished early. I had time to build in meerkat rebellions, animal-naming functions, monkey escapes in response to overcrowding, and vaccination campaigns (the latter was a response to measles outbreaks in 2019, not the thing that happened a year later).
<br/>
<br/>
I didn't get extra credit and nobody actually played it, but I was happy. That wasn't a feeling I had ever associated with programming before, and I hold onto that spirit when things get hard now. I keep several aquariums at home, so it was a no-brainer to do an undersea theme for the reboot of the game.
</blockquote>

##### What did I hope to learn?
<blockquote>There isn't one easy way to port code from C++ to Rust. Partial transpilers from C do exist, but ultimately, this project is more like copying a painting than following a recipe. I had only ever used Rust for a class project involving multi-threaded race conditions... which was informative but dry. I wanted to see how Rust's memory-safe approach made me feel about a program with a lot of moving parts and randomly-triggered events--the same scenarios that used to give me valgrind headaches searching for memory leaks.
</blockquote>

##### How did it go?
<blockquote>I knew Rust would be different, but I didn't know it was THAT different. However, I was delighted to use the needed retrofits as a learning experience:
1. <b>OOP went out the window.</b> My original game used "normal" C++ object-oriented programming: lots of instances of classes (in this case Zoo.cpp, a header file, Animal.cpp, a header file, Meerkat.cpp, and so on.), and methods specific to those classes. This is an encapsulation ethos which Rust happily turns on its head, opting for a library of traits that has no simple correlate to other programming languages I've learned. For example, I wanted to use the equivalent of a getter function to retrieve and print my aquarium's bank balance, but... without my
aquarium being a class, setters and getters also don't exist. Where do I store the bank balance, if not with an
instance of aquarium? I ended up making a mutable variable in main() and scratching my head.
2. <b>Cargo and crates and... what?</b> The only other Rust program I created did not require use of the signature package manager, "Cargo," so I had a rude awakening here. The first time I tried to compile, I found that I hadn't declared dependencies in my Cargo.toml file, so I couldn't use the random number generator crate (library).
3. <b>No more playing loosey goosey with imports.</b> I'd developed a bad habit as a frazzled student to just import a whole library even when only one function is required. It's just so easy! Well, in C and C++ it was, anyway. Rust brought the hammer down hard, and it put the nail in my crate, to mix more metaphors than is comprehensible.
</blockquote>

##### How could this be improved?
<blockquote>Still need to build out the gameplay!</blockquote>

##### Much love to the Monterey Bay Aquarium
<blockquote>Thank you for providing me decades of inspiration.</blockquote>
- Heather
7/4/21
