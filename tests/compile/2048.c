type Cell = (int, int);

int board_w = 4;
int board_h = 4;
int seed = 24681357;

int board = 0;
int score = 0;
bool game_over = false;
bool win_flag = false;

int key_w = 119;
int key_a = 97;
int key_s = 115;
int key_d = 100;
int key_r = 114;
int key_q = 113;

int cell_value(int node) {
  return _cell_value (node as &Cell);
}

int _cell_value(int value, int next) {
  return value;
}

int cell_next(int node) {
  return _cell_next (node as &Cell);
}

int _cell_next(int value, int next) {
  return next;
}

int make_cell(int value, int next) {
  return new value next;
}

int rand_next() {
  seed = seed * 1103515245 + 12345;
  if (seed < 0) seed = -seed;
  return seed;
}

void clear_screen() {
  printf("\x1b[2J");
  printf("\x1b[H");
}

void color_reset() {
  printf("\x1b[0m");
}

void color_text() {
  printf("\x1b[38;5;230m");
}

void color_dim() {
  printf("\x1b[38;5;243m");
}

void color_border() {
  printf("\x1b[38;5;137m");
}

void color_value(int n) {
  if (n == 0) {
    printf("\x1b[38;5;238m");
    return;
  }

  if (n == 2) {
    printf("\x1b[38;5;230m\x1b[48;5;236m");
    return;
  }

  if (n == 4) {
    printf("\x1b[38;5;229m\x1b[48;5;239m");
    return;
  }

  if (n == 8) {
    printf("\x1b[38;5;231m\x1b[48;5;208m");
    return;
  }

  if (n == 16) {
    printf("\x1b[38;5;231m\x1b[48;5;202m");
    return;
  }

  if (n == 32) {
    printf("\x1b[38;5;231m\x1b[48;5;196m");
    return;
  }

  if (n == 64) {
    printf("\x1b[38;5;231m\x1b[48;5;160m");
    return;
  }

  if (n == 128) {
    printf("\x1b[38;5;232m\x1b[48;5;220m");
    return;
  }

  if (n == 256) {
    printf("\x1b[38;5;232m\x1b[48;5;214m");
    return;
  }

  if (n == 512) {
    printf("\x1b[38;5;232m\x1b[48;5;208m");
    return;
  }

  if (n == 1024) {
    printf("\x1b[38;5;231m\x1b[48;5;166m");
    return;
  }

  if (n == 2048) {
    printf("\x1b[38;5;226m\x1b[48;5;52m");
    return;
  }

  printf("\x1b[38;5;231m\x1b[48;5;88m");
}

int make_zeroes(int n) {
  if (n <= 0) return 0;
  return make_cell 0 (make_zeroes (n - 1));
}

int copy_cells(int node) {
  if (node == 0) return 0;

  return make_cell
    (cell_value node)
    (copy_cells (cell_next node));
}

void free_cells(int node) {
  if (node == 0) return;
  free_cells (cell_next node);
  delete node;
}

int nth_value(int node, int index) {
  if (node == 0) return 0;
  if (index == 0) return cell_value node;
  return nth_value (cell_next node) (index - 1);
}

int set_nth_copy(int node, int index, int value) {
  if (node == 0) return 0;

  if (index == 0) {
    return make_cell value (copy_cells (cell_next node));
  }

  return make_cell
    (cell_value node)
    (set_nth_copy (cell_next node) (index - 1) value);
}

int board_index(int x, int y) {
  return y * board_w + x;
}

int board_get(int x, int y) {
  return nth_value board (board_index x y);
}

void board_set(int x, int y, int value) {
  int old_board = 0;

  old_board = board;
  board = set_nth_copy board (board_index x y) value;
  free_cells old_board;
}

void reset_board() {
  if (board != 0) {
    free_cells board;
  }
  board = make_zeroes (board_w * board_h);
}

int count_empty_nodes(int node) {
  if (node == 0) return 0;

  if (cell_value node == 0) {
    return 1 + count_empty_nodes (cell_next node);
  }

  return count_empty_nodes (cell_next node);
}

int count_empty() {
  return count_empty_nodes board;
}

