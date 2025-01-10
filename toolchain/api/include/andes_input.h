#include <stdbool.h>
#include <stdint.h>

enum InputDeviceType { DISCONNECTED, UNKNOWN, DUALSHOCK_4 };

struct InputDevice {
  enum InputDeviceType type;
  uint8_t address;
  struct {
    uint32_t buttons;
    uint8_t leftStickX;
    uint8_t leftStickY;
    uint8_t rightStickX;
    uint8_t rightStickY;
    uint8_t leftTrigger;
    uint8_t rightTrigger;
    uint16_t gyroX;
    uint16_t gyroY;
    uint16_t gyroZ;
    uint16_t accelX;
    uint16_t accelY;
    uint16_t accelZ;
  } state;
};

enum GamePadButton {
  BUTTON_X,
  BUTTON_A,
  BUTTON_B,
  BUTTON_Y,
  BUTTON_LB,
  BUTTON_RB,
  BUTTON_LT,
  BUTTON_RT,
  BUTTON_SELECT,
  BUTTON_START,
  BUTTON_LSB,
  BUTTON_RSB,
  BUTTON_DPADUP,
  BUTTON_DPADRIGHT,
  BUTTON_DPADDOWN,
  BUTTON_DPADLEFT,
};

enum GamePadStick {
  STICK_LEFT,
  STICK_RIGHT,
};

enum GamePadTrigger {
  TRIGGER_LEFT,
  TRIGGER_RIGHT,
};

bool JOY_getButtonPressed(uint32_t pad, enum GamePadButton button);
uint8_t JOY_getTrigger(uint32_t pad, enum GamePadTrigger trigger);
uint8_t JOY_getStickX(uint32_t pad, enum GamePadStick stick);
uint8_t JOY_getStickY(uint32_t pad, enum GamePadStick stick);

void I_JOY_transferInputs();

void I_JOY_beforeProcess();
