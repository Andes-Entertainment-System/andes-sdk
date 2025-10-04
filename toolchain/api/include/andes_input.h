#include <stdbool.h>
#include <stdint.h>

#define JOY_MAX_GAMEPADS 2

enum InputDeviceType { DISCONNECTED, UNKNOWN, DUALSHOCK_3, DUALSHOCK_4 };
typedef enum InputDeviceType InputDeviceType;

struct InputDevice {
  InputDeviceType type;
  uint8_t address;
  struct {
    uint32_t buttons;
    int16_t leftStickX;
    int16_t leftStickY;
    int16_t rightStickX;
    int16_t rightStickY;
    int16_t leftTrigger;
    int16_t rightTrigger;
  } state;
};
typedef struct InputDevice InputDevice;

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
typedef enum GamePadButton GamePadButton;

enum GamePadStick {
  STICK_LEFT,
  STICK_RIGHT,
};
typedef enum GamePadStick GamePadStick;

enum GamePadTrigger {
  TRIGGER_LEFT,
  TRIGGER_RIGHT,
};
typedef enum GamePadTrigger GamePadTrigger;

bool JOY_getButtonPressed(uint32_t pad, GamePadButton button);
bool JOY_getButtonJustPressed(uint32_t pad, GamePadButton button);
bool JOY_getButtonJustReleased(uint32_t pad, GamePadButton button);
int16_t JOY_getTrigger(uint32_t pad, GamePadTrigger trigger);
int16_t JOY_getStickX(uint32_t pad, GamePadStick stick);
int16_t JOY_getStickY(uint32_t pad, GamePadStick stick);

void I_JOY_transferInputs();

void I_JOY_beforeProcess();
