type A = (int, int, int);

int main() {
  let a = new 10 20 30;
  printf("%d %d %d", a as &A);
  delete a;
  return 0;
}