int place_nth_empty(int node, int target, int value) {
  if (node == 0) return 0;

  if (cell_value node == 0) {
    if (target == 0) {
      return make_cell value (copy_cells (cell_next node));
    }

    return make_cell
      0
      (place_nth_empty (cell_next node) (target - 1) value);
  }

  return make_cell
    (cell_value node)
    (place_nth_empty (cell_next node) target value);
}

void spawn_one() {
  int empty = 0;
  int pick = 0;
  int value = 2;
  int old_board = 0;

  empty = count_empty;
  if (empty == 0) return;

  pick = rand_next() % empty;
  if (rand_next() % 10 == 0) value = 4;

  old_board = board;
  board = place_nth_empty board pick value;
  free_cells old_board;
}

void init_game() {
  reset_board;
  score = 0;
  game_over = false;
  win_flag = false;
  spawn_one;
  spawn_one;
}

void print_spaces(int n) {
  int i = 0;
  while (i < n) {
    printf(" ");
    i = i + 1;
  }
}

int digits(int n) {
  if (n < 10) return 1;
  if (n < 100) return 2;
  if (n < 1000) return 3;
  if (n < 10000) return 4;
  if (n < 100000) return 5;
  return 6;
}

void draw_cell_value(int v) {
  int pad = 0;
  int left = 0;
  int right = 0;

  color_value v;

  if (v == 0) {
    printf("      ");
    color_reset;
    return;
  }

  pad = 6 - digits v;
  left = pad / 2;
  right = pad - left;

  print_spaces left;
  printf("%d", v);
  print_spaces right;
  color_reset;
}

void draw_separator() {
  int x = 0;

  color_border;
  printf("+");
  while (x < board_w) {
    printf("------");
    printf("+");
    x = x + 1;
  }
  color_reset;
  printf("\r\n");
}

void draw_board_row(int y) {
  int x = 0;

  color_border;
  printf("|");
  color_reset;

  while (x < board_w) {
    draw_cell_value (board_get x y);
    color_border;
    printf("|");
    color_reset;
    x = x + 1;
  }

  printf("\r\n");
}

void draw_board() {
  int y = 0;

  clear_screen;
  color_text;
  printf("2048   score: %d\r\n", score);
  printf("wasd move, r restart, q quit\r\n");
  if (win_flag) {
    printf("you reached 2048!\r\n");
  }
  color_dim;
  printf("linked-list board version\r\n");
  color_reset;
  printf("\r\n");

  draw_separator;
  while (y < board_h) {
    draw_board_row y;
    draw_separator;
    y = y + 1;
  }

  if (game_over) {
    color_text;
    printf("game over\r\n");
    color_reset;
  }
}

int compact_1(int a, int b, int c, int d) {
  if (a != 0) return a;
  if (b != 0) return b;
  if (c != 0) return c;
  if (d != 0) return d;
  return 0;
}

int compact_2(int a, int b, int c, int d) {
  if (a != 0) return compact_1 b c d 0;
  if (b != 0) return compact_1 c d 0 0;
  if (c != 0) return compact_1 d 0 0 0;
  if (d != 0) return 0;
  return 0;
}

int compact_3(int a, int b, int c, int d) {
  if (a != 0) return compact_2 b c d 0;
  if (b != 0) return compact_2 c d 0 0;
  if (c != 0) return compact_2 d 0 0 0;
  return 0;
}

int compact_4(int a, int b, int c, int d) {
  if (a != 0) return compact_3 b c d 0;
  if (b != 0) return compact_3 c d 0 0;
  if (c != 0) return compact_3 d 0 0 0;
  return 0;
}

int merge_mark(int v) {
  score = score + v;
  if (v == 2048) win_flag = true;
  return v;
}

int merge_1(int a, int b, int c, int d) {
  int p1 = compact_1 a b c d;
  int p2 = compact_2 a b c d;

  if (p1 != 0 && p1 == p2) return merge_mark (p1 + p2);
  return p1;
}

