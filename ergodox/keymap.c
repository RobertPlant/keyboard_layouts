#include QMK_KEYBOARD_H
#include "keymap_uk.h"

#define KC_MAC_UNDO LGUI(KC_Z)
#define KC_MAC_CUT LGUI(KC_X)
#define KC_MAC_COPY LGUI(KC_C)
#define KC_MAC_PASTE LGUI(KC_V)
#define KC_PC_UNDO LCTL(KC_Z)
#define KC_PC_CUT LCTL(KC_X)
#define KC_PC_COPY LCTL(KC_C)
#define KC_PC_PASTE LCTL(KC_V)
#define ES_LESS_MAC KC_GRAVE
#define ES_GRTR_MAC LSFT(KC_GRAVE)
#define ES_BSLS_MAC ALGR(KC_6)
#define NO_PIPE_ALT KC_GRAVE
#define NO_BSLS_ALT KC_EQUAL
#define LSA_T(kc) MT(MOD_LSFT | MOD_LALT, kc)
#define BP_NDSH_MAC ALGR(KC_8)

enum custom_keycodes {
  RGB_SLD = EZ_SAFE_RANGE,
  STM_GO_ERR,
  STM_JS_DEBUG,
  STM_WPM
};

char wpm[16];

const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
  [0] = LAYOUT_ergodox_pretty(
    KC_ESCAPE,      KC_1,           KC_2,           KC_3,           KC_4,           KC_5,           KC_DELETE,                                      KC_F5,          KC_6,           KC_7,           KC_8,           KC_9,           KC_0,           KC_BSPACE,
    KC_TAB,         KC_Q,           KC_W,           KC_E,           KC_R,           KC_T,           _______,                                        KC_NONUS_HASH,  KC_Y,           KC_U,           KC_I,           KC_O,           KC_P,           _______,       
    KC_LGUI,        KC_A,           KC_S,           KC_D,           KC_F,           KC_G,                                                                           KC_H,           KC_J,           KC_K,           KC_L,           KC_SCOLON,      KC_QUOTE,
    KC_LSHIFT,      KC_NONUS_BSLASH,KC_Z,           KC_X,           KC_C,           KC_V,           _______,                                        KC_SCOLON,      KC_B,           KC_N,           KC_M,           KC_COMMA,       KC_DOT,         KC_SLASH,
    KC_LCTRL,       MO(3),          MO(2),          MO(1),          KC_LALT,                                                                                                        MO(2),          KC_UNDS,        KC_PMNS,        KC_EQUAL,        KC_RCTRL,
                                                                                                    _______,        _______,        KC_MPLY,        KC_MNXT,
                                                                                                                    KC_HOME,        KC_PGUP,
                                                                                    KC_SPACE,       KC_BSPACE,      KC_END,         KC_PGDOWN,      KC_TAB,         KC_ENTER
  ),
  [1] = LAYOUT_ergodox_pretty(
    KC_ESCAPE,      KC_F1,          KC_F2,          KC_F3,          KC_F4,          KC_F5,          _______,                                        _______,        KC_F6,          KC_F7,          KC_F8,          KC_MINUS,       KC_EQUAL,       KC_DELETE,
    _______,        KC_EXLM,        KC_AT,          KC_LCBR,        KC_RCBR,        KC_PIPE,        _______,                                        _______,        _______,        _______,        _______,        _______,        KC_ASTR,        _______,
    _______,        KC_HASH,        KC_DLR,         KC_LPRN,        KC_RPRN,        KC_GRAVE,                                                                       KC_LEFT,        KC_DOWN,        KC_UP,          KC_RIGHT,       KC_PLUS,        _______,       
    _______,        KC_PERC,        KC_CIRC,        KC_LBRACKET,    KC_RBRACKET,    KC_TILD,        _______,                                        _______,        KC_AMPR,        _______,        _______,        _______,        KC_BSLASH,      _______,       
    _______,        KC_EQUAL,       _______,        _______,        _______,                                                                                                        _______,        _______,        _______,        _______,       _______,       
                                                                                                    _______,        _______,        _______,        _______,       
                                                                                                                    _______,        _______,       
                                                                                    _______,        _______,        _______,        _______,        _______,        _______
  ),
  [2] = LAYOUT_ergodox_pretty(
    _______,        _______,        _______,        KC_MS_WH_UP,    _______,        _______,        _______,                                        _______,        _______,        _______,        _______,        _______,        _______,        KC_DELETE,       
    _______,        _______,        KC_MS_UP,       KC_MS_WH_DOWN,  _______,        _______,        RESET,                                          _______,        _______,        KC_7,           KC_8,           KC_8,           _______,        _______,       
    _______,        KC_MS_LEFT,     KC_MS_DOWN,     KC_MS_RIGHT,    _______,        _______,                                                                        _______,        KC_4,           KC_5,           KC_6,           _______,        _______,
    _______,        _______,        _______,        _______,        _______,        _______,        _______,                                        _______,        _______,        KC_1,           KC_2,           KC_3,           _______,        _______,       
    _______,        _______,        _______,        KC_MS_BTN1,     KC_MS_BTN2,                                                                                                     KC_0,           _______,        _______,        _______,        _______,       
                                                                                                    _______,        _______,        _______,        _______,
                                                                                                                    _______,        _______,       
                                                                                    _______,        _______,        _______,        _______,        _______,        KC_WWW_BACK
  ),
  [3] = LAYOUT_ergodox_pretty(
    _______,        _______,        _______,        _______,        _______,        _______,        _______,                                        _______,        _______,        _______,        _______,        _______,        _______,        _______,       
    _______,        STM_GO_ERR,     STM_WPM,        _______,        _______,        _______,        _______,                                        _______,        _______,        _______,        _______,        _______,        _______,        _______,       
    _______,        _______,        _______,        STM_JS_DEBUG,   _______,        _______,                                                                        _______,        _______,        _______,        _______,        _______,        _______,       
    _______,        _______,        _______,        _______,        _______,        _______,        _______,                                        _______,        _______,        _______,        _______,        _______,        _______,        _______,       
    _______,        _______,        _______,        _______,        _______,                                                                                                        _______,        _______,        _______,        _______,        _______,       
                                                                                                    _______,        _______,        RESET,          _______,       
                                                                                                                    _______,        _______,       
                                                                                    _______,        _______,        _______,        _______,        _______,        _______
  ),
};

bool process_record_user(uint16_t keycode, keyrecord_t *record) {
  switch (keycode) {
    case STM_GO_ERR:
		if (record->event.pressed) {
			SEND_STRING("if err != nil {\n\t\n}" SS_TAP(X_UP));
		}
		break;
    case STM_JS_DEBUG:
		if (record->event.pressed) {
			SEND_STRING("^ywoconsole.log('" SS_TAP(X_ESCAPE) "pa');" SS_TAP(X_ESCAPE));
		}
		break;
	case STM_WPM:
		if (record->event.pressed) {
            sprintf(wpm, "%03d", get_current_wpm());
			send_string(wpm);
			
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
