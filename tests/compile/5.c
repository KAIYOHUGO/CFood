float a = 2.0;
float b = 3.0;
// ctor magic
float r = a ## b;
int main(void) {
  printf("a = %f, b = %f\n", a, b);
  printf("a ## b = %f\n", r);
}