int merge_2(int a, int b, int c, int d) {
  int p1 = compact_1 a b c d;
  int p2 = compact_2 a b c d;
  int p3 = compact_3 a b c d;
  int p4 = compact_4 a b c d;

  if (p1 != 0 && p1 == p2) {
    if (p3 != 0 && p3 == p4) return merge_mark (p3 + p4);
    return p3;
  }

  if (p2 != 0 && p2 == p3) return merge_mark (p2 + p3);
  return p2;
}

int merge_3(int a, int b, int c, int d) {
  int p1 = compact_1 a b c d;
  int p2 = compact_2 a b c d;
  int p3 = compact_3 a b c d;
  int p4 = compact_4 a b c d;

  if (p1 != 0 && p1 == p2) {
    if (p3 != 0 && p3 == p4) return 0;
    return p4;
  }

  if (p2 != 0 && p2 == p3) return p4;
  if (p3 != 0 && p3 == p4) return merge_mark (p3 + p4);
  return p3;
}

int merge_4(int a, int b, int c, int d) {
  int p1 = compact_1 a b c d;
  int p2 = compact_2 a b c d;
  int p3 = compact_3 a b c d;
  int p4 = compact_4 a b c d;

  if (p1 != 0 && p1 == p2) {
    if (p3 != 0 && p3 == p4) return 0;
    return 0;
  }

  if (p2 != 0 && p2 == p3) return 0;
  if (p3 != 0 && p3 == p4) return 0;
  return p4;
}

bool any_equal_horizontal(int x, int y) {
  if (y >= board_h) return false;
  if (x >= board_w - 1) return any_equal_horizontal 0 (y + 1);

  if (board_get x y == board_get (x + 1) y) return true;
  return any_equal_horizontal (x + 1) y;
}

bool any_equal_vertical(int x, int y) {
  if (x >= board_w) return false;
  if (y >= board_h - 1) return any_equal_vertical (x + 1) 0;

  if (board_get x y == board_get x (y + 1)) return true;
  return any_equal_vertical x (y + 1);
}

bool can_move_any() {
  if (count_empty > 0) return true;
  if (any_equal_horizontal 0 0) return true;
  if (any_equal_vertical 0 0) return true;
  return false;
}

bool row_same_left(int y, int a, int b, int c, int d) {
  if (board_get 0 y != a) return false;
  if (board_get 1 y != b) return false;
  if (board_get 2 y != c) return false;
  if (board_get 3 y != d) return false;
  return true;
}

bool row_same_right(int y, int a, int b, int c, int d) {
  if (board_get 3 y != a) return false;
  if (board_get 2 y != b) return false;
  if (board_get 1 y != c) return false;
  if (board_get 0 y != d) return false;
  return true;
}

bool col_same_up(int x, int a, int b, int c, int d) {
  if (board_get x 0 != a) return false;
  if (board_get x 1 != b) return false;
  if (board_get x 2 != c) return false;
  if (board_get x 3 != d) return false;
  return true;
}

bool col_same_down(int x, int a, int b, int c, int d) {
  if (board_get x 3 != a) return false;
  if (board_get x 2 != b) return false;
  if (board_get x 1 != c) return false;
  if (board_get x 0 != d) return false;
  return true;
}

void set_row_left(int y, int a, int b, int c, int d) {
  board_set 0 y a;
  board_set 1 y b;
  board_set 2 y c;
  board_set 3 y d;
}

void set_row_right(int y, int a, int b, int c, int d) {
  board_set 3 y a;
  board_set 2 y b;
  board_set 1 y c;
  board_set 0 y d;
}

void set_col_up(int x, int a, int b, int c, int d) {
  board_set x 0 a;
  board_set x 1 b;
  board_set x 2 c;
  board_set x 3 d;
}

void set_col_down(int x, int a, int b, int c, int d) {
  board_set x 3 a;
  board_set x 2 b;
  board_set x 1 c;
  board_set x 0 d;
}

