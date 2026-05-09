
type Vec = (int, int);

Vec vec_add(int a, int b, int c, int d) {
  return (a + c, b + d);
}

// type: (Vec, Vec) -> Vec -> Vec
// ==> (Vec, Vec, Vec) -> Vec
Vec -> Vec vec_add_3_curry(Vec a, Vec b) {
  return vec_add vec_add (a, b);
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
  // type: (int, int) <=> Vec
  let a = (1, 2);

  // type: (int, int) -> (int, int)
  let add_a = vec_add a;

  let i = 0;
  while(i < 10) {

    if(second add_a (i, i + 1) > 7) {
      return;
    } else {
      // type: float
      let b = 1.1 + 10.2;
      float c = 2;
    }
  }
}
