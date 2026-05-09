// define type alias for apply list
type Vec = (int, int);

Vec vec_add(int a, int b, int c, int d) {
  return (a + c, b + d);
}

Vec vec_add_3(Vec a, Vec b, Vec c) {
  return vec_add vec_add (a, b, c);
}

// get first element of Vec
int first(int a, int _) {
  return a;
}

// get second element of Vec
int second(int _, int b) {
  return b;
}

void main(void) {
  Vec a = (1, 2);
  Vec b = (3, 4);
  Vec c = (5, 6);
  int array[2];

  int f = first vec_add_3 (a, b, c);
  array[0] = f;
  array[1] = f;
}
