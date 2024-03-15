# rodomopo

Pomodoro timers can be distracting for focused work.

Lower the power that **procrastination** has over you and stop the ruminating **doubts** of whether you worked enough with this **dead simple** inverse po-mo-do-ro timer.

## Why?
This tool provides the same two main benefits that pomodoro timers do:

1. By providing an objective measure of the total amount of work done, it produces the (true!) sensation that you are moving forwards in your project. After a bad day at work, I find that I can safely say to myself "at least I put in the hours" and be a little less frustrated and anxious of whether you should have stayed longer.
2. By having a minimum target for each working session, I have noticed two main benefits:
- If the target is small enough, it helps combat procrastination. This is the core insight behind the pomodoro philosophy. Before starting each working block, I know I *only* need to work for the following, say, 25 minutes.
- It gently pushes you to do at least the minimum amount of work, which usually results in longer, more focused sessions.

### Why not pomodoro then?
Pomodoro timers are good for many things!
However, I hate it when I am in the flow and the timer goes off: it forces me to take a break when I could have been gliding through focused work.

## How?
You could do it all by hand:

1. Set a focused working hours goal for the day, and a minimum duration for the work blocks.
2. Start a stopwatch with your focused work.
3. When you feel like stopping, check if you have completed the minimum duration for the work block.
4. a) if yes, annotate it on a ledger. b) if not, keep working until you hit the minimum.
5. After a break, repeat from 1.

`rodomopo` automates away the annoying need for a watch, pen and paper, and basic arithmetic.

## Installation
Two options:
- Click on *Releases* and download the latest executable.
- Clone this repository and enjoy the source code.

To run `rodomopo`, just write
```
rodomopo
```
in your command line.

### Recommendations
- Make a keybinding for opening a terminal and running `rodomopo`. Logging and checking working time will be one keystroke away.
- Set a reasonable minimum time for working blocks (default, 25 minutes), and a reasonable time for the day's work (default, 3 hours). Remember: this is focused work. None of the email-checking, paper-filling, people-meeting, coffee-taking, window-watching. So 3 hours is likely enough! For this, run ***Coming soon!


## Tech
The program is dead simple. This is what executing `rodomopo` does:
(Note: I used it to learn Rust, so the code might not be very idiomatic. Apologies, rustaceans!)

1. It reads the status of the current timestamp (open or closed) from a one-line file called `status.txt`.
- If the timestamp was closed, it opens it by annotating the starting time. Then, the program is closed.
- If the timestamp was open, it checks if you have worked past the minimum time.
    - If you haven't, it pushes you to keep working.
    - If you have, it adds a timestamp to a file called `timestamps.dat`
2. It prints the current daily progress.

In Linux, both `status.txt` and `timestamps.dat` live in `/home/username/.rodomopo`.

# Roadmap-o
Check TODOS.md
