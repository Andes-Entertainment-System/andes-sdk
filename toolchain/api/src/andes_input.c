#include <andes_input.h>
#include <andes_storage.h>

static struct InputDevice gamePads[4];

bool JOY_getButtonPressed(uint32_t pad, enum GamePadButton button) {
  return (gamePads[pad].state.buttons >> button) & 1;
}

uint8_t JOY_getTrigger(uint32_t pad, enum GamePadTrigger trigger) {
  return trigger == TRIGGER_LEFT ? gamePads[pad].state.leftTrigger : gamePads[pad].state.rightTrigger;
}

uint8_t JOY_getStickX(uint32_t pad, enum GamePadStick stick) {
  return stick == STICK_LEFT ? gamePads[pad].state.leftStickX : gamePads[pad].state.rightStickX;
}

uint8_t JOY_getStickY(uint32_t pad, enum GamePadStick stick) {
  return stick == STICK_LEFT ? gamePads[pad].state.leftStickY : gamePads[pad].state.rightStickY;
}

void I_JOY_transferInputs() { STO_copyRegisterToPtr(&gamePads, REG_GAMEPAD_STATES, 0, sizeof(struct InputDevice) * 4); }

void I_JOY_beforeProcess() { I_JOY_transferInputs(); }