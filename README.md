## SNES Paint
We gotta figure out how to structure this project.

I think:
 - app.rs handles anything window and input related. Heavy use of eframe
 - color.rs handles color stuff: we got color math to do to go from rgb888->bgr555
 - paint.rs handles canvas and palette state.