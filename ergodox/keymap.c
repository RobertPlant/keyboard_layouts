#include QMK_KEYBOARD_H
#include "keymap_uk.h"

enum custom_keycodes {
  RGB_SLD = EZ_SAFE_RANGE,
  GO_ERR,
  JS_DEBUG,
  JS_CONST,
  TMUX_LEADER,
  WPM
};

char wpm[16];

// clang-format off
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
  [0] = LAYOUT_ergodox_pretty(
    KC_ESCAPE, KC_1,           KC_2,       KC_3,          KC_4,        KC_5,      KC_DELETE,                       KC_F5,         KC_6,      KC_7,     KC_8,     KC_9,     KC_0,      KC_BSPACE,
    KC_TAB,    KC_Q,           KC_W,       KC_E,          KC_R,        KC_T,      _______,                         KC_NONUS_HASH, KC_Y,      KC_U,     KC_I,     KC_O,     KC_P,      _______,       
    KC_LGUI,   KC_A,           KC_S,       KC_D,          KC_F,        KC_G,                                                      KC_H,      KC_J,     KC_K,     KC_L,     KC_SCOLON, KC_QUOTE,
    KC_LSHIFT, KC_NONUS_BSLASH,KC_Z,       KC_X,          KC_C,        KC_V,      KC_EQUAL,                        KC_SCOLON,     KC_B,      KC_N,     KC_M,     KC_COMMA, KC_DOT,    KC_SLASH,
    KC_LCTRL,  MO(3),          MO(2),      MO(1),         KC_LALT,                                                                           MO(2),    KC_UNDS,  KC_PMNS,  KC_EQUAL,  KC_RCTRL,
                                                                              KC_A, TMUX_LEADER, KC_PLUS,   KC_MNXT,
                                                                                              KC_HOME,  KC_PGUP,
                                                                       KC_SPACE,  KC_BSPACE,  KC_END,   KC_PGDOWN, KC_TAB,        KC_ENTER
  ),
  [1] = LAYOUT_ergodox_pretty(
    KC_ESCAPE, KC_F1,          KC_F2,      KC_F3,         KC_F4,       KC_F5,     _______,                         _______,       KC_F6,     KC_F7,    KC_F8,    KC_F9,    KC_F10,    KC_DELETE,
    _______,   KC_EXLM,        KC_AT,      KC_LCBR,       KC_RCBR,     KC_PIPE,   _______,                         _______,       _______,   _______,  _______,  _______,  KC_ASTR,   _______,
    _______,   KC_HASH,        KC_DLR,     KC_LPRN,       KC_RPRN,     KC_GRAVE,                                                  KC_LEFT,   KC_DOWN,  KC_UP,    KC_RIGHT, KC_PLUS,   _______,       
    _______,   KC_PERC,        KC_CIRC,    KC_LBRACKET,   KC_RBRACKET, KC_TILD,   _______,                         _______,       KC_AMPR,   _______,  _______,  _______,  KC_BSLASH, _______,       
    _______,   KC_EQUAL,       _______,    _______,       _______,                                                                           _______,  _______,  _______,  KC_F11,    KC_F12,
                                                                                  _______,    _______,  _______,   _______,      
                                                                                              _______,  _______,  
                                                                       _______,   KC_DELETE,  _______,  _______,   _______,       _______
  ),
  [2] = LAYOUT_ergodox_pretty(
    _______,   _______,        _______,    KC_MS_WH_UP,   _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   KC_DELETE,       
    _______,   _______,        KC_MS_UP,   KC_MS_WH_DOWN, _______,     _______,   _______,                         _______,       _______,   KC_7,     KC_8,     KC_8,     _______,   _______,       
    _______,   KC_MS_LEFT,     KC_MS_DOWN, KC_MS_RIGHT,   _______,     _______,                                                   _______,   KC_4,     KC_5,     KC_6,     _______,   _______,
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   KC_1,     KC_2,     KC_3,     _______,   _______,       
    _______,   _______,        _______,    KC_MS_BTN1,    KC_MS_BTN2,                                                                        KC_0,     _______,  _______,  _______,   _______,       
                                                                                  _______,    _______,  _______,   _______,
                                                                                              _______,  _______,  
                                                                       _______,   _______,    _______,  _______,   _______,       KC_WWW_BACK
  ),
  [3] = LAYOUT_ergodox_pretty(
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,       
    _______,   GO_ERR,         WPM,        _______,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,
    _______,   JS_CONST,       _______,    JS_DEBUG,      _______,     _______,                                                   _______,   _______,  _______,  _______,  _______,   _______,
    _______,   _______,        _______,    _______,       _______,     _______,   _______,                         _______,       _______,   _______,  _______,  _______,  _______,   _______,       
    _______,   _______,        _______,    _______,       _______,                                                                           _______,  _______,  _______,  _______,   _______,       
                                                                                  _______,  _______,     RESET,    _______,       
                                                                                            _______,     _______,    
                                                                       _______,   _______,  _______,     _______,  _______,       _______
  ),
};
// clang-format on

bool process_record_user(uint16_t keycode, keyrecord_t *record) {
  switch (keycode) {
  /**
   * Example go snippet for an error
   */
  case GO_ERR:
    if (record->event.pressed) {
      SEND_STRING("if err != nil {\n\t\n}" SS_TAP(X_UP));
    }
    break;

  /**
   * Takes the function name and logs it for some hacky debugging.
   * Accepts named functions only:
   *
   * eg:
   * exec() {
   * }
   *
   * will change to insert
   * exec() {
   *   console.log('exec');
   * }
   **/
  case JS_DEBUG:
    if (record->event.pressed) {
      SEND_STRING(
          "^ywoconsole.log('" SS_TAP(X_ESCAPE) "pa');" SS_TAP(X_ESCAPE));
    }
    break;

  /**
   * Takes the function name and logs it for some hacky debugging.
   * Accepts const anonmymous functions only:
   *
   * eg:
   * const exec = () => {
   * }
   *
   * will change to insert
   * const exec = () => {
   *   console.log('exec');
   * }
   **/
  case JS_CONST:
    if (record->event.pressed) {
      SEND_STRING(
          "^t=yboconsole.log('" SS_TAP(X_ESCAPE) "pa');" SS_TAP(X_ESCAPE));
    }
    break;

  /**
   * Prints out the current WPM since we don't have a screen to display it
   **/
  case WPM:
    if (record->event.pressed) {
      sprintf(wpm, "%03d", get_current_wpm());
      send_string(wpm);
    }
    break;

  /**
   * Send the Tmux leader
   **/
  case TMUX_LEADER:
    if (record->event.pressed) {
      SEND_STRING(SS_LCTL("a"));
    }
    break;
  }

  return true;
}

uint32_t layer_state_set_user(uint32_t state) {
  uint8_t layer = biton32(state);
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
  case 4:
    ergodox_right_led_1_on();
    ergodox_right_led_2_on();
    break;
  case 5:
    ergodox_right_led_1_on();
    ergodox_right_led_3_on();
    break;
  case 6:
    ergodox_right_led_2_on();
    ergodox_right_led_3_on();
    break;
  case 7:
    ergodox_right_led_1_on();
    ergodox_right_led_2_on();
    ergodox_right_led_3_on();
    break;
  default:
    break;
  }
  return state;
};