bool move_left_rows(int y) {
  int v0 = 0;
  int v1 = 0;
  int v2 = 0;
  int v3 = 0;
  int a = 0;
  int b = 0;
  int c = 0;
  int d = 0;
  bool changed = false;

  if (y >= board_h) return false;

  v0 = board_get 0 y;
  v1 = board_get 1 y;
  v2 = board_get 2 y;
  v3 = board_get 3 y;

  a = merge_1 v0 v1 v2 v3;
  b = merge_2 v0 v1 v2 v3;
  c = merge_3 v0 v1 v2 v3;
  d = merge_4 v0 v1 v2 v3;

  if (!row_same_left y a b c d) {
    set_row_left y a b c d;
    changed = true;
  }

  if (move_left_rows (y + 1)) changed = true;
  return changed;
}

bool move_right_rows(int y) {
  int v0 = 0;
  int v1 = 0;
  int v2 = 0;
  int v3 = 0;
  int a = 0;
  int b = 0;
  int c = 0;
  int d = 0;
  bool changed = false;

  if (y >= board_h) return false;

  v0 = board_get 3 y;
  v1 = board_get 2 y;
  v2 = board_get 1 y;
  v3 = board_get 0 y;

  a = merge_1 v0 v1 v2 v3;
  b = merge_2 v0 v1 v2 v3;
  c = merge_3 v0 v1 v2 v3;
  d = merge_4 v0 v1 v2 v3;

  if (!row_same_right y a b c d) {
    set_row_right y a b c d;
    changed = true;
  }

  if (move_right_rows (y + 1)) changed = true;
  return changed;
}

bool move_up_cols(int x) {
  int v0 = 0;
  int v1 = 0;
  int v2 = 0;
  int v3 = 0;
  int a = 0;
  int b = 0;
  int c = 0;
  int d = 0;
  bool changed = false;

  if (x >= board_w) return false;

  v0 = board_get x 0;
  v1 = board_get x 1;
  v2 = board_get x 2;
  v3 = board_get x 3;

  a = merge_1 v0 v1 v2 v3;
  b = merge_2 v0 v1 v2 v3;
  c = merge_3 v0 v1 v2 v3;
  d = merge_4 v0 v1 v2 v3;

  if (!col_same_up x a b c d) {
    set_col_up x a b c d;
    changed = true;
  }

  if (move_up_cols (x + 1)) changed = true;
  return changed;
}

bool move_down_cols(int x) {
  int v0 = 0;
  int v1 = 0;
  int v2 = 0;
  int v3 = 0;
  int a = 0;
  int b = 0;
  int c = 0;
  int d = 0;
  bool changed = false;

  if (x >= board_w) return false;

  v0 = board_get x 3;
  v1 = board_get x 2;
  v2 = board_get x 1;
  v3 = board_get x 0;

  a = merge_1 v0 v1 v2 v3;
  b = merge_2 v0 v1 v2 v3;
  c = merge_3 v0 v1 v2 v3;
  d = merge_4 v0 v1 v2 v3;

  if (!col_same_down x a b c d) {
    set_col_down x a b c d;
    changed = true;
  }

  if (move_down_cols (x + 1)) changed = true;
  return changed;
}

bool move_left() {
  return move_left_rows 0;
}

bool move_right() {
  return move_right_rows 0;
}

bool move_up() {
  return move_up_cols 0;
}

bool move_down() {
  return move_down_cols 0;
}

void after_player_move(bool changed) {
  if (!changed) return;

  spawn_one;

  if (count_empty == 0 && !can_move_any) {
    game_over = true;
  }
}

void handle_input(int ch) {
  bool changed = false;

  if (ch == key_q) {
    game_over = true;
    return;
  }

  if (ch == key_r) {
    init_game;
    return;
  }

  if (game_over) return;

  if (ch == key_a) changed = move_left;
  if (ch == key_d) changed = move_right;
  if (ch == key_w) changed = move_up;
  if (ch == key_s) changed = move_down;

  after_player_move changed;
}

void game_loop() {
  int ch = 0;

  while (true) {
    draw_board;

    if (scanf("%c", &ch) != 1) return;

    handle_input ch;

    if (game_over && ch == key_q) return;
  }
}

int main() {
  enable_raw_mode;
  init_game;
  game_loop;
  draw_board;
  free_cells board;
  color_reset;
  disable_raw_mode;
  return 0;
}
