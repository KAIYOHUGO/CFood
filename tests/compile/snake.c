
type Snake_Node = (int, int, int);

int width = 24;
int height = 16;
int seed = 1234567;

int snake_head = 0;
int food_x = 10;
int food_y = 8;

int dir_x = 1;
int dir_y = 0;
int score = 0;
bool game_over = false;

int key_w = 119;
int key_a = 97;
int key_s = 115;
int key_d = 100;
int key_q = 113;

int node_x(int node) {
  return _node_x (node as &Snake_Node);
}

int _node_x(int x, int y, int next) {
  return x;
}

int node_y(int node) {
  return _node_y (node as &Snake_Node);
}

int _node_y(int x, int y, int next) {
  return y;
}

int node_next(int node) {
  return _node_next (node as &Snake_Node);
}

int _node_next(int x, int y, int next) {
  return next;
}

int make_node(int x, int y, int next) {
  return new x y next;
}

bool eq_int(int a, int b) {
  return a == b;
}

bool inside(int x, int y) {
  return x >= 0 && x < width && y >= 0 && y < height;
}

int rand_next() {
  seed = seed * 1103515245 + 12345;
  if (seed < 0) seed = -seed;
  return seed;
}

bool snake_contains(int node, int x, int y) {
  if (node == 0) return false;

  if (eq_int (node_x node) x && eq_int (node_y node) y) return true;

  return snake_contains (node_next node) x y;
}

bool snake_contains_except_tail(int node, int x, int y) {
  if (node == 0) return false;
  if (node_next node == 0) return false;

  if (eq_int (node_x node) x && eq_int (node_y node) y) return true;

  return snake_contains_except_tail (node_next node) x y;
}

int snake_copy(int node) {
  if (node == 0) return 0;

  return make_node
    (node_x node)
    (node_y node)
    (snake_copy (node_next node));
}

int snake_without_last(int node) {
  int x = 0;
  int y = 0;
  int next = 0;

  if (node == 0) return 0;

  x = node_x node;
  y = node_y node;
  next = node_next node;

  if (next == 0) return 0;

  return make_node x y (snake_without_last next);
}

int snake_move_to(int node, int next_x, int next_y) {
  return make_node next_x next_y (snake_without_last node);
}

int snake_grow_to(int node, int next_x, int next_y) {
  return make_node next_x next_y (snake_copy node);
}

void place_food() {
  int next_x = 0;
  int next_y = 0;

  while (true) {
    next_x = rand_next() % width;
    next_y = rand_next() % height;

    if (!snake_contains snake_head next_x next_y) {
      food_x = next_x;
      food_y = next_y;
      return;
    }
  }
}

void free_snake_nodes(int node) {
  if (node == 0) return;
  free_snake_nodes (node_next node);
  delete node;
}

void free_snake() {
  free_snake_nodes snake_head;
  snake_head = 0;
}

void init_game() {
  free_snake;

  snake_head =
    make_node 6 8
      (make_node 5 8
        (make_node 4 8 0));

  dir_x = 1;
  dir_y = 0;
  score = 0;
  game_over = false;
  place_food;
}

void clear_screen() {
  printf("\x1b[2J");
  printf("\x1b[H");
}

void color_reset() {
  printf("\x1b[0m");
}

void color_wall() {
  printf("\x1b[38;5;245m");
}

void color_head() {
  printf("\x1b[38;5;46m");
}

void color_body() {
  printf("\x1b[38;5;34m");
}

void color_food() {
  printf("\x1b[38;5;196m");
}

void color_text() {
  printf("\x1b[38;5;229m");
}

void draw_cell(int x, int y) {
  if (x == -1 || x == width || y == -1 || y == height) {
    color_wall;
    printf("##");
    return;
  }

  if (eq_int x food_x && eq_int y food_y) {
    color_food;
    printf("**");
    return;
  }

  if (snake_head != 0 &&
      eq_int x (node_x snake_head) &&
      eq_int y (node_y snake_head)) {
    color_head;
    printf("@@");
    return;
  }

  if (snake_contains snake_head x y) {
    color_body;
    printf("oo");
    return;
  }

  printf("  ");
}

void draw_board() {
  int y = -1;

  clear_screen;
  color_text;
  printf("score: %d\r\n", score);
  printf("wasd move, q quit\r\n");

  while (y <= height) {
    int x = -1;

    while (x <= width) {
      draw_cell x y;
      x = x + 1;
    }

    color_reset;
    printf("\r\n");
    y = y + 1;
  }

  color_reset;
}

void set_direction(int ch) {
  if (ch == key_w && dir_y != 1) {
    dir_x = 0;
    dir_y = -1;
    return;
  }

  if (ch == key_s && dir_y != -1) {
    dir_x = 0;
    dir_y = 1;
    return;
  }

  if (ch == key_a && dir_x != 1) {
    dir_x = -1;
    dir_y = 0;
    return;
  }

  if (ch == key_d && dir_x != -1) {
    dir_x = 1;
    dir_y = 0;
    return;
  }

  if (ch == key_q) {
    game_over = true;
    return;
  }
}

void step_game() {
  int next_x = 0;
  int next_y = 0;
  bool eat = false;
  int old_snake = 0;
  int new_snake = 0;

  next_x = node_x snake_head + dir_x;
  next_y = node_y snake_head + dir_y;
  eat = eq_int next_x food_x && eq_int next_y food_y;

  if (!inside next_x next_y) {
    game_over = true;
    return;
  }

  if (eat) {
    if (snake_contains snake_head next_x next_y) {
      game_over = true;
      return;
    }

    old_snake = snake_head;
    new_snake = snake_grow_to snake_head next_x next_y;
    snake_head = new_snake;
    score = score + 1;
    place_food;
    free_snake_nodes old_snake;
    return;
  }

  if (snake_contains_except_tail snake_head next_x next_y) {
    game_over = true;
    return;
  }

  old_snake = snake_head;
  new_snake = snake_move_to snake_head next_x next_y;
  snake_head = new_snake;
  free_snake_nodes old_snake;
}

void game_loop() {
  int ch = 0;

  while (!game_over) {
    draw_board;

    if (scanf("%c", &ch) != 1) {
      game_over = true;
      return;
    }

    set_direction ch;
    if (game_over) return;

    step_game;
  }
}

void show_game_over() {
  color_text;
  printf("game over, score = %d\r\n", score);
  color_reset;
}

int main() {
  enable_raw_mode;
  init_game;
  game_loop;
  draw_board;
  show_game_over;
  free_snake;
  disable_raw_mode;
  return 0;
}
