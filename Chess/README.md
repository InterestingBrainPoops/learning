# GarbanzoBean  
## What am I?  
I am not a toaster, and I am a chess engine made 100% in rust.  
  
## Structure
There are 2 parts so far:  
 - Board  
   - Chess board implementation, allows for make/unmake move.  
 - Minimax  
   - Uses the Chess board implementation, and traverses a MiniMax game tree to determine the value of a node.  
   - Function to get best move at a given state, essentially a wrapper around the Minimax() function.  