#include <andes_input.h>
#include <andes_storage.h>

static struct InputDevice gamePads[JOY_MAX_GAMEPADS];

bool JOY_getButtonPressed(uint32_t pad, enum GamePadButton button) {
  return (gamePads[pad].state.buttons >> button) & 1;
}

int16_t JOY_getTrigger(uint32_t pad, enum GamePadTrigger trigger) {
  return trigger == TRIGGER_LEFT ? gamePads[pad].state.leftTrigger : gamePads[pad].state.rightTrigger;
}

int16_t JOY_getStickX(uint32_t pad, enum GamePadStick stick) {
  return stick == STICK_LEFT ? gamePads[pad].state.leftStickX : gamePads[pad].state.rightStickX;
}

int16_t JOY_getStickY(uint32_t pad, enum GamePadStick stick) {
  return stick == STICK_LEFT ? gamePads[pad].state.leftStickY : gamePads[pad].state.rightStickY;
}

void I_JOY_transferInputs() {
  STO_copyRegisterToPtr(&gamePads, REG_GAMEPAD_STATES, 0, sizeof(struct InputDevice) * JOY_MAX_GAMEPADS);
}

void I_JOY_beforeProcess() { I_JOY_transferInputs(); }