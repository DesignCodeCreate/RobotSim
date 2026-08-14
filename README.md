# A (fairly) complete 2D Robotics simulation (Still heavily under work)

- The model below has a fixed configuration - whilst it is as simple in the code as adding a link, the mathematics of the system cannot be automated as it is hand derived and hard coded.
- The target can be moved around with its two coordinate sliders - and the “analytical” solver can approximate a solution for the system.
- As the system contains redundant components (having more links and joints than it needs in 2D space, the system has infinite solutions to coordinates that it can reach.
- This means in order to find a solution, we must constrain the system further. I have a crude approach (visible in the mathematics showing how no arm can be “d” units away from any other arm, and so this entails an “arc” like configuration)


The numerical solver attempts to find this value of “d” by searching for a value of d that minimizes the error (or difference) between the target and the end effector of the system. It does so in “steps” and which can be edited by the slider. 
The error capacity slider limits how many solutions can be pushed to the solutions array and so naturally the smaller this value gets fewer solutions are appended to this array, yet they are more precise.


You can try it out for yourself by cloning the repo and running the project
