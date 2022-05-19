Hello, this is the latest assignment from Boss Man. He told me he wants us to implement something called Conway's Game of Life. Apparently it's some kind of cellular automata (read about it more on the wikipedia [here](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)). Not sure how that's supposed to help our small startup that mainly develops websites for dairy companies, but whatever. I made a stump implementation for you. I would do more but my plane to Haiti is about to leave, so you are on your own.

Here is the list of requirements from Boss Man:

* Two methods that create a new Grid - one with all cells Dead, the other should take a Vec of coordinates of Alive cells. The dimentions of the grid should be variable.
* A mark method that takes coordinates of a cell and changes its state. If the coordinates are invalid, it should fail gracefully.
* A step method that changes the cells according to the rules, or in other words creates a next generation grid.
* A better implementation of the Display trait that prints the grid (what I wrote is almost perfect though).
* An Iterator over the generations of the Grid.
* You cannot use for loops throughout your code. Boss Man said for loops confuse him, so we have to dance around this now I guess.

Anyway, I wish you luck with this. The future of our small startup *that is about to go big and let me cash out on that company stock I have been paid with all this time* depends on you. Make Boss Man proud.