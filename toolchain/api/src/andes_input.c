#include <andes_input.h>
#include <andes_storage.h>

#include "string.h"

static struct InputDevice gamePads[JOY_MAX_GAMEPADS];
static uint32_t lastButtons[JOY_MAX_GAMEPADS];

bool JOY_getButtonPressed(uint32_t pad, enum GamePadButton button) {
  return (gamePads[pad].state.buttons >> button) & 1;
}

bool JOY_getButtonJustPressed(uint32_t pad, enum GamePadButton button) {
  return ((gamePads[pad].state.buttons >> button) & 1) > ((lastButtons[pad] >> button) & 1);
}

bool JOY_getButtonJustReleased(uint32_t pad, enum GamePadButton button) {
  return ((gamePads[pad].state.buttons >> button) & 1) < ((lastButtons[pad] >> button) & 1);
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
  for (int i = 0; i < JOY_MAX_GAMEPADS; i++) {
    lastButtons[i] = gamePads[i].state.buttons;
  }
  STO_copyRegisterToPtr(&gamePads, REG_GAMEPAD_STATES, 0, sizeof(struct InputDevice) * JOY_MAX_GAMEPADS);
}

void I_JOY_beforeProcess() { I_JOY_transferInputs(); }