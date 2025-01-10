#include <andes_input.h>

extern void setup();
extern void process();

void I_setup() { setup(); }

void I_process() {
  I_JOY_beforeProcess();
  process();
}