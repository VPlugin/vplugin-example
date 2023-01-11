#include <stdint.h>
#include <stdio.h>
#include <assert.h>
#include <GL/glew.h>

#define COLOR_INCREMENTOR 0.05

static inline void print_log(const char* message) {
     fprintf(stdout, "vplugin-example-plugin: %s\n", message);
}

static float color_amount_red   = 0.0f;
static float color_amount_green = 0.0f;
static float color_amount_blue  = 0.0f;

int vplugin_example_init() {
     print_log("Hello from the plugin!");
     assert(glewInit() == GLEW_OK);
     int32_t glMajor = 0;
     glGetIntegerv(GL_MAJOR_VERSION, &glMajor);
     if (glMajor < 3) {
          print_log("OpenGL version is not new enough!");
          return -1;
     }

     return 0;
}

void vplugin_example_on_redraw() {
          if (color_amount_red < 0.99f) color_amount_red += COLOR_INCREMENTOR;
          else color_amount_red = 0;

          if (color_amount_green < 0.99f) color_amount_green += COLOR_INCREMENTOR;
          else color_amount_green = 0;
          
          if (color_amount_blue < 0.99f) color_amount_blue += COLOR_INCREMENTOR;
          else color_amount_blue = 0.0f;

          glClearColor(color_amount_red, color_amount_green, color_amount_blue, 0.0f);
          glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

void vplugin_exit() {
     print_log("Goodbye! Plugin was requested to be removed.");
     return;
}
