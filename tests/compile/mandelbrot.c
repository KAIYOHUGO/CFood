type Complex = (float, float);

float real_part(float r, float _) { return r; }
float image_part(float _, float i) { return i; }

Complex complex_mul(Complex a, Complex b) {
  return (real_part(a) * real_part(b) - image_part(a) * image_part(b),
          real_part(a) * image_part(b) + image_part(a) * real_part(b));
}
Complex complex_add(Complex a, Complex b) {
  return (real_part(a) + real_part(b), image_part(a) + image_part(b));
}

float complex_abs_square(Complex a) {
  return real_part(a) * real_part(a) + image_part(a) * image_part(a);
}

int main(void) {
  int width = 50;
  int height = 40;
  int max_iter = 100;

  for (int y = 0; y < height; y = y + 1) {
    for (int x = 0; x < width; x = x + 1) {
      float cx = (x as float - width as float / 2.0) * 3.5 / width as float;
      float cy = (y as float - height as float / 2.0) * 2.0 / height as float;
      let c = (cx, cy);

      let z = (0.0, 0.0);
      int iter = 0;

      while (complex_abs_square(z) <= 4.0 && iter < max_iter) {
        z = complex_add(complex_mul(z, z), c);
        iter = iter + 1;
      }

      int color;
      if (iter == max_iter) {
        color = 16;
      } else {
        color = 17 + (iter * 214 / max_iter);
        if (color > 231)
          color = 231;
      }

      printf("\x1b[48;5;%dm  \x1b[0m", color);
    }
    printf("\n");
  }

  return 0;
}
