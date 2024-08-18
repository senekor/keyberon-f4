use keyberon::action::{k, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

macro_rules! hold_tap {
    ($hold:expr, $tap:expr) => {
        HoldTap(&HoldTapAction {
            timeout: 200,
            tap_hold_interval: 0,
            config: HoldTapConfig::Default,
            hold: $hold,
            tap: k($tap),
        })
    };
}

const A_2: Action = hold_tap!(k(LAlt), Kb2);
const C_3: Action = hold_tap!(k(LCtrl), Kb3);
const S_4: Action = hold_tap!(k(LShift), Kb4);
const S_C: Action = hold_tap!(k(LShift), CapsLock);

const MY_PLUS: Action = Action::ModifiedKeyCode(&(&[LShift], Equal));
const MY_AMPRS: Action = Action::ModifiedKeyCode(&(&[LShift], Kb7));

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    { //[+·· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [Escape Q       W       F       P       B       J       L       U       Y     '\'' BSpace],
        [Tab    A       R       S       T       G       M       N       E       I       O   Enter],
        [{S_C}  Z       X       C       D       V       K       H       ,       .       /    RAlt],
        [n      n       n     LCtrl   Space    (2)   LShift    (1)    LGui      n       n       n],
    }{//[+·· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [t      n       $      '{'     '}'      %       @       |      '`'      #     '\''      t],
        [t  {MY_AMPRS}  !      '('     ')'     '_'      ^       =      '"'      ;       :       t],
        [t  {MY_PLUS}   *      '['     ']'    '\\'      ~       -       <       >       ?       t],
        [t      t       t       t       t       t      (3)      t       t       t       t       t],
    }{//[+·· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [t      n     Home     Up      End      n       5       6       7       8       9       t],
        [t    LGui    Left    Down    Right  Delete     0       1     {A_2}   {C_3}   {S_4}     t],
        [t      n     PgUp      n    PgDown     n      '_'      -       ,       .       /       t],
        [t      t       t       t       t       t       t       t       t       t       t       t],
    }{//[+·· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
        [t     F9      F10     F11     F12      n       n       n       n       n       n       n],
        [t     F5      F6      F7      F8       n       n      LGui    LAlt   LCtrl   LShift    t],
        [t     F1      F2      F3      F4       n       n       n       n       n       n       t],
        [t      t       t       t       t       t       t       t       t       t       t       t],
    } //[+·· ···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+··· ···+],
};
