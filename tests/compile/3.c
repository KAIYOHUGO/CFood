int main() {
  for (let i = 1, let b = 1.0; i <= 100; i = i + 1) {
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
        printf("Number = %f\n", i as float);
      }
    }
  }

  return 0;
}
