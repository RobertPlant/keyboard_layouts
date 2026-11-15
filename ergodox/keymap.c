#include QMK_KEYBOARD_H
#include "keymap_uk.h"

enum custom_keycodes {
  TMUX_LEADER = SAFE_RANGE,
};

// clang-format off
// KC_NO = dead switch, needs resoldering: left thumb top-left, key below backspace
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
  [0] = LAYOUT_ergodox_pretty(
    KC_ESCAPE, KC_1,           KC_2,       KC_3,          KC_4,        KC_5,      KC_DELETE,                       KC_F5,         KC_6,      KC_7,     KC_8,     KC_9,     KC_0,      KC_BSPC,
    KC_TAB,    KC_Q,           KC_W,       KC_E,          KC_R,        KC_T,      KC_PMNS,                         KC_NUHS,       KC_Y,      KC_U,     KC_I,     KC_O,     KC_P,      KC_NO,
    KC_LGUI,   KC_A,           KC_S,       KC_D,          KC_F,        KC_G,                                                      KC_H,      KC_J,     KC_K,     KC_L,     KC_SCLN,   KC_QUOTE,
    KC_LSFT,   KC_NUBS,        KC_Z,       KC_X,          KC_C,        KC_V,      KC_EQUAL,                        KC_SCLN,       KC_B,      KC_N,     KC_M,     KC_COMMA, KC_DOT,    KC_SLASH,
    KC_LCTL,   MO(3),          MO(2),      MO(1),         KC_LALT,                                                                           MO(2),    KC_UNDS,  KC_PMNS,  KC_EQUAL,  KC_RCTL,
                                                                           KC_NO, TMUX_LEADER, KC_MPLY,  KC_MNXT,
                                                                                              KC_HOME,  KC_PGUP,
                                                                       KC_SPACE,  KC_BSPC,    KC_END,   KC_PGDN,   KC_TAB,        KC_ENTER
  ),
  [1] = LAYOUT_ergodox_pretty(
    KC_ESCAPE, KC_F1,          KC_F2,      KC_F3,         KC_F4,       KC_F5,     _______,                         _______,       KC_F6,     KC_F7,    KC_F8,    KC_F9,    KC_F10,    KC_DELETE,
    _______,   KC_EXLM,        UK_AT,      KC_LCBR,       KC_RCBR,     UK_PIPE,   _______,                         _______,       _______,   _______,  _______,  _______,  KC_ASTR,   _______,
    _______,   UK_HASH,        KC_DLR,     KC_LPRN,       KC_RPRN,     KC_GRAVE,                                                  KC_LEFT,   KC_DOWN,  KC_UP,    KC_RIGHT, KC_PLUS,   _______,       
    _______,   KC_PERC,        KC_CIRC,    KC_LBRC,       KC_RBRC,     UK_TILD,   _______,                         _______,       KC_AMPR,   _______,  _______,  _______,  UK_BSLS,   _______,       
    _______,   KC_EQUAL,       _______,    _______,       _______,                                                                           _______,  _______,  _______,  KC_F11,    KC_F12,
                                                                                  _______,    _______,  KC_F13,    KC_F14,       
                                                                                              _______,  KC_F15,   
                                                                       _______,   KC_DELETE,  _______,  _______,   KC_INS,        _______
  ),
  [2] = LAYOUT_ergodox_pretty(
    _______,   _______,        _______,    MS_WHLU,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,
    _______,   _______,        MS_UP,      MS_WHLD,       _______,     _______,   _______,                         _______,       _______,   KC_7,     KC_8,     KC_9,     _______,   _______,
    _______,   MS_LEFT,        MS_DOWN,    MS_RGHT,       _______,     _______,                                                   _______,   KC_4,     KC_5,     KC_6,     _______,   _______,
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   KC_1,     KC_2,     KC_3,     _______,   _______,       
    _______,   _______,        _______,    MS_BTN1,       MS_BTN2,                                                                           KC_0,     _______,  _______,  _______,   _______,       
                                                                                  _______,    _______,  _______,   _______,
                                                                                              _______,  _______,  
                                                                       _______,   _______,    _______,  _______,   _______,       KC_WBAK
  ),
  [3] = LAYOUT_ergodox_pretty(
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,       
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,
    _______,   _______,        _______,    _______,       _______,     _______,                                                   _______,   _______,  _______,  _______,  _______,   _______,
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,       
    _______,   _______,        _______,    _______,       _______,                                                                           _______,  _______,  _______,  _______,   _______,       
                                                                                  _______,  _______,     QK_BOOT,  _______,       
                                                                                            _______,     _______,    
                                                                       _______,   _______,  _______,     _______,  _______,       _______
  ),
};
// clang-format on

bool process_record_user(uint16_t keycode, keyrecord_t *record) {
  switch (keycode) {
  case TMUX_LEADER:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL("a"));
    }
    break;
  }

  return true;
}

layer_state_t layer_state_set_user(layer_state_t state) {
  uint8_t layer = get_highest_layer(state);
  ergodox_board_led_off();
  ergodox_right_led_1_off();
  ergodox_right_led_2_off();
  ergodox_right_led_3_off();
  switch (layer) {
  case 1:
    ergodox_right_led_1_on();
    break;
  case 2:
    ergodox_right_led_2_on();
    break;
  case 3:
    ergodox_right_led_3_on();
    break;
  default:
    break;
  }
  return state;
};
