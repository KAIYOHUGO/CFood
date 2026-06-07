int  main() {
  let i = 1;
  while (i <= 100) {
    if (i % 3 == 0) {
      if (i % 5 == 0) {
        printf("FizzBuzz\n");
      } else {
        printf("Fizz\n");
      }
    } else {
      if (i % 5 == 0) {
        printf("Buzz\n");
      } else {
        printf("Number = %d\n", i);
      }
    }
    i = i + 1;
  }
  return 0;
}
