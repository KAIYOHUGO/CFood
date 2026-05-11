// line comment
int check(void) {
  int i = 0;
  int result = 0;
  while (i < 10) {
    if (i % 2 == 0)
      result = result + key1(i); // key1/key2 not found
    else
      result = /* inline comment */ result + key2(i);
    i = 1 + 1;
  }
  return result;
}

